//! Shapes textured from image files: an SGI texture beside a trihedron, and a GIF texture
//! repeated on one face of a box.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::{TextureImage, ViewParameters};
use crate::dataset_shape::locate_dataset_file;
use martensite_tkg3d::top_abs::ShapeEnum;

/// `bug5988`: a box textured from an SGI image is not black when a trihedron is displayed
/// with it.
pub(crate) fn textured_box_beside_trihedron() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC5988: A textured shape (AisTextureShape) becomes black if we display an AisPlaneTrihedron with it",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    show(&mut run, "box", &box_sized([100., 100., 100.])?)?;
    run.session().fit()?;
    println!("display box");
    run.session().check_color(200, 200, [0., 0., 0.])?;
    println!("display trihedron");
    run.session().display_trihedron("vtr")?;
    println!("display textured box");
    let floor = locate_dataset_file("OCC5988_2d_floor.rgb")?;
    run.session()
        .set_texture("box", TextureImage::File(&floor))?;
    if run.session().pixel_color_name(200, 200)? != "BLACK" {
        println!("box is not black - OK");
        println!("OK OCC5988");
    } else {
        println!("box is black - Faulty");
        println!("Faulty OCC5988");
    }
    run.dump("bug5988.png")?;
    run.finish()
}

/// The camera of `bug22196`.
#[allow(clippy::excessive_precision)] // The camera values are the original's, digit for digit.
fn bug22196_camera() -> ViewParameters {
    ViewParameters {
        scale: Some(236.95268872530238),
        eye: Some([3.7940978443168705, 0.31258558886503313, 0.36550370223815498]),
        at: Some([0.58906967866866666, 0.98487348454501, 1.4950272490152485]),
        proj: Some([
            0.92521193709866123,
            -0.1940727987718808,
            -0.32606536189388785,
        ]),
        up: Some([
            0.2658610208502189,
            -0.28157324388568683,
            0.92197311561681172,
        ]),
    }
}

/// `bug22196`: the second face of a box, textured from a GIF file repeated 3x2, in a fixed
/// camera.
pub(crate) fn repeated_face_texture() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC22196", "================", ""]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "b", &b)?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    let faces = all_distinct(&b, ShapeEnum::Face);
    let face = faces
        .get(1)
        .ok_or("explode b f: expected a second face b_2")?;
    show(&mut run, "b_2", face)?;
    let tiles = locate_dataset_file("OCC23385-carrelage1.gif")?;
    run.session()
        .set_texture("b_2", TextureImage::File(&tiles))?;
    run.session().set_texture_repeat("b_2", [3., 2.])?;
    run.session().set_view_parameters(&bug22196_camera())?;
    run.dump("bug22196.png")?;
    run.finish()
}
