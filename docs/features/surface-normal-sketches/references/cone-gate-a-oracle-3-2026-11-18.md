# Gate A — Final Oracle review, attempt 3 of 3

Verdict: **ACCEPT for the next Phase A delivery step: commit the reviewed candidate and open its ready PR.** No material correctness blocker remains in this frozen scope. This does not approve merging ahead of required hosted checks, complete #414/#418, or authorize versions, tags or release.

Reviewed immutable tree `1474c2ee5e672f30e62b8b7f6b344c4656c0bb17` against main merge-base `9e11ab3864c8786ce05b3bdf35123f4a87fc1b05`. The previous accepted correctness review was tree `cb24809b7ce1041300da4fced24e11f6596a347a`; the final delta was inspected separately. All 31 final delta paths match that immutable tree. Its 15 changed non-document source/test/build paths match the successful full-gate manifest, and all 207 unaffected prior snapshot paths still match their review-2 hashes. No builds, tests or source changes were performed by this review.

## Correctness and final delta

Review 2 resolved the four original material blockers: common-chart/shared ideal incidence; executed q/L topology and full-depth normal-map composition; independently enforced epsilon/1000 decoded storage; and source-bound integration of that same reference. Those numerical, geometry and modelling-check sources are unchanged. The approved exact-reference/separately-bounded-native-proxy contract remains intact; no stronger literal stored-surface closure or collar obligation is introduced here.

Bug #547 changes the private cone failure payload to `Box<dyn ConeFailureCause>` in `api/error.rs:48–62`, retaining `Error + Send + Sync`. The concrete cone implementation preserves the borrowed planar-work method. Error-source upcasting retains concrete identity rather than adding a wrapping source layer (`api/error.rs:153–160`). Focused controls in `operation/cone/error/tests/boundary.rs` verify Display/Debug, downcasts, native-leaf pointer identity, borrowed work identity, cancellation class and Send/Sync. No success-path geometry or numerical policy changed.

The new public concurrency control uses barriers around two independent public operations and two shared readers, compares complete evidence and encoded bytes, and verifies independently owned exports. The sanitizer driver selects that exact nonignored control and preserves nonzero/startup-failure propagation. No new material regression was found.

## Memory remediation and accepted tradeoff

The requested matched standalone production consumer was executed on genuinely fresh baseline/current/fixed builds. It retains the accepted cylinder workload, full constructor checks and two isolated exports. All nine runs preserve native SHA256 `77958916…` and public-evidence SHA256 `560adbd…`. Corrected versus baseline medians are 3.683161061 versus 3.774414419 seconds; median process RSS is 51484 versus 51328 KiB. Separate normalized diagnostics show identical allocation call counts and no measured tracked peak-heap/stack increase. These small nonexclusive-host samples do not establish a statistical speedup or universal memory bound.

The residual +156 KiB (~0.3%) median RSS and quantified compiled-code increase remain real reported costs. The user's explicit “accepted” decision, recorded in `PRODUCTION-MEMORY-ACCEPTANCE.md:90–97` and the analytic-carrier ADR, resolves review 2's sole remaining blocker for this identified candidate. It is not reclassified as zero growth and grants no blanket performance waiver. The rejected inline trial was removed; its findings and cleanup receipt remain.

## Evidence and delivery limits

The canonical local PR receipt reports exit 0 with identical before/after source manifests: 89075 outer tests passed, zero failed, 158 ignored, plus nine separate unsafe controls. Raw artifact hashes were independently checked against the receipt. Nested worker/SDK summaries are explicitly excluded from totals. Full Sketch includes 447 passes, with ignored workers exercised through parents. Earlier fixed-workload five-second, same-policy replay and V3/V4 geometry/history/bounds controls remain applicable because numerical sources did not change.

Local TSan failed at startup before main with exit 139; it supplies **no instrumented cone coverage**. Hosted checks, including the new cone TSan control, remain required publication/merge gates. A successful older component's TSan cannot substitute for this coverage. Any required hosted failure must be resolved before merge.

Phase A is the canonical-frame, single-common-chart curved outer-plus-hole operation in both senses. Wider charts/seams/text/placement, primitive lowering/deletion and broader application QA remain B/C work. This acceptance authorizes delivery of the bounded Phase A result only.
