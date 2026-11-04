//! Selection-buffer images of shaded boxes: depth, detected object, owner,
//! entity and selection-mode colours.
use super::prelude::*;
use crate::primitive_shapes::box_at;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;

const OCC28205: &str = "0028205: Visualization - add functionality for dumping results of detection algorithms into image";
const RULE: &str = "=================================================================================================";

/// Display the boxes shaded; `show_shaded(&mut run, &boxes)?`.
fn show_shaded(
    run: &mut SessionRun,
    boxes: &[(&str, [f64; 3], [f64; 3])],
) -> Result<(), Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    for &(name, corner, size) in boxes {
        run.session()
            .display(name, &box_at(corner, size)?, &shaded)?;
    }
    Ok(())
}

/// `bug28205_1`: two coloured boxes turned by a mouse rotation, dumped as every depth and
/// colour selection image, then as the view image.
pub(crate) fn rotated_selection_images() -> Result<(), Box<dyn Error>> {
    banner(&[
        RULE,
        OCC28205,
        "Selection image from normalized depth",
        RULE,
    ]);
    let mut run = SessionRun::new()?;
    show_shaded(
        &mut run,
        &[
            ("b0", [0.; 3], [1., 2., 3.]),
            ("b1", [3., 2., 1.], [1., 2., 3.]),
        ],
    )?;
    run.session()
        .set_color("b0", Color::named(NamedColor::GREEN))?;
    run.session()
        .set_color("b1", Color::named(NamedColor::RED))?;
    run.session().fit()?;
    run.session().update_view(|view| {
        view.start_rotation(300, 200, 0.)?;
        view.rotation(100, 200)
    })?;
    let images = [
        ("bug28205_1_depth.png", SelectionImage::NORMALIZED_DEPTH),
        (
            "bug28205_1_depthinv.png",
            SelectionImage::NORMALIZED_DEPTH_INVERTED,
        ),
        (
            "bug28205_1_object.png",
            SelectionImage::COLORED_DETECTED_OBJECT,
        ),
        ("bug28205_1_owner.png", SelectionImage::COLORED_OWNER),
        ("bug28205_1_entity.png", SelectionImage::COLORED_ENTITY),
    ];
    for (name, kind) in images {
        run.selection_dump(name, kind)?;
    }
    run.dump("bug28205_1.png")?;
    run.finish()
}

/// `bug28205_2`: four shaded boxes with different active selection modes, dumped as the
/// selection-mode colour image, then as the view image.
pub(crate) fn selection_mode_image() -> Result<(), Box<dyn Error>> {
    banner(&[
        RULE,
        OCC28205,
        "Selection image from colors of selection modes.",
        RULE,
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let boxes = [0., 2., 4., 6.].map(|c| [c; 3]);
    let named = [
        ("b0", boxes[0]),
        ("b1", boxes[1]),
        ("b2", boxes[2]),
        ("b3", boxes[3]),
    ];
    show_shaded(&mut run, &named.map(|(n, c)| (n, c, [1.; 3])))?;
    run.session().fit()?;
    for (name, mode) in [("b0", 1), ("b0", 2), ("b1", 4), ("b2", 2), ("b3", 3)] {
        run.session().set_object_selection_mode(name, mode, true)?;
    }
    run.selection_dump(
        "bug28205_2_selmode.png",
        SelectionImage::COLORED_SELECTION_MODE,
    )?;
    run.dump("bug28205_2.png")?;
    run.finish()
}
