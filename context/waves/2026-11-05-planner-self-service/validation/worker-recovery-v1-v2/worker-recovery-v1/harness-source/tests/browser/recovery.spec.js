import {test,expect} from '@playwright/test';

async function count(page,value) {
  await page.locator('#count').evaluate((element,value)=>{element.value=String(value);element.dispatchEvent(new Event('input'));},value);
}

async function observeWorkers(page) {
  await page.addInitScript(() => {
    window.__terrainWorkers = [];
    const RealWorker = window.Worker;
    window.Worker = class extends RealWorker {
      constructor(...args) {super(...args); this.sent = []; this.retired = false; window.__terrainWorkers.push(this);}
      postMessage(message,...args) {this.sent.push(structuredClone(message)); return super.postMessage(message,...args);}
      terminate() {this.retired = true; return super.terminate();}
    };
  });
}

async function stall(page, stage) {
  await page.route('**/worker.js', async route => {
    const response = await route.fetch();
    let code = await response.text();
    if (stage === 'init') code = code.replace('await init();','await new Promise(() => {});');
    else code = code.replace('const started = performance.now();','while (true) {}\n      const started = performance.now();');
    await route.fulfill({response,body:code});
  });
}

async function noResult(page) {
  await expect(page.locator('#export')).toBeDisabled();
  await expect(page.locator('#map')).toBeHidden();
  await expect(page.locator('#territories tr')).toHaveCount(0);
  for (const id of ['demand-ratio','revenue-ratio','radius']) await expect(page.locator('#'+id)).toHaveText('—');
}

async function lateReply(page, index=0) {
  await page.evaluate(index => {
    const worker = window.__terrainWorkers[index];
    const init = worker.sent.find(x=>x.type==='init');
    const request = worker.sent.find(x=>x.type==='plan');
    worker.onmessage({data:{type:'ready',generation:init.generation}});
    worker.onmessage({data:{type:'error',generation:init.generation,id:request?.id,message:'LATE ERROR MUST NOT APPEAR'}});
    worker.onmessage({data:{type:'result',generation:init.generation,id:request?.id,ms:0,result:{site_count:999,territory_count:99,geojson:'stale',svg:'stale',summaries:[],passes:true,demand_ratio:1,revenue_ratio:1,radius_degrees:0}}});
  },index);
}

async function liveWorkerBound(page) {
  expect(await page.evaluate(()=>window.__terrainWorkers.filter(w=>!w.retired).length)).toBeLessThanOrEqual(1);
}

test('editing CSV immediately clears every previous result and export',async({page})=>{
  await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  await page.locator('#csv').fill('site_id,demand,revenue,latitude,longitude\nprivate-site,1,1,0,0\n');
  await noResult(page);await expect(page.locator('#status')).toContainText('CSV changed');
  await count(page,1);
  await expect(page.locator('#status')).toContainText('1 sites · 1 territories');
  await expect(page.locator('#export')).toBeEnabled();
});

test('changing settings clears the previous map before the debounce',async({page})=>{
  await page.clock.install();await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  await page.clock.pauseAt(new Date());
  await page.locator('#count').evaluate(e=>{e.value='5';e.dispatchEvent(new Event('input'));});
  await noResult(page);
  await page.clock.resume();await expect(page.locator('#status')).toContainText('18 sites · 5 territories');
});

test('stop terminates real executing WASM worker and retry preserves inputs',async({page})=>{
  await observeWorkers(page);await stall(page,'plan');
  const wasm=page.waitForResponse(r=>r.url().endsWith('.wasm'));
  await page.goto('/TERRAIN/');expect((await wasm).status()).toBe(200);
  await expect(page.locator('#status')).toContainText('limit is two minutes');
  const csv=await page.locator('#csv').inputValue();await page.locator('#stop').click();
  await noResult(page);await expect(page.locator('#retry')).toBeVisible();
  expect(await page.evaluate(()=>window.__terrainWorkers[0].retired)).toBe(true);
  await lateReply(page);await expect(page.locator('#status')).toContainText('Planning stopped');
  await page.unroute('**/worker.js');await page.locator('#retry').click();
  await expect(page.locator('#export')).toBeEnabled();await expect(page.locator('#csv')).toHaveValue(csv);
  await expect(page.locator('#status')).toContainText('18 sites · 3 territories');await liveWorkerBound(page);
});

test('initialization deadline retires hung worker and rejects its late ready',async({page})=>{
  await observeWorkers(page);await page.clock.install();await stall(page,'init');
  await page.goto('/TERRAIN/');
  await expect.poll(()=>page.evaluate(()=>window.__terrainWorkers.length)).toBe(1);
  await page.clock.fastForward(15001);await expect(page.locator('#status')).toContainText('loading timed out');
  await noResult(page);await lateReply(page);await expect(page.locator('#status')).toContainText('loading timed out');
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#export')).toBeEnabled();await liveWorkerBound(page);
});

test('computation deadline can terminate a blocked worker and retry',async({page})=>{
  await observeWorkers(page);await page.clock.install();await stall(page,'plan');await page.goto('/TERRAIN/');
  await expect(page.locator('#status')).toContainText('limit is two minutes');
  await page.clock.fastForward(120001);await expect(page.locator('#status')).toContainText('timed out after two minutes');
  await noResult(page);expect(await page.evaluate(()=>window.__terrainWorkers[0].retired)).toBe(true);
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#export')).toBeEnabled();
  await lateReply(page);await expect(page.locator('#status')).toContainText('18 sites');await liveWorkerBound(page);
});

