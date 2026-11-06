import {test,expect} from '@playwright/test';

test('real WASM partitions sample sites, changes method, and exports',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));
 const wasm=page.waitForResponse(response=>response.url().endsWith('.wasm'));
 await page.goto('/TERRAIN/');expect((await wasm).status()).toBe(200);
 await expect(page.locator('#status')).toContainText('18 sites · 3 territories');
 await expect(page.locator('#map')).toBeVisible();
 expect(await page.locator('#map').evaluate(image=>image.complete&&image.naturalWidth>0)).toBeTruthy();
 await page.locator('#count').focus();await page.locator('#count').press('ArrowRight');
 await expect(page.locator('#status')).toContainText('18 sites · 4 territories');
 await expect(page.locator('#territories tr')).toHaveCount(4);
 await page.locator('#method').selectOption('graph');
 await expect(page.locator('#status')).toContainText('18 sites · 4 territories');
 await expect(page.locator('#export')).toBeEnabled();
 const download=page.waitForEvent('download');await page.locator('#export').click();
 expect((await download).suggestedFilename()).toBe('terrain-plan.geojson');
 await page.locator('#share').click();await page.reload();
 await expect(page.locator('#status')).toContainText('18 sites · 4 territories');
 await expect(page.locator('#method')).toHaveValue('graph');
 expect(errors).toEqual([]);
});

test('malformed CSV recovers and mobile view fits',async({page})=>{
 await page.setViewportSize({width:390,height:844});await page.goto('/TERRAIN/');
 await expect(page.locator('#export')).toBeEnabled();
 await page.locator('#csv').fill('bad,csv\n1,2');await page.locator('#apply').click();
 await expect(page.locator('#status')).toContainText('missing required header');
 await expect(page.locator('#export')).toBeDisabled();
 await page.locator('#sample').click();await expect(page.locator('#status')).toContainText('18 sites');
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();
});

test('sample failure gives a readable fallback',async({page})=>{
 await page.route('**/sample.csv',route=>route.fulfill({status:503,body:'unavailable'}));
 await page.goto('/TERRAIN/');await expect(page.locator('#status')).toContainText('Choose Load sample to retry');
 await expect(page.getByRole('link',{name:'Source & research'})).toBeVisible();
});

test('oversized file invalidates pending plans and exports',async({page})=>{
 await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
 await page.locator('#count').evaluate(e=>{e.value='4';e.dispatchEvent(new Event('input'));});
 await page.locator('#file').setInputFiles({name:'large.csv',mimeType:'text/csv',buffer:Buffer.alloc(500001,65)});
 await expect(page.locator('#status')).toContainText('up to 500 KB');
 await page.waitForTimeout(400);
 await expect(page.locator('#status')).toContainText('up to 500 KB');
 await expect(page.locator('#export')).toBeDisabled();
});

test('runtime files stay within one immutable source revision',async({page})=>{
 const responses=[];page.on('response',r=>responses.push(r.url()));
 await page.goto('/TERRAIN/');await expect(page.locator('#export')).toBeEnabled();
 const manifest=await (await page.request.get('/TERRAIN/BUILD-MANIFEST.json')).json();
 expect(manifest.runtime).toBe(`assets/${manifest.commit}`);
 const script=await page.locator('script[type="module"]').getAttribute('src');
 expect(script).toBe(`${manifest.runtime}/app.js`);
 for(const name of ['app.js','planner-session.js','worker.js','terrain_web.js','terrain_web_bg.wasm','sample.csv']){
  const seen=responses.filter(url=>url.endsWith('/'+name));expect(seen.length,name).toBeGreaterThan(0);
  expect(seen.every(url=>url.includes(`/TERRAIN/${manifest.runtime}/`)),name).toBeTruthy();
 }
});

test('source and build guide remain accessible with JavaScript disabled',async({browser,baseURL})=>{
 const context=await browser.newContext({javaScriptEnabled:false,baseURL});const page=await context.newPage();
 try{
  await page.goto('/TERRAIN/');
  await page.getByRole('link',{name:'Build guide & downloads'}).click();
  await expect(page.getByRole('heading',{name:'Receive without Git'})).toBeVisible();
  const link=page.getByRole('link',{name:'Complete committed source (tar.xz)'});
  await expect(link).toHaveAttribute('href','source.tar.xz');
  const response=await page.request.get('/TERRAIN/source.tar.xz');
  expect(response.ok()).toBeTruthy();expect((await response.body()).length).toBeLessThan(5_000_000);
 }finally{await context.close();}
});
