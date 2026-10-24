# Release 0.4.0 compiler package identity

The native Linux capture at `2abcde640092d97e5f7562d334d458c5e03d8fbe`
contains the same unsafe diagnostics as the green capture at
`659cc64a26e91f8c4ef5e70e4dc99b519156dd0d`. The approved TKERNEL package
version changed from 0.3.6 to 0.4.0. Its source tree did not change.

The retained archive contains both complete compiler JSONL streams, reports,
stderr, locked dependency records, workspace/toolchain configuration, input
hashes, the original comparison, and a portable verifier. Extract it into a
scratch directory and run `python3 martensite-040-inventory-reconciliation/verify.py`.
The verifier uses the retained files; it needs no compiler, Git checkout, network
or dependency cache.

After normalizing the physical checkout prefix and that exact package identity,
all 5,155 raw unsafe diagnostic events match, including rendered warnings,
source spans, expansion chains and target metadata. All 449 ordered summary
rows match after changing only the 421 TKERNEL labels. The three unsafe-emitting
source trees match the captures and merged main `991df6d7`; all 90 external lock
entries, compiler and build configuration are unchanged. The comparison report
records the revisions and Git tree identities.

A separate independent comparison restricted normalization to the applicable
path and package fields and found no remaining difference. No count, location,
diagnostic, target, compiler check or acceptance rule changed.

The four supported target baselines change only active TKERNEL labels: Linux
421, Intel macOS 416, ARM macOS 416 and Windows 405. Existing reconciliation
metadata is retained. Linux is verified by the actual capture above. The other
three native targets remain **pending** the exact release matrix; these label
updates do not establish a native pass. The unsupported Linux ARM baseline is
unchanged.
