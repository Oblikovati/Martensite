# Spec review: B1 #572 prefix port

Pinned main merge-base `b61a031d0bde0e21d283e0f94975f79b9ae15574`; reviewed only the seven-path delta against B1 checkpoint `5696a884ecd989fc78cd2de07cff0db1b8b88b4d`. All seven tracked/untracked paths match freeze `/tmp/martensite-572-b1-prefix-focused/source-before.json`, SHA256 `e88808643db7f246bf59220d14186bc66883013039b424904462f6face984cf0`.

Verdict: **source/spec ACCEPT; no findings.**

The occurrence implementation and both test files are byte-identical to reviewed main fix `5578942`, so its accepted prefix proof applies directly. The retained index is the first failed literal predicate or segment count: earlier rows are proven valid, the marker refuses, later rows execute the original predicate. The range guard precedes all marker comparisons, preserving missing/empty/maximum-index refusal. Later valid rows are never globally rejected because of an earlier failure.

B1 changes only the view field, initialization and value passed to the existing accessor. The first actual bound-certificate check still runs with no cached marker, followed by complete owner/native admission. The marker remains attached to that same immutable census and owned epoch. Both solver and conditioned bindings continue through the same metadata accessor; per-domain parameter slots, mode, full captured query, source/policy/fallback bindings and current native checks remain intact. No numerical, work, callback, cancellation, default, public API or limit changes appear.

Main’s exhaustive prefix controls cover all indices, missing/corrupted first and later rows, valid/invalid rows after a failure, signed zero, aliases, empty inventories and `usize::MAX`. The B1 real-reader mutation test also adds first/later occurrence truncation. Existing V3–V6 proof-kind, certificate/refusal, integral-program and alias controls are retained.

The supplied terminal receipt binds this exact freeze and records unchanged source, 54 focused tests plus two complete O controls passing. Both full operations retain 994535 arithmetic operations, 444 integration cells and 148 spans. This is reviewed execution evidence, not a test run performed by this reviewer.

The allocation claim is accurately limited to removing cache-owned heap storage. The receipt explicitly marks timing unusable for acceptance; diagnostic durations establish no speedup or performance gate. Matched runtime/RSS, canonical/hosted checks and integration remain separate completion conditions. This review does not reaccept unrelated accumulated B1 capability work.
