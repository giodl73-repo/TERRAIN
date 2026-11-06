import init, {plan_json} from './pkg/terrain_web.js';
self.onmessage = async ({data}) => {
  const {id, generation} = data;
  try {
    if (data.type === 'init') {
      await init();
      self.postMessage({type:'ready', generation});
    } else if (data.type === 'plan') {
      const started = performance.now();
      const result = JSON.parse(plan_json(data.csv, data.count, data.graph, data.demand, data.revenue));
      self.postMessage({type:'result', id, generation, result, ms:performance.now()-started});
    }
  } catch (error) {
    // Rust validation failures are thrown strings. A trap/JS failure may leave
    // the engine unusable and needs a new worker rather than a warm retry.
    self.postMessage({type:'error', id, generation, message:String(error), fatal:typeof error !== 'string'});
  }
};
