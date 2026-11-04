//! Named original vertices and shared located wires retain their actual shape identity.
#[cfg(test)]
mod cpu;
mod geometry;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib,
    brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape},
};
type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(super) fn wire_bounds(shape: &Shape) -> Result<[f64; 6], Failure> {
    let mut bounds = martensite_tkmath::bnd::BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true)?;
    bounds.get()
}

fn checkpoint(name: &str, actual: &[f64], expected: [f64; 3]) {
    if actual
        .iter()
        .zip(expected)
        .any(|(value, expected)| (value - expected).abs() > 0.0001)
    {
        println!("Error: {name}, ({actual:?}) is not equal to expected ({expected:?})");
        return;
    }
    println!("Check of {name} OK: value = ({actual:?}), expected = ({expected:?})");
}

fn show_vertices(session: &mut ViewerSession, inputs: &geometry::LocatedWireInputs) -> Outcome {
    // These are the construction's named vertices, not identities inferred from coordinates.
    let named = [
        ("w11", &inputs.located_copies[0]),
        ("w11_1", inputs.vertices[0].as_ref()),
        ("w11_2", inputs.vertices[1].as_ref()),
        ("w11_3", inputs.vertices[2].as_ref()),
    ];
    session.display_many(&named, &DisplayRequest::default())?;
    session.fit()?;
    Ok(())
}

const WIRE_BANNER: &[&str] = &[
    "=========",
    "0031131: Mesh - location of Wire affects Poly_Polygon3D",
    "=========",
    "",
];

/// Mesh the three shallow located wire copies together and check the unlocated bounds.
/// Example: `located_wire_polygons()?`.
pub(crate) fn located_wire_polygons() -> Outcome {
    banner(WIRE_BANNER);
    let inputs = geometry::located_inputs()?;
    mesh_shape(&inputs.compound, &MeshRequest::new(1.))?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_auto_triangulation(false);
    show_vertices(run.session(), &inputs)?;
    let bounds = wire_bounds(&inputs.located_copies[0])?;
    checkpoint("Min", &bounds[..3], [-1., -1., -1.]);
    checkpoint("Max", &bounds[3..], [11., 11., 1.]);
    run.dump("bug31131.png")?;
    run.finish()
}
