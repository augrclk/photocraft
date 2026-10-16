import argparse
import json
from pathlib import Path
import subprocess
parser=argparse.ArgumentParser()
parser.add_argument('--repo',type=Path,required=True)
parser.add_argument('--out',type=Path,required=True)
args=parser.parse_args()
repo=args.repo.resolve()
out=args.out.resolve()
out.mkdir(parents=True,exist_ok=True)
work=out
for name,theme in [('transfer','proMedium'),('transfer-light','proLight')]:
    dynamics={k:{'jitter':v,'minimum':m,'control':'fade','fadeSteps':37,'curve':[[0,0],[0.3,0.6],[1,1]]} for k,v,m in [('opacity',0.03,0.173),('flow',0.07,0.283),('wetness',0.11,0.433),('mix',0.13,0.643)]}
    script=[['engine.execute',{'command':'prefs.set','params':{'path':'interface.language','value':'en'}}],['engine.execute',{'command':'file.new','params':{'width':480,'height':360}}],['engine.execute',{'command':'tools.setBrush','params':{'brush':{'transfer':{'enabled':True,**dynamics}}}}],['ui.set',{'theme':theme,'panels':{'brush_settings':True},'brushSection':6}]]
    r=subprocess.run([str(repo/'target/debug/examples/snapshot'),'--out',str(out/f'oct16-5278-{name}.png'),'--size','1200x1000','--script',json.dumps(script)],cwd=repo,capture_output=True,text=True)
    (work/f'oct16-5278-capture-{name}.log').write_text(r.stdout+r.stderr)
    assert r.returncode==0 and '"ok":false' not in r.stderr,r.stderr
print('Captured two native merged Transfer views.')
