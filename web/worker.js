import init,{plan_json} from './pkg/terrain_web.js';
self.onmessage=async({data})=>{
 try{
  if(data.type==='init'){await init();self.postMessage({type:'ready'});}
  else if(data.type==='plan'){const started=performance.now();const result=JSON.parse(plan_json(data.csv,data.count,data.graph,data.demand,data.revenue));self.postMessage({type:'result',id:data.id,result,ms:performance.now()-started});}
 }catch(error){self.postMessage({type:'error',id:data.id,message:String(error)});}
};
