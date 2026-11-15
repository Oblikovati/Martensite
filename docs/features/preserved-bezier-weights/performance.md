# Coefficient-retention performance qualification

The final campaign compared identical native construction workloads against
main `b63b9b8ba4dd7d25a951dd45bfd144b282715940`. Baseline libraries were built in
the #456 lane; their relevant geometry/filling/codec sources matched that main.
This was not a previous-release benchmark or a complete body-operation benchmark.

Both sides constructed the same coefficient buffers, two native boundary curves,
and a ruled side surface. The strict factory admits private owned curve state and
builds the surface directly from those actual rows. Legacy filling still uses its
existing classification and admission behavior. A shared contiguous copy path
avoids repeated indexed reads and writes when the complete requested ranges are
present. Malformed ranges retain the original indexed fallback and error order.

## Measurements

Native libraries used the same unoptimized profile; the standalone probe used
`rustc -C opt-level=2`. Each process was pinned to CPU 3. Twelve balanced blocks
rotated baseline, candidate legacy and candidate strict order. GNU time measured
child peak RSS; the native thread CPU clock measured each construction batch.
The root agent reserved its feature builds during the final 13.28-second window
on 2026-11-15, 07:07:09–07:07:22 UTC. Unrelated host activity was uncontrolled.

Median thread CPU microseconds per complete patch:

| Poles per curve | Weights | Main legacy | Candidate legacy | Owned factory |
| --- | --- | ---: | ---: | ---: |
| 2 | Unit | 3.410 | 3.311 | 2.200 |
| 2 | Rational | 5.041 | 4.913 | 2.733 |
| 8 | Unit | 5.359 | 4.644 | 3.494 |
| 8 | Rational | 7.641 | 6.472 | 3.731 |
| 26 | Unit | 10.829 | 8.440 | 7.194 |
| 26 | Rational | 15.261 | 10.757 | 6.516 |

Separate decoder workloads used the same six pole/weight combinations. All 24
candidate-versus-baseline timing comparisons had negative 95% bootstrap intervals
for the median within-block CPU difference. Literal encoded bytes agreed across
all three variants for each of the twelve workloads before timing. Each measured
process also checked coefficient witnesses, native evaluation finiteness and
final encoding. These checks concern the native patch and codec, not immutable
body publication or the feature's certified row acceptance.

Peak RSS medians were about 9.7–9.9 MiB. Candidate strict minus baseline medians
ranged from −48 to +112 KiB across workloads. This campaign does not establish
peak-memory parity. Source inspection shows no allocation in the contiguous copy
path; the owned factory avoids the legacy fill's copied curve states and repeated
surface capture. Those facts do not replace a representative body-level memory
comparison. No universal performance claim follows from these measurements.

## Earlier investigations

The retained records include unsuccessful approaches. The original borrowed-fill
opt-in repeated validation and capture; a quiet confirmation measured a real
approximately 3% slowdown for a 26-pole unit-weight fill. Fusing validation with
another owned copy made that workload slower and was discarded. The owned patch
factory removed the repeated handle captures. Its first campaign was faster on
strict construction but measured a 3–5% legacy construction slowdown. The reviewed
contiguous copy path removed that observed slowdown in the final campaign.

The first two campaign files contain unusable peak-RSS fields: Python inherited
high-water state before launching the probe. Later campaigns used GNU time's child
measurement instead. Earlier per-mapping inspection also found a four-KiB ELF
zero-fill layout difference, not additional logical BSS storage. Neither result
supports silently correcting or normalizing measured peak RSS.

Compressed raw records and deterministic paired summaries are in
[references](references/). The final records include probe hashes, library hashes,
source hashes and the exact runtime observations. Temporary probe sources and
executables were removed after verifying no live process referenced them; the
cleanup receipt records their hashes. Feature caller wiring,
its accepted workload and the canonical affected-package gate remain separate
qualification requirements. Existing APIs, defaults and binary versions remain
unchanged; no package version or release tag is changed.
