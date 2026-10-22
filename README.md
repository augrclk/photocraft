# PhotoCraft: main integration, 22 October 2026

Eight existing PR branches integrate upstream main 6649b78526826cc543f5aced1195c06c0b7e850a. Each source below passed its full touched-package tests, strict all-target Clippy (including desktop compilation), fmt, layer and wasm checks; engine branches also passed parity and panic-hunt. The PR descriptions carry exact per-head package coverage and counts. Public history is retained through merge commits, and both progress histories and independent incoming/PR regression tests remain.

| PR | Tested source | Inspected native captures |
| --- | --- | --- |
| #4829 | aaa0d505224e3a4ad620cfdb4978462b0d0e23dc | [oct22-4829-fade.png](oct22-4829-fade.png) |
| #5224 | 374ff76458b139c840dcdb3dc12c31d38586aa36 | [oct22-5224-spacing.png](oct22-5224-spacing.png) |
| #5250 | 1b9aa44e602060b82671a5c7125a4f9d9d2a99cd | [oct22-5250-scatter.png](oct22-5250-scatter.png) |
| #5269 | 871b357e36dc8f974aa94b2558656fadff0cc6a7 | [oct22-5269-geometry.png](oct22-5269-geometry.png) |
| #5278 | 7a2fb82843a27c04b434b3e66654ff49243fa0b4 | [oct22-5278-brush.png](oct22-5278-brush.png) · [oct22-5278-mixer.png](oct22-5278-mixer.png) |
| #5338 | 86c40a5c4e6d549925c9a552f568be46cb4fdfe3 | [oct22-5338-grid.png](oct22-5338-grid.png) |
| #5352 | 825766445997671fa4abdfd5b22ef7a0ff78f455 | [oct22-5352-panel.png](oct22-5352-panel.png) · [oct22-5352-panel-dialog.png](oct22-5352-panel-dialog.png) |
| #5356 | 5262a21e7e3ce899c13fff36d8f59d8a95431a23 | [oct22-5356-restore.png](oct22-5356-restore.png) |

## What these captures show

Native egui/wgpu on Apple M4 / Metal, English interface, Pro Medium, synthetic 480x360 document, 1200x1000 screen points at 1x. The exact control requests are in each matching *-fixture.json file.

The views cover stable Fade/curve controls, the shared Spacing header, Scatter's Both Axes header, tip geometry, compact Transfer Minimums with Brush versus Mixer Brush applicability, the compact tip grid, preset-panel workflow and the disabled size-restore affordance. Some panels scroll naturally; the test suites assert controls beyond the captured viewport and all supported themes. Captures document the current merged source, not a Photoshop pixel oracle, physical-tablet test or new performance result.

The #5278 caller now passes the raw Minimum key to the shared widget, which translates it once. It retains incoming Mixer Brush applicability and the curve-bearing Fade width reservation. The #5338 tip selection keeps failed-pick atomicity while passing the active tool to the section renderer. #5352's helper now re-queries the popup row after hover, because it can clamp upward by 28 points on the taller Brushes tab; one click and all existing dialog, preset and journal assertions remain.

Supplementary macOS desktop runs found intermittent upstream socket/log tests. The Unix-socket handoff failure recurred even in an isolated process: the client reported Invalid argument (OS error 22), retried and timed out. Its module matches upstream main byte-for-byte. The [bounded diagnostic log](desktop-socket-diagnostic.log) includes temporary print-only instrumentation identifying that error; its line numbers include those extra prints. All diagnostic edits were then removed, restoring the exact upstream module. The root cause remains unestablished. The required gates cover all touched UI/engine package tests and desktop all-target strict Clippy. #5352 and #5338 also have a successful broader desktop run, with the scheduling recorded in their PRs; those runs do not establish that the intermittent upstream failure is fixed.

The optional desktop reproduction runners add no retries, skips, mocks or changed assertions. They execute every selected desktop unit test once per process while UI/integration harnesses use their normal scheduling. The first validated branch used shared-process serial execution instead.

## Reproduce

For each row, check out that exact source revision and build the standard native snapshot example:

    cargo build --locked --profile test -p photocraft-ui-egui --features photocraft-io/heif --example snapshot
    python3 capture.py --repo /path/to/photocraft --out /path/to/captures --pr NUMBER

The fixture JSON files retain exact requests for any extra observed click captured in review. No PhotoCraft source files were modified by this driver. Earlier before/after screenshots and timings linked from the PRs remain historical proof at their original revisions.

To use the optional desktop-unit isolation on this native Apple Silicon host, keep both oct22-test-runner files together, make the shell script executable and set CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER to its absolute path when invoking cargo test. The test source and assertions stay unchanged. Use the host's target triple for other platforms.

Original synthetic fixtures and driver code: @augrclk, MIT OR Apache-2.0, licenses included. Captures show the existing PhotoCraft UI; its existing asset/brand restrictions remain in effect.
