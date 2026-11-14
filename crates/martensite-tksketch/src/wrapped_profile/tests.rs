//! P1 controls intentionally cannot publish an accepted modelling result.
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};

pub(super) fn contour(first: u64, poles: &[&[[f64; 2]]]) -> Vec<AuthoredPiece> {
    let count = poles.len() as u64;
    poles
        .iter()
        .enumerate()
        .map(|(i, poles)| AuthoredPiece {
            span: CurveSpan {
                curve: SurfaceCurveId(first + i as u64),
                level: 0,
                index: 0,
            },
            start: CornerSource::Authored(SurfaceVertexId(first + i as u64)),
            end: CornerSource::Authored(SurfaceVertexId(first + (i as u64 + 1) % count)),
            poles: poles.to_vec(),
            chart_center: 0.,
        })
        .collect()
}

const OUTER: &[&[[f64; 2]]] = &[
    &[[0., 0.], [2., 0.]],
    &[[2., 0.], [2.5, 0.75], [2., 1.5]],
    &[[2., 1.5], [1.5, 2.], [0.5, 2.], [0., 1.5]],
    &[[0., 1.5], [0., 0.]],
];
const HOLE: &[&[[f64; 2]]] = &[
    &[[0.5, 0.5], [0.5, 1.]],
    &[[0.5, 1.], [1., 1.]],
    &[[1., 1.], [1., 0.5]],
    &[[1., 0.5], [0.5, 0.5]],
];

pub(super) fn request() -> Request {
    Request {
        region: SurfaceRegionId(5),
        loops: vec![contour(10, OUTER), contour(20, HOLE)],
        radius: 10.,
        depths: [-1., 2.],
        tolerance: 1e-5,
        max_fit_level: 8,
    }
}

#[test]
fn rational_loop_with_hole_freezes_measures_and_exports_without_publication() {
    let result = pipeline::wrap_extrude(&request()).unwrap();
    assert_eq!(result.keys.len(), 117);
    assert!(result.planar.area.encloses(3.525));
    assert_eq!(result.planar.certificate.components, 2);
    assert_eq!(
        result.report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert!(!result.report.is_exact());
    assert!(result.report.is_valid(), "{:?}", result.report);
    assert!(result.volume_estimate > 0.);
    assert!((0. ..=1e-8).contains(&result.integration_estimate));
    // Green's integral gives authored area 3.775 - 0.25 = 3.525.
    assert!((result.volume_estimate - 11.10375).abs() < 1e-5);
    assert_roundtrip(&result);
    assert_storage(&result);
    assert_cones(&result);
    assert_clearance(&result);
}

fn assert_storage(result: &pipeline::Unqualified) {
    assert_eq!(result.storage.len(), 19);
    for stored in &result.storage {
        let expected = match stored.span.curve.0 {
            10 => 1,
            11 => 2,
            12 => 3,
            _ => 0,
        };
        assert_eq!(stored.span.level, expected);
        assert_piece_storage(stored);
    }
}

fn assert_piece_storage(stored: &storage::StoredProof) {
    assert!(stored.depths.normal_sweep_error_bound() <= stored.depths.requested_limit());
    assert_eq!(
        stored.depths.stored_radii(),
        stored.certificate.stored_radii()
    );
    let report = &stored.certificate;
    assert!(
        report
            .radial_distance_bounds()
            .iter()
            .all(|bound| *bound < 1e-10)
    );
    assert!(report.scaled_ruling_bound() < 1e-10);
    assert!(report.height_difference_bound() < 1e-10);
    assert!(report.normal_sweep_error_bound() < 1e-10);
    assert!(stored.profile.developed_position_bound() <= request().tolerance / 4.);
    assert!(stored.profile.developed_derivative_bound().is_finite());
}

fn assert_roundtrip(result: &pipeline::Unqualified) {
    let (export, bindings) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    assert_eq!(bindings.len(), result.keys.len());
    assert!(!export.is_null());
    let reencoded = crate::surface_sketch::encoding::encode(&export, &bindings).unwrap();
    assert!(
        reencoded == result.bytes,
        "stored representation changed on readback"
    );
    let unique: std::collections::BTreeSet<_> = result.keys.iter().collect();
    assert_eq!(unique.len(), result.keys.len());
}

#[test]
fn rational_boss_and_pocket_interoperate_with_an_analytic_carrier() {
    use martensite_tkbo::brep_algo_api::{BRepAlgoAPICut, BRepAlgoAPIFuse};
    let result = pipeline::wrap_extrude(&request()).unwrap();
    assert!(result.report.is_valid(), "{:?}", result.report);
    let (tool, _) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    let carrier = carrier();
    let boss = BRepAlgoAPIFuse::from_shapes(&carrier, &tool).unwrap();
    assert!(!boss.has_errors());
    check_boolean_volume(boss.shape(), 29.355);
    let pocket = BRepAlgoAPICut::from_shapes(&carrier, &tool).unwrap();
    assert!(!pocket.has_errors());
    check_boolean_volume(pocket.shape(), 18.25125);
}

fn carrier() -> Shape {
    use crate::surface_sketch::*;
    normal_extrude(
        &carrier_rectangle(),
        [-2., 0.],
        NormalExtrusionOptions {
            resolution: check_resolution(),
            volume_relative_error: 1e-9,
        },
    )
    .unwrap()
    .export_brep()
    .unwrap()
    .solid
}

fn carrier_rectangle() -> crate::surface_sketch::CylinderRectangle {
    use crate::surface_sketch::*;
    CylinderRectangle {
        support: carrier_support(),
        region: SurfaceRegionId(99),
        s: [0., 4.],
        z: [-0.5, 2.5],
        corners: [100, 101, 102, 103].map(SurfaceVertexId),
        boundaries: [100, 101, 102, 103].map(SurfaceCurveId),
    }
}

fn carrier_support() -> crate::surface_sketch::CylinderSupport {
    use crate::surface_sketch::*;
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: -0.1,
        normal: NormalSense::Outward,
    }
}

