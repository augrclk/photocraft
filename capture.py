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
for label, theme, size, scale, scrollbar_x, scrollbar_y in [('medium', 'proMedium', '1200x900', 1, 840, 425), ('light', 'proLight', '1200x900', 1, 840, 425), ('scale2', 'proMedium', '900x720', 2, 583, 360)]:
    for tool in ['brush', 'mixerBrush']:
        name = f'{label}-{tool}'
        if args.only and args.only != name:
            continue
        dynamics = {
            key: {'jitter': jitter, 'minimum': minimum, 'control': 'penPressure', 'fadeSteps': steps}
            for key, jitter, minimum, steps in [('opacity', 0.13, 0.17, 119), ('flow', 0.27, 0.29, 229), ('wetness', 0.41, 0.43, 339), ('mix', 0.63, 0.67, 449)]
        }
        script = [
            ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
            ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
            ['ui.set', {'theme': theme, 'tool': tool, 'panels': {'brush_settings': True}, 'brushSection': 6}],
            ['engine.execute', {'command': 'tools.setBrush', 'params': {'brush': {'transfer': {'enabled': True, **dynamics}}}}],
            ['ui.click', {'x': scrollbar_x, 'y': scrollbar_y}],
        ]
        path = out / f'3825-transfer-{args.phase}-{name}.png'
        result = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
        (pathlib.Path(__file__).resolve().parent / f'3825-transfer-{args.phase}-{name}.log').write_text(result.stdout + result.stderr)
        if '"ok":false' in result.stdout + result.stderr:
            raise RuntimeError(f'Fixture failed: {name}')
        print(path)
