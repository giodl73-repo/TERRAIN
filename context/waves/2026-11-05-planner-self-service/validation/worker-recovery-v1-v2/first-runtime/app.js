import {PlannerSession} from './planner-session.js';
const $ = selector => document.querySelector(selector);
const params = new URLSearchParams(location.search);
const format = value => value === null ? '∞' : value.toLocaleString('en-US', {maximumFractionDigits:2});
let generation = 0, timer, sample, sampleAbort, svgUrl, completed;

function controls() {
  $('#count-value').textContent = $('#count').value;
  $('#demand-value').textContent = Number($('#demand-limit').value).toFixed(2);
  $('#revenue-value').textContent = Number($('#revenue-limit').value).toFixed(2);
}

function clearResult() {
  completed = null;
  $('#export').disabled = true;
  $('#map').hidden = true;
  $('#map').removeAttribute('src');
  if (svgUrl) URL.revokeObjectURL(svgUrl);
  svgUrl = null;
  $('#territories').replaceChildren();
  for (const id of ['demand-ratio','revenue-ratio','radius']) $(`#${id}`).textContent = '—';
  $('#verdict').textContent = 'No current plan';
}

function error(message) {
  clearResult();
  $('#verdict').textContent = 'Check input or retry';
  $('#status').textContent = message;
}

const session = new PlannerSession({
  onState(state) {
    $('#stop').disabled = !['initializing','planning'].includes(state);
    $('#retry').hidden = !['failed','stopped'].includes(state);
    if (state === 'initializing') $('#status').textContent = 'Loading Rust planning engine…';
    if (state === 'planning') $('#status').textContent = 'Planning… You can stop this calculation; the limit is two minutes.';
  },
  onError:error,
  onResult(result, ms, input) {
    completed = {geojson:result.geojson, input, generation};
    $('#status').textContent = `${result.site_count} sites · ${result.territory_count} territories · ${ms.toFixed(1)} ms · processing stays on your device`;
    $('#verdict').textContent = result.passes ? 'Within balance limits' : 'Review balance';
    $('#demand-ratio').textContent = format(result.demand_ratio);
    $('#revenue-ratio').textContent = format(result.revenue_ratio);
    $('#radius').textContent = format(result.radius_degrees);
    svgUrl = URL.createObjectURL(new Blob([result.svg], {type:'image/svg+xml'}));
    $('#map').src = svgUrl;
    $('#map').hidden = false;
    for (const territory of result.summaries) {
      const row = document.createElement('tr');
      for (const value of [territory.id,territory.site_count,format(territory.demand),format(territory.revenue)]) {
        const cell = document.createElement('td');
        cell.textContent = value;
        row.append(cell);
      }
      $('#territories').append(row);
    }
    $('#export').disabled = false;
  }
});

function invalidate(message) {
  generation++;
  clearTimeout(timer);
  session.invalidate();
  clearResult();
  if (message) $('#status').textContent = message;
  return generation;
}

function plan() {
  controls();
  const current = invalidate('Planning…');
  timer = setTimeout(() => {
    if (current !== generation) return;
    const csv = $('#csv').value;
    if (new TextEncoder().encode(csv).length > 500000) {
      error('Use a CSV file up to 500 KB.');
      return;
    }
    session.submit({csv, count:Number($('#count').value), graph:$('#method').value==='graph',
      demand:Number($('#demand-limit').value), revenue:Number($('#revenue-limit').value)});
  }, 100);
}

async function loadSample() {
  const current = invalidate('Loading public sample…');
  if (sample !== undefined) {
    $('#csv').value = sample;
    plan();
    return;
  }
  sampleAbort?.abort();
  const abort = new AbortController();
  sampleAbort = abort;
  const deadline = setTimeout(() => abort.abort(), 10000);
  try {
    const response = await fetch('./sample.csv', {signal:abort.signal});
    if (!response.ok) throw new Error('unavailable');
    const reader = response.body.getReader();
    const chunks = [];
    let bytes = 0;
    while (true) {
      const {value, done} = await reader.read();
      if (done) break;
      bytes += value.byteLength;
      if (bytes > 500000) {
        await reader.cancel();
        throw new Error('oversized');
      }
      chunks.push(value);
    }
    const content = new Uint8Array(bytes);
    let offset = 0;
    for (const chunk of chunks) {content.set(chunk, offset); offset += chunk.byteLength;}
    if (sampleAbort !== abort) return;
    sample = new TextDecoder('utf-8', {fatal:true}).decode(content);
    if (current === generation) {$('#csv').value = sample; plan();}
  } catch {
    if (current === generation) error('The public sample could not load. Choose Load sample to retry, or use your own local CSV.');
  } finally {
    clearTimeout(deadline);
    if (sampleAbort === abort) sampleAbort = null;
  }
}

for (const id of ['count','demand-limit','revenue-limit']) $(`#${id}`).oninput = plan;
$('#method').onchange = plan;
$('#apply').onclick = plan;
$('#csv').oninput = () => invalidate('CSV changed. Choose “Plan these sites” to refresh.');
$('#sample').onclick = loadSample;
$('#stop').onclick = () => {
  invalidate();
  sampleAbort?.abort();
  sampleAbort = null;
  session.stop();
  $('#status').textContent = 'Planning stopped. Your CSV and settings are kept. Choose Retry planner to resume.';
};
$('#retry').onclick = () => {session.stop(); plan();};
$('#file').onchange = async () => {
  const file = $('#file').files[0];
  if (!file) return;
  const current = invalidate('Reading local CSV…');
  if (file.size > 500000) {error('Use a CSV file up to 500 KB.'); return;}
  try {
    const csv = await file.text();
    if (current !== generation) return;
    $('#csv').value = csv;
    plan();
  } catch {
    if (current === generation) error('Could not read this CSV file.');
  }
};
$('#share').onclick = async () => {
  const current = generation;
  const url = new URL(location.href);
  url.search = '';
  for (const [key,id] of [['count','count'],['method','method'],['demand','demand-limit'],['revenue','revenue-limit']]) url.searchParams.set(key,$(`#${id}`).value);
  history.replaceState(null,'',url);
  try {
    await navigator.clipboard.writeText(url.href);
    if (current === generation) $('#status').textContent = 'Settings copied. Shared links load the public sample, not your CSV.';
  } catch {
    if (current === generation) $('#status').textContent = 'Copy settings from the address bar. Shared links load the public sample.';
  }
};
$('#export').onclick = () => {
  if (!completed || completed.generation !== generation) return;
  const url = URL.createObjectURL(new Blob([completed.geojson], {type:'application/geo+json'}));
  const link = document.createElement('a');
  link.href = url;
  link.download = 'terrain-plan.geojson';
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
};
for (const [key,id,min,max] of [['count','count',1,8],['demand','demand-limit',1,3],['revenue','revenue-limit',1,3]]) {
  const value = Number(params.get(key));
  if (params.has(key) && Number.isFinite(value) && value >= min && value <= max && (key !== 'count' || Number.isInteger(value))) $(`#${id}`).value = String(value);
}
if (['greedy','graph'].includes(params.get('method'))) $('#method').value = params.get('method');
controls();
for (const control of document.querySelectorAll('.controls button,.controls input,.controls select,.controls textarea')) control.disabled = false;
session.start();
loadSample();
window.addEventListener('pagehide', () => {
  generation++;
  clearTimeout(timer);
  sampleAbort?.abort();
  session.stop();
  if (svgUrl) URL.revokeObjectURL(svgUrl);
});
