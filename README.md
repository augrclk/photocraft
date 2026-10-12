# PhotoCraft #4283 visual checks

Generated from the project's own offscreen `snapshot` example on Apple M4 / Metal, with English UI and a blank canvas. No personal image assets were used. Captured and visually inspected by Codex for @augrclk.

- `4283-before.png`: upstream 0.8.1, commit 2db6b7d8f9ddb5be8d29f61b918133b22e41c5bc. Shape Dynamics selected, then click Noise at 300,300: Noise replaces the properties pane.
- `4283-after.png`: proposed commit edf1968e955c21e2ceb5986f0862f7d79a73a2ed, rebased onto 0.8.2 / d43db3e. Same initial section and click: Noise turns on and Shape Dynamics stays open.
- `4283-light.png`: proposed commit, Pro Light, click Smoothing off with 65% / Pulled String configured. Controls stay open.
- `4283-scale2.png`: proposed commit, 1200x800 points at 2x scale; five switches shown with Smoothing off.

The before and after bases are recorded explicitly; the before snapshot predates the upstream 0.8.2 release. The code PR does not include these binary files.

## Reproduce the after capture

```sh
cargo run -p photocraft-ui-egui --example snapshot -- --out after.png --size 1440x900 --scale 1 --click-at 300,300 --script '[["engine.execute",{"command":"prefs.set","params":{"path":"interface.language","value":"en"}}],["engine.execute",{"command":"tools.setBrush","params":{"brush":{"shapeDynamics":{"enabled":true}}}}],["ui.set",{"tool":"brush","panels":{"brush_settings":true},"brushSection":1}]]'
```
