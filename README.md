# Brush Settings: Hue Jitter amount track — #3825

Native offscreen captures of PhotoCraft's real Color Dynamics section, on Apple M4 / Metal. The fixture is a generated white 480×360 document with Hue Jitter 37%, Saturation Jitter 23%, Brightness Jitter 41% and Purity −16%. No external artwork or brush pack.

Before uses the exact baseline production source at `6f85054288a564f9fbbccb99268ffdb34e581584`. Only regression tests and progress documentation were added when the before binary was built. After uses the source commit recorded below; both builds use the same capture script and brush settings.

The rainbow Hue Jitter track becomes the same themed amount track as the other jitter controls. The percentage, neighboring settings and brush preview are preserved. Color pickers elsewhere keep their hue spectrum.

Captures: Pro Medium and Pro Light at 1200×900 logical points, scale 1; Pro Medium at 900×720 logical points, scale 2. All six PNGs were inspected. A regression also checks the rendered themed track in all 14 themes. A real panel interaction test checks numeric editing, dragging to the 100% limit, neighboring settings, `tools.setBrush` routing and journal replay.

Reproduce the after captures using the included `capture-3825-hue.py` in a `work/` directory next to a `work/photocraft` checkout and an `outputs/` directory:

```sh
CARGO_INCREMENTAL=0 cargo build --locked -p photocraft-ui-egui --features photocraft-io/heif --example snapshot
python3 capture-3825-hue.py after
```

These review artifacts live in a separate fork evidence branch and add no image assets to the code PR. This contribution addresses only the Hue Jitter appearance item in #3825; other layout and functional items remain open.

After source: `441c663775d260e47be4463912a8bfd7bee5c77b`. Local UI library/integration tests with HEIF: 2,699 passed, zero failed, 12 ignored. All-target strict Clippy, fmt, layers and wasm passed.
