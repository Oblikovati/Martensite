# Default surface measurements against the previous release

The actual published `v0.3.6` revision `98d408444db53e6f4410adf578590e53d9062aff`
and candidate `8190ae446554eb9007d15eae98982b2369558f37` execute the same retained
producer over six owned shapes and five modes. All thirty rows, sixteen raw
property fields and explicit error bits match the owned payload exactly. The
independent closed-form and enclosure controls pass.

| Median of three interleaved processes | Published | Candidate |
| --- | ---: | ---: |
| Parent wall, milliseconds | 11.193 | 8.463 |
| Child CPU, milliseconds | 10.994 | 8.377 |
| Peak RSS, KiB | 12,224 | 12,588 |

The short process measurements include startup, construction and output on a
shared host. They do not establish a controlled speedup or a general performance
change. GNU time rounded most CPU readings to zero at its 0.01-second resolution;
the retained supplemental measurements use high-resolution child-CPU and monotonic
wall windows. The first coarse pass remains in the evidence.

Compiler, profile, configuration and default-feature fingerprints agree, with
empty recorded Rust flags. Full build-launcher arguments and environment were not
independently retained, so wrapper and profile-override absence are not proven.
This is a bounded compatibility measurement, not full-corpus accuracy evidence.
The adaptive narrow-square zero is retained as existing behavior.

The [sealed proof](surface-defaults-proof.tar.gz) contains the complete report,
raw records, inputs, fingerprints and portable verifier. Check its companion
checksum, extract it into an empty temporary directory, enter the extracted
`martensite-040-surface-benchmark` directory and run `python3 -I verify.py` there. Verification reads the retained evidence; rerunning the measurements
requires the separately recorded binaries. Independent review found no remaining
claim or artifact mismatch.
