//! Original regression asserting that the actual shape dump names the locked flag.
#[path = "common/geometry_io.rs"]
mod geometry_io;
use geometry_io::stream_bytes;
use martensite_tkbrep::brep_tools;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::IoStream;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

#[test]
fn bug28204_original_locked_vertex_dump_names_locked_flag() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "locked_shape_dump::bug28204_original_locked_vertex_dump_names_locked_flag",
        || Ok(bug28204_original_locked_vertex_dump_names_locked_flag_body()?),
    );
    Ok(())
}

fn bug28204_original_locked_vertex_dump_names_locked_flag_body() -> Result<(), Failure> {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 0.))?.vertex()?;
    vertex.set_locked(true)?;
    let mut stream = IoStream::from_bytes(Vec::new());
    brep_tools::dump(&vertex, &mut stream)?;
    let output = String::from_utf8(stream_bytes(stream)).unwrap();
    assert!(
        output.contains("Locked"),
        "Dump does not show flag Locked: {output}"
    );
    Ok(())
}
