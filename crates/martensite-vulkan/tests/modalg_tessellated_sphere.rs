//! The property-check tolerances and actual geometry removal are kept through shared adapters.
use crate::{
    primitive_shapes::sphere,
    session_run::banner,
    shape_checks::{MassCheck, MassKind, check_mass},
    shape_mesh::{MeshRequest, mesh_shape, remove_geometry_only},
};
use martensite_tkbrep::topods::Shape;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Measure the actual sphere before and after its geometry is removed; `tessellated_sphere_properties()?`.
pub(crate) fn tessellated_sphere_properties() -> Outcome {
    banner(&[
        "",
        "0029731: Modeling Algorithms - calculation of surface/volume of tessellated geometry",
        "",
    ]);
    let shape = sphere(10.)?;
    check_three(&shape, [4188.79, 1256.64, 62.8319])?;
    mesh_shape(&shape, &MeshRequest::new(0.01))?;
    remove_geometry_only(&shape)?;
    check_three(&shape, [4183.13, 1255.75, 62.8215])?;
    println!("TEST COMPLETED");
    Ok(())
}
fn check_three(shape: &Shape, expected: [f64; 3]) -> Outcome {
    for (kind, value, flag) in [
        (MassKind::Volume, expected[0], "v"),
        (MassKind::Area, expected[1], "s"),
        (MassKind::Length, expected[2], "l"),
    ] {
        println!("checkprops sp1 -{flag} {value}");
        check_mass(shape, &MassCheck::near(kind, value))?;
    }
    Ok(())
}
