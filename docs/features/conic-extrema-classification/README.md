# Conic extrema classification

Bug [#406](https://github.com/Oblikovati/Martensite/issues/406) corrected minimum
flags for elementary conic roots in 2D and 3D. New computations use
`PointCurveClassification::DifferentialV2`. Explicit `LegacyV1` selection
supports replay through the elementary, generic and locate solvers.

The current design is [the exact stationary-jet decision](exact-jet-decision.md).
It supersedes [the initial classification decision](decision.md). Uncertain
roots retain their point and distance and expose `StationaryNature::Unresolved`;
their `is_min` accessor returns the named classification error.

Run the complete applicable local checks with `cargo xtask pr --base origin/main`.
The focused conic regression is
`cargo test -p martensite-tkgeombase --test extrema_conic_classification`.
