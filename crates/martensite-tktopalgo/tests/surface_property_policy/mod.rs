//! Owned controls for independent integration and representation choices (#277).
mod fixtures;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::f64::consts::PI;

#[test]
fn established_geometry_and_mesh_measurements_keep_closed_form_controls() {
    let (face, _) = fixtures::meshed_cylinder();
    let mut geometry = GProps::new();
    BRepGProp::surface_properties(&face, &mut geometry, false, false).unwrap();
    let mut mesh = GProps::new();
    BRepGProp::surface_properties(&face, &mut mesh, false, true).unwrap();
    assert!((geometry.mass() - PI).abs() < 1e-12);
    assert!((mesh.mass() - 2.0_f64.sqrt() * 2.0).abs() < 1e-12);
}

#[test]
fn established_ellipse_modes_remain_distinct_from_the_independent_enclosure() {
    let shape = fixtures::elliptic_prism_faces();
    let mut fixed = GProps::new();
    BRepGProp::surface_properties_eps(&shape, &mut fixed, 1.0, false).unwrap();
    let mut adaptive = GProps::new();
    let error = BRepGProp::surface_properties_eps(&shape, &mut adaptive, 1e-5, false).unwrap();
    println!(
        "fixed={:.17} adaptive={:.17} estimate={:.17e}",
        fixed.mass(),
        adaptive.mass(),
        error
    );
    assert!((fixed.mass() - 41.6317152760022).abs() < 0.002);
    assert!((adaptive.mass() - 41.6317152760022).abs() > 0.1);
}
mod options;

mod representations;

fn fixed(
    representation: martensite_tktopalgo::brep_gprop::SurfaceRepresentation,
) -> martensite_tktopalgo::brep_gprop::SurfacePropertyOptions {
    martensite_tktopalgo::brep_gprop::SurfacePropertyOptions {
        integration: martensite_tktopalgo::brep_gprop::SurfaceIntegration::FixedOrder,
        representation,
        skip_shared: false,
    }
}
mod closed_forms;
mod isolation;
mod legacy_replay;
mod replay;
mod support;

#[test]
fn narrow_parameter_square_records_existing_fixed_and_adaptive_outputs() {
    let face = fixtures::narrow_parameter_square();
    let mut fixed = GProps::new();
    BRepGProp::surface_properties_eps(&face, &mut fixed, 1.0, false).unwrap();
    let mut adaptive = GProps::new();
    let error = BRepGProp::surface_properties_eps(&face, &mut adaptive, 1e-5, false).unwrap();
    println!(
        "narrow fixed={:?} adaptive={:?} error={:?}",
        support::property_bits(&fixed),
        support::property_bits(&adaptive),
        error.to_bits()
    );
    assert!((fixed.mass() - 1.0).abs() < 1e-12);
    assert_eq!(adaptive.mass(), 0.0);
}
mod empty;
