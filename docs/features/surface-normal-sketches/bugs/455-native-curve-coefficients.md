# Native curve coefficient replay (#455)

The intrinsic oblique cylinder attachment failed byte-stable capture. A single
105-byte circle record reproduced the failure without topology, surfaces or
locations: legacy decoding normalized the three directions, discarded stored Y,
and rebuilt the frame. Several X/Y components changed in their low bits.
A separate 49-byte line record with `Direction3::new(2., 3., 7.)` also changed
because direction normalization is not bitwise idempotent.

`CurveFrameReadPolicy::PreserveStoredV1` retains checked coefficients.
`Axis2::from_stored_coefficients` reuses the outward stored-frame validation and
requires a direct frame. `Direction3::from_stored_coefficients` requires the
stored vector to lie in its outward normalization enclosure. Both retain the
original values after validation. Neither admits arbitrary affine frames.

Existing constructors and readers keep `LegacyReplay`. Explicit geometry-policy
constructors select curve decoding independently of surface and location
policies. Trimmed and offset curve bases propagate the selected policy, as do embedded
curves in extrusion/revolution surfaces and recursive surface bases.

Regression controls cover the original circle, a bounded integer-direction
corpus, all conic classes, recursive curve bases, malformed and truncated records,
indirect frames, and explicit legacy/default equivalence. These are coefficient
capture checks, not a claim that every native geometry type has a bitwise replay
contract. The attachment consumer supplies the end-to-end cylinder check.

Legacy conic frame construction remains deferred until all radius/focal scalars
have been read. A parallel main/X frame therefore preserves the old scalar-read
failure precedence and stream cursor, including truncated records. The explicit
checked policy validates its stored frame while reading it; that new policy has
no legacy malformed-record precedence contract.
