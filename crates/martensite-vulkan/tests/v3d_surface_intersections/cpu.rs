use super::*;
#[test]
fn restored_trimmed_surface_intersection_keeps_real_geometry_bindings_without_shape_conversion() {
    let surfaces = geometry::trimmed_inputs().unwrap();
    let bindings = geometry::actual_intersections(&surfaces).unwrap();
    assert!(bindings.curves.iter().all(|(_, curve)| !curve.is_null()));
    assert_eq!(bindings.authored_res_curve_count(), 0);
}
#[test]
fn torus_restoration_and_actual_intersection_preserve_literal_res_name_mismatch() {
    let surfaces = geometry::torus_inputs().unwrap();
    let bindings = geometry::actual_intersections(&surfaces).unwrap();
    assert_eq!(bindings.authored_res_curve_count(), 0);
    assert!(
        bindings
            .curves
            .iter()
            .all(|(name, curve)| name.starts_with("result") && !curve.is_null())
    );
}
