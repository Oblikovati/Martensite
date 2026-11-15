# Coefficient-retention performance qualification

The main-baseline campaign compared identical native construction workloads against
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

## Previous-release comparison and isocurve correction

A separate disposable checkout built the previous release, `v0.13.0` at
`d051ff699982faba4ec2df3ee9a47e14f38dedbd`. The candidate was compared with that
release on complete native patches, grouped face-table decoding and paired U/V
isocurve construction, with 2, 8 and 26 poles and unit or varying rational weights.
The probe checked a closed-form rational midpoint, both isocurves and the decoded
surface before timing. This remains native patch/codec evidence, not full body
publication evidence.

The first 648-record campaign found a repeatable small isocurve slowdown after the
new axis-convention check. The implementation now computes private isocurve flags
at admission and edit boundaries. The hot path selects one stored boolean as
before; public directional flags and legacy behavior stay unchanged. Sixteen
geometry controls, eight codec controls and strict all-target geometry/BRep
Clippy passed. Independent review checked every flag-assignment path.

The repeat campaign measured commit `b480f8c9` in 12 balanced blocks, rotating
release legacy, candidate legacy and candidate strict order. Libraries used the
unoptimized profile with debug information and incremental compilation disabled;
the probe used `rustc -C opt-level=2`, CPU 3 and native thread CPU time. The root
agent reserved its builds during 16:30:40–16:30:58 UTC on 2026-11-15; unrelated host
activity remained uncontrolled. There were 648 records. Construction medians were
8.1–26.3% lower on the candidate legacy path and 37.0–66.6% lower on the strict
factory. Isocurve comparisons improved or had paired bootstrap intervals spanning
zero. These observations do not prove universal timing parity.

Two short decoder comparisons appeared slower, so a separate confirmation used
10,000 decodes per process instead of 500. Its 72 records cover 12 balanced blocks
on the 8-pole unit and 26-pole rational cases, during 16:34:56–16:35:33 UTC under the
same build reservation. Median native CPU microseconds per grouped decode:

| Case | Release legacy | Candidate legacy | Candidate strict |
| --- | ---: | ---: | ---: |
| 8 poles, unit | 29.710 | 29.532 | 29.713 |
| 26 poles, rational | 62.954 | 62.390 | 56.050 |

The unit strict comparison's paired interval spans zero; the earlier positive
rational legacy result reversed. No repeatable decoder slowdown was established
by this confirmation. The rational strict comparison was faster. Raw records keep
both campaigns, including the initial unfavorable results.

### Memory attribution and remaining limit

GNU time recorded each child's peak RSS. The confirmation's median paired candidate-minus-release peak RSS differences
remained +40–94 KiB. Anonymous-page differences ranged from
−4 to +16 KiB; file-backed residency and process layout contribute to total RSS.
Total peak-memory parity is therefore **not established**.

Native heap snapshots retained the matching workload's objects at 512 and 1,024
instances. The strict 8-pole unit decoder used 32 fewer live heap bytes than the
release at 1,024 instances; the strict 26-pole rational decoder used 2,896 fewer.
Their second 512-instance growth was also lower. The candidate legacy rational
decoder used 1,312 more live bytes, with 352 more bytes in the second increment.
This small difference does not establish a fixed per-object increase. All sampled
legacy patch and isocurve workloads had identical live heap to the release.

The earlier cold-to-live strict isocurve delta was misleading: the strict preflight
left fewer bytes live before retention. At 26 rational poles, final live heap was
144 bytes lower and the second 512-isocurve-pair increment was identical across
variants. Both cold and live counters are retained; the cold subtraction is not
reported as extra object storage.

For unit patches all encoded bytes agree. For rational patches the strict record
changes only the two directional-rationality flag bytes, from `(false, true)` to
`(true, false)`; coefficients and evaluated geometry agree. This is the declared
truthful-axis behavior, not a byte-identical replay claim. The witness file records
exact offsets and hashes. Artifact hashes, compiler/profile details and raw
measurements are retained under `references/464-release-*`.


### Controlled memory follow-up, 2026-11-15

The follow-up compared v0.13.0 (`d051ff699`), current main (`28a1c720`)
and the qualified candidate (`8e9c6843`). Library profiles stayed at opt0;
probe programs used opt2 on CPU 3. Separate crate metadata allowed all three
versions to link into one executable. Every process warmed all four paths before
retaining 239, 512 or 1,024 patches, isocurve pairs or decoded shape tables.
The final shared-image campaign ran 864 processes in 12 balanced blocks.

Against current main, the candidate's legacy path had exactly equal heap and
resident-memory growth in every shared-image control. Strict decode and isocurve
resident growth was also equal. Strict rational patches used 12, 28 and 52 KiB
less anonymous memory at the three counts. Strict rational setup left a bounded
464–1,872-byte live-heap difference for isocurves/decoding; it did not grow with
the retained count. These allocator counters include setup and allocation layout.

The release-relative 8 KiB anonymous increase at 1,024 decoded tables already
exists on current main. Its `BinShapeSet` occupies 1,152 bytes versus 1,144 in
v0.13.0, and `BinSurfaceSet` occupies 136 versus 128. Main's frame-read policy
accounts for that layout change; #464 adds no further object-size growth.
A temporary private policy-packing experiment retained the same sizes and was
removed. The surface and curve sizes remain 200 and 144 bytes in all versions.

Standalone programs checked the effect of executable layout. The first candidate
program included both strict and legacy runtime branches and was 65,376 bytes
larger than the main program. Fixed-path programs then linked only the selected
construction path: the strict executable was 658,136 bytes smaller than main,
while the legacy executable was 44,776 bytes larger. Both results are retained.

The final fixed-path confirmation ran 384 processes in 16 balanced blocks after
the coordinating agent confirmed its builds had finished. An earlier run may
have overlapped compilation and remains recorded separately. Each process retained
1,024 objects and recorded GNU peak RSS, executable-backed resident pages,
anonymous pages and native live heap. Paired median differences for the strict
path, in KiB, were:

| Workload | Peak versus v0.13.0 | Peak versus main | Executable residency versus main |
| --- | ---: | ---: | ---: |
| 8-pole unit patch | -162 | -132 | -130 |
| 26-pole rational patch | -318 | -294 | -116 |
| 8-pole unit isocurves | -144 | -114 | -140 |
| 26-pole rational isocurves | -96 | -70 | -92 |
| 8-pole unit decode | -56 | -8 | -88 |
| 26-pole rational decode | -56 | -136 | -124 |

Strict peak medians were lower for every measured workload. Some paired bootstrap
95% intervals span zero, including unit patch and unit decode versus main; those
comparisons do not establish an improvement. All strict executable-residency
intervals versus main were below zero. Legacy peak intervals versus both baselines
span zero throughout. Legacy unit decode had a small positive resident-RSS interval
versus main; executable-page layout and startup remain part of that observation.
The shared-image test found no additional retained resident growth for that path.

These controls found no repeatable peak-memory regression for the measured strict
workloads, and separated the earlier unexplained peak signal from per-object storage.
They do not prove universal parity for arbitrary callers, binary layouts or
platforms. The prior standalone measurements and unfavorable observations remain
in the record. Raw samples, all paired differences, intervals and artifact hashes
are in `references/464-{same-image,standalone,fixed-path,controlled}-memory*`.
