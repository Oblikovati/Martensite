# Dataset executable fingerprint cost (#228)

The isolated case runner hashes its executable before launch, in the child, and
after exit. Those checks remain unchanged. Development builds now optimize the
`sha2` package; the test profile inherits that setting. This fingerprint change
left kernel compilation, debug assertions, process limits, fixtures and test
selection unchanged. The later [TKMath optimization](assembly-test-cost.md) is
measured separately.

## Measurements

On Linux x86_64, the TKShHealing `sewing_tol_1::a2` case used a roughly 315 MiB
debug executable. At `cbd543a2`, it passed in 19.28 seconds; a traced repeat took
19.40 seconds (19.03 user CPU seconds, 334364 KiB peak RSS). Three executable
reads were each followed by approximately 6.3 seconds of hashing. The child
entry-to-completion interval was 56 milliseconds.

At the fix baseline `4df6d1ec`, unoptimized repeats took 29.24 and 29.62 seconds
(the latter used 29.03 user CPU seconds and 335500 KiB peak RSS). With
the profile override it took 1.65 seconds, then 1.29 seconds (0.79 user CPU
seconds, 335816 KiB peak RSS). This was a shared, busy host, not a controlled
benchmark. The measurements show a substantial CPU improvement, not a memory
reduction or an exact full-suite speedup.

[CI run 39793111570](https://github.com/Oblikovati/Martensite/actions/runs/39793111570)
spent 7036.60 seconds on the 927-case TKShHealing dataset suite. That historical
run is not a matched before/after benchmark of the fix.

## Reproduction

Run the same assertion-bearing case with and without the override:

```sh
cargo test --locked --config 'profile.dev.package.sha2.opt-level=0' -p martensite-tkshhealing --test dataset_cases sewing_tol_1::a2 -- --exact --nocapture
cargo test --locked -p martensite-tkshhealing --test dataset_cases sewing_tol_1::a2 -- --exact --nocapture
```

The explicit fingerprint probe prints executable size and elapsed time for three
hashes. Compare the same binary size and host conditions; compilation is excluded.
It is not a timing threshold in normal CI:

```sh
cargo test --locked -p martensite-process-policy --features case-runner measure_three_executable_fingerprints -- --ignored --nocapture
```

The ordinary regression test checks the SHA-256 known answer, same-length content
changes, and missing-file rejection. No dependency versions or release profiles
changed. No fingerprint cache or skipped integrity check was introduced.
