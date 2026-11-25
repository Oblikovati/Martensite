# Graph reader counter probe, 2026-11-20

The unchanged outward accumulation V2 operation passed: 994,535 arithmetic operations, 444 integration cells and 148 retained spans. The test ran once, with TKBO compiled as a normal dependency. B1 was clean at `1e1800dff56893d3cbd334dca045f0f21d362db0` before and after the probe.

## Observed reader work

Each row includes the initial bound-certificate eligibility probe, which ran while constructing the view before the `begin` record. The unified column counts calls during the reader. These values came from the actual executions, including every evaluated relation-filter and physical-prefix predicate.

| Sources | Domains | Unified calls | Bound / metadata / query calls | Occurrence checks | Relation predicates | Prefix predicates |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 12 | 78 | 234 | 235 | 470 | 16,920 | 3,044 |
| 137 | 9,453 | 28,359 | 28,360 | 56,720 | 23,311,920 | 3,913,544 |
| 11 | 66 | 198 | 199 | 398 | 13,134 | 2,378 |

All three views qualified for the owned path. At `begin`, each had performed exactly one native-current sweep, observing respectively 12, 137 and 11 source captures through the existing correspondence observer. All three native sweep/capture deltas between `begin` and `end` were zero. Other native-handle operations were not counted by that observer.

The 137-source view retained 9,453 genuine geometric reports: 9,316 completed empty domains and 137 completed domains with contacts. The 11-source view retained 66 geometric reports: 55 empty and 11 with contacts. Both had zero Bernstein reports, capture failures or solver fallback reports. The 12-source view retained 78 original solver reports. Its conditioned-record categories are inapplicable; the probe did not classify its solver contacts into empty/contact totals.

Actual geometric source-match calls were 28,360 and 199 for the 137- and 11-source views. The source implementation constructs two full keys per geometric match. Thus 56,720 and 398 key constructions follow from the counted call sites and that implementation; Math key construction itself was not instrumented.

## Scope and interpretation

The reader revisits completed evidence. Its successful three matrix passes produced exactly three unified reads per domain in this run, plus the initial bound read. Each bound read repeated immutable occurrence metadata checks and reconstructed a captured pair query. On the 137-source view, these repeated checks evaluated 27,225,464 relation/prefix predicates altogether. The native capture epoch already worked as intended.

This probe supports a bounded follow-up design for the sealed owned view: reuse per-occurrence metadata decisions and actual captured-source bindings while retaining the existing live path and its alias-mutation guards. A future change must preserve domain order, first unavailable diagnostics, policy/source bindings, cancellation and actual work receipts. This probe made no optimization and does not establish the time such a change would save.

Issue #565 concerns repeated polynomial primitive normalization in generic solver membership. Math and `source.primitive` were deliberately uninstrumented. Their cost is unmeasured here. The complete graph reader call path has no new membership solve; the retained geometric records and zero fallback counts above distinguish the current 137/11-source paths from that older solver diagnosis.

Issue [#572](https://github.com/Oblikovati/Martensite/issues/572) tracks the repeated owned occurrence checks. Its main-based implementation is in progress; no speedup has been measured.

## Reproduction and cleanup

The command and normal low-storage environment are recorded in [command receipt](references/graph-reader-attribution-2026-11-20/command.json). Debug assertions and overflow checks remained enabled; no compiler flags or wrappers were added. The existing exact test filter was:

```
surface_sketch::wrapped_profile::operation::cone::tests::narrow::complete::directed::outward_accumulation_v2_preserves_complete_actual_si_and_original_caps
```

The build completed successfully and the selected test passed once, with 518 tests filtered out. Its API time was diagnostic only because other QA work was running. No timing is usable for performance acceptance.

Only `paired_binding.rs`, `bernstein_binding.rs` and graph-reader `joins.rs` were temporarily changed. Counters belonged to each view, with no global or thread-local state. Inner predicate counts accumulated locally and were added once per helper call. The begin/end output classified existing retained records without solving new queries or changing polls or arithmetic charges.

All patched hashes remained unchanged during the run. The finalizer restored the three exact source backups with fresh mtimes and verified a clean checkout at the original SHA. After confirming no target consumers remained, it removed 13 newly created or modified probe build artifacts, totaling 142,691,801 bytes. `terminal.json`, `source.json`, raw logs, parsed observations and `summary.json` retain the evidence. Temporary implementation scripts and source backups were discarded after summarization.
