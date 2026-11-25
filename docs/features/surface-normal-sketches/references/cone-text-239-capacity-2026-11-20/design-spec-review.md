# Design review: explicit larger cone-family domain

Verdict: **legitimate explicit feature capacity, with the stated version/budget boundaries; no intrinsic soundness blocker found. This is not a behavior-preserving refactor or proof of completion.**

The actual 239-curve probe refused `Domain(Sector(SpanCount(239)))` with zero arithmetic operations. Source confirms chart-domain admission delegates to `sector::admit`, whose ordered checks are policy, 1..=128 count, angle and original control geometry. The cap is a pre-proof input-capacity bound; the chart’s positive-X wedge, certified sine, strict (-pi,pi) membership and width below 2pi are separate numerical obligations. No evidence supports attributing this refusal to metric-budget exhaustion.

The approved P5 fixture contains 239 curves, 20 contours and 18 materials. A separately selected family behavior can admit that workload while preserving legacy V2/default refusal and replay. This must be recorded as an intentional kernel capacity decision, not described as merely relabeling an increased limit. Versioning alone would not justify increasing precision, tolerances, work allowances or retrying a refused request; the proposed unchanged one-million global arithmetic budget and unchanged geometric requirements avoid those shortcuts.

Implementation conditions:

- Extract shared policy/count/geometry admission mechanically first, preserving V1/V2 check order, errors, polls and work exactly. V3’s 4096 maximum is fixed at selection time, checked once against the complete original family inventory, and explicitly recorded alongside actual count and source/version. It is an admission ceiling, not a guarantee that every 4096-span request completes.
- Reuse the same numerical proof pipeline. Preserve original spatial/metric limits, precision, global charging on success/failure/cancellation and typed incomplete refusal; no splitting into separately reset budgets, dropped spans, increased limits after refusal or alternate geometry.
- Bind version/cap/count and complete raw source order in domain, boundary and reference authority. Legacy `matches` must reject V3. New selected matchers and execution/result replay must retain the actual selection; old directed selection/defaults remain unchanged. Test cross-version rejection and source/count/cap mutations.
- Cover legacy 128/129 and V3 128/129/239/4096/4097 admission boundaries without claiming full-operation success for mere admission. Existing per-contour joins/component caps remain independent; do not silently enlarge them during extraction.

The first qualification target should be the unchanged complete 239-curve/18-material operation, retaining all geometry, history and native checks. If it refuses, record the next actual blocker and stop that attempt. No detached proof expansion or performance claim is justified here. No builds or source edits were performed.
