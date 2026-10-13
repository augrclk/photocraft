import json
import pathlib
import subprocess
import sys

work = pathlib.Path(__file__).resolve().parent
repo = work / 'photocraft'
outputs = work.parent / 'outputs'
phase = sys.argv[1]
binary = work / '3825-hue-before-snapshot' if phase == 'before' else repo / 'target/debug/examples/snapshot'
for case, theme, size, scale in [('dark', 'proMedium', '1200x900', 1), ('light', 'proLight', '1200x900', 1), ('scale2', 'proMedium', '900x720', 2)]:
    script = [
        ['engine.execute', {'command': 'prefs.set', 'params': {'path': 'interface.language', 'value': 'en'}}],
        ['engine.execute', {'command': 'file.new', 'params': {'width': 480, 'height': 360}}],
        ['engine.execute', {'command': 'tools.setBrush', 'params': {'brush': {'colorDynamics': {'enabled': True, 'hueJitter': 0.37, 'saturationJitter': 0.23, 'brightnessJitter': 0.41, 'purity': -0.16}}}}],
        ['ui.set', {'theme': theme, 'panels': {'brush_settings': True}, 'brushSection': 5}],
    ]
    path = outputs / f'3825-hue-{phase}-{case}.png'
    result = subprocess.run([str(binary), '--out', str(path), '--size', size, '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
    (work / f'3825-hue-{phase}-{case}.log').write_text(result.stdout + result.stderr)
    if '"ok":false' in result.stdout + result.stderr:
        raise RuntimeError('A native capture command failed; inspect its saved log.')
print(f'Captured three {phase} Color Dynamics fixtures.')
