# Brush Settings: named, stateful section lock controls

Source before: 583425b4df1c57a4515f591fed64fcedf8440cba. Source after: 17abae97413def4f80cc84b7d41f819d6cff970e.

While following the lock controls in [#3477](https://github.com/storytold/photocraft/issues/3477), inspection found that the twelve existing lock hit targets did not describe themselves to AccessKit. The issue does not explicitly report this accessibility defect; this is a related defect reproduced in the current implementation. No Photoshop parity or screen-reader hardware test is claimed.

Give each existing target a button name composed from the translated Lock label and section name, plus its current toggled state. Keep the same IDs, rectangles, pointer dispatch, icons and tools.setBrush command path. No new command, asset, dependency or localization key.

Two regressions fail on the before source because the named lock controls do not exist in the accessibility tree. After the fix they verify all twelve named/stateful controls across fourteen themes, enabled lock access on disabled sections, Space activation without selecting/enabling a section or changing other brush/preset fields, one minimal command per activation, and replay in a fresh engine session. Full current UI + HEIF/examples/doctests: 2795 passed, zero failed, 12 ignored. Strict all-target Clippy (-D warnings), fmt, layers and wasm passed.

The same original full-app egui/wgpu driver runs before and after. Four cases cover Medium, Light, 2x scale and Turkish, with preference-selected themes/language verified at runtime. A synthetic white document and non-default working brush are used. Both phases click the same measured Scattering lock target; the pointer emits one lock-only command and leaves all other fields, saved presets and selected section unchanged. Before, none of the twelve section-specific lock button names can be found. After, all twelve names and toggle states are present; the named Scattering button can also be focused and activated with Space, producing one lock-only command. Raw observed nodes and states are in the two JSON files.

All eight images were inspected. Each corresponding before/after pair is pixel-identical: the change is accessibility metadata, not a visual redesign. The screenshots establish visual preservation; the AccessKit records and input tests establish the fix. No performance, stroke quality or readiness percentage claim is made.

To reproduce, copy the included driver to crates/ui-egui/examples/brush_lock_a11y_evidence.rs in a checkout of each source revision and create an output directory. Run `cargo run --locked -p photocraft-ui-egui --features photocraft-io/heif --example brush_lock_a11y_evidence -- before OUTPUT before-state.json` on the before revision; repeat with `after OUTPUT after-state.json` on the after revision. Remove the temporary example before running the full package gates.

The temporary driver is original @augrclk code under the included MIT OR Apache-2.0 licenses. Captures document the app's existing UI; its asset licenses and brand restrictions remain unchanged. No proprietary assets or personal images were used.
