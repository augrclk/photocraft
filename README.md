# Brush Tip Shape Spacing header evidence

Addresses only the duplicated Spacing label / combined checkbox and percentage header subsection of https://github.com/storytold/photocraft/issues/3825.

Before source: clean main `99e1138877e6aa7e856c6883b415e708d042fc59`. After source: `02fff63cb4b8a439dab8a3bd53376f96367ee991`. All twelve captures were generated with the native egui snapshot example and inspected (Apple M4 / Metal). Synthetic white 480×360 document, English UI. Pro Medium and Pro Light at 1200×900; Pro Medium at 900×720 with 2× display scale. Each enabled/disabled fixture retains `spacing = 0.427` (shown as 43%). The script clicks the section scrollbar so the whole spacing control is visible.

| Before | After, On | After, Off |
| --- | --- | --- |
| ![Duplicated labels](3825-spacing-before-medium-on.png) | ![Single shared header](3825-spacing-after-medium-on.png) | ![Value retained while disabled](3825-spacing-after-medium-off.png) |

Capture names identify phase, theme/scale and checkbox state. Images and this script live in a separate fork branch and are not part of the code PR.

## Verification

Three new egui regression tests. The single-label/header-layout regression fails before the fix (two labels) and passes afterward across all 14 themes and both checkbox states. Two behavioral guards pass both before and after: disabled keyboard/text and slider drags, active edits, 1..1000% limits, fractional-value retention on toggle, and exact existing `tools.setBrush` journal routing.

Full UI package with HEIF, including example compilation and doctests: 2,792 passed, zero failed, 12 ignored. Strict all-target Clippy, fmt, layers and wasm passed.

```sh
cargo test --locked -p photocraft-ui-egui --features photocraft-io/heif --no-fail-fast
cargo clippy --locked -p photocraft-ui-egui --features photocraft-io/heif --all-targets -- -D warnings
cargo fmt --all -- --check
cargo xtask layers
cargo xtask wasm
cargo build --locked -p photocraft-ui-egui --features photocraft-io/heif --example snapshot
python3 capture.py --repo /path/to/photocraft --out /path/to/captures --phase after
```

For before captures, build the exact before revision and use `--phase before`. The same fixture is used in both phases.
