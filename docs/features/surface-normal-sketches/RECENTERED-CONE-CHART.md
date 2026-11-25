# Recentered cone charts

A profile on the developed cone can straddle the physical angle `π` even
when it uses only a small fraction of one physical revolution. The
datum-centered chart refused that profile because its half-angle tangent
has a pole at `π`. The opt-in recentered chart chooses a separate rational
fit center while keeping the authored coordinates and physical datum.

Call `normal_extrude_cone_profile_with_selection` with
`DirectedRecenteredChartRequireCertifiedV3` for the required exact
self-intersection check. It uses the same directed carrier construction,
reference proof and publication pipeline as the previous family chart.
`PositiveXRecenteredChartV4` also supports the explicit rounded proxy with
native search. Earlier selectors retain their defaults and replay behavior.

## Admission and bounds

The toolkit estimates the angular extremes from the least and greatest
control-pole slopes. This takes one pass over the selected family and
stores two scalars. With strictly positive developed X, every point of a
polynomial Bézier span lies in the angular sector of its control poles.
The estimate chooses a center. Bounded arithmetic independently checks
the original angular hull, its width below one physical revolution, and
strict containment in the selected tangent chart.

The domain receipt keeps the certified source angles relative to the
original datum and the offset from that datum to the fit center. The
reference proof subtracts the offset before checking the displaced
authored profile against the tangent chart. It adds the offset when
mapping the fitted angular hull back to the developed positive-X
halfplane. Both checks use the domain receipt's fixed-grid precision.
The native cap parameters retain the continuous physical angles.

The current operation uses one positive-X chart for a family in the
canonical frame. It refuses families whose required angular hull cannot
be certified within a single revolution. The separate
[cone region atlas](CONE-REGION-ATLAS.md) certifies disconnected
regions whose combined authored lift spans more than one turn.

## Verification

The crossing fixture is a square with a square hole, on a cone with a
semi-angle of 0.125 radians. The original family chart refuses the
fixture. The recentered operation checks both normal senses against
independently calculated vertex positions at the requested depths and
retains the hole, the authored depth roles and the certified spatial
bound. The directed path checks exact self-intersection coverage and
replay of the encoded BRep, history and budget in a separate process.
The rounded path has its own complete native-search control.

The focused suites passed 240 Math tests and 130 Sketch cone tests. The
legacy cone control reproduced its BRep, history and budget across the
baseline and candidate builds. Strict Clippy passed for both crates.

On the existing outward directed O. control, three runs per build gave
median API times of 5.358 s before the change and 5.337 s after it. The
complete test process, including independent validation after the API
call, took 9.43 s and 9.49 s. Peak RSS medians were 66,432 KiB and
66,636 KiB. Separate 25 ms samples attributed most of the observed RSS
difference to file-backed pages. Arithmetic work was identical in all
six runs. This measurement leaves the release performance gate open.
[Local measurements and replay digest](references/recentered-cone-chart-2026-11-20.json)
