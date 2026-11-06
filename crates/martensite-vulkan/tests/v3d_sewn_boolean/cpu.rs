use super::*;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

fn volume(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass()
}

#[test]
fn sewn_case_has_closed_tetrahedron_volume() {
    let solid = geometry::sewn_tetrahedron().unwrap();
    assert!(
        BRepCheckAnalyzer::new(&solid, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert!((volume(&solid) - 1. / 3.).abs() <= 1e-12);
}

#[test]
fn genuine_cut_removes_positive_volume_bounded_by_the_actual_tool() {
    let result = sphere_minus_tetrahedron().unwrap();
    let whole = 4. * std::f64::consts::PI / 3.;
    let removed = whole - volume(&result);
    assert!(
        removed > 0. && removed <= 1. / 3. + 1e-12,
        "removed={removed}; expected positive volume bounded by tetrahedron1/3"
    );
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}
