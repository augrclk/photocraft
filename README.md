# Brush Settings: #5338 review revision

Source: c022e82e104b8f9fefc3eec865baa55342101cfb, child of 86c40a5c4e6d549925c9a552f568be46cb4fdfe3. Existing PR: https://github.com/storytold/photocraft/pull/5338.

Both CodeRabbit findings are addressed. The body uses the finite height left after the footer, with no positive floor and the existing 450-point unbounded fallback. Both section-list and options viewports are bounded, so the section list cannot force the footer offscreen. The existing resize regression checks 900x600, 900x480 and 900x320 across all 14 themes and growth back to 600. It directly checks the rendered preview mesh against its clipping rectangle and screen bounds, footer action buttons, lower Spacing labels after scrolling, and unchanged brush/journal state. The original production code fails this expanded regression.

Full UI-egui package with HEIF, examples and doctests: 3833 passed, 0 failed, 17 ignored. All-target Clippy (-D warnings), fmt, layer and wasm checks passed. No engine command, image codec or desktop socket module changed. Hosted CI is separate from local results.

Inspected native egui/wgpu captures on Apple M4 / Metal, English, synthetic white 480x360 document, 900-point screen width, 1x scale. The fixtures give exact requests and source-tree identity. The 600-point view uses Pro Medium; 320-point views use Pro Medium and Pro Light. These are current-source snapshots, not a Photoshop pixel oracle or physical-tablet measurement.

- [oct22-5338-review-proMedium-600.png](oct22-5338-review-proMedium-600.png)
- [oct22-5338-review-proMedium-320.png](oct22-5338-review-proMedium-320.png)
- [oct22-5338-review-proLight-320.png](oct22-5338-review-proLight-320.png)

To replay, keep capture.py and the three fixture files together, check out the exact source above in a separate PhotoCraft checkout, then run:

    cargo build --locked --profile test -p photocraft-ui-egui --features photocraft-io/heif --example snapshot
    python3 /path/to/this-evidence/capture.py --repo /path/to/photocraft --out /path/to/captures

The public driver verifies the checked-out source tree and uses the exact fixture requests, with no task-local validation files required and no PhotoCraft source edits. Original driver/fixtures are MIT OR Apache-2.0; existing PhotoCraft asset/brand restrictions remain.
