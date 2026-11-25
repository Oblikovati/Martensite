# Surface sketch toolkit

`martensite-tksketch` constructs validated bodies from surface-bound sketches.
It extrudes solved profiles with holes along cylinder and cone normals,
constructs full-turn bands with apertures, and applies bosses and pockets to
selected faces. Applications can reuse a revision-bound chart to map sketch
edits and replay a feature with the same authored curve and region identities.

The toolkit depends on the existing public geometry builders and TKBO result
validation. It keeps the published body and semantic history immutable; native
exports have independent ownership.

See [supported operations and examples](../surface-normal-sketches/README.md),
[the geometry contract](../surface-normal-sketches/DESIGN.md) and
[verification evidence](../surface-normal-sketches/VERIFICATION.md).
