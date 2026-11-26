// SPDX-License-Identifier: GPL-2.0-only
use super::{
    BRepGProp, GProps, HistoryRole, SpatialPrismFailure, archive, extrude_wire, options, recipe,
    snapshot,
};

use martensite_tkbo::profile_extrusion::{
    ExtrudeOptions, ProfileCurve, ProfileCurveId, ProfilePlane, ProfileVertex, ProfileVertexId,
    SketchProfile, build_planar_line_regions_with_work_and_cancel, build_planar_regions, extrude,
};

#[test]
fn retention_refuses_a_rounded_translated_cap_and_admits_the_adjacent_exact_vector() {
    let x = 134_217_728.0;
    let points = [[x, 0., 0.], [x, 3., 0.], [x, 3., 5.], [x, 0., 5.]];
    let wire = recipe(&snapshot(points));
    let half_ulp = (x.next_up() - x) / 2.0;
    let direction = [3. + half_ulp, 0., 0.];
    let refused = extrude_wire(&wire, direction, options(), || false);
    assert!(
        matches!(refused,
        Err(SpatialPrismFailure::SweepVertexTrace { role: HistoryRole::EndVertex, bound, limit, .. })
        if bound >= half_ulp && bound > limit && limit == options().source_trace_limit),
        "{refused:?}"
    );

    let exact = extrude_wire(&wire, [3., 0., 0.], options(), || false).unwrap();
    let native = exact.export_brep(|| false).unwrap();
    let mut measure = GProps::default();
    BRepGProp::volume_properties(&native.solid, &mut measure, true, false, false).unwrap();
    assert!((measure.mass() - 45.).abs() < 1e-11);
    assert!(exact.volume().encloses(45.) && exact.volume().encloses(measure.mass()));
}

#[test]
fn scoped_lines_preserve_default_planar_prism_body_and_history() {
    let uv = [[0., 0.], [3., 0.], [3., 5.], [0., 5.]];
    let profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: uv
            .into_iter()
            .enumerate()
            .map(|(i, point)| ProfileVertex {
                id: ProfileVertexId(i as u64),
                point,
            })
            .collect(),
        curves: (0..4)
            .map(|i| ProfileCurve::Line {
                id: ProfileCurveId(i + 10),
                start: ProfileVertexId(i),
                end: ProfileVertexId((i + 1) % 4),
            })
            .collect(),
    };
    let options = options();
    let legacy = build_planar_regions(&profile, options.resolution).unwrap();
    let scoped = build_planar_line_regions_with_work_and_cancel(
        &profile,
        options.resolution,
        options.planar_work,
        || false,
    )
    .unwrap();
    let built = [&legacy, &scoped].map(|regions| {
        extrude(
            &regions.regions()[0],
            [0., 0., 5.],
            &ExtrudeOptions {
                resolution: options.resolution,
            },
        )
        .unwrap()
    });
    assert_eq!(built[0].history(), built[1].history());
    assert_eq!(built[0].volume(), built[1].volume());
    assert_eq!(archive(built[0].solid()), archive(built[1].solid()));
}
