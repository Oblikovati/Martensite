# Volume properties

`BRepGProp::volume_properties` uses adaptive Gauss-Kronrod integration for geometric
faces. It publishes the accumulated mass, center of mass and inertia only after every
selected face completes. An incomplete face integration returns `Failure` with exception
type `BRepGProp_IncompleteIntegration`; the message identifies the face's traversal
index, returned status and expected completed status. The caller's existing properties
remain unchanged on a geometric integration error (#246).

An empty shape, a shape with no eligible faces, or an `only_closed` selection with no
closed shells completes successfully with zero mass. Faces outside selected closed
shells do not affect that selection's completion.

The explicit `volume_properties_gk` and `volume_properties_gk_plane` methods retain
their numeric status contract: negative integration status indicates failure, and an
empty closed-shell selection returns `-1`. Callers of those methods must inspect the
status before using the accumulated properties.

The geometric route requests relative quadrature error `1e-5`. Triangulation and
mesh-only face processing use their existing route.

Inner U integration keeps every interval of positive width, including intervals narrower
than `1e-9` in the surface's parameter space. A fixed parameter-width cutoff can discard a
real contribution without including it in the reported integration error. A U interval
with no positive width is omitted; adaptive quadrature evaluates the remaining intervals
against the requested error budget (#386).

The regression constructs a skewed planar polygon, sweeps it, checks geometric
validity, and compares its volume with an independent polygon-area calculation.
Generated inputs vary skew sign, magnitude, scale, and translation. The small failing
input's relative error was about `6.25e-11` while the integrator reported about
`6.8e-16`; its explicit integration request was `1e-12`.
