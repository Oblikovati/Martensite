# Storage certificates

This is the storage portion of P2 under #418. It implements conservative bounds
for two stored cylindrical curves and their linear depth interpolation. It does
not establish authored-profile conformance, regularity, topology or a valid body.

The public entry point is `certify_cylindrical_storage` in TKMath's
`predicates::roots::metric_bounds`. Its inputs are the actual stored control
points and retained weights, two positive radii, finite nonnegative error caps,
and explicit precision and work limits. Coordinates are in a local cylinder
chart at the origin with axis +Z. Missing weights mean exactly one.

## Bounds

For each depth curve, let `C=(X,Y,Z)/W` be its homogeneous Bernstein expression,
with positive weights. Products of stored binary64 poles and weights are treated
as exact real products and enclosed by outward dyadic intervals.

The radial residual is `D=X²+Y²-r²W²`. A Bernstein hull supplies `M >= |D|` over
the complete parameter interval `[0,1]`. If `w` is a positive lower bound for W,
then the radial distance is bounded by `M/(r*w²)`, since
`|rho-r| = |D|/(W²*(rho+r))` and `rho+r >= r > 0`.

The XY ruling residual uses the two numerators
`r0*X1*W0-r1*X0*W1` and `r0*Y1*W0-r1*Y0*W1`. The sum of their absolute hull
bounds, divided by `r0*w0*w1`, bounds the XY distance of C1 from `(r1/r0)*C0`.
The L1 bound also bounds the Euclidean distance. The height bound comes from
`Z1*W0-Z0*W1`, divided by `w0*w1`.

The separate bounds are not sufficient to enforce one storage allowance. Let
`b0` be the first radial bound, `q=r1/r0`, `e` the XY ruling bound and `h` the
height bound. Provided `b0 < r0`, the first curve has no point on the axis. Its XY
projection can be normalized to radius r0 while retaining its height. The
distance of the complete linear depth interpolation from this normalized
reference sweep is at most `max(b0, q*b0+e)+h`. This expression is also evaluated
outward and checked against its own unchanged cap. No exact normal-extrusion
claim is made for the stored surface itself.

All five results are conservative upper bounds, not tight maxima. An enclosure
above a requested cap returns `Uncertified`; it does not claim that the actual
geometry violates that cap. Failure to exclude the axis or retain a positive
dyadic denominator returns an explicit unresolved cause.

## Arithmetic and source binding

The proof loop uses fixed-grid dyadic intervals. It defaults to 128 fractional
bits and 100,000 interval operations. The supported ranges are 8 through 4096
fractional bits, stored degree 1 through 25, and at most one million operations.
No requested setting is clamped. Transient integers are limited to 16,384 bits.
The binomial coefficients through total degree 50 are cached per operation.

Bernstein products and residuals use integer arithmetic with directed division;
there is no gcd reduction in the polynomial loop. The existing exact-ratio
rounder is used only when publishing a binary64 bound. Arithmetic operations
poll cancellation and refuse before exceeding the requested operation count.
The operation context is Send, and report/request types have compile-time
Send/Sync controls. No global cache or mutable algorithm state is used.

Reports retain the original coefficient, weight and radius bits, the requested
caps and options, positive weight minima, and the arithmetic operation count.
`matches_source` checks the complete supplied representation, including signed
zero and whether weights were omitted. These bits bind proof evidence to its
inputs; they are not material or topology identities.

## Native binding

The private wrapped operation runs storage certification after freezing and
decoding. It looks up every span by its authored curve and canonical dyadic
interval. Missing or duplicate keys refuse. It captures actual native weights,
including unit weights after non-rational storage, rather than reusing the
fitter's requested coefficients.

The current native bridge requires identity locations, the local +Z cap frames,
and the expected stored radii. Each side must have two depth rows, identical
retained weights at both depths, and control points and weights matching the
captured boundary curves bit for bit. Under these conditions its Bezier surface
is the linear interpolation certified above. A changed side pole refuses before
an old boundary proof can be used. Changing both a boundary and its side row
still requires a new mathematical certificate and refuses when its bound fails.

The wrapped operation allocates a cap no greater than epsilon/1000, rounded
down. Each residual and the combined sweep bound must meet it. This protects the
single storage allowance from the sum of individually admissible errors.

## Evidence and remaining work

Closed-form controls cover exact generators, a rounded rational circle,
nonzero radial/ruling/height bounds and combined-budget failure. Independent
exact-ratio tests check signed outward arithmetic. An exact Jacobian quadratic-
form evaluation confirmed the nonzero residual `17/4` used by the radius-4
oracle; dividing by 4 gives its conservative radial bound `17/16`.
The [retained oracle](references/rational-storage-oracle.json) records that call.

Other controls cover nonfinite input, nonpositive or missing weights, unsupported
degree, cancellation and work exhaustion, mismatched coefficient counts,
subnormal precision refusal, source-bit changes, and native binding mutations.
Native conformance estimates and sampled body checks are not used as proof of
these storage bounds.

P2 still needs authored angular and height conformance, derivative bounds,
endpoint/join evidence, and a priori canonical piece sizing. Before publication,
the operation must also account for rounding of the requested radius-plus-depth
sums and any frame transformation, and bind the certified parameter domains to
actual face trims. P3 through P6 remain open. The native bridge currently refuses
non-identity frames; oblique-frame qualification belongs to P5.
