# Exact signs at certified point roots

Bug #542 identified Sturm–Tarski query-chain construction that point-root
classification and empty root sets do not need. Its first implementation
bypassed that work in the default shared evaluator. The full PR gate rejected
that default change: the complete-owned-request callback control observed
`8238` calls against the existing `8268` pin.

Legacy evaluation therefore constructs the original eager query chain for
empty, point-only and mixed isolations. The existing `8268`, `2866` and `3696`
owned-operation callback expectations remain unchanged. The original point-sign
calculation is shared with the explicit version-2 filter introduced by
[#544](../filtered-root-signs/README.md). Only that opt-in policy skips unused
chain work for empty or point-only isolations. The standalone root-sign API and
pair sign policy 1 retain their original evaluation and cancellation sequence.

## Source-bound diagnosis

The diagnostic replay used three actual cubic pairs, with primary root policy
version 1, width `0.001`, degree cap `32`, derived sign admission and world-error
request `0.0001`. Remaining subdivision caps were `3910`, `4024` and `3910` for
source pairs `[18,19]`, `[2,3]` and `[19,20]`. Each completed with genuine graph
evidence and one contact. These observations establish no performance result.

Pairs `[18,19]` and `[19,20]` each have only the accepted exact root `t=1`.
Pair `[2,3]` also retains rejected nonpoint candidates in brackets
`[353 * 2^-10, 177 * 2^-9]` and `[407 * 2^-9, 815 * 2^-10]`.
Every membership isolation includes its full candidate isolation.

An independent replay on original candidate `54e2df1` used the original eager
Math source, byte-equal to main `9e11` for `tarski.rs`. It measured legacy
callback counts `1184`, `1687`, `1184`, with total subdivisions `0`, `29`, `0`.
The earlier `694`, `1687`, `694` counts came from the rejected default shortcut
and are not legacy replay expectations.

## Verification

Legacy controls compare exact results and every observed cancellation
checkpoint with the original eager evaluation for empty, singleton, mixed and
irrational-only isolations. Independently known signs at `-1`, `0` and `1`,
repeated roots, zero and subnormal queries remain covered. Source/query admission
and invalid-policy precedence stay upstream. The version-2 controls separately
check its one entry poll plus one poll per exact point, including cancellation
at every observed checkpoint.

```sh
cargo test -p martensite-tkmath --lib predicates::roots::signs::
cargo test -p martensite-tkmath --test toolkit exact_roots::signs::
```

After remediation, the full Math library passed 681 tests, the focused root
controls passed 637, the public sign controls passed 15 and the public pair
controls passed 22. Strict all-target Math Clippy passed. The unchanged
complete-owned-operation callback controls remain required before integration.
Full PR and complete-cone performance qualification are separate
coordinator-owned gates.
