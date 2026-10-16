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
cases = [('off','proMedium','1200x900',1,'off'),('fade-curve','proMedium','1200x900',1,'fade'),
         ('pressure-curve','proMedium','1200x900',1,'penPressure'),('light','proLight','1200x900',1,'fade'),
         ('scale2','proMedium','900x720',2,'penPressure')]
for name,theme,size,scale,control in cases:
    brush = {'shapeDynamics': {'enabled':True,'size':{'control':control,'fadeSteps':123,'minimum':0.2,'curve':[[0,0],[0.3,0.6],[1,1]]}}}
    script = [['engine.execute',{'command':'prefs.set','params':{'path':'interface.language','value':'en'}}],
              ['engine.execute',{'command':'file.new','params':{'width':480,'height':360}}],
              ['engine.execute',{'command':'tools.setBrush','params':{'brush':brush}}],
              ['ui.set',{'theme':theme,'panels':{'brush_settings':True},'brushSection':1}]]
    p = subprocess.run([str(repo/'target/debug/examples/snapshot'),'--out',str(out/f'oct16-4829-{name}.png'),
                        '--size',size,'--scale',str(scale),'--script',json.dumps(script)],cwd=repo,capture_output=True,text=True)
    (work/f'oct16-capture-{name}.log').write_text(p.stdout+p.stderr)
    assert p.returncode==0 and '\"ok\":false' not in p.stderr, p.stderr
print('Captured five native merged Fade/curve views.')
