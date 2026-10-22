//! tests/demo/draw: the TKMesh-owned case.
use crate::support::draw::{Draw, Reply, Tcl};
use crate::support::mesh_export::wavefront;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// The `imagedir` of the case: the harness gives each case a results directory; here a
/// directory below the system temporary directory.
fn imagedir() -> String {
    let directory = std::env::temp_dir().join("martensite-tkmesh-draw-images/demo/draw");
    std::fs::create_dir_all(&directory)
        .unwrap_or_else(|error| panic!("imagedir {}: {error}", directory.display()));
    directory.to_string_lossy().into_owned()
}

/// demo/draw/bug31908: `wavefront` with a file name longer than 100 characters (the full
/// path padded with `1` to 255 characters) must write the file.
pub fn bug31908(draw: &mut Draw) -> Tcl {
    draw.puts("==========");
    draw.puts(
        "0031908: Draw Harness, TKTopTest - wavefront file path length is limited to 100 characters",
    );
    draw.puts("==========");
    draw.puts("");
    // pload MODELING
    draw.command(&["box", "b", "10", "20", "30"], |draw| {
        let shape = BRepPrimAPIMakeBox::from_sizes(10., 20., 30.)?
            .shape()?
            .clone();
        draw.set_shape("b", &shape);
        Ok(Reply::ok(""))
    })?;
    // this test check use wavefront command with long file name
    let mut file_name = format!("{}/bug31908", imagedir());
    // but full path must be less than 255
    while file_name.len() < 255 {
        file_name.push('1');
    }
    wavefront(draw, &["b", &file_name])?;
    draw.puts("TEST COMPLETED");
    Ok(())
}
