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
fn adaptive_ellipse_area_meets_the_independent_enclosure_target() {
    let shape = fixtures::elliptic_prism_faces();
    let mut fixed = GProps::new();
    BRepGProp::surface_properties_eps(&shape, &mut fixed, 1.0, false).unwrap();
    let adaptive = BRepGProp::surface_properties_with_options(
        &shape,
        martensite_tktopalgo::brep_gprop::SurfacePropertyOptions {
            integration: martensite_tktopalgo::brep_gprop::SurfaceIntegration::AdaptiveEstimate {
                requested_relative_target: 1e-5,
            },
            ..Default::default()
        },
    )
    .unwrap();
    assert!((fixed.mass() - 41.6317152760022).abs() < 0.002);
    assert!((adaptive.properties().mass() - 41.6317152760022).abs() <= 41.6317152760022 * 1e-5);
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
fn nonzero_narrow_parameter_square_keeps_its_physical_area() {
    let face = fixtures::narrow_parameter_square();
    let fixed_result = BRepGProp::surface_properties_with_options(
        &face,
        fixed(martensite_tktopalgo::brep_gprop::SurfaceRepresentation::PreferGeometry),
    )
    .unwrap();
    let adaptive = BRepGProp::surface_properties_with_options(
        &face,
        martensite_tktopalgo::brep_gprop::SurfacePropertyOptions {
            integration: martensite_tktopalgo::brep_gprop::SurfaceIntegration::AdaptiveEstimate {
                requested_relative_target: 1e-5,
            },
            ..Default::default()
        },
    )
    .unwrap();
    assert!((fixed_result.properties().mass() - 1.0).abs() < 1e-12);
    assert!((adaptive.properties().mass() - 1.0).abs() < 1e-12);
}
mod empty;
