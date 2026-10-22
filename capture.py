"""Native captures of the exact merged PR sources; no source mutation."""
import argparse,json,subprocess
from pathlib import Path
parser=argparse.ArgumentParser()
parser.add_argument("--repo",type=Path,required=True)
parser.add_argument("--out",type=Path,required=True)
parser.add_argument("--pr",type=int,required=True)
parser.add_argument("--click",action="append",help="Observed screen-point position X,Y to append; repeat for a menu and its row")
parser.add_argument("--suffix",default="")
args=parser.parse_args()
repo=args.repo.resolve();out=args.out.resolve();out.mkdir(parents=True,exist_ok=True)
dynamics={k:{"jitter":j,"minimum":m,"control":"fade","fadeSteps":37,"curve":[[0,0],[0.3,0.6],[1,1]]} for k,j,m in [("opacity",0.03,0.173),("flow",0.07,0.283),("wetness",0.11,0.433),("mix",0.13,0.643)]}
cases={
4829:[("fade",1,"brush",{"shapeDynamics":{"enabled":True,"size":{"control":"fade","fadeSteps":123,"minimum":0.2,"curve":[[0,0],[0.3,0.6],[1,1]]}}})],
5224:[("spacing",0,"brush",{"spacing":0.427,"spacingEnabled":False})],
5250:[("scatter",2,"brush",{"scattering":{"enabled":True,"bothAxes":True,"scatter":{"jitter":2.137,"control":"penPressure"},"count":5}})],
5269:[("geometry",0,"brush",{"angle":-23,"roundness":0.473,"hardness":0.65,"spacing":0.31,"flipX":True})],
5278:[("brush",6,"brush",{"transfer":{"enabled":True,**dynamics}}),("mixer",6,"mixerBrush",{"transfer":{"enabled":True,**dynamics}})],
5338:[("grid",0,"brush",{"size":17.25,"hardness":0.65})],
5352:[("panel",0,"brush",{"size":37.25})],
5356:[("restore",0,"brush",{"size":37.25})],
}
for name,section,tool,brush in cases[args.pr]:
    script=[["engine.execute",{"command":"prefs.set","params":{"path":"interface.language","value":"en"}}],
    ["engine.execute",{"command":"file.new","params":{"width":480,"height":360}}],
    ["ui.set",{"theme":"proMedium","tool":tool,"panels":{"brush_settings":True},"brushSection":section}],
    ["engine.execute",{"command":"tools.setBrush","params":{"brush":brush}}]]
    if args.pr==5224:script.append(["ui.click",{"x":840,"y":560}])
    for point in args.click or []:
        x,y=map(float,point.split(","));script.append(["ui.click",{"x":x,"y":y}])
    file=out/f"oct22-{args.pr}-{name}{args.suffix}.png"
    r=subprocess.run([str(repo/"target/debug/examples/snapshot"),"--out",str(file),"--size","1200x1000","--scale","1","--script",json.dumps(script)],cwd=repo,text=True,capture_output=True)
    log=r.stdout+r.stderr
    assert r.returncode==0 and '"ok":false' not in log,(args.pr,log)
    assert file.is_file() and file.stat().st_size>10000
    (out/f"oct22-{args.pr}-{name}{args.suffix}-fixture.json").write_text(json.dumps({"size":"1200x1000","scale":1,"script":script},indent=2)+"\n")
    print(file)
