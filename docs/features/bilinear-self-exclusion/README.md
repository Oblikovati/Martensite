# Bilinear support self-exclusion

Feature #213 adds an owned certificate of global injectivity for a supported continuous bilinear surface on a closed parameter rectangle. It does not replace general surface validation or certify every spline. Unsupported and folded projections return `Inconclusive`; that result does not assert a self-intersection.

The numeric input contains four finite poles in boundary order `(00,10,11,01)`, two finite strictly increasing parameter intervals and four positive finite weights. A certificate requires identical weights. It also requires each span and its reciprocal to be finite in binary64. Other positive spans remain explicitly inconclusive rather than being normalized through overflow.

For a spline support, admission requires degree `(1,1)`, exactly two poles in each direction, no periodic direction, exactly two knots per direction and endpoint multiplicities `(2,2)`. An external evaluation representation can replace the stored net, so a nonempty override returns the explicit `EvaluationOverride` inconclusive result. The adapter reads override presence and the remaining support fields under one read guard, checks stored numeric values, copies the admitted input and releases the guard before certification. Constant weights cancel from the rational basis. The degree-1 clamped basis therefore represents the bilinear map on the recorded knot rectangle. No conversion ancestry or surface-family label establishes this result.

## Exact sign argument

For each coordinate-plane projection, the implementation calls the existing exact orientation predicate on the original supplied corner coordinates. It never subtracts floating-point tangents to form a rounded determinant. The four consecutive boundary turns are the four corner Jacobian determinants, cyclically ordered. All must be strictly positive or all strictly negative. Their strict consistency also makes the projected boundary a convex quadrilateral.

In normalized parameters, write the projected map as

`F(s,t) = A + B s + C t + D s t`.

Its Jacobian determinant is affine:

`J(s,t) = det(B,C) + s det(B,D) + t det(D,C)`.

It is also the bilinear convex combination of its four corner values. Consistently strict corner signs therefore make `J` nonzero throughout the rectangle, including its boundary.

For any two parameter points `p` and `q`, let `m=(p+q)/2`. Direct expansion gives the exact identity

`F(p) - F(q) = DF(m) (p-q)`.

The rectangle is convex, so `m` remains inside it. Since `DF(m)` is invertible, `F(p)=F(q)` implies `p=q`. Thus the projection is globally injective, and so is the three-dimensional patch. This argument covers both signs and nonplanar bilinear nets. It uses no sampling, tessellation or metric tolerance.

## Binding and limits

The certificate's fields are private. The spline certificate owns the validated poles, intervals and weights together with its admission layout, including the observed absence of an evaluation override. It retains no surface handle, cache, read guard or mutable geometry alias. `same_representation` compares those recorded numeric values bit for bit. This binds mathematical input, not entity identity. A certificate remains about that owned snapshot after the source surface changes; it never certifies a later-mutated handle automatically.

A caller must bind the certificate to the actual immutable support and prove any requested trim domain lies within the recorded rectangle. The certificate says nothing about topology, trims, tolerance consistency, other faces, or support extrapolation outside that rectangle. It certifies the exact continuous map represented by the supplied binary64 coefficients. It does not claim that rounded evaluations are bitwise injective, stable or error-bounded. Those are separate validation obligations.

Tests include independent planar and nonplanar examples, reversed and cyclic boundary order, bow-tie and concave projections, determinant underflow and overflow cases, domain and weight restrictions, and real spline input isolation after mutation. A finite-grid identity control checks the algebraic midpoint identity; it is a test of the implementation argument, not the acceptance predicate or a sampled exclusion certificate.
