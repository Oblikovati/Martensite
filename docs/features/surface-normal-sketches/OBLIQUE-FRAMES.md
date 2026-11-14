# Oblique frame implementation gate

The wrapped-profile entry point currently refuses noncanonical cylinder frames.
P5 requires oblique placement. Removing that refusal without certifying the
stored placement would leave a gap in the world-space error contract.

## Existing interfaces to reuse

`surface_sketch/admission.rs::frame_inputs` checks finite coordinates and the
axis/radial Gram residual against 64 machine epsilons. That admission check is
not a certificate of an exactly orthonormal stored matrix.

`martensite-tkmath/src/gp/transform/forms.rs::Transform3::set_values` derives a
scale and orthogonalizes the matrix. The certificate must inspect the resulting
stored coefficients, rather than certify the supplied coefficients and assume
that the setter preserved them. Existing `Location`, native shape movement and
frozen encoding remain the transport interfaces.

The current publication budget already separates profile, storage, cap trace,
clearance and volume evidence. Placement needs an explicit allocation and
achieved bound within the original requested spatial epsilon. It must not be
added outside that epsilon or hidden by enlarging native tolerances.

## Frame meaning

For the new oblique capability, define the ideal right-handed frame by exact
normalization of the requested axis, projection of the requested radial vector
onto its perpendicular plane, and normalization of that projection. The tangent
is their cross product. Record this meaning in the API documentation when the
capability lands. Degenerate, nonfinite and uncertifiable frames refuse with
typed input or incomplete diagnostics.

The canonical frame keeps its existing path, coefficients and result bytes.
Other frames use the same geometry and publication pipeline, with a certified
placement step; they do not dispatch to the legacy rectangle operation.

## Required placement evidence

Let `R` be the ideal orthogonal frame, `A` the actual stored linear transform,
and `t` its stored translation. For canonical ideal and retained points `x` and
`y`, use the bound

```
||A y + t - (R x + o)||
    <= ||A|| ||y - x|| + ||A - R|| ||x|| + ||t - o||.
```

The certificate must establish each term from retained coefficients and
certified input bounds. An interval enclosure of the exact normalized frame
can use the existing exact-ratio arithmetic and outward square-root bounds.
A certified Gram-residual bound can establish upper and positive lower stretch
bounds for `A`. No sampled matrix or point checks may stand in for these bounds.

The input radius, physical depths and full authored profile bounds supply the
position bound. Include cap realization, storage and decoded-location rounding;
account for large origins and refuse when storage cannot meet the requested
epsilon. Report the requested epsilon, canonical allocations, placement
allocation, actual transform coefficients and achieved errors separately.

A positive lower stretch bound is required before transporting separation and
topology evidence. Propagate the upper stretch bound to tolerance and distance
bounds. Re-run the ordered native checks on the placed result before publication.
The original canonical validation report alone does not validate the placed body.

Use a certified determinant interval to transport retained flux and its error.
The authored ideal normal-volume oracle remains invariant under the ideal rigid
frame. Keep the native volume estimate distinct and recompute or transport it
with its numerical error explicitly. Do not label it a certified body volume.

## Complete-operation verification

Implement one obliquely placed profile with a hole through publication, detached
export and history before extending the glyph controls. Required evidence is:

- independent transformed-point and normal-direction oracles, including inward
  depths;
- exact canonical replay, plus deterministic oblique bytes, history and receipts
  in two processes and concurrent calls;
- per-face trims, shared incidences, cap/side correspondence and ordered native
  validation in the physical frame;
- independent ideal volumes and determinant-scaled retained-flux enclosures;
- near-degenerate, reflected, nonfinite and large-origin refusal controls;
- source mutation, cancellation and detached-export isolation;
- identical accepted P4 workloads measured before and after the change, with
  elapsed time, CPU and process peak memory.

Only after these checks pass should admission accept oblique frames. Arbitrary
chart-origin shifts and conical authored profiles remain separate parts of the
full capability; this gate does not qualify either one.