fn check_resolution() -> martensite_tkbo::profile_extrusion::Resolution {
    use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}

fn check_boolean_volume(shape: &Shape, expected: f64) {
    use martensite_tkbo::body_check::validate_body;
    use martensite_tkgeombase::gprop::GProps;
    use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};
    let mut props = GProps::new();
    let status =
        BRepGProp::volume_properties_gk(shape, &mut props, 1e-8, true, VolumeGKFlags::default())
            .unwrap();
    assert!(status >= 0., "{status}");
    assert!(
        (props.mass() - expected).abs() < 1e-4,
        "actual={}, expected={expected}",
        props.mass()
    );
    let report = validate_body(shape, &check_resolution()).unwrap();
    assert!(report.is_valid(), "{report:?}");
}

#[test]
fn admission_refuses_open_joins_duplicate_ids_and_unbounded_charts() {
    for variant in 0..4 {
        let mut request = request();
        match variant {
            0 => request.loops[0][0].end = CornerSource::Authored(SurfaceVertexId(99)),
            1 => request.loops[0][0].span.curve = request.loops[1][0].span.curve,
            2 => request.loops[0][0].poles[0][0] = f64::INFINITY,
            _ => request.depths = [2., -1.],
        }
        let error = pipeline::wrap_extrude(&request).err().unwrap();
        assert_eq!(
            (error.region, error.stage),
            (request.region, Stage::Admission)
        );
        assert!(!error.cause.to_string().is_empty());
    }
}

fn assert_cones(result: &pipeline::Unqualified) {
    use std::cmp::Ordering;
    assert_eq!(result.cones.pieces.len(), 19);
    assert_eq!(result.cones.joins.len(), 19);
    for proof in result.cones.pieces.iter().chain(&result.cones.joins) {
        assert_eq!(
            proof.spans.len(),
            proof.certificate.directional_margin_bounds().len()
        );
        assert!(
            proof
                .certificate
                .directional_margin_bounds()
                .iter()
                .all(|margin| margin.compare_f64(0.).unwrap() == Ordering::Greater)
        );
    }
}

fn assert_clearance(result: &pipeline::Unqualified) {
    assert_eq!(result.clearance.spans.len(), 19);
    assert_eq!(result.clearance.certificate.pair_count(), 152);
    assert_eq!(result.clearance.certificate.excluded_pairs().len(), 19);
    assert!(result.clearance.certificate.clearance_lower_bound() >= 4. * request().tolerance);
}
