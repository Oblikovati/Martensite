# Standards review — B1 #566/#567 adaptation

ACCEPT at source level: zero documented-standard or actionable heuristic findings. Reviewed only the frozen 25-path delta against B1 commit `3b6bb98910e80faf6bc12cbe10e7c153c75de957`; trusted main merge-base is `b61a031d0bde0e21d283e0f94975f79b9ae15574`.

All paths match `/tmp/martensite-566-567-b1-focused/source-attempt2-before.json`, SHA-256 `e12e516fdd1470ad26ec5e6808bdf9557ef4c9f362554cd18f69396ed6331eda`. The complete six-file native-incidence subtree, including controls, is byte-identical to accepted main commit `9774ac64`.

## Documented standards

- `AGENTS.md:148–156` requires shared logic and scoped changes. The private `SelectedStore` supplies the existing index from the selected solver or conditioned report vector. Its real Bernstein caller uses the same lookup driver; no second indexing algorithm or public extension point is introduced. First-key insertion, selected-vector addresses, actual append order and ColdUnique's complete identity-inventory condition remain intact.
- Reuse state is local to enumeration. Native observations borrow the immutable phase snapshot and source bindings; the view expires with that phase. This follows the operation-local ownership rules and shared-incidence reuse requirement. Neither index iteration determines persistent identity or publication order.
- Topology/cardinality retain B1's existing dual-evidence axes and unified certificate consumption. Completed empty conditioned proofs still compare against actual observed vertices. Source binding, query mode, policy, numerical execution, shared work budgets, failure order and cancellation callbacks remain in their existing paths.
- No unsafe permission, dependency, public API, version, algorithm-default, tolerance or cap changes appear. Existing numerical baselines and tests remain unchanged. Native discovery observer plumbing/cost is inherited unchanged from reviewed main. Changed test-fixture visibility is confined to sibling tests.
- New conditioned controls compare complete outcomes, source slots, work and callback order with literal lookup; seeded failures and malformed mixed stores cannot borrow another vector's addresses. Native extra-vertex and source-binding refusal controls preserve ordered diagnostics. New/changed Rust files remain below 500 lines, with focused function bodies within the meaningful-body convention.
- Fixed changelog entries cite both issues. Documentation states index lifetime, evidence ownership and measurement limits without claiming accepted performance; no actionable humanizer finding.

The attempt-2 repair adds only a `cfg(test)` factory returning an owned opaque lookup (`use<>`) and passes it into the existing Bernstein cancellation control. The caller has no retained census borrow, assertions are unchanged, and production code is unaffected. The original compile failure remains recorded.

## Heuristics and remaining gates

No actionable Fowler smell: the small private store enum consolidates one selected-storage decision without speculative abstraction.

Reviewer ran no builds/tests. Focused functional execution, canonical/hosted gates and matched runtime/peak-RSS acceptance remain separate conditions; source acceptance grants no merge or performance approval.
