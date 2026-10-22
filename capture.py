import argparse,json,subprocess
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--repo',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();repo=a.repo.resolve();a.out.mkdir(parents=True,exist_ok=True)
fixtures=sorted(Path(__file__).resolve().parent.glob('*-fixture.json'))
assert len(fixtures)==3
tree=subprocess.check_output(['git','rev-parse','HEAD^{tree}'],cwd=repo,text=True).strip()
for fixture in fixtures:
 d=json.loads(fixture.read_text());assert tree==d['source_tree'],'Check out the documented source revision first'
 file=a.out.resolve()/(fixture.name.removesuffix('-fixture.json')+'.png')
 r=subprocess.run([str(repo/'target/debug/examples/snapshot'),'--out',str(file),'--size',d['size'],'--scale',str(d['scale']),'--script',json.dumps(d['script'])],cwd=repo,capture_output=True,text=True)
 assert r.returncode==0 and '\"ok\":false' not in r.stdout+r.stderr,r.stdout+r.stderr
 assert file.is_file() and file.stat().st_size>10000
 print(file)
