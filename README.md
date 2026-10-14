# Brush Settings section locks: visible keyboard focus

Follow-up in existing [PR #5365](https://github.com/storytold/photocraft/pull/5365). Before source: 17abae97413def4f80cc84b7d41f819d6cff970e; after source: 61b0f4699f6d358586db5d887a6817076dcc3b91.

The earlier head already names the twelve section lock buttons and reports their state. It does not paint a visible indicator when one is keyboard-focused. Standard PhotoCraft icon buttons use the theme accent for this feedback. Paint a one-point theme-accent outline inside the existing lock target only while it has focus; keep its glyph, ID, target and command path.

The new focus regression fails at the previous PR head, while the two name/state regressions pass. All three pass after this change. The focus regression covers fourteen themes, locked/unlocked states, focus handoff and Tab leaving a lock, with no brush/preset/selection edits or commands from focus changes. Full UI + HEIF/examples/doctests: 2796 passed, zero failed, 12 ignored; strict all-target Clippy (-D warnings), fmt, layers and wasm passed.

Ten native full-app egui/wgpu captures were inspected. Medium, Light, 2x and Turkish fixtures request focus through AccessKit and dispatch ordinary Space and Tab events through the app. The canvas window.togglePanels Tab binding is cleared in these isolated test preferences, as documented for keyboard navigation fixtures; the product's default canvas shortcut is unchanged. Both source revisions have the same focus state. Before, the indicator is absent; after, it is present. Space sends one lock-only command, preserves focus feedback and all other fields/presets/selection; Tab leaves the lock and removes its indicator without editing settings. The two idle captures are pixel-identical. In each focused pair, changed pixels are confined to the lock target (including antialias padding); the JSON summary records the measured bounds.

The [original eight name/state captures](https://github.com/augrclk/photocraft/blob/9bdbaa73beab00d2d4fe6814b109ba50877a58bc/README.md) remain historical evidence at 17abae97413def4f80cc84b7d41f819d6cff970e; they are not relabeled as images of the new head. Current tests still cover that metadata and activation behavior. No native hardware screen-reader, Photoshop pixel-oracle, performance or readiness claim is made.

To reproduce, copy the included driver to crates/ui-egui/examples/brush_lock_focus_evidence.rs on each source revision and create OUTPUT. Run `cargo run --locked -p photocraft-ui-egui --features photocraft-io/heif --example brush_lock_focus_evidence -- before OUTPUT before-state.json`; repeat with `after OUTPUT after-state.json` at the after revision. Remove the temporary example before the full package gates.

Original @augrclk driver under the included MIT OR Apache-2.0 licenses. Captures document existing app assets; their licenses and brand restrictions remain unchanged. Synthetic white document, no proprietary assets or personal images.
