use super::*;
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkmath::gp::{Plane3, Point3};

fn independent_plane_surface() -> Handle<dyn GeomSurface> {
    let plane = GeomPlane::from_pln(Plane3::default());
    Handle::new(plane).cast()
}

#[test]
fn independent_plane_ui_so_keeps_closed_form_half_parameter_line() {
    let curve = actual_caught_isoline(&independent_plane_surface()).unwrap();
    for v in [-1., 0., 1.] {
        assert!(curve.value(v).unwrap().distance(&Point3::new(0.5, v, 0.)) < 1e-14);
    }
    assert_eq!(report_isoline_catch(&Ok(curve)), "OK OCC22910");
}

#[test]
fn case_surface_actual_failure_keeps_authored_caught_faulty_branch() {
    let surface = read_dataset_surface("s.rle").unwrap();
    let result = actual_caught_isoline(&surface);
    let failure = result
        .as_ref()
        .expect_err("authored exceptionTODO; expected actual UIso failure");
    assert_eq!(failure.exception_type(), "Standard_ConstructionError");
    assert!(failure.to_string().contains("Weights values too small"));
    assert_eq!(report_isoline_catch(&result), "Faulty OCC22910");
}

#[test]
fn successful_isoline_binding_u_does_not_create_authored_missing_result() {
    let curve = actual_caught_isoline(&independent_plane_surface()).unwrap();
    let bindings = BTreeMap::from([("u", curve)]);
    assert!(!bindings.contains_key("result"));
    missing_result_checkprops(&bindings);
}
