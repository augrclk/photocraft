import argparse
import json
import pathlib
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--phase', choices=['before', 'after'], required=True)
parser.add_argument('--repo', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent / 'photocraft')
parser.add_argument('--out', type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent.parent / 'outputs')
args = parser.parse_args()
repo, out = args.repo.resolve(), args.out.resolve()
out.mkdir(parents=True, exist_ok=True)
for label, theme, size, scale in [('medium', 'proMedium', '1200x900', 1), ('light', 'proLight', '1200x900', 1), ('scale2', 'proMedium', '900x720', 2), ('medium-lower', 'proMedium', '1200x900', 1)]:
    for state, control in [('active', 'penPressure'), ('off', 'off')]:
        transfer = {'enabled': True}
        for key, jitter, minimum in [('opacity', .03, .17), ('flow', .07, .28), ('wetness', .11, .43), ('mix', .13, .64)]:
            transfer[key] = {'jitter': jitter, 'control': control, 'minimum': minimum, 'fadeSteps': 37}
        script = [
            ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
            ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
            ['ui.set', {'theme': theme, 'tool': 'mixerBrush', 'panels': {'brush_settings': True}, 'brushSection': 6}],
            ['engine.execute', {'command': 'tools.setBrush', 'params': {'brush': {'size': 47, 'angle': -23, 'roundness': .473, 'transfer': transfer}}}],
        ]
        path = out / f'3825-minimum-{args.phase}-{label}-{state}.png'
        extra = ['--click-at', '839,617'] if label == 'medium-lower' else []
        r = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script), *extra], cwd=repo, capture_output=True, text=True, check=True)
        (pathlib.Path(__file__).resolve().parent / f'3825-minimum-{args.phase}-{label}-{state}.log').write_text(r.stdout + r.stderr)
        if '"ok":false' in r.stdout + r.stderr:
            raise RuntimeError(f'Fixture failed: {path.name}')
        print(path)
