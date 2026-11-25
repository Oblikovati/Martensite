// SPDX-License-Identifier: GPL-2.0-only
//! Real closed graph loops cross the old integral occurrence cap without chunkwise magnitudes.
use super::*;
fn subdivided_ring() -> SketchProfile {
    let mut points: Vec<[f64; 2]> = (0..17).map(|i| [f64::from(i), 0.]).collect();
    points.extend((0..17).map(|i| [17., f64::from(i)]));
    points.extend((0..17).map(|i| [17. - f64::from(i), 17.]));
    points.extend((0..17).map(|i| [0., 17. - f64::from(i)]));
    let mut source = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: Vec::new(),
        curves: Vec::new(),
    };
    append_loop(&mut source, &points);
    append_loop(&mut source, &[[5., 5.], [12., 5.], [12., 12.], [5., 12.]]);
    source
}
fn append_loop(source: &mut SketchProfile, points: &[[f64; 2]]) {
    let first = source.vertices.len() as u64;
    for (index, point) in points.iter().enumerate() {
        let id = first + index as u64;
        source.vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point: *point,
        });
        source.curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(first + (index as u64 + 1) % points.len() as u64),
        });
    }
}
#[test]
fn explicit_whole_face_combines_outer_chunks_and_hole_before_area() {
    let source = subdivided_ring();
    let result = build(&source, version(8192), 8192).unwrap();
    assert_ring_areas(&result);
    assert!(
        result
            .integral_program_evidence()
            .unwrap()
            .matches_profile(&source)
    );
    assert_legacy_count_refusal(&source);
}
#[test]
fn whole_face_component_limit_is_not_a_per_chunk_allowance() {
    let requested = PlanarProofVersion::FilteredCompositeBernsteinV5 {
        max_components: 2,
        max_total_subintervals: 8192,
    };
    let failure = build(&subdivided_ring(), requested, 8192).unwrap_err();
    let proof = failure.integral_program_failure().unwrap();
    let numerical = proof.numerical_cause().unwrap();
    assert!(matches!(numerical.cause, martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramFailure::ComponentCount { actual: 3, limit: 2 }));
    assert!(proof.integration_subintervals() <= proof.granted_subintervals());
}

fn assert_legacy_count_refusal(source: &SketchProfile) {
    let legacy = build_planar_regions_with_proof_and_cancel(
        source,
        resolution(),
        PlanarProofWorkLimit::CurrentDefault,
        PlanarProofVersion::FilteredBernsteinV4,
        || false,
    )
    .unwrap_err();
    assert!(legacy.integral_program_failure().is_none());
    assert!(format!("{legacy}").contains("CurveCount"));
}

fn assert_ring_areas(result: &PlanarRegionSet) {
    assert_eq!(result.regions().len(), 2);
    assert!(
        result
            .regions()
            .iter()
            .any(|region| region.area().encloses(240.))
    );
    assert!(
        result
            .regions()
            .iter()
            .any(|region| region.area().encloses(49.))
    );
}
