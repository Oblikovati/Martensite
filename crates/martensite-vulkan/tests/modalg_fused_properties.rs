//! All twelve mass/moment comparisons use properties of the actual constructed result.
use crate::{
    common_operation, mesh_information,
    primitive_shapes::{box_sized, rotated_about, translated},
    session_run::banner,
    shape_mesh::{MeshRequest, mesh_shape},
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;
type Outcome = Result<(), Box<dyn std::error::Error>>;
#[derive(Clone, Copy)]
pub(crate) enum Kind {
    Length,
    Area,
    Volume,
}

/// Fuse the original moved/rotated boxes, mesh and compare all properties; `fused_mesh_properties()?`.
pub(crate) fn fused_mesh_properties() -> Outcome {
    banner(&["========", "OCC29734", "========", ""]);
    let first = box_sized([1., 2., 3.])?;
    let second = box_sized([3., 2., 1.])?;
    let second = translated(&second, [0.5; 3])?;
    let second = rotated_about(&second, [0.; 3], [1.; 3], 30.)?;
    let result = common_operation::direct(&first, &second, Operation::Fuse);
    mesh_shape(&result, &MeshRequest::new(0.01))?;
    mesh_information::print_information(&result)?;
    for kind in [Kind::Length, Kind::Area, Kind::Volume] {
        compare(&result, kind)?;
    }
    println!("TEST COMPLETED");
    Ok(())
}
pub(crate) fn properties(
    shape: &Shape,
    kind: Kind,
    triangulation: bool,
) -> Result<GProps, Failure> {
    let mut properties = GProps::new();
    match kind {
        Kind::Length => BRepGProp::linear_properties(shape, &mut properties, false, triangulation)?,
        Kind::Area => BRepGProp::surface_properties(shape, &mut properties, false, triangulation)?,
        Kind::Volume => {
            BRepGProp::volume_properties(shape, &mut properties, false, false, triangulation)?
        }
    }
    // The source -full property command also computes center and inertia even though only moments are parsed.
    let _center = properties.centre_of_mass();
    let _inertia = properties.matrix_of_inertia();
    let _axes = property_axes(&properties)?;
    Ok(properties)
}
fn compare(shape: &Shape, kind: Kind) -> Outcome {
    println!();
    let geometry = properties(shape, kind, false)?;
    let mesh = properties(shape, kind, true)?;
    let name = match kind {
        Kind::Length => "length",
        Kind::Area => "area",
        Kind::Volume => "volume",
    };
    compare_value(name, geometry.mass(), mesh.mass(), false);
    let (gx, gy, gz) = geometry.principal_properties()?.moments();
    let (mx, my, mz) = mesh.principal_properties()?.moments();
    for (name, g, m) in [("IX", gx, mx), ("IY", gy, my), ("IZ", gz, mz)] {
        compare_value(name, g, m, true);
    }
    Ok(())
}
fn compare_value(name: &str, geometry: f64, mesh: f64, moment: bool) {
    if (geometry - mesh).abs() > 1.0e-7 {
        println!("Error : The {name} by geometry is {geometry}, by triangulation is  {mesh}");
    } else {
        let prefix = if moment { "moments " } else { "" };
        println!("The {prefix}{name} are equal  {geometry}");
    }
}

fn property_axes(properties: &GProps) -> Result<martensite_tkmath::gp::Axis2, Failure> {
    use martensite_tkmath::gp::{Axis2, Direction3};
    let principal = properties.principal_properties()?;
    Axis2::with_x_direction(
        properties.centre_of_mass(),
        Direction3::from_vector(principal.third_axis_of_inertia())?,
        Direction3::from_vector(principal.first_axis_of_inertia())?,
    )
}
