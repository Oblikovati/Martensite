//! `generate_images` of the cone/cylinder and sphere groups: surface-normal selection images
//! of shaded quadrics with the whole-object, vertex and face selection modes added in turn.
use super::prelude::*;
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::SessionRun;
use martensite_tkbrep::topods::Shape;
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;

/// Show `shape` shaded as `name`, fit, then dump the surface normals with the default mode,
/// after adding mode 1 and after adding mode 4, as `<prefix>_selmode_<mode>.png`.
fn normal_images(run: &mut SessionRun, name: &str, shape: &Shape, images: [&str; 3]) -> Outcome {
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display(name, shape, &shaded)?;
    run.session().fit()?;
    run.selection_dump(images[0], SelectionImage::SURFACE_NORMAL)?;
    for (mode, image) in [(1, images[1]), (4, images[2])] {
        run.session().set_selection_mode(mode, true)?;
        run.selection_dump(image, SelectionImage::SURFACE_NORMAL)?;
    }
    Ok(())
}

/// `cone_cylinder/generate_images`: a cylinder, a cone and a truncated cone in turn.
pub(crate) fn cone_cylinder_normal_images() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
    ]);
    banner(&[
        "Generating images based on detection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = SessionRun::with_view_size(600, 400)?;
    let shapes = [
        ("cyl", cylinder(10., 20.)?, "cylinder"),
        ("cone", cone(10., 0., 20.)?, "cone"),
        ("tr_cone", cone(10., 5., 10.)?, "truncated_cone"),
    ];
    for (index, (name, shape, label)) in shapes.iter().enumerate() {
        let image = |mode: i32| format!("generate_images_{label}_selmode_{mode}.png");
        let [a, b, c] = [image(0), image(1), image(4)];
        normal_images(&mut run, name, shape, [&a, &b, &c])?;
        if index < 2 {
            run.session().remove(&[name])?;
        }
    }
    run.finish_without_image()
}

/// `sphere/generate_images`: a unit sphere.
pub(crate) fn sphere_normal_images() -> Outcome {
    banner(&[
        "=================================",
        "000032366: Visualization, SelectMgrViewerSelector3d::ToPixMap() - add option dumping surface normals",
    ]);
    banner(&[
        "Generating images based on detection of Select3DSenstiveSphere",
        "=================================",
    ]);
    let mut run = SessionRun::with_view_size(600, 400)?;
    let images = [
        "generate_images_selmode_0.png",
        "generate_images_selmode_1.png",
        "generate_images_selmode_4.png",
    ];
    normal_images(&mut run, "s", &sphere(1.)?, images)?;
    run.finish_without_image()
}
