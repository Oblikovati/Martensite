# Surface sketch toolkit

`martensite-tksketch` constructs validated bodies from surface-bound sketches.
Its first operation extrudes a rectangular cylindrical region along the support
normals, including regions crossing the angular seam.

The toolkit depends on the existing public geometry builders and TKBO result
validation. It keeps the published body and semantic history immutable; native
exports have independent ownership.

See [supported operations and examples](../surface-normal-sketches/README.md),
[the geometry contract](../surface-normal-sketches/DESIGN.md) and
[verification evidence](../surface-normal-sketches/VERIFICATION.md).
