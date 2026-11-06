# Selected pipeline observation

This private `cfg(test)` module observes the pipeline used by a completed ordinary frame. It changes no shader, image threshold, public renderer option, API inventory, or completion status.

`completed_owned_frame` retains the same frame resources after the ordinary submission fence and state commits. The diagnostic captures those resources, checks that one layer contains one ordinary uploaded draw, then borrows that draw's pipeline. The canonical ordinary recorder binds this exact field. OIT, capping, clipping, multisampling and other specialized paths are refused. No pipeline is recreated for inspection.

The test constructor admits the selected physical device, enables the executable feature and applies the capture flag at the existing graphics-pipeline creation sink. The profile stays fixed for that runtime. Normal constructors keep `None`, their original extension/feature chains and their original flags. Module records come from successful creation and replacement calls, including the final fragment replacement.

The reader accepts at most 16 executable properties and 16 representations. Two bounded representation calls retain full metadata and owned bytes, with an 8 MiB query limit and a 16 MiB operation-family limit. Incomplete results, changing counts or metadata, ambiguous reported stages and missing assembly remain typed gaps. Stage flags are driver hints, not a universal one-to-one hardware mapping. Names and binary/text bytes are retained unchanged.

The family budget is reserved before allocating or dispatching representation byte reads. Failed calls keep their reservation and all allocated bytes. Field-wise raw snapshots preserve allocated and reported counts, API status, and unknown validity on errors. Capture and module inputs remain attached to later query failures; retention never charges the same bytes twice.

CPU controls:

```sh
cargo test -p martensite-vulkan --lib backend::diagnostic_executable:: --locked --offline -- --show-output --test-threads=1
```

Two deliberately ignored hardware entries require explicit selection with `--ignored --exact`: `gradient_background::device_tests::elliptical::executable::actual_selected_ellipse_pipeline_preserves_full_buffers_and_retains_ir` and `gradient_background::device_tests::elliptical::executable::actual_selected_device_reports_typed_executable_admission`. The first requires a retained artifact directory and an admitted device. The second records unsupported admission without fabricating IR or choosing a different device.

The capture entry keeps all existing six ellipse and four older-gradient pairs, including their solid controls, then observes the same 64×64 varying fan and two constant controls. Its Off/On/Off buffers must agree exactly. These contexts use the new test executable; the first Off context is not a pre-patch Main producer. Executed 2026-10-27 on AMD Radeon 8060S (RADV, Mesa 26.1.6) with pinned validation 1.4.313: both entries passed, the Off/On/Off color and depth buffers were byte-identical, and the selected fragment executable returned NIR, ACO IR and final assembly. Lavapipe reported a typed unsupported admission (extension absent). Evidence is kept outside the repository. Native image differences remain failures, even when introspection succeeds.

This checkpoint imports neither the old experimental transform-feedback path nor its float attachment override. It grants no rendering repair, full original-case, platform or release credit. Remove this diagnostic after the ellipse blocker is resolved and its final rendering regression replaces it.
