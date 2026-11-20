# Spec review: B1 adaptation of #566/#567

Pinned main merge-base: `b61a031d0bde0e21d283e0f94975f79b9ae15574`. Reviewed the complete twenty-five-path WIP adaptation against B1 checkpoint `3b6bb989`, excluding unrelated accumulated capability work. All paths match freeze SHA256 `e12e516fdd1470ad26ec5e6808bdf9557ef4c9f362554cd18f69396ed6331eda` before and after review.

Verdict: **source/spec ACCEPT; no findings.**

This supersedes the first freeze. The only follow-up adds a test-only owned lookup factory and passes it to the existing direct Bernstein cancellation test. Explicit empty lifetime capture avoids borrowing the census across its mutable evaluation. Original assertions remain unchanged; no production body changed. The failed compilation ran zero tests.

The reviewed requirements retain “FIRST existing outcome” and require ColdUnique only when the “SELECTED store is empty.” SelectedStore explicitly chooses solver or conditioned records from the same immutable exclusion selector used by execution. ColdUnique additionally requires a complete identity occurrence inventory. Seeded, aliased and incomplete inventories retain IndexedMap; `from_keys([])` itself remains IndexedMap. Complete triangular traversal cannot repeat a key under the admitted identity mapping. Neither shortcut skips a numerical call. Both evaluators retain the first recorded slot regardless of success, failure or cancellation, and remember the actual selected-vector append address.

Only lookup plumbing changes around the existing evaluator. Poll locations, same-physical-source refusal, source-slot mapping, cold execution, fallback evidence, work charging and obligation refresh remain in place. The mode cannot change during enumeration, so report-vector address spaces remain separate without adding authority or policy to the key.

All six native incidence helper/test files are byte-identical to accepted main head `9774ac64`. B1 retains both evidence arms in domain-axis and report-index selection, and cardinality still obtains unified certificates. Native observation counts remain actual vertex-ID counts, including unexpected incidences in zero-contact conditioned domains. Current/source validation, first duplicate vertex choice, raw signed-zero parameter ordering and first encountered diagnostic order remain intact. `measure_parameters` and its unified evidence consumption are unchanged.

Controls compare literal/indexed conditioned full outcomes, raw source bits, fallback reports, slots, work, typed partial cancellation and callback sequences. They cover selected-store seeding, first duplicate failure, malformed mixed stores, aliases and ColdUnique guards. Native controls retain literal cardinality comparisons and add conditioned empty-domain/current-source mutation cases. Cancellation controls sample five stop positions; they do not claim every poll cutoff.

Focused/full-operation tests are underway. Canonical/hosted validation, exact operation replay and matched time/RSS acceptance remain execution conditions. No executed-pass or performance claim follows from this source review; the prior main acceptance does not qualify the adapted B1 binary.
