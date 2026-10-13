# Brush Settings Control rows — #3825 (partial)

Native offscreen screenshots from PhotoCraft's `snapshot` example, using a synthetic 480 × 360 blank document, English UI, Apple M4 / Metal.

- After source: `a26251e8bc6838e21f1d5b57aa357377faf8b3e2`, based on upstream `04f0b0e86bffa8dfea6ded4412d9cef11146e1bf`.
- Before image reuses the minimum-field contribution's after capture at `18ae8dc411ba248a1c911891c658838ba2c436ae`. Its entire `crates/ui-egui/src/brush_sections.rs` is byte-identical to this PR's baseline `04f0b0e8`; the minimum-field fix is already present. This is a historical capture, not a newly rendered baseline build.
- Shape Dynamics: Size Control Off, Minimum Diameter 20%; Roundness Control Off, Minimum Roundness 35%. After captures retain 123 Fade steps while Off.
- `3825-fade-active.png`: Size Control Fade with 123 steps; other controllers remain Off.
- `3825-fade-transfer.png`: Control Off for Opacity, Flow, Wetness and Mix, with minimums 40%, 55%, 60% and 75% retained. The last rows are below the scroll viewport.
- Standard captures: 1200 × 900, scale 1. `3825-fade-scale2.png`: 900 × 720 logical points, scale 2. Light capture: Pro Light.

Build: `CARGO_INCREMENTAL=0 cargo build --locked -p photocraft-ui-egui --features photocraft-io/heif --example snapshot`.

After script (use with `snapshot --out <png> --size 1200x900 --scale 1 --script '<JSON>'`):

```json
[
  ["engine.execute", {"command":"prefs.set","params":{"path":"interface.language","value":"en"}}],
  ["engine.execute", {"command":"file.new","params":{"width":480,"height":360}}],
  ["engine.execute", {"command":"tools.setBrush","params":{"brush":{"shapeDynamics":{"enabled":true,"size":{"control":"off","minimum":0.2,"fadeSteps":123},"roundness":{"control":"off","minimum":0.35}},"transfer":{"enabled":true,"opacity":{"control":"off","minimum":0.4},"flow":{"control":"off","minimum":0.55},"wetness":{"control":"off","minimum":0.6},"mix":{"control":"off","minimum":0.75}}}}}],
  ["ui.set", {"theme":"proMedium","panels":{"brush_settings":true},"brushSection":1}]
]
```

Use `brushSection:6` for Transfer, `theme:proLight` for light, or `size.control:fade` for active Fade. All 14 bundled themes were rendered and visually inspected at 1200 × 900 and 900 × 720, scale 1. Tests additionally cover every controller, disabled input, reactivation and command routing in all five dynamic sections.

This PR addresses the shared Control-row layout/Fade-field subset of #3825. The tablet warning, other layout items and brush-feel assessment remain open. The screenshot assets live only in this fork evidence branch, outside the code PR.
