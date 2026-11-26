// SPDX-License-Identifier: GPL-2.0-only
//! The actual spherical primitive band, its placed original occurrence and trace refusals.
use super::*;
use martensite_tkbrep::brep::spherical_attachment::{
    SphericalFaceDomain, SphericalFaceError, capture_spherical_band,
};
use martensite_tkg3d::geom::{GeomCircle, GeomSphericalSurface};
use martensite_tkmath::predicates::roots::metric_bounds::spherical_band::SphericalBandError;
use martensite_tkprim::{brep_prim::BRepPrimOneAxis, brep_prim_api::BRepPrimAPIMakeSphere};
use std::f64::consts::TAU;

const LIMIT: f64 = 1e-9;

fn native(mut maker: BRepPrimAPIMakeSphere, delta: [f64; 3]) -> (Solid, Face, Face) {
    let mut solid = topods::solid(maker.shape().unwrap()).unwrap();
    let original = maker.sphere().lateral_face().unwrap();
    if delta != [0.; 3] {
        translated(&mut solid, delta);
    }
    let actual = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|face| face.is_partner(&original))
        .unwrap();
    (solid, topods::face(&actual).unwrap(), original)
}

fn capture_band(
    solid: &Solid,
    face: &Face,
    limit: f64,
) -> Result<SphericalFaceDomain, SphericalFaceError> {
    capture_spherical_band(
        solid,
        face,
        SOURCE,
        FaceCaptureLimits::bounded(),
        limit,
        || false,
    )
}

#[test]
fn actual_translated_native_band_has_the_shared_curved_seam_and_bounded_rims() {
    fn shared<T: Send + Sync>() {}
    shared::<SphericalFaceDomain>();
    let (solid, actual, original) = native(
        BRepPrimAPIMakeSphere::with_latitudes(2., 0., 0.5).unwrap(),
        [8., -4., 16.],
    );
    assert!(
        BRepCheckAnalyzer::new(&solid, true, false, true)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let band = capture_band(&solid, &actual, LIMIT).unwrap();
    assert_eq!(band.source(), SOURCE);
    assert_eq!(band.latitude(), [0., 0.5]);
    let [trace, requested] = band.trace();
    assert_eq!(requested, LIMIT);
    assert!(trace > 0. && trace < 1e-13);
    assert!(matches!(
        capture_band(&solid, &original, LIMIT),
        Err(SphericalFaceError::Source(Failure::FaceOccurrence))
    ));
    let seam = Explorer::new(&actual, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|edge| topods::edge(&edge).unwrap())
        .find(|edge| BRepTool::is_closed_on_face(edge, &actual).unwrap())
        .unwrap();
    let retained = BRepTool::curve_with_location(&seam).unwrap();
    assert!(retained.curve.downcast::<GeomCircle>().is_some());
    assert_eq!([retained.first, retained.last], [TAU, TAU + 0.5]);
    assert!(BRepTool::curve_on_face(&seam, &actual).unwrap().is_stored);
}

#[test]
fn partial_and_pole_bands_and_excess_native_trace_refuse() {
    let (sphere, pole_face, _) = native(BRepPrimAPIMakeSphere::new(2.).unwrap(), [0.; 3]);
    assert!(matches!(
        capture_band(&sphere, &pole_face, LIMIT),
        Err(SphericalFaceError::Source(Failure::Representation(_)))
    ));
    let (wedge, partial, _) = native(
        BRepPrimAPIMakeSphere::with_latitudes_angle(2., 0., 0.5, 1.).unwrap(),
        [0.; 3],
    );
    assert!(matches!(
        capture_band(&wedge, &partial, LIMIT),
        Err(SphericalFaceError::Rectangle(_))
    ));
    let (solid, face, _) = native(
        BRepPrimAPIMakeSphere::with_latitudes(2., 0., 0.5).unwrap(),
        [0.; 3],
    );
    assert!(matches!(capture_band(&solid, &face, 0.),
        Err(SphericalFaceError::Numeric(SphericalBandError::Trace { bound, limit: 0. })) if bound > 0.
    ));
}

#[test]
fn the_immutable_native_capture_survives_source_edits_and_refuses_late_cancel() {
    let (solid, face, _) = native(
        BRepPrimAPIMakeSphere::with_latitudes(2., 0., 0.5).unwrap(),
        [0.; 3],
    );
    let mut reached = 0;
    let captured = capture_spherical_band(
        &solid,
        &face,
        SOURCE,
        FaceCaptureLimits::bounded(),
        LIMIT,
        || {
            reached += 1;
            false
        },
    )
    .unwrap();
    let mut remaining = 0;
    assert!(matches!(
        capture_spherical_band(
            &solid,
            &face,
            SOURCE,
            FaceCaptureLimits::bounded(),
            LIMIT,
            || {
                remaining += 1;
                remaining == reached
            }
        ),
        Err(SphericalFaceError::Cancelled)
    ));
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    surface
        .downcast::<GeomSphericalSurface>()
        .get()
        .unwrap()
        .set_radius(2.25)
        .unwrap();
    assert!(matches!(capture_band(&solid, &face, LIMIT),
        Err(SphericalFaceError::Numeric(SphericalBandError::Trace { bound, .. })) if bound > LIMIT
    ));
    assert_eq!(captured.latitude(), [0., 0.5]);
    assert!(captured.trace()[0] < LIMIT);
}
