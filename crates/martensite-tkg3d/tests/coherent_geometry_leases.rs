//! Owned admission is copied only from overlapping retained geometry leases.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::{
    ClosedSplineExtrusionInput, GeomBSplineCurve, GeomSurfaceOfLinearExtrusion,
};
use martensite_tkmath::gp::{Direction3, Point3};

fn supported_surface() -> GeomSurfaceOfLinearExtrusion {
    let points = [
        Point3::default(),
        Point3::new(1., 0., 0.),
        Point3::default(),
    ];
    let curve = GeomBSplineCurve::new(
        &Array1::from_shared(&points, 1),
        &Array1::from_shared(&[0., 1.], 1),
        &Array1::from_shared(&[3, 3], 1),
        2,
        false,
    )
    .unwrap();
    GeomSurfaceOfLinearExtrusion::new(&Handle::new(curve).cast(), Direction3::Z).unwrap()
}

fn owned_input(surface: &GeomSurfaceOfLinearExtrusion) -> ClosedSplineExtrusionInput {
    let extrusion = surface.try_input_lease().unwrap();
    assert!(!extrusion.has_evaluation_override());
    let basis = extrusion.basis().cast::<GeomBSplineCurve>();
    let spline = basis.get().unwrap().try_input_lease().unwrap();
    spline
        .closed_extrusion_input(extrusion.direction(), [-1., 1.])
        .unwrap()
}

#[test]
fn overlapping_leases_copy_the_same_owned_input_as_existing_admission() {
    let surface = supported_surface();
    let ClosedSplineExtrusionInput::Captured(input, layout) = owned_input(&surface) else {
        panic!("expected stored quadratic input")
    };
    assert_eq!(input.poles(), &[[0., 0., 0.], [1., 0., 0.], [0., 0., 0.]]);
    assert_eq!(input.direction(), [0., 0., 1.]);
    assert_eq!(layout.degree, 2);
    assert_eq!(layout.extrusion_interval, [-1., 1.]);
}

#[test]
fn copied_input_is_independent_after_all_leases_are_released() {
    let surface = supported_surface();
    let snapshot = owned_input(&surface);
    surface.set_direction(Direction3::X);
    let ClosedSplineExtrusionInput::Captured(input, _) = snapshot else {
        panic!("expected stored quadratic input")
    };
    assert_eq!(input.direction(), [0., 0., 1.]);
    assert_eq!(surface.try_input_lease().unwrap().direction(), [1., 0., 0.]);
}
