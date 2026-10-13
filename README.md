# Transfer Minimum layout evidence

Addresses only the Transfer Minimum layout subsection of https://github.com/storytold/photocraft/issues/3825.

Before: snapshot built from clean main `99e1138877e6aa7e856c6883b415e708d042fc59`. After: tested source `b621108e0d18d904adc628b2f611b18ce6205f45`. All 16 native images inspected on Apple M4 / Metal. Synthetic white 480×360 document, English UI, Mixer Brush, Transfer enabled. Jitter amounts 3/7/11/13%, exact stored minima 0.173/0.283/0.433/0.643 (shown as 17/28/43/64%). Each Control is Pen Pressure in active fixtures and Off in inactive fixtures; 37 Fade steps retained. Angle −23°, roundness 0.473, size 47 px. Pro Medium and Pro Light at 1200×900, plus Pro Medium at 900×720/2× scale. Identical values in each phase.

The medium-lower captures click the visible section scrollbar at logical screen position (839, 617), showing Mix Minimum and the following Mixer Brush settings. Images and the reproduction script live on a separate fork branch, outside the code PR.

| Before | After |
| --- | --- |
| ![Separate full-width blocks](3825-minimum-before-medium-active.png) | ![Compact Minimum rows](3825-minimum-after-medium-active.png) |
| ![Mix minimum before](3825-minimum-before-medium-lower-active.png) | ![Mix minimum after](3825-minimum-after-medium-lower-active.png) |

Four new egui tests: one layout regression fails on the baseline and passes afterward; three behavior guards pass before and after. Layout is covered in all 14 themes at 280/340 logical-pixel widths, with Control Off/Fade/Pen Pressure. Guards exercise all four minima, numeric and pointer limits, label scrubbing, exact fractional-value retention and independent Off/Fade reactivation. The real window test asserts minimal tools.setBrush payloads and one journal entry for a multi-frame scrub. Its initial expectation was corrected to allow the existing coalesce metadata; no production command change was needed.

Full UI/HEIF package including examples and doctests: 2,793 passed, zero failed, 12 ignored. Strict all-target Clippy, fmt, layers and wasm passed.

```sh
cargo test --locked -p photocraft-ui-egui --features photocraft-io/heif --no-fail-fast
cargo clippy --locked -p photocraft-ui-egui --features photocraft-io/heif --all-targets -- -D warnings
cargo fmt --all -- --check
cargo xtask layers
cargo xtask wasm
cargo build --locked -p photocraft-ui-egui --features photocraft-io/heif --example snapshot
python3 capture.py --repo /path/to/photocraft --out /path/to/captures --phase after
```

For before captures, build the exact baseline and pass --phase before.
