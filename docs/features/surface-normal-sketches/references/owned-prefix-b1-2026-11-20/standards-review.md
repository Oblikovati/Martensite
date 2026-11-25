# Standards review — B1 scalar-prefix port

ACCEPT at source level: zero documented-standard or actionable heuristic findings. Reviewed the seven-path adaptation relative to B1 HEAD `5696a884ecd989fc78cd2de07cff0db1b8b88b4d`, with trusted main merge-base `b61a031d0bde0e21d283e0f94975f79b9ae15574`. Earlier B1 flag-cache and main scalar-prefix reviews remain separate candidate evidence.

All seven paths match `/tmp/martensite-572-b1-prefix-focused/source-before.json`, SHA-256 `e88808643db7f246bf59220d14186bc66883013039b424904462f6face984cf0`. The occurrence implementation, original child tests and new prefix tests are byte-identical to main `5578942`; their accepted predicate/prefix review applies unchanged.

## Documented standards

- `AGENTS.md:148–156` requires scoped, shared implementation. B1 glue replaces only the private optional vector with an optional scalar and passes that copied scalar into the existing shared accessor. There is no second caching algorithm. The actual-first-bound-certificate check, owner/native-source admission, immutable view lifetime, and common solver/conditioned metadata routing remain unchanged.
- Main's prefix rules therefore apply to both B1 readers: success before the first failure is reusable, that failed row refuses, later rows run the literal predicate, and missing/out-of-range rows refuse. Per-domain source, parameter, query mode, captured report and effective policy checks remain intact. No numerical policy, cap, default, public API, unsafe authority, dependency or version changes occur.
- Production loop observers and counting remain absent; existing test instrumentation is unchanged. All changed/new Rust files remain below 500 lines with focused function bodies. Existing real-reader mutations gain an additional truncation case; no tests or numerical baselines are weakened. The byte-identical prefix controls cover valid/invalid rows after a refusal, empty input and maximum index.
- The Fixed changelog and feature documentation describe the marker accurately. They distinguish removal of cache-owned heap storage from unchanged predicate/query allocations and explicitly withhold any measured RSS claim. No actionable humanizer finding.

## Heuristics and execution limits

No actionable Fowler smell. Reusing the accepted main helper keeps the B1 adaptation cohesive and avoids duplicate validation logic.

Reviewer ran no builds/tests. The coordinating agent reports 54 focused tests and two full-O runs on unchanged source; this source review does not independently certify those executions or infer performance acceptance. Canonical/hosted gates and matched runtime/peak-RSS qualification remain separate conditions.
