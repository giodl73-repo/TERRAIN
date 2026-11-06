// One live worker and at most one current request; retired generations cannot publish.
export const INITIALIZATION_DEADLINE_MS = 15_000;
export const PLANNING_DEADLINE_MS = 120_000;

export class PlannerSession {
  constructor({onState, onResult, onError}) {
    this.onState = onState;
    this.onResult = onResult;
    this.onError = onError;
    this.generation = 0;
    this.request = 0;
    this.worker = null;
    this.pending = null;
    this.active = null;
    this.timer = null;
    this.state = 'stopped';
  }

  stateChanged(state) {
    this.state = state;
    this.onState(state);
  }

  retire() {
    this.generation++;
    clearTimeout(this.timer);
    this.timer = null;
    if (this.worker) this.worker.terminate();
    this.worker = null;
    this.active = null;
  }

  fail(message) {
    this.pending = null;
    this.retire();
    this.stateChanged('failed');
    this.onError(message);
  }

  start() {
    this.retire();
    const generation = this.generation;
    this.stateChanged('initializing');
    try {
      const worker = new Worker(new URL('./worker.js', import.meta.url), {type:'module'});
      this.worker = worker;
      const current = () => this.worker === worker && this.generation === generation;
      worker.onerror = event => {
        if (!current()) return;
        event.preventDefault();
        this.fail('Planner could not load or run. Choose Retry planner, or inspect the source.');
      };
      worker.onmessageerror = () => {
        if (current()) this.fail('Planner response could not be read. Choose Retry planner.');
      };
      worker.onmessage = ({data}) => {
        if (!current() || data.generation !== generation) return;
        if (data.type === 'ready' && this.state === 'initializing') {
          clearTimeout(this.timer);
          this.timer = null;
          this.stateChanged('ready');
          if (this.pending) this.dispatch();
        } else if (data.type === 'error' && this.state === 'initializing') {
          this.fail('Planner could not load. Choose Retry planner, or inspect the source.');
        } else if (this.active && data.id === this.active.id && this.state === 'planning') {
          if (data.type !== 'result' && data.type !== 'error') return;
          if (data.type === 'error' && data.fatal) {
            this.fail('The planning engine stopped. Choose Retry planner, or use fewer sites.');
            return;
          }
          clearTimeout(this.timer);
          this.timer = null;
          const input = this.active.input;
          this.active = null;
          this.stateChanged('ready');
          if (data.type === 'error') this.onError(data.message);
          else this.onResult(data.result, data.ms, input);
        }
      };
      this.timer = setTimeout(() => {
        if (current()) this.fail('Planner loading timed out after 15 seconds. Choose Retry planner.');
      }, INITIALIZATION_DEADLINE_MS);
      worker.postMessage({type:'init', generation});
    } catch {
      this.fail('Planner could not start. Choose Retry planner, or inspect the source.');
    }
  }

  invalidate() {
    this.pending = null;
    this.request++;
    if (this.active) {
      this.retire();
      this.stateChanged('stopped');
    }
  }

  submit(input) {
    this.invalidate();
    this.pending = Object.freeze({...input});
    if (!this.worker) this.start();
    if (this.state === 'ready') this.dispatch();
  }

  dispatch() {
    if (!this.pending || !this.worker || this.state !== 'ready') return;
    const input = this.pending;
    this.pending = null;
    const id = ++this.request;
    const generation = this.generation;
    this.active = {id, input};
    this.stateChanged('planning');
    this.timer = setTimeout(() => {
      if (this.generation === generation && this.active?.id === id) {
        this.fail('Planning timed out after two minutes. Choose Retry planner or use fewer sites.');
      }
    }, PLANNING_DEADLINE_MS);
    try {
      this.worker.postMessage({type:'plan', id, generation, ...input});
    } catch {
      this.fail('Planner could not accept this request. Choose Retry planner.');
    }
  }

  stop() {
    this.pending = null;
    this.request++;
    this.retire();
    this.stateChanged('stopped');
  }
}
