import argparse
import json
import pathlib
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--repo', type=pathlib.Path, required=True)
parser.add_argument('--out', type=pathlib.Path, required=True)
args = parser.parse_args()
repo = args.repo.resolve()
outputs = args.out.resolve()
outputs.mkdir(parents=True, exist_ok=True)
work = outputs
cases = [('off', 'proMedium', '1200x900', 1, 1, 'off'), ('active', 'proMedium', '1200x900', 1, 1, 'fade'), ('transfer', 'proMedium', '1200x900', 1, 6, 'off'), ('light', 'proLight', '1200x900', 1, 1, 'off'), ('scale2', 'proMedium', '900x720', 2, 1, 'off')]
for case, theme, size, scale, section, control in cases:
    brush = {'shapeDynamics': {'enabled': True, 'size': {'control': control, 'minimum': 0.2, 'fadeSteps': 123}, 'roundness': {'control': 'off', 'minimum': 0.35}}, 'transfer': {'enabled': True, **{k: {'control': 'off', 'minimum': v} for k, v in [('opacity', 0.4), ('flow', 0.55), ('wetness', 0.6), ('mix', 0.75)]}}}
    script = [['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}], ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}], ['engine.execute', {'command': 'tools.setBrush', 'params': {'brush': brush}}], ['ui.set', {'theme': theme, 'panels': {'brush_settings': True}, 'brushSection': section}]]
    path = outputs / f'4829-refresh-{case}.png'
    result = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
    (work / f'4829-refresh-{case}.log').write_text(result.stdout + result.stderr)
    if '"ok":false' in result.stdout + result.stderr:
        raise RuntimeError(f'Capture command failed: {case}')
print('Captured five native refreshed Fade-control fixtures.')
