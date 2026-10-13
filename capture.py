import argparse
import json
import pathlib
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--phase', choices=['before', 'after'], required=True)
parser.add_argument('--repo', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent / 'photocraft')
parser.add_argument('--out', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent.parent / 'outputs')
parser.add_argument('--only')
args = parser.parse_args()
repo = args.repo.resolve()
out = args.out.resolve()
out.mkdir(parents=True, exist_ok=True)
for label, theme, size, scale, scrollbar_x, scrollbar_y in [('medium', 'proMedium', '1200x900', 1, 840, 560), ('light', 'proLight', '1200x900', 1, 840, 560), ('scale2', 'proMedium', '900x720', 2, 583, 500)]:
    for enabled in [False, True]:
        name = f'{label}-' + ('on' if enabled else 'off')
        if args.only and args.only != name:
            continue
        script = [
            ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
            ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
            ['ui.set', {'theme': theme, 'tool': 'brush', 'panels': {'brush_settings': True}, 'brushSection': 0}],
            ['engine.execute', {'command': 'tools.setBrush', 'params': {'brush': {'spacing': 0.427, 'spacingEnabled': enabled}}}],
            ['ui.click', {'x': scrollbar_x, 'y': scrollbar_y}],
        ]
        path = out / f'3825-spacing-{args.phase}-{name}.png'
        result = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
        (pathlib.Path(__file__).resolve().parent / f'3825-spacing-{args.phase}-{name}.log').write_text(result.stdout + result.stderr)
        if '"ok":false' in result.stdout + result.stderr:
            raise RuntimeError(f'Fixture failed: {name}')
        print(path)
