# Brush Settings minimum controls — #3825

Offscreen review images from PhotoCraft's native `snapshot` example, using a synthetic 480 × 360 blank document, English UI and Apple M4 / Metal. No personal image assets.

- Before: upstream `8c135457b4aecfda0992ce5ab4eb82fe713a73d6`, with test-only additions; production code unchanged.
- After: `18ae8dc411ba248a1c911891c658838ba2c436ae`, based on `4eb8ca25bd1e332eff7bf33013ef7ee89ac9dac3`.
- Shape Dynamics starts with Size Control Off / Minimum Diameter 20%, Roundness Control Off / Minimum Roundness 35%.
- Transfer starts with Control Off for Opacity, Flow, Wetness and Mix, retaining minimums 40%, 55%, 60% and 75%.
- `3825-active.png`: Size Control Pen Pressure; Minimum Diameter is editable at the same 20%, while Minimum Roundness stays disabled at 35%.
- Standard captures: 1200 × 900 at scale 1. `3825-scale2.png`: 900 × 720 logical points at scale 2.

The snapshot tool is built with `CARGO_INCREMENTAL=0 cargo build --locked --workspace --features heif --example snapshot`. Its `--script` calls, in order:

```json
[
  ["engine.execute", {"command":"prefs.set","params":{"path":"interface.language","value":"en"}}],
  ["engine.execute", {"command":"file.new","params":{"width":480,"height":360}}],
  ["engine.execute", {"command":"tools.setBrush","params":{"brush":{"shapeDynamics":{"enabled":true,"size":{"control":"off","minimum":0.2},"roundness":{"control":"off","minimum":0.35}},"transfer":{"enabled":true,"opacity":{"control":"off","minimum":0.4},"flow":{"control":"off","minimum":0.55},"wetness":{"control":"off","minimum":0.6},"mix":{"control":"off","minimum":0.75}}}}}],
  ["ui.set", {"theme":"proMedium","panels":{"brush_settings":true},"brushSection":1}]
]
```

Use `brushSection:6` for Transfer, `theme:proLight` for the light capture, or `size.control:penPressure` for the active-control capture. Run the binary with `--out <png> --size 1200x900 --scale 1 --script '<JSON above>'`.

All 14 bundled themes were also inspected at 1200 × 900 and 900 × 720, scale 1. The code PR addresses only the minimum-input state described in #3825; the issue's other items remain open.
