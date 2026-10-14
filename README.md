# Brush Settings original-size affordance evidence

Size-button subset of [#3825](https://github.com/storytold/photocraft/issues/3825). Main already restores sampled-tip sizes in both sections. The remaining defect is the disappearing control for computed tips and its enabled state when there is no size change. The broader assessment remains open.

Before: production main 99e1138877e6aa7e856c6883b415e708d042fc59. After: tested source 85f75d1f9e9cde7d24889c5f6fb578b7921e5f48. Real full-app egui/wgpu rendering; original generated 20x12 grayscale tip fixture, no Photoshop assets or pixel oracle. Eight native captures inspected. Medium and Light themes are applied through prefs and verified against actual UI theme and Tokens; the fourth case renders at 2x pixels per point.

## Observations

- Computed primary and Dual Brush tips: before, no Restore original size button; after, a visible disabled button.
- Sampled primary tip at its original size (20 px): before, enabled; after, disabled.
- Sampled Dual Brush tip changed to 37.25 px: enabled in both phases; clicking restores 20 px through exactly one tools.setBrush command. Its size is displayed rounded by the existing field, but the exact imported value is retained while idle.
- Merely displaying all four cases leaves the brush and journal unchanged. A no-op click makes no command in either phase; the change makes that unavailability visible and accessible.

## Validation

Three new regressions fail on unchanged main and pass after the fix. Covers all 14 actual themed row palettes, a fixed button/field rectangle, disabled pointer and keyboard input, fractional-size retention, clamped original sizes, both real panel command routes, preset preservation, and a disabled Dual Brush section.

Full UI package with HEIF, examples and doctests: 2792 passed, zero failed, 12 ignored. All-target Clippy with -D warnings, fmt, layers and wasm passed. No paint algorithm, engine command, format path, model schema, dependency or performance budget changed.

The temporary evidence example is kept here rather than in the source PR. Run it with the phase (before/after), output directory, and JSON metadata path. Both screenshots and raw observed state are included. The driver and generated tip fixture are original work by @augrclk under the repository's MIT OR Apache-2.0 licenses included here. Captures document the existing app UI; its asset licenses and brand restrictions remain unchanged.
