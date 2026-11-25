# Current UV material attribution

One complete outward `O.` operation passed at source checkpoint `11884b4c`,
with geometric planar proof V6 and volume policy 2. It retained 994535 arithmetic
units, 444 integration cells and 148 fitted spans under the original limits.
The ordinary test profile used opt0, TKBO opt1 and Math opt3, with debug assertions
and overflow checks enabled. Compilation ran concurrently elsewhere. These
instrumented timings do not qualify runtime, RSS or the five-second target.

The API took 8.021473 s; the complete test, including independent checks, took
13.24 s. The O137 UV capture took 4.134473 s and dot11 took 0.076133 s.

| O137 scope | Seconds | Parent scope |
| --- | ---: | --- |
| Program certification | 3.156081 | UV capture |
| Census | 0.314818 | Program certification |
| Partition | 0.405358 | Program certification |
| Contact transport | 1.883531 | Program certification |
| Graph contact verification | 0.483454 | Program certification |
| Graph faces / nesting / face certification | 0.060258 | Program certification |
| Native region construction and publication | 0.975713 | UV capture |
| Native validation of two regions | 0.948080 | Native region construction and publication |
| Native faces | 0.023926 | Native region construction and publication |

Parent times include their children. Adding every row would count work twice.
Admission, native topology construction, assembly, program issuance, UV selection
and selected-material retention were also observed; the raw stage records are
in the [receipt](references/uv-material-attribution-2026-11-20/summary.json).

The generic [region builder](../../../crates/martensite-tkbo/src/profile_extrusion/build_regions.rs)
constructed and validated two O bounded regions. The caller then selected
exactly one by authored loop identity in
[planar::select](../../../crates/martensite-tksketch/src/wrapped_profile/planar.rs).
The individual native checks took 0.764715 s and 0.183365 s. This probe did not
bind either timing to the selected region ID. No saving from selective native
construction is established.

All seven temporarily modified source files were restored byte for byte; Git
was clean afterward. The emitted probe artifacts were discarded, reclaiming
143153367 bytes. Verbatim logs, source hashes and 112 stage observations are
preserved with the receipt. No numerical policy, allowance or baseline changed.
Final-body qualification was outside this probe's stage timers. Wider charts,
attachment and text acceptance remain outside this witness.
