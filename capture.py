import argparse
import hashlib
import json
import pathlib
import re
import subprocess

work = pathlib.Path(__file__).resolve().parent
p = argparse.ArgumentParser()
p.add_argument('--phase', choices=['before', 'after'], required=True)
p.add_argument('--repo', type=pathlib.Path, default=work / 'photocraft')
p.add_argument('--out', type=pathlib.Path, default=work.parent / 'outputs')
a = p.parse_args()
repo, out = a.repo.resolve(), a.out.resolve()
out.mkdir(parents=True, exist_ok=True)
if a.phase == 'before':
    base = '99e1138877e6aa7e856c6883b415e708d042fc59'
    changed = subprocess.check_output(['git', 'diff', '--name-only'], cwd=repo, text=True).splitlines()
    assert set(changed) <= {'crates/ui-egui/src/brush_sections.rs', 'crates/ui-egui/src/brush_panel_tests.rs'}
    original = subprocess.check_output(['git', 'show', base + ':crates/ui-egui/src/brush_sections.rs'], cwd=repo, text=True)
    current = (repo / 'crates/ui-egui/src/brush_sections.rs').read_text()
    test_suffix = '\n#[cfg(test)]\n#[path = "brush_tip_grid_tests.rs"]\nmod tip_grid_tests;\n'
    assert current == original or current == original + test_suffix, 'before captures require unchanged production code'
    # The other allowed file is included only under cfg(test), never by snapshot.

source = (repo / 'crates/paint/src/presets.rs').read_text()
builtins = re.findall(r'\bg\(\s*"([^"]+)"', source)
assert len(builtins) == len(set(builtins)) and len(builtins) > 20
cases = [
    ('medium', 'proMedium', '1200x900', 1, 64, 0, True),
    ('light', 'proLight', '1200x900', 1, 64, 0, True),
    ('compact', 'proMedium', '900x720', 1, 64, 0, True),
    ('scale2', 'proMedium', '900x720', 2, 64, 0, True),
    ('small600', 'proMedium', '900x600', 1, 64, 0, True),
    ('short', 'proMedium', '1200x900', 1, 1, 0, True),
    ('empty', 'proMedium', '1200x900', 1, 0, 0, True),
    ('dual', 'proMedium', '1200x900', 1, 64, 4, True),
    ('dual-off', 'proMedium', '1200x900', 1, 64, 4, False),
]
scripts = {}
for label, theme, size, scale, count, section, enabled in cases:
    def execute(command, params): return ['engine.execute', {'command': command, 'params': params}]
    script = [execute('prefs.set', {'path': 'interface.language', 'value': 'en'}),
              execute('file.new', {'width': 480, 'height': 360})]
    script += [execute('brush.presets.delete', {'name': name}) for name in builtins]
    for i in range(count):
        script += [execute('tools.setBrush', {'reset': True, 'brush': {
            'size': 5 + i * 3, 'hardness': [1.0, .75, .5, .25][i % 4],
            'angle': i * 7 % 180, 'roundness': [.35, .6, .85, 1.0][i % 4],
            'spacing': .15, 'pressureSize': False,
        }}), execute('brush.presets.save', {'name': f'Grid tip {i:04}', 'group': 'Synthetic grid'})]
    if count:
        script += [execute('tools.setBrush', {'preset': f'Grid tip {min(7, count - 1):04}'})]
    script += [execute('tools.setBrush', {'dualBrush': {'enabled': enabled, 'size': 47}}),
               ['ui.set', {'theme': theme, 'tool': 'brush', 'panels': {'brush_settings': True}, 'brushSection': section}]]
    scripts[label] = script
    image = out / f'3825-grid-{a.phase}-{label}.png'
    r = subprocess.run([str(repo / 'target/debug/examples/snapshot'), '--out', str(image), '--size', size,
                        '--scale', str(scale), '--script', json.dumps(script)], cwd=repo, capture_output=True, text=True, check=True)
    log = r.stdout + r.stderr
    (work / f'3825-grid-{a.phase}-{label}.log').write_text(log)
    assert '"ok":false' not in log, log
    print(image, flush=True)
(work / f'3825-grid-{a.phase}-scripts.json').write_text(json.dumps(scripts, sort_keys=True, indent=2) + '\n')
(work / f'3825-grid-{a.phase}-fixture-sha.txt').write_text(hashlib.sha256(json.dumps(scripts, sort_keys=True).encode()).hexdigest() + '\n')
