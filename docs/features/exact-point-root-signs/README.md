# Exact signs at certified point roots

Bug #542 concerns the shared exact root-sign evaluator. It built a
Sturm–Tarski query chain before examining the certified root brackets, even
though classification of a point bracket used only exact polynomial
substitution. Empty root sets also built an unused chain.

The evaluator now checks whether every certified bracket is a singleton.
For that case, including an empty isolation, it polls for cancellation at
entry and before each root, then uses the same exact point-sign calculation.
A cancellation returns its original evaluation-stage cause without partial
signs. Source and query admission, root isolation, ordering, multiplicities,
and subdivision receipts remain upstream and unchanged.

If any bracket is not a singleton, the whole original eager path runs,
including classification of earlier point roots. All original candidates
remain required, including roots later rejected by pair membership.

Removing unused chain construction also removes its internal cancellation
polls. Poll-count schedules for point-only and empty isolations therefore
change. Mixed and non-point isolations retain the original poll sequence.
This internal cost fix changes no public API, numeric policy, requested
budget, or accepted geometric result.

## Diagnosis

Temporary probes on the existing cone fixture replayed three actual cubic
pairs through normal Math dependency builds, with both pair-policy versions.
The root policy was version 1, width `0.001`, degree cap `32`, derived sign
admission and world-error request `0.0001`. Remaining subdivision caps were
`3910`, `4024` and `3910` for source pairs `[18,19]`, `[2,3]` and `[19,20]`.

All six calls completed with genuine graph evidence and one contact.
Membership consumed 321–341 ms, about 87% of each 369–390 ms call. Source
preparation and equation derivation took tens of microseconds. These local
diagnostic measurements identify the cost; they do not establish an
end-to-end performance result.

Pairs `[18,19]` and `[19,20]` each had only the accepted exact root `t=1`.
Pair `[2,3]` also retained two rejected non-point candidates, in brackets
`[353 * 2^-10, 177 * 2^-9]` and `[407 * 2^-9, 815 * 2^-10]`.
Every membership isolation matched its full candidate isolation. Thus the
mixed pair must keep the eager path despite its single accepted point root.
All temporary source instrumentation was restored byte for byte before this
main-based fix was implemented.

## Verification

Focused unit controls use independently known signs at `-1`, `0` and `1`,
repeated roots, zero and subnormal queries, and empty isolations. They check
cancellation at every observed checkpoint without fixing internal poll
counts. Mixed `0, sqrt(2)` and irrational-only cases compare results and every
cancellation checkpoint with the original eager evaluator. Admission controls
cover query degree and invalid policy precedence. The existing independent
sign corpus supplies broader numeric and input-failure coverage.

Run the focused checks with:

```sh
cargo test -p martensite-tkmath --lib predicates::roots::signs::tarski::tests
cargo test -p martensite-tkmath --test toolkit exact_roots::signs::
```

The full PR gate and the complete cone performance comparison remain separate
qualification steps owned by the coordinator.
