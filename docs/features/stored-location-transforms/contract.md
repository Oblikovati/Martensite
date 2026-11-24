# Stored rigid location transforms

Bug #451 recorded coefficient drift when a normalized oblique placement passed
through BinTools location decoding. Both transform readers called `set_values`,
which factors determinant-derived scale and orthogonalizes. The owned fixture
changes three coefficients on its first read, by at most
`1.11022302462515654e-16`. A second read can change another coefficient. Raw binary
scalar encoding preserves the twelve coefficient bits.

`LocationTransformReadPolicy::LegacyReplay` remains the default and retains this
reconstruction behavior. `PreserveStoredRigidV1` adds checked coefficient capture
for frozen placements. It is independent of
`SurfaceFrameReadPolicy::PreserveStoredV1` from #432. Callers must record both
policies for replay; neither changes the wire format or its version.

## Admission and representation

`Transform3::from_stored_rigid_coefficients` treats the twelve finite binary64
coefficients as exact inputs. It passes the linear matrix columns Z, X and Y,
and the translation, to `Axis3::from_stored_coefficients`. That constructor's
[outward frame reconstruction contract](../stored-surface-frames/contract.md)
is the admission rule. It bounds each retained coefficient by an enclosure of
one exact orthonormal frame. The retained coefficients need not themselves be an
exact real orthogonal matrix. This rule adds no caller tolerance or modelling
error allowance. Inputs outside these enclosures are refused, including ordinary
scaled similarities, shears, singular matrices and nonfinite coefficients.
Values indistinguishable from rigid-frame roundoff under those enclosures have
the same admission semantics as #432.

The result has `Compound` form. A direct frame uses scale `+1` and the stored
linear coefficients. An indirect frame uses scale `-1` and their negation.
Multiplication by those exact signs restores all original coefficient bits,
including signed zero. Translation bits are retained unchanged. This keeps
`is_negative()` consistent with the stored frame handedness and retains the
orthogonal-matrix representation expected by transform arithmetic.

Inversion follows the existing transpose and unit-sign arithmetic. It has the
same floating-frame limitation as the admission contract; it is not an exact
inverse of an arbitrary affine matrix. No general affine transform is admitted
through this API. Powers and composition still perform floating arithmetic.
The preservation claim covers elementary stored datum coefficients, not a claim
that composing a location chain introduces no roundoff.

## Reader selection

Use `BinIStream::read_transform_with_policy` for a standalone record and
`BinLocationSet::read_with_policy` for a grouped location table. Use
`BinShapeReader::with_read_policies(surface_policy, location_policy)` for the
shape-reference format, or `BinShapeSet::with_read_policies` for grouped shapes.
`BinShapeSet::with_console_and_read_policies` also accepts an injected console.
Existing constructors and methods keep legacy location reconstruction.

A shape reader keeps both policies across `clear()` and cached references.
A refused stored transform returns an error; it never falls back to legacy
normalization. Frozen geometry must still recapture and certify the actual
decoded placement. Selecting this policy does not establish whole-operation
geometry, topology or error-budget acceptance.

## Verification

Owned tests exercise the public scalar-transform and location-table paths,
already-normalized oblique direct and indirect frames, repeated replay, signed
zero and reflection, inverse arithmetic, malformed matrices and legacy controls.
Complete grouped and reference shape-reader tests check policy propagation,
including reference caches and `clear()`. No writer, format or legacy default
changes are required.
