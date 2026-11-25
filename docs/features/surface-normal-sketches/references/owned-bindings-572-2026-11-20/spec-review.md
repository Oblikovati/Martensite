# Spec review: B1 adaptation of #572

Pinned main merge-base: `b61a031d0bde0e21d283e0f94975f79b9ae15574`. Reviewed only the twelve-path adaptation against B1 checkpoint `e20ffc78021473f77c1dfdfcc1af7646d59943d0`, including all untracked tests. All twelve hashes match `/tmp/martensite-572-b1-focused/source-before.json`, SHA256 `2b1377bea405d7bc7ec456b969f5e3828929dc259425b482c6cea2312808627d`, before and after review.

Verdict: **source/spec ACCEPT; no findings.**

The requirement is to reuse “immutable metadata” while preserving “ordinary live-source freshness checks, complete source/policy/mode/alias binding” and error precedence. The occurrence module and its tests are byte-identical to main-line fix `77582311`. Checked occurrence access refuses malformed indices before prefix slicing; source operands, history order, parent kinds, orientation and first physical-match semantics remain unchanged.

Initial eligibility still executes the first actual bound certificate while both cached fields are absent. It therefore retains live occurrence metadata, parameter, report-source, mode and policy checks before owned-manifest/native admission. Eager retained flags cannot grant a certificate: both solver and Bernstein bindings consume metadata through the same census-bound view, then retain their original per-domain query/source and effective-policy verification. Genuine fallback source and policy/work checks remain intact. The unified reader still checks live native representations; ordinary borrowed views never gain owned flags.

No numerical engine, defaults, behavior selector, limits, callback/cancellation schedule, failed-work accounting, proof kind or publication output changes appear in this delta. The only unrelated-looking edits widen two existing test helpers within the parent test module, for reuse by new controls.

New V3–V6 controls require actual geometric/excluded/solver kinds rather than fabricated empty solver evidence, compare contact identities and coordinates, exercise first/later metadata/source/mode/policy refusals, reversed parameter axes, and native-alias freshness across epochs. Actual predicate observers require initial live eligibility plus one occurrence capture, then zero additional history/prefix predicates across repeated domain reads. Existing integral-program, work and cancellation controls remain unchanged.

Focused/full-operation execution is underway, not established by this source review. Canonical and hosted validation, matched runtime/RSS within the approved policy, and full output/error/callback replay qualification remain execution conditions. The documentation explicitly records the added O(n) allocation and pending performance evidence. This review does not reaccept the unrelated accumulated B1 capability implementation.
