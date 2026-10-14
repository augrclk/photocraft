# Fresh brush preset names: case identity regression

Follow-up in existing [PR #5352](https://github.com/storytold/photocraft/pull/5352), within the New Brush Preset workflow of [#3477](https://github.com/storytold/photocraft/issues/3477). No additional PR was opened.

The previous PR head 613e12b8c99ee420514a9892cfee689eb4a84d4f opens the shared dialog correctly but checks generic Brush N suggestions case-sensitively. The engine's preset lookup and save use ASCII case-insensitive identity. With brush 1 / BRUSH 2 already saved, switching Brush to Eraser clears the current preset; the dialog then suggests Brush 1, shows a duplicate warning and disables OK. The fix uses the existing paint::presets::find helper and suggests Brush 3.

Before source: 613e12b8c99ee420514a9892cfee689eb4a84d4f. After source: 9f0dcb5e09f44b17a4861e1f8cda05debeee8de3. Same real full-app egui/wgpu driver, no widget shim. English; a white 480x360 document; custom brush 1 size 31 and BRUSH 2 size 52; real Brush to Eraser switch; actual preference-selected theme checked against UiState and Tokens. Four cases in each phase: Settings menu, picker plus, 2x scale, and menu on the Brushes tab. Eight native images inspected.

Before: Brush 1, warning visible, OK disabled, 38 presets remain 38 and clicking makes no save. After: Brush 3, no warning, OK enabled, 38 becomes 39 with exactly one save command. Every pre-existing preset is unchanged, with full preset-struct comparisons covering names, brush settings and captures. Opening never changes the library or active brush.

Two new regressions fail on the previous PR head and pass after the fix. The unit test checks holes among differently cased numeric names; the full-app test checks both entry points in all 14 themes, real tool switching, OK/Enter and unchanged existing presets. Full current UI package with HEIF, examples and doctests: 2794 passed, zero failed, 12 ignored. Strict all-target Clippy (-D warnings), fmt, layers and wasm passed.

The original confirmation fix retains its own immutable [evidence snapshot](https://github.com/augrclk/photocraft/blob/7cfe04fd6b98ac26d218edf30fdbc693f314b067/README.md) at head 613e12b8c99ee420514a9892cfee689eb4a84d4f: five menu-path assertions failed on main, three prior dialog guards passed, and eight captures showed immediate save before versus deferred confirmation after. Those initial captures are historical evidence of that initial head, not relabeled as new captures. The full current test run revalidates all original confirmation and capture-option tests as well as the two new name regressions.

The driver is an original temporary verification example, kept in this evidence branch rather than the source PR. Driver and fixture code by @augrclk use the included MIT OR Apache-2.0 licenses. Captures document the existing app UI; its asset licenses and brand restrictions remain unchanged. No Photoshop assets or pixel oracle were used.
