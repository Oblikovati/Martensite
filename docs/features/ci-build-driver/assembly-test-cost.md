# Assembly integration test cost

Release rehearsal [40097469762](https://github.com/Oblikovati/Martensite/actions/runs/40097469762)
failed `bugs_modalg_7::bug28402` at its 1,000-second limit on Windows x64 and
Intel macOS (#271). The child had started the first assembly-volume calculation;
it had not reached its volume report or the later assembly/box timing comparison.

The public fixture contains 93,312 placed occurrences of one solid, producing
559,872 face occurrences. Every occurrence has a distinct exact shape/location
key. Caching those keys would add work without a hit. Reusing transformed solid
moments would change floating-point association and integration-error accounting,
so this fix retains the existing integration algorithm and ordered contributions.

A short debugger probe found matrix/coordinate operations, quadrature-table
copies and vector allocation in TKMath. One earlier sample landed in
`metallic::pow` beneath the quadrature error estimator. These samples identify
candidates; they are not percentages of total runtime.

## Measured change

`profile.dev.package.martensite-tkmath.opt-level = 3` optimizes TKMath in
workspace development and test builds. Other package settings stay unchanged.
The compiler command retained `debug-assertions=on` and debug information;
overflow checks retain the enabled development default. Release profiles,
dependency versions, algorithms, expected results and test budgets are unchanged.

The same exact dataset case ran on the shared Linux x86_64 host with Rust 1.99.0.
The baseline checkout was `061fe7ff`; the relevant math, geometry and topology
sources were compared with `f5251eed` and were identical. The intervening
TKernel changes concerned document-file handling, outside this workload.
Existing SHA-256 and big-integer optimization settings were retained in every run.

| Build | Wall seconds | User CPU seconds | Peak RSS KiB |
| --- | ---: | ---: | ---: |
| Baseline | 487.25 | 486.97 | 565,200 |
| TKTopAlgo at opt-level 1, discarded | 460.64 | 460.40 | 565,716 |
| TKMath at opt-level 3, retained | 195.73 | 195.52 | 565,568 |

The retained trial was 2.49 times faster, a 59.83% reduction in elapsed time.
Peak memory was comparable. These are local measurements, not predictions for
hosted runners. The trial build took 33.64 seconds wall time and 104.78 seconds
user CPU, with 1,998,416 KiB peak RSS; it included dependency rebuilds and is not
an isolated compiler-cost comparison.

The complete numerical and diagnostic report was identical across all three
runs after excluding timing lines: 1,160 bytes, SHA-256
`9311656a0d509fb9afde0f50d764fc3fd62479829410a7e33b13b91f21a4ce73`.
It includes assembly and box mass, center of mass, inertia and principal moments
at the existing six-significant-digit display precision. This comparison does
not prove equality of every internal floating-point bit.
All 38 existing BRepGProp toolkit regressions also passed under the retained
setting. Full affected-package and native platform validation remain release
requirements; these focused measurements do not replace them.

Reproduce the case with the retained setting:

```sh
cargo test --locked -p martensite-tkprim --test dataset_cases bugs_modalg_7::bug28402 -- --exact --nocapture
```

For the baseline, add `--config 'profile.dev.package.martensite-tkmath.opt-level=0'`
to the Cargo invocation. Measure the already-built executable separately to keep
compilation out of the runtime comparison. Temporary counting and debugger probes
were removed after investigation; neither memoization nor an algorithm shortcut
was added to production code.
