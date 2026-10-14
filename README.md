# Primary tip selection: respect Capture Brush Size in Preset

Selection follow-up in existing [PR #5338](https://github.com/storytold/photocraft/pull/5338). The size-capture option is part of [#3477](https://github.com/storytold/photocraft/issues/3477)'s preset workflow; grid layout is tracked in #3825.

The model explicitly says that `PresetCaptures::size = false` preserves the tool's size when a preset is picked (`BrushPreset::picked_over`). The primary Brush Tip Shape grid instead assigned the saved size unconditionally. Before source: 77d70a4d58f2f30ae10512eba564dcaf0f88bb7b. After source: ae5f013108849df5e8ba67c1fe48e6a799dd8409. The fix checks the existing capture flag only after a primary cell is picked; it does not replace the working brush with the whole preset. Dynamics, flips, locks and the spacing switch remain with the working brush, as this grid intends.

## Native reproduction

Same original full-app egui/wgpu driver in both phases; English, white 480x360 document, verified preference-selected Medium/Light themes, 1x/2x scales. Save an original synthetic 2x2 sampled tip through brush.presets.save, with saved size 73 and captureSize false or true. Set the working tool to size 17.25, with different hardness/angle/roundness/spacing and other working settings. Click the actual named cell in the Brush Settings window.

Before, all four cases become size 73, including the three size-off cases whose model oracle says 17.25. After, size-off cases keep exactly 17.25 and omit size from tools.setBrush; the size-on control still becomes 73. Each selection records brush.presets.setCurrent and tools.setBrush once. Saved preset structs remain unchanged; all non-tip working fields are preserved. Eight native captures inspected. The size field rounds 17.25 for display but the underlying state is checked exactly.

## Tests and scope

The existing selection/command test was extended for both capture choices, fractional working size, unchanged source presets, exact minimal patches and replay through a fresh engine session. It fails on the old head only in the size-off arm, and passes after the fix. No additional test function was needed. Full current UI + HEIF/examples/doctests: 2797 passed, zero failed, 12 ignored; strict all-target Clippy (-D warnings), fmt, layers and wasm passed.

The original eight new grid/viewport regressions, all-theme geometry, keyboard/scrolling tests and responsive-window tests pass in the full current run. Original [18 layout captures and 24 MP rendering measurements](https://github.com/augrclk/photocraft/blob/b195afe128a33fdab89134b0f313e92401fcf282/README.md) remain historical evidence at 77d70a4d58f2f30ae10512eba564dcaf0f88bb7b; they are not relabeled as measurements of this new head. The rendering/virtualization function is byte-identical in the follow-up (sha256 d0df42da08592e082870996b637190ebbe96b01e76a27b71e32ff7e486bbe8eb); the change is in the picked-cell event branch. No new performance number or budget claim is made.

The generated tip fixture and temporary driver are original @augrclk code under the included MIT OR Apache-2.0 licenses. Captures document the existing app UI; its asset licenses and brand restrictions remain unchanged. No Photoshop assets or pixel oracle were used.
