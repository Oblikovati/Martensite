## Standards

**Source-level ACCEPT for r3.** No documented-standard breach or actionable Fowler-baseline smell found in the isolated #570 delta.

Reviewed all 32 paths in `/tmp/martensite-570-source-freeze-r3.json` (manifest SHA256 `e030fab4954178e532f0eff96b9e2ae7a77f531594fdeced9a15e510a3128162`) at B1 HEAD `2e8ab886d4755c9c0165d7f59f2a92ead61f3612`. All current hashes match. The 22 recovered pre-change Rust hashes also match `/tmp/martensite-pre570-source-recovery/manifest.json`; those saved files, seven new Rust files and the three prose paths define this review, not B1's unrelated working diff. Standards sources: canonical `origin/main:AGENTS.md` at `b61a031d0bde0e21d283e0f94975f79b9ae15574`, `CONTRIBUTING.md`, and the supplied Fowler baseline.

- `conical_sum.rs` gives conical aggregation one private boundary and reuses `bounded_dyadic::Arithmetic`. It introduces no second arithmetic engine, global/TLS state, dependency, unsafe code or public accumulation axis. Exact and outward modes have present callers and distinct, documented work semantics; the adapter is not speculative generality.
- Parent integration version 2 is separated from explicit scalar algorithm 1. Defaults, the shared exact `BoundsSum`, standalone integral admission and cylindrical behavior stay outside the new selection. The kernel decision, containment controls and original-limit complete-operation fixtures satisfy AGENTS.md's source-level requirements for declaring a new numerical behavior; their execution remains a separate gate.
- Work/cell receipts remain operation-local. Failure accounting retains typed arithmetic/span/cap causes, and Sketch preserves the primary leaf failure when bookkeeping also refuses. Source-policy binding includes the new parent selection and resource bits. Existing preparation and integration routines are reused rather than copied into another production path.
- The unreleased analytic-flux error stabilization follows the recorded decision. Its added fields have a current Sketch caller; legacy Debug layout is retained. The compile remediations narrow visibility to the private integral ancestor and use the actual fixture policy type, without broadening published APIs.
- Changed Rust files are below 500 lines; responsibilities are split among sum, admission, execution, binding and failure modules. Tests include independent rational/closed-form expectations and deterministic work/cancellation controls. Changelog and supporting documents state pending qualification and make no speedup claim.

No Cargo, tests, benchmark, source/Git mutation was performed by this review. Compilation, complete-operation acceptance, matched time/RSS and canonical/hosted checks remain pending with root.

Findings: 0 documented-standard breaches; 0 actionable baseline smells. Worst Standards finding: none.
