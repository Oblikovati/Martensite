//! Closed forms and actual mutable builders exercise the coherent source/domain join.
use super::*;
use martensite_tkbrep::{
    brep::BRepTFace,
    topods::{self, Shape},
};
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Handle,
};
use martensite_tkg3d::{geom::GeomBSplineSurface, top_abs::Orientation};
use martensite_tkmath::{
    gp::{Point3, Transform3, Vector3},
    toploc::Location,
};
fn affine_poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            poles
                .set_value(u, v, Point3::new(f64::from(u - 1), f64::from(v - 1), 0.))
                .unwrap();
        }
    }
    poles
}
fn affine() -> Handle<GeomBSplineSurface> {
    let poles = affine_poles();
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    Handle::new(
        GeomBSplineSurface::new(
            &poles,
            (&knots, &mults),
            (&knots, &mults),
            (1, 1),
            (false, false),
        )
        .unwrap(),
    )
}
fn face(surface: &Handle<GeomBSplineSurface>) -> Face {
    let fields = Handle::new(BRepTFace::new());
    fields.set_surface(&surface.cast());
    let shape = Shape::from_parts(fields.cast(), Location::default(), Orientation::Forward);
    topods::face(&shape).unwrap()
}
fn request() -> SurfaceSelfQuery {
    SurfaceSelfQuery {
        bounds: [[0., 1.], [0., 1.]],
        periodic: [false, false],
        tolerances: [1e-7, 1e-8],
        tangency: true,
    }
}
fn certify(surface: &Handle<GeomBSplineSurface>, query: SurfaceSelfQuery) -> FaceQueryReport {
    certify_face_query(&face(surface), query, FaceQueryPolicy::bounded(), || false)
}
#[test]
fn actual_root_support_and_unchanged_query_settings_join_the_owned_certificate() {
    let query = request();
    let report = certify(&affine(), query);
    assert!(report.is_certified());
    assert_eq!(report.request(), query);
    let source = report.input().unwrap();
    assert!(source.cells().iter().any(|(id,cell)| matches!(cell,martensite_tkbrep::brep::face_input_capture::CapturedFaceCell::Face {surface,..} if *id == source.root().owner && Some(*surface) == report.surface_owner())));
}
#[test]
fn source_mutation_after_capture_cannot_change_the_published_proof() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<FaceQueryReport>();
    let surface = affine();
    let report = certify(&surface, request());
    let old = format!("{:?}", report);
    surface.set_pole(1, 1, Point3::new(9., 9., 9.)).unwrap();
    assert_eq!(format!("{:?}", report), old);
    assert!(report.is_certified());
}
#[test]
fn nonperiodic_trim_tail_and_fabricated_periodicity_remain_incomplete() {
    let surface = affine();
    for axis in 0..2 {
        let mut query = request();
        query.bounds[axis][1] = 1f64.next_up();
        assert!(!certify(&surface, query).is_certified());
        query = request();
        query.periodic[axis] = true;
        assert!(!certify(&surface, query).is_certified());
    }
}
#[test]
fn smaller_closed_query_is_covered_without_altering_requested_tolerance() {
    let mut query = request();
    query.bounds = [[0.25, 0.75], [0.125, 0.875]];
    query.tolerances = [0., f64::from_bits(1)];
    let report = certify(&affine(), query);
    assert!(report.is_certified());
    assert_eq!(report.request(), query);
}
fn annular_poles() -> Array2<'static, Point3> {
    let rows = annular_rows();
    let mut poles = Array2::with_bounds(1, 6, 1, 2).unwrap();
    for (u, p) in rows.iter().enumerate() {
        for v in 1..=2 {
            poles
                .set_value(
                    u as i32 + 1,
                    v,
                    Point3::new(f64::from(v) * p[0], f64::from(v) * p[1], f64::from(v - 1)),
                )
                .unwrap();
        }
    }
    poles
}
fn annulus() -> Handle<GeomBSplineSurface> {
    let poles = annular_poles();
    let uk = Array1::from_shared(&[0., 1., 2., 3.], 1);
    let um = Array1::from_shared(&[2; 4], 1);
    let vk = Array1::from_shared(&[0., 1.], 1);
    let vm = Array1::from_shared(&[2; 2], 1);
    Handle::new(
        GeomBSplineSurface::new(&poles, (&uk, &um), (&vk, &vm), (2, 1), (true, false)).unwrap(),
    )
}
#[test]
fn periodic_query_covers_adjacent_alias_tail_without_claiming_a_pole_collapse() {
    let mut query = request();
    query.periodic = [true, false];
    query.bounds[0] = [0., 3f64.next_up()];
    let report = certify(&annulus(), query);
    assert!(report.is_certified(), "{:?}", report.incomplete());
    assert_eq!(report.request(), query);
    assert_eq!(report.proof().unwrap().certificate(),Some(&martensite_tkmath::predicates::roots::surface_query::SurfaceQueryTheorem::AxialAngularCycle));
}
#[test]
fn undeclared_multiple_periods_and_nonperiodic_v_tail_are_refused() {
    let mut query = request();
    query.periodic = [true, false];
    query.bounds[0] = [0., 6f64.next_up()];
    assert!(!certify(&annulus(), query).is_certified());
    query.bounds = [[0., 3.], [0., 1f64.next_up()]];
    assert!(!certify(&annulus(), query).is_certified());
}
#[test]
fn transformed_support_is_not_silently_replaced_by_unrounded_affine_poles() {
    let surface = affine();
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(1., 0., 0.));
    let fields = Handle::new(BRepTFace::new());
    fields.set_surface(&surface.cast());
    let shape = Shape::from_parts(
        fields.cast(),
        Location::new(transform),
        Orientation::Forward,
    );
    let placed = topods::face(&shape).unwrap();
    assert!(
        !certify_face_query(&placed, request(), FaceQueryPolicy::bounded(), || false)
            .is_certified()
    );
}
#[test]
fn cancellation_and_work_limits_never_expose_a_successful_partial_proof() {
    let surface = affine();
    let report = certify_face_query(
        &face(&surface),
        request(),
        FaceQueryPolicy::bounded(),
        || true,
    );
    assert_eq!(report.incomplete(), Some(&FaceQueryIncomplete::Cancelled));
    assert!(!report.is_certified());
    let mut policy = FaceQueryPolicy::bounded();
    policy.proof.max_steps = 0;
    let report = certify_face_query(&face(&surface), request(), policy, || false);
    assert!(matches!(
        report.incomplete(),
        Some(FaceQueryIncomplete::Proof { .. })
    ));
    assert!(report.proof().is_some());
    assert!(!report.is_certified());
}

fn annular_rows() -> [[f64; 2]; 6] {
    [
        [1., 0.],
        [1., 2.],
        [-0.5, 1.],
        [-2., 0.],
        [-0.5, -1.],
        [1., -2.],
    ]
}
