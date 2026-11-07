# Adaptive surface integration accuracy

Bug [#407](https://github.com/Oblikovati/Martensite/issues/407) exposed a general
nested-integration defect. An inner error budget stayed zero, which disabled U
refinement. Each outer sample then erased the inner error, including on the
second outer order. A V-constant integrand could report zero error while its U
integral was inaccurate.

[Numerical behavior 2](decision.md) initializes and updates the inner budget,
uses the same inner convergence rule for both outer orders, and retains the
weighted sum of inner and outer estimates. It removes the absolute parameter
cutoff that discarded nonzero intervals. Exhausted work returns
`StdFail_NotDone`, with direction, interval cap, error estimate and requested
target. The existing work limits remain unchanged. Fixed-order integration and
public option/result shapes remain unchanged.

The error is still a quadrature estimate. It is not a certified bound. In the
owned API, a failed computation publishes no result. An axis ratio of `1e-6`
at target `1e-12` exceeds the current inner refinement capacity; this is an
explicitly tested failure, not a completed measurement.

## Independent controls

The first case has elliptic axes 1 and 0.6, U bounds pi/2 to 3*pi/2, and extrusion
height 2. Its area is `4 E(16/25)`. The [rational enclosure](reference-bounds.json)
uses the existing independent elliptic series and Machin pi proof, with the
next-term geometric tail. Regenerate it with:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 docs/features/surface-integration-accuracy/reference_bounds.py
```

The enclosure width is below `1e-50`. Tests use its nearest binary64 value,
`5.105399772679626`. They check requested relative targets `1e-5`, `1e-9` and
`1e-12`, and compare actual error with the reported estimate plus a binary64
roundoff allowance. Centroid checks use independently integrated weighted
elliptic arc length.

[Eccentricity controls](../../../crates/martensite-tktopalgo/tests/data/surface_integration/eccentricity-controls.csv) were evaluated with mpmath
at 80 decimal digits using `4*ellipe(1-b*b)`. These are high-precision numerical
controls, separate from the rational certificate. Generated variations cover
model scales `1e-6` through `1e6`, reversed faces, period shifts, shifted
half-periods, V spans `1e-13` through `1e9`, and nearly circular or flat ellipses.

## OCCT evidence

The [OCCT 8.0.0 probe](occt_reference.cpp) uses the public bounded-face API and
explicit integration targets. Its whole-face results reproduce the old error
bit for bit. That result cannot serve as an accurate area oracle.

The probe also integrates an exact partition of the same surface into 32 U
subfaces. Its summed area agrees with the rational enclosure within binary64
roundoff. [Recorded output](../../../crates/martensite-tktopalgo/tests/data/surface_integration/occt-reference.csv) retains both computations.
Tests require agreement with the independently checked partitioned area and
centroid. No tessellation participates in either computation.

Build the probe against an OCCT installation with:

```sh
g++ -std=c++17 -I"$OCCT_PREFIX/include/opencascade" \
  docs/features/surface-integration-accuracy/occt_reference.cpp \
  -L"$OCCT_PREFIX/lib" -Wl,-rpath-link,"$OCCT_PREFIX/lib" \
  -lTKTopAlgo -lTKBRep -lTKGeomBase -lTKG3d -lTKMath -lTKernel \
  -o /tmp/oblikovati-surface-occt-probe
LD_LIBRARY_PATH="$OCCT_PREFIX/lib" /tmp/oblikovati-surface-occt-probe
```

## Verification

Run `cargo test -p martensite-tktopalgo --lib --test toolkit`. The existing
surface-policy replay test compares complete property and error bytes across
two processes and one/four workers. Its owned-result tests also check source
isolation, representation choice, and completed-empty results.

The numerical migration changes old inaccurate adaptive values. The earlier
[decision](../surface-property-policy/decision.md) and
[historical replay artifacts](../surface-property-policy/legacy-replay/README.md)
remain intact. Tests now gate adaptive results against independent area
controls and require nonzero narrow parameter ranges to retain their area.
