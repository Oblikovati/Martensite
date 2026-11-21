//! #546: numerical quadrature agrees with certified patch flux only at the same field origin.
use super::*;
use crate::surface_sketch::wrapped_profile::pipeline::NATIVE_VOLUME_RELATIVE_TOLERANCE;
use martensite_tkbrep::{topexp::Explorer, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_gprop::{
    BRepGPropDomain, BRepGPropFace, BRepGPropVinertGK, VinertGKOptions,
};
#[derive(Debug)]
struct NumericalFlux {
    value: f64,
    absolute_error_estimate: f64,
}
pub(super) fn check(result: &WrappedConeNormalResult, detached: &DetachedWrappedBodies) {
    let report = &result.reports()[&SurfaceRegionId(1)];
    let faces = normals::faces(&detached.bodies);
    assert_eq!(faces.len(), report.coverage.faces);
    let measured = sum_faces(&faces, result.support().apex);
    assert_agreement(
        &measured,
        [report.boundary_flux.lower, report.boundary_flux.upper],
    );
}
fn sum_faces(faces: &[Shape], origin: [f64; 3]) -> NumericalFlux {
    let mut sum = NumericalFlux {
        value: 0.,
        absolute_error_estimate: 0.,
    };
    for face in faces {
        let measured = face_flux(face, origin);
        sum.value += measured.value;
        sum.absolute_error_estimate += measured.absolute_error_estimate;
    }
    sum
}
fn face_flux(face: &Shape, origin: [f64; 3]) -> NumericalFlux {
    let face = topods::face(face).unwrap();
    let mut surface = BRepGPropFace::with_face(&face, false).unwrap();
    let mut domain = BRepGPropDomain::with_face(&face).unwrap();
    let mut properties = BRepGPropVinertGK::new();
    properties.set_location(Point3::new(origin[0], origin[1], origin[2]));
    // The per-face API returns an absolute estimate; the whole-body API reports relative status.
    let absolute_error_estimate = properties
        .perform_domain(&mut surface, &mut domain, native_options())
        .unwrap();
    assert!(absolute_error_estimate.is_finite() && absolute_error_estimate >= 0.);
    NumericalFlux {
        value: properties.mass(),
        absolute_error_estimate,
    }
}
fn native_options() -> VinertGKOptions {
    VinertGKOptions {
        tolerance: NATIVE_VOLUME_RELATIVE_TOLERANCE,
        cg_flag: false,
        i_flag: false,
    }
}
fn assert_agreement(measured: &NumericalFlux, expected: [f64; 2]) {
    assert!(measured.value.is_finite());
    let scale = measured
        .value
        .abs()
        .max(expected[0].abs())
        .max(expected[1].abs());
    let numerical_target = NATIVE_VOLUME_RELATIVE_TOLERANCE * scale;
    let separation = (expected[0] - measured.value)
        .max(measured.value - expected[1])
        .max(0.);
    // Native estimates are not bounds; the requested relative tolerance defines this numerical control.
    assert!(
        separation <= numerical_target,
        "same-origin numerical flux={measured:?}, expected={expected:?}, requested_relative_tolerance={NATIVE_VOLUME_RELATIVE_TOLERANCE}, numerical_target={numerical_target}"
    );
}
fn primitive_faces(root: &Shape) -> Vec<Shape> {
    let mut explorer = Explorer::new(root, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut faces = Vec::new();
    while explorer.more() {
        faces.push(explorer.current().clone());
        explorer.next().unwrap();
    }
    faces
}
#[test]
fn closed_unit_box_flux_is_one_at_both_origins() {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    let solid = BRepPrimAPIMakeBox::from_sizes(1., 1., 1.)
        .unwrap()
        .solid()
        .unwrap();
    let faces = primitive_faces(&solid);
    assert_eq!(faces.len(), 6);
    for origin in [[0.; 3], [1., 0., 0.]] {
        assert_agreement(&sum_faces(&faces, origin), [1.; 2]);
    }
}
#[test]
fn open_unit_x_patch_flux_depends_on_origin() {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let plane = Plane3::new(
        Point3::new(1., 0., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    )
    .unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (0., 1., 0., 1.))
        .unwrap()
        .face()
        .unwrap();
    let faces = primitive_faces(&face);
    assert_eq!(faces.len(), 1);
    assert_agreement(&sum_faces(&faces, [0.; 3]), [1. / 3.; 2]);
    assert_agreement(&sum_faces(&faces, [1., 0., 0.]), [0.; 2]);
}
