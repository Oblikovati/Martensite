# Standards review — B1 adaptation of #572

ACCEPT at source level: zero documented-standard or actionable heuristic findings. This covers only the frozen twelve-path adaptation relative to `e20ffc78021473f77c1dfdfcc1af7646d59943d0`; B1's wider capability is outside this review. Trusted main merge-base is `b61a031d0bde0e21d283e0f94975f79b9ae15574`.

All twelve source hashes match `/tmp/martensite-572-b1-focused/source-before.json`, SHA-256 `2b1377bea405d7bc7ec456b969f5e3828929dc259425b482c6cea2312808627d`. The occurrence module and its tests are byte-identical to accepted main commit `77582311`.

## Documented standards

- `AGENTS.md:148–156` requires shared, scoped implementation. Solver and Bernstein bindings now use the same view-owned metadata accessor, preserving the shared occurrence predicates. The cache is admitted only after the existing actual-first-report eligibility check, exact owner binding and native-source validation. That initial check remains live before flags are installed. Immutable census/owner borrows bound their lifetime.
- Per-domain parameter slots, physical sources, query mode, captured queries and policy remain checked. Conditioned source matching, genuine fallback eligibility and original report work validation are unchanged. Earlier behavior versions remain selectable; no numerical baseline, tolerance, arithmetic, default, public API, dependency, unsafe authority or package version changes occur.
- Observer variants, loop calls and counter storage remain test-only. New Rust files are 95–143 lines, and modified Rust files stay below 500. Functions retain focused responsibilities; multiline test assertions do not introduce substantive nesting or mixed production concerns.
- New V3–V6 controls compare actual proof kind, contact bits, exact endpoint flags and keys; they exercise source/history/mode/policy refusals and native alias mutation. Existing main-line controls are retained. Counter expectations increase to include B1's existing first live eligibility check rather than skipping it. This respects `AGENTS.md`'s verification and no-weakened-tests rules.
- The Fixed changelog cites #572. Documentation states the O(n), one-byte-per-occurrence allocation and unchanged per-domain query allocations, and explicitly leaves time/peak-RSS qualification pending. No actionable humanizer finding.

## Heuristic assessment and remaining gates

No actionable Fowler smell. Moving metadata behavior into its owning view reduces cross-module coupling without adding a public abstraction. Test-only sibling visibility changes reuse existing fixture setup.

No builds or tests were run by this reviewer. Focused/canonical execution and matched runtime/peak-RSS acceptance remain separate gates; source acceptance grants no performance or merge approval.
