# Stored analytic surface frames

Bug #432 exposed a loss between binary geometry storage and reconstruction. The
writer stores the origin and all main, X and Y direction coefficients. Legacy
readers normalize those directions and reconstruct X/Y. On the translated oblique
fixture, X.z changes from `0.03878767021121578` to `0.03878767021121576`.

`SurfaceFrameReadPolicy::LegacyReplay` remains the default. Callers that require
coefficient capture select `PreserveStoredV1` when constructing a `BinShapeSet`,
`BinShapeReader` or `BinSurfaceSet`, or pass it to
`read_with_surface_frame_policy` or `BinSurfaceSet::read_surface_with_frame_policy`.
The console variants accept the same policy. The policy stays fixed for the life
of a reader, including after `clear()`, so cached references share one interpretation.
Grouped versions 1–4 and recognized legacy headers keep their existing wire format.
The caller must record the policy for replay; headers do not select it.

## Numerical admission

`Axis3::from_stored_coefficients` treats finite binary64 values as exact real
inputs. It retains every origin and direction bit after the checks pass, including
signed zero. Origin coordinates must be finite. Each stored direction's outward
squared-norm enclosure must lie within `[1/2, 2]`. After interval normalization,
the main/X dot enclosure must lie within `[-1/2, 1/2]`. These guards bound the
conditioning of reconstruction; they do not admit a frame by themselves.

Outward arithmetic reconstructs a particular exact orthonormal frame:

```
N = normalize(stored_main)
X = normalize(N × (stored_X × N))
Y = normalize(N × X)
```

The outward enclosure of `dot(cross(stored_X, stored_Y), stored_main)` must exclude
zero. Its sign selects direct Y or its negation. Each original direction
coefficient must belong to its reconstructed component enclosure. Otherwise the
constructor refuses the frame. Thus accepted stored coefficients lie in boxes
containing one exact orthonormal frame. Their departure from it is bounded by
those boxes' widths. The retained floating values need not themselves form an
exact Euclidean orthonormal frame. This admission rule has no modelling tolerance
or caller error parameter.

Each elementary arithmetic result expands by one adjacent representable value
in each outward direction. Operations refuse nonfinite bounds and division by
an interval that does not have a strictly positive lower bound. Square-root
candidates expand outward and then require independent outward-square evidence:
the lower candidate's squared upper bound must be at most the input lower bound,
and the upper candidate's squared lower bound must be at least the input upper
bound. A zero lower candidate is certified directly. An inconclusive bracket is
refused. This square check avoids relying on host square-root accuracy.

The supported arithmetic assumes ordinary IEEE binary64 rounding for addition,
subtraction, multiplication and division with gradual underflow, as used by the
kernel's supported Rust targets. No floating environment is changed. Severe
underflow, overflow or wide enclosures can produce a conservative refusal.
Accumulated or transformed legacy frames are accepted only when they satisfy this
same contract.

## Scope and compatibility

The policy covers frames of planes, cylinders, cones, spheres and tori, including
those beneath trimmed and offset surfaces. Both grouped tables and reference
records use the same decoder. Constructors validate the complete frame before
publishing an owned `Axis3`; they do not normalize accepted coordinates.

Curves, conic Axis2 frames and standalone directions keep legacy decoding under
both policies. Extrusion and revolution direction records therefore retain their
existing interpretation. This policy does not promise byte preservation for an
entire BRep or for derived geometry evaluation. No binary format or package version
changes are part of this fix.

The fix lands on `main` before the candidate and feature branches receive it by
forward merge. Feature #414 keeps its strict coefficient checks and its oblique
fixture; callers requiring the stored frame select this boundary explicitly.
