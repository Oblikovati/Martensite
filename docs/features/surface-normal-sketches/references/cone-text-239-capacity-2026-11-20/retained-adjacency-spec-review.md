# Focused proof review: retained adjacency and product refinement

Reviewed the changed actual-certificate paths only. All sixty guarded source hashes match `/tmp/martensite-cone-retained-adjacency-20261120/source-refined.json`, SHA256 `2d02cdf4ae393c8473cb067486201af3e907f82fd9c1efcd4cc7ae4aacb0dbf1`, before and after review.

Two concrete findings:

1. `hierarchy/visit.rs::leaf_pair` propagates `refine::separated(...)?` without the known side identities. Execution cleared FailurePosition before entering hierarchy, so cancellation/work failure within this new pair-specific subdivision returns bare Arithmetic with side/cap=None. Wrap it in the existing SidePair containing the two actual source IDs and original cause. Add cancellation/low-budget controls verifying IDs, nested cause and exact completed work; recursive cancellation classification already supports SidePair.

2. The retained-projection tests check minimum_step, ruling bounds and the direction heuristic, but never exercise the assembled weighted derivative lower bound. Add a permanent independent nonuniform-weight rational example checking degree, orientation and both rows, plus zero/negative-step refusal. Product tests establish geometric separation/overlap only; add cancellation/work exhaustion and stable refusal-context controls for the new recursion.

No numerical soundness defect found in the reviewed arguments. For projected controls with consecutive increment >=δ>0, the rational derivative’s pairwise Bernstein numerator contains `(p_j-p_i)(j-i)w_iw_j`; bounding increments and weights gives `n*δ*(wmin/wmax)^2`. Positivity and identical weights across depth are checked by existing capture, so rowwise derivative and ruling convex-average bounds apply throughout the ruled side. The floating direction is only a candidate: charged outward predicates validate every required inequality before the existing coupled f/D theorem consumes it.

Homogeneous U splitting restricts all weighted coordinates and weights; V splitting interpolates homogeneous rows with their shared denominator. Positive-weight control hulls enclose every subpatch. The product traversal partitions all four parameters, accepts only strict disjoint hulls and refuses at unresolved depth/work/cancellation. It cannot turn overlapping patches valid merely by spending more work.

OriginalV2 retains old paths; retained projection/refinement is selected only by FamilyV3 domain authorities. No new identity is derived from subdivision ordering, and all numerical work shares the existing arithmetic ledger. The adjacency-only whole-text result does not qualify nonincident refinement; the refined full test is still compiling. No builds, edits or performance acceptance occurred.
