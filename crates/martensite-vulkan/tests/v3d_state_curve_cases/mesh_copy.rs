//! A geometry-less shape retains its only triangulation when copied with default options.
use super::Outcome;
use crate::primitive_shapes::box_sized;
use crate::session_run::banner;
use crate::shape_mesh::{MeshRequest, check_real, mesh_shape, remove_geometry_only};
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

fn exact_counts(shape: &Shape, name: &str) -> Outcome {
    println!("checktrinfo {name} -tri 12 -nod 24");
    let info = super::mesh_information::information(shape)?;
    check_real(
        "Number of triangles",
        f64::from(info.summary.triangles),
        12.,
        0.,
        0.,
    );
    check_real(
        "Number of nodes",
        f64::from(info.summary.nodes),
        24.,
        0.,
        0.,
    );
    Ok(())
}
/// Full `bug32717` body with the bugs group's completion; `triangulation_only_copy()?`.
pub(crate) fn triangulation_only_copy() -> Outcome {
    banner(&[
        "=================================================",
        "0032717: Modeling Algorithms - BRepBuilderAPI_Copy should not remove triangulation from surface-less faces",
        "=================================================",
        "",
    ]);
    let original = box_sized([1., 2., 3.])?;
    mesh_shape(&original, &MeshRequest::new(1.))?;
    exact_counts(&original, "b")?;
    remove_geometry_only(&original)?;
    super::mesh_information::print_information(&original)?;
    exact_counts(&original, "b")?;
    let mut copy = BRepBuilderAPICopy::with_shape(&original, true, false)?;
    let copied = copy.shape()?;
    super::mesh_information::print_information(copied)?;
    exact_counts(copied, "bb")?;
    println!("TEST COMPLETED");
    Ok(())
}
