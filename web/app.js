const $=selector=>document.querySelector(selector);
let ready=false,request=0,geojson,svgUrl,timer,sample;
const params=new URLSearchParams(location.search);
const worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
const format=value=>value===null?'∞':value.toLocaleString('en-US',{maximumFractionDigits:2});
function controls(){ $('#count-value').textContent=$('#count').value;$('#demand-value').textContent=Number($('#demand-limit').value).toFixed(2);$('#revenue-value').textContent=Number($('#revenue-limit').value).toFixed(2);}
function plan(){controls();request++;if(!ready)return;clearTimeout(timer);$('#status').textContent='Planning…';$('#export').disabled=true;timer=setTimeout(()=>worker.postMessage({type:'plan',id:request,csv:$('#csv').value,count:Number($('#count').value),graph:$('#method').value==='graph',demand:Number($('#demand-limit').value),revenue:Number($('#revenue-limit').value)}),100);}
function error(message){$('#status').textContent=message;$('#verdict').textContent='Check input';$('#export').disabled=true;$('#map').hidden=true;$('#territories').replaceChildren();for(const id of ['demand-ratio','revenue-ratio','radius'])$(`#${id}`).textContent='—';}
worker.onerror=()=>error('Planner could not load. Reload to retry or inspect the source above.');
worker.onmessage=({data})=>{
 if(data.type==='ready'){ready=true;document.querySelectorAll('button,input,select,textarea').forEach(control=>control.disabled=false);plan();}
 else if(data.type==='error'&&(data.id===undefined||data.id===request)){error(data.message);}
 else if(data.type==='result'&&data.id===request){
  const result=data.result;geojson=result.geojson;
  $('#status').textContent=`${result.site_count} sites · ${result.territory_count} territories · ${data.ms.toFixed(1)} ms · processing stays on your device`;
  $('#verdict').textContent=result.passes?'Within balance limits':'Review balance';
  $('#demand-ratio').textContent=format(result.demand_ratio);$('#revenue-ratio').textContent=format(result.revenue_ratio);$('#radius').textContent=format(result.radius_degrees);
  if(svgUrl)URL.revokeObjectURL(svgUrl);svgUrl=URL.createObjectURL(new Blob([result.svg],{type:'image/svg+xml'}));$('#map').src=svgUrl;$('#map').hidden=false;
  $('#territories').replaceChildren();for(const territory of result.summaries){const row=document.createElement('tr');for(const value of [territory.id,territory.site_count,format(territory.demand),format(territory.revenue)]){const cell=document.createElement('td');cell.textContent=value;row.append(cell);}$('#territories').append(row);}$('#export').disabled=false;
 }
};
for(const id of ['count','demand-limit','revenue-limit'])$(`#${id}`).oninput=plan;
$('#method').onchange=plan;$('#apply').onclick=plan;
$('#csv').oninput=()=>{request++;clearTimeout(timer);$('#export').disabled=true;$('#status').textContent='CSV changed. Choose “Plan these sites” to refresh.';};
$('#sample').onclick=()=>{$('#csv').value=sample;plan();};
$('#file').onchange=async()=>{const file=$('#file').files[0];if(!file)return;request++;clearTimeout(timer);const generation=request;$('#export').disabled=true;if(file.size>500000){error('Use a CSV file up to 500 KB.');return;}try{const csv=await file.text();if(generation!==request)return;$('#csv').value=csv;plan();}catch{if(generation===request)error('Could not read this CSV file.');}};
$('#share').onclick=async()=>{const url=new URL(location.href);url.search='';for(const [key,id]of [['count','count'],['method','method'],['demand','demand-limit'],['revenue','revenue-limit']])url.searchParams.set(key,$(`#${id}`).value);history.replaceState(null,'',url);try{await navigator.clipboard.writeText(url.href);$('#status').textContent='Settings copied. Shared links load the public sample, not your CSV.';}catch{$('#status').textContent='Copy settings from the address bar. Shared links load the public sample.';}};
$('#export').onclick=()=>{if(!geojson)return;const url=URL.createObjectURL(new Blob([geojson],{type:'application/geo+json'}));const link=document.createElement('a');link.href=url;link.download='terrain-plan.geojson';link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
try{const response=await fetch('./sample.csv');if(!response.ok)throw new Error();sample=await response.text();$('#csv').value=sample;for(const[key,id,min,max]of [['count','count',1,8],['demand','demand-limit',1,3],['revenue','revenue-limit',1,3]]){const value=Number(params.get(key));if(params.has(key)&&Number.isFinite(value)&&value>=min&&value<=max&&(key!=='count'||Number.isInteger(value)))$(`#${id}`).value=String(value);}if(['greedy','graph'].includes(params.get('method')))$('#method').value=params.get('method');controls();worker.postMessage({type:'init'});}catch{error('Planner could not load its sample. Reload to retry, or inspect the source above.');}
