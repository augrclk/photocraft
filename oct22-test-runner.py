"""Run every selected desktop unit test once in its own process, retaining assertions."""
import subprocess,sys
binary,*arguments=sys.argv[1:]
if '--list' in arguments:
    raise SystemExit(subprocess.run([binary,*arguments]).returncode)
listed=subprocess.run([binary,*arguments,'--list','--format','terse'],text=True,capture_output=True)
assert listed.returncode==0,(listed.stdout,listed.stderr)
names=[line.removesuffix(': test') for line in listed.stdout.splitlines() if line.endswith(': test')]
assert names and len(names)==len(set(names)),listed.stdout
print(f'Local runner: {len(names)} desktop unit tests, one process each; no retries or added skips.',flush=True)
failed=[]
for index,name in enumerate(names,1):
    print(f'Desktop unit isolation {index}/{len(names)}: {name}',flush=True)
    result=subprocess.run([binary,*arguments,'--exact',name,'--test-threads=1'])
    if result.returncode:failed.append({'test':name,'exit':result.returncode})
print(f'Desktop isolation executed {len(names)} selected tests exactly once; failed processes: {failed}',flush=True)
raise SystemExit(1 if failed else 0)