test('initialization WASM failure retries successfully without reload',async({page})=>{
  await observeWorkers(page);await page.route('**/*.wasm',route=>route.fulfill({status:503,body:'offline'}));await page.goto('/TERRAIN/');
  await expect(page.locator('#retry')).toBeVisible();await noResult(page);
  await page.unroute('**/*.wasm');await page.locator('#retry').click();await expect(page.locator('#export')).toBeEnabled();await liveWorkerBound(page);
});

test('worker crash is recoverable without losing private CSV',async({page})=>{
  await observeWorkers(page);await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  const csv='site_id,demand,revenue,latitude,longitude\nPRIVATE-ONLY,1,1,0,0\n';
  await page.locator('#csv').fill(csv);await count(page,1);await expect(page.locator('#export')).toBeEnabled();
  await page.evaluate(()=>window.__terrainWorkers.at(-1).onerror(new ErrorEvent('error',{message:'injected worker crash',cancelable:true})));
  await noResult(page);await expect(page.locator('#status')).toContainText('could not load or run');
  await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('1 sites · 1 territories');await expect(page.locator('#csv')).toHaveValue(csv);
});

test('unreadable worker response retires current worker and ignores retired errors',async({page})=>{
  await observeWorkers(page);await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  await page.evaluate(()=>window.__terrainWorkers.at(-1).onmessageerror(new MessageEvent('messageerror')));
  await noResult(page);await expect(page.locator('#status')).toContainText('response could not be read');
  await page.locator('#retry').click();await expect(page.locator('#export')).toBeEnabled();await lateReply(page);await expect(page.locator('#status')).toContainText('18 sites');
});

test('rapid new plans keep only one worker and the last input',async({page})=>{
  await observeWorkers(page);await stall(page,'plan');await page.goto('/TERRAIN/');await expect(page.locator('#status')).toContainText('limit is two minutes');
  await page.unroute('**/worker.js');
  await page.locator('#count').evaluate(e=>{for(const value of ['4','5','6','7','8']){e.value=value;e.dispatchEvent(new Event('input'));}});
  await expect(page.locator('#status')).toContainText('18 sites · 8 territories');await expect(page.locator('#territories tr')).toHaveCount(8);
  await lateReply(page);await expect(page.locator('#status')).toContainText('18 sites · 8 territories');await liveWorkerBound(page);
  const counts=await page.evaluate(()=>window.__terrainWorkers.map(w=>w.sent.filter(m=>m.type==='plan').length));expect(Math.max(...counts)).toBe(1);
});

test('sample failure still permits private CSV and sample can retry',async({page})=>{
  await page.route('**/sample.csv',route=>route.fulfill({status:503,body:'unavailable'}));await page.goto('/TERRAIN/');
  await expect(page.locator('#status')).toContainText('Choose Load sample to retry');
  await page.locator('#csv').fill('site_id,demand,revenue,latitude,longitude\nLOCAL,1,1,0,0\n');await count(page,1);await expect(page.locator('#export')).toBeEnabled();
  await page.unroute('**/sample.csv');await page.locator('#sample').click();await expect(page.locator('#status')).toContainText('18 sites · 1 territories');
});

test('late public sample cannot replace a private CSV or stopped state',async({page})=>{
  let release;let started;const observed=new Promise(resolve=>started=resolve);
  await page.route('**/sample.csv',async route=>{started();await new Promise(resolve=>release=resolve);try{await route.fulfill({body:'site_id,demand,revenue,latitude,longitude\nPUBLIC,1,1,0,0\n'});}catch{}});
  await page.goto('/TERRAIN/');await observed;
  const csv='site_id,demand,revenue,latitude,longitude\nPRIVATE,1,1,0,0\n';await page.locator('#csv').fill(csv);await count(page,1);await expect(page.locator('#export')).toBeEnabled();
  release();await expect(page.locator('#csv')).toHaveValue(csv);await expect(page.locator('#status')).toContainText('1 sites · 1 territories');
});

test('500001 multibyte UTF8 bytes are rejected before worker admission',async({page})=>{
  await observeWorkers(page);await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  const initial=await page.evaluate(()=>window.__terrainWorkers.reduce((n,w)=>n+w.sent.filter(x=>x.type==='plan').length,0));
  await page.locator('#csv').fill('é'.repeat(250000)+'a');await page.locator('#apply').click();await expect(page.locator('#status')).toContainText('up to 500 KB');await noResult(page);
  expect(await page.evaluate(()=>window.__terrainWorkers.reduce((n,w)=>n+w.sent.filter(x=>x.type==='plan').length,0))).toBe(initial);
});

test('real graph mode completes the admitted 2000-site workload and exports all features',async({page})=>{
  test.setTimeout(180000);
  await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
  const csv='site_id,demand,revenue,latitude,longitude\n'+Array.from({length:2000},(_,i)=>`S${i},${i%5+1},${(i%7+1)*10},${i/10000},${(i%7)/10000}
`).join('');
  await page.locator('#csv').fill(csv);await count(page,8);await page.locator('#method').selectOption('graph');
  await expect(page.locator('#status')).toContainText('2000 sites · 8 territories',{timeout:150000});await expect(page.locator('#export')).toBeEnabled();
  const download=page.waitForEvent('download');await page.locator('#export').click();const file=await download;const stream=await file.createReadStream();const chunks=[];for await(const chunk of stream)chunks.push(chunk);
  expect(JSON.parse(Buffer.concat(chunks)).features.length).toBe(2008);
});
