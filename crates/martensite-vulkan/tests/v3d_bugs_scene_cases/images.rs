//! Image comparisons of dumps and image files: colour tolerance on two flat
//! backgrounds, border filtering of two GIF files, and a sub-shape meshed alone or with its
//! shape.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::dataset_shape::locate_dataset_file;
use crate::dump_diff::{DiffOptions, diff_dumps, diff_files};
use crate::shape_mesh::remove_triangulation;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Comparison options with colour tolerance `tolerance` and diff image `diff_image`.
fn tolerant(tolerance: f64, diff_image: &'static str) -> DiffOptions {
    DiffOptions {
        tolerance,
        black_white: false,
        border_filter: false,
        diff_image: Some(diff_image),
    }
}

/// `bug29847`: GRAY50 and GRAY51 backgrounds differ at every pixel with tolerance 0 and 0.01,
/// and nowhere with tolerance 0.1.
pub(crate) fn background_tolerance() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0029847: Visualization, Image_Diff - Tolerance is not effective for 24/32bit image formats",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_background_color(Color::named(NamedColor::GRAY50))?;
    run.dump("bug29847_127.png")?;
    run.session()
        .set_background_color(Color::named(NamedColor::GRAY51))?;
    run.dump("bug29847_130.png")?;
    let (first, second) = ("bug29847_127.png", "bug29847_130.png");
    let checks = [
        (0., "bug29847_0.png", 167281, "0.0 "),
        (0.1, "bug29847_1.png", 0, "0.1 "),
        (0.01, "bug29847_01.png", 167281, "0.01"),
    ];
    for (tolerance, diff_image, expected, label) in checks {
        let count = diff_dumps(first, second, &tolerant(tolerance, diff_image))?;
        println!("{count}");
        if count != expected {
            println!("Error: difference with tolerance {label} is incorrect");
        }
    }
    run.finish()
}

/// `bug23425`: the border filter compares two GIF files in both orders
/// without an exception. The counts are the recorded ones for these files.
pub(crate) fn gif_border_filter() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR23425: diffimage generate O S D _ E x c e p t i o n during execution",
        "============",
        "",
    ]);
    let linux = locate_dataset_file("bug23425_B8_Linux.gif")?;
    let windows = locate_dataset_file("bug23425_B8_Win.gif")?;
    let options = DiffOptions {
        tolerance: 0.01,
        black_white: false,
        border_filter: true,
        diff_image: None,
    };
    for (reference, new, native) in [(&linux, &windows, 519), (&windows, &linux, 520)] {
        let count = diff_files(reference, new, &options)?;
        println!("{count}");
        if count != native {
            println!("Error: {count} differing groups; native diffimage counts {native}");
        }
    }
    // The case opens no viewer; the completion line ends the test.
    println!("TEST COMPLETED");
    Ok(())
}

/// The third face of `c` alone, zoomed, translated and hovered, then dumped as `image`.
fn dump_hovered_face(
    run: &mut SessionRun,
    face: &Shape,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    show(run, "c_3", face)?;
    run.session().fit()?;
    run.session().zoom(4.)?;
    run.session()
        .update_view(|view| view.translate(70., 20., 0., true))?;
    run.session().move_cursor(200, 200)?;
    run.dump(image)
}

/// `bug25814`: a cylinder face shows the same wireframe whether meshed alone or as part of
/// its cylinder.
pub(crate) fn sub_shape_triangulation() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25814",
        "Check that the discretization of the sub-shape is identical in shaded and wireframe modes",
        "if the whole shape was already displayed",
        "============",
        "",
    ]);
    let c = cylinder(100., 350.)?;
    let faces = all_distinct(&c, ShapeEnum::Face);
    let face = faces
        .get(2)
        .ok_or("explode c f: expected a third face c_3")?;
    let mut run = SessionRun::with_view_size(900, 900)?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    dump_hovered_face(&mut run, face, "bug25814_subshape_triangulation.png")?;
    run.session().clear()?;
    remove_triangulation(face)?;
    show(&mut run, "c", &c)?;
    run.session().fit()?;
    run.session().clear()?;
    dump_hovered_face(&mut run, face, "bug25814_shape_triangulation.png")?;
    let options = DiffOptions {
        tolerance: 0.1,
        black_white: false,
        border_filter: false,
        diff_image: Some("bug25814_diff.png"),
    };
    let difference = diff_dumps(
        "bug25814_subshape_triangulation.png",
        "bug25814_shape_triangulation.png",
        &options,
    )?;
    if difference != 0 {
        println!(
            "ERROR : Test failed: there is a difference between images. Discretization of the sub-shape in wireframe mode is incorrect"
        );
    }
    run.finish()
}
