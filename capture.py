import argparse
import base64
import hashlib
import json
import pathlib
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--phase', choices=['before', 'after'], required=True)
parser.add_argument('--repo', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent / 'photocraft')
parser.add_argument('--out', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent.parent / 'outputs')
args = parser.parse_args()
work = pathlib.Path(__file__).resolve().parent
repo, out = args.repo.resolve(), args.out.resolve()
out.mkdir(parents=True, exist_ok=True)
fixtures = {p.stem: p.read_bytes() for p in (work / '3825-pose-fixtures').glob('*.abr')}
assert set(fixtures) == {'none', 'x-only', 'y-only', 'both'}
(work / f'3825-pose-{args.phase}-fixture-shas.json').write_text(json.dumps({k: hashlib.sha256(v).hexdigest() for k, v in fixtures.items()}, indent=2) + '\n')
cases = [('medium', 'proMedium', '1200x900', 1, ['none', 'x-only', 'y-only', 'both']),
         ('light', 'proLight', '1200x900', 1, ['x-only', 'y-only']),
         ('scale2', 'proMedium', '900x720', 2, ['x-only', 'y-only'])]
for label, theme, size, scale, names in cases:
    for name in names:
        script = [
            ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
            ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
            ['ui.set', {'theme': theme, 'tool': 'brush', 'panels': {'brush_settings': True}, 'brushSection': 7}],
            ['engine.execute', {'command': 'brush.presets.importAbr', 'params': {'data': base64.b64encode(fixtures[name]).decode(), 'group': 'Synthetic pose', 'select': True}}],
        ]
        path = out / f'3825-pose-{args.phase}-{label}-{name}.png'
        r = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
        (work / f'3825-pose-{args.phase}-{label}-{name}.log').write_text(r.stdout + r.stderr)
        if '"ok":false' in r.stdout + r.stderr:
            raise RuntimeError(f'Fixture failed: {path.name}')
        print(path)
for name in ['x-only', 'y-only']:
    script = [
        ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
        ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
        ['ui.set', {'theme': 'proMedium', 'tool': 'brush', 'panels': {'brush_settings': False}}],
        ['engine.execute', {'command': 'brush.presets.importAbr', 'params': {'data': base64.b64encode(fixtures[name]).decode(), 'group': 'Synthetic pose', 'select': True}}],
        ['engine.execute', {'command': 'paint.stroke', 'params': {'color': '#245ca0', 'seed': 19, 'points': [
            {'x': 64, 'y': 120, 'pressure': .71, 'tiltX': 13, 'tiltY': -27},
            {'x': 416, 'y': 120, 'pressure': .83, 'tiltX': -31, 'tiltY': 42},
        ]}}],
    ]
    path = out / f'3825-pose-{args.phase}-paint-{name}.png'
    r = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', '1200x900', '--scale', '1', '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
    (work / f'3825-pose-{args.phase}-paint-{name}.log').write_text(r.stdout + r.stderr)
    if '"ok":false' in r.stdout + r.stderr:
        raise RuntimeError(f'Fixture failed: {path.name}')
    print(path)
