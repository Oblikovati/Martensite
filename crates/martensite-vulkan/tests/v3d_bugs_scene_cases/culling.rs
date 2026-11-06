//! Drawn-sphere scenes: frustum culling of objects inside and outside a fixed 512x512 view
//! (the reference dumps with and without culling are the same image), a transparent sphere, and the
//! structures culled among 2000 nested boxes.
//!
//! Counter-bearing images and both measured FPS returns are unconditional source operations;
//! they stay separate from the tolerance-bearing comparison of the reference images.
use super::culling_phase::observe_culling_phase;
use super::prelude::*;
use crate::case_view::viewer_session::ViewParameters;
use crate::dump_diff::{DiffOptions, diff_dumps};
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::ais::{
    colored_sphere as vdraw_sphere_object, print_colored_sphere_info as print_vdraw_sphere_info,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// The 512x512 window of both cases.
const WINDOW: i32 = 512;
/// Ten objects, 30 percent of them inside the view.
const OBJECTS: i32 = 10;
const INNER: i32 = OBJECTS * 30 / 100;

/// The fixed view of both cases, culling off: no automatic depth fit, scale
/// 1.953125, eye (0.577, -0.577, 0.577), depth range 1 to 512, no objects left.
fn culling_view() -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::with_view_size(WINDOW, WINDOW)?;
    let session = run.session();
    session.set_window_origin(32, 32);
    session.set_perf_update_interval(0.)?;
    session.set_frustum_culling(false)?;
    session.set_auto_z_fit(false)?;
    session.set_view_parameters(&ViewParameters {
        scale: Some(1.953125),
        eye: Some([
            0.577_350_269_189_625_7,
            -0.577_350_269_189_625_7,
            0.577_350_269_189_625_7,
        ]),
        ..ViewParameters::default()
    })?;
    session.set_z_range(1., 512.)?;
    session.remove(&[])?;
    Ok(run)
}

/// Print the visible-object count line framed by empty lines, as both cases do.
fn print_visible_count() {
    println!();
    println!("Total number of visible objects: {INNER}");
    println!();
}

/// The x (and y) of object `i`: inner objects on the diagonal of the window at `inner_step`,
/// outer objects beyond the window's corner at `outer_step` (integer arithmetic).
fn object_offset(i: i32, inner_origin: i32, inner_step: i32, outer_step: i32) -> f64 {
    let offset = if i < INNER {
        inner_origin + (i + 1) * inner_step
    } else {
        -WINDOW - 100 * 3 + (i - INNER + 1) * outer_step
    };
    f64::from(offset)
}

/// Compare the two dumps with tolerance 0.1, printing the case's error on a difference.
fn compare_culling(on: &str, off: &str, diff: &'static str) -> Result<(), Box<dyn Error>> {
    let options = DiffOptions {
        tolerance: 0.1,
        black_white: false,
        border_filter: false,
        diff_image: Some(diff),
    };
    if diff_dumps(on, off, &options)? != 0 {
        println!(
            "ERROR : Test failed: there is a difference between images rendered with and without clipping"
        );
    }
    Ok(())
}

/// A fineness-10 sphere of radius 100, quiet.
fn draw_sphere(run: &mut SessionRun, name: &str, at: f64) -> Result<(), Box<dyn Error>> {
    let sphere = vdraw_sphere_object([at, at, 0.], 10, 100., false)?;
    run.session().display_object(name, sphere.cast())?;
    Ok(())
}

/// The sphere names of `bug24307_1`: `inner_sph<i>` then `outer_sph<i>`.
fn sphere_names() -> Vec<String> {
    let prefix = |i| if i < INNER { "inner_sph" } else { "outer_sph" };
    (0..OBJECTS).map(|i| format!("{}{i}", prefix(i))).collect()
}

/// Draw a temporary sphere and remove it: the statistics of a
/// default sphere, printed and discarded.
fn print_default_sphere(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    println!("Compute Triangulation...");
    let sphere = vdraw_sphere_object([0.; 3], 10, 100., false)?;
    print_vdraw_sphere_info(&sphere)?;
    run.session().display_object("tmp_sph", sphere.cast())?;
    println!();
    run.session().remove(&["tmp_sph"])?;
    Ok(())
}

const SPHERE_COMPARISON: [&str; 3] = [
    "bug24307_1_cull_on_ref.png",
    "bug24307_1_cull_off_ref.png",
    "bug24307_1_diff.png",
];
const BOX_COMPARISON: [&str; 3] = [
    "bug24307_2_cull_on_ref.png",
    "bug24307_2_cull_off_ref.png",
    "bug24307_2_diff.png",
];

fn prepare_spheres(run: &mut SessionRun) -> Result<Vec<String>, Box<dyn Error>> {
    print_default_sphere(run)?;
    print_visible_count();
    println!("Start displaying spheres without clipping...");
    let names = sphere_names();
    let (inner_step, outer_step) = (WINDOW / (INNER + 1), 100 * 3 / (OBJECTS - INNER + 1));
    for (i, name) in (0..OBJECTS).zip(&names) {
        let at = object_offset(i, -WINDOW / 2, inner_step, outer_step);
        draw_sphere(run, name, at)?;
    }
    Ok(names)
}

fn spheres_with_culling(run: &mut SessionRun, names: &[String]) -> Result<(), Box<dyn Error>> {
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    run.session().erase(&names)?;
    println!("Start displaying spheres with clipping...");
    run.session().set_frustum_culling(true)?;
    run.session().show_erased(&names)?;
    observe_culling_phase(
        run,
        SPHERE_COMPARISON[0],
        "bug24307_1_cull_on.png",
        "spheres",
    )?;
    println!();
    Ok(())
}

/// `bug24307_1`: ten actual spheres, both measured phases and all four source dumps.
pub(crate) fn culled_spheres() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC24307 Objects clipping algorithm using BVH performance test: Solid spheres test",
        "========",
    ]);
    let mut run = culling_view()?;
    let names = prepare_spheres(&mut run)?;
    observe_culling_phase(
        &mut run,
        SPHERE_COMPARISON[1],
        "bug24307_1_cull_off.png",
        "spheres",
    )?;
    println!();
    spheres_with_culling(&mut run, &names)?;
    finish_culling(run, SPHERE_COMPARISON)
}

fn finish_culling(run: SessionRun, images: [&'static str; 3]) -> Result<(), Box<dyn Error>> {
    let [on, off, diff] = images;
    compare_culling(on, off, diff)?;
    run.finish()
}

/// The boxes of `bug24307_2`: `inner_box<i>` and `outer_box<i>`, 100 wide, at their offsets.
fn culling_boxes() -> Result<Vec<(String, Shape)>, Box<dyn Error>> {
    let inner_step = WINDOW / (2 * (INNER + 1));
    let outer_step = 100 * 3 / (OBJECTS - INNER + 1);
    let named = (0..OBJECTS).map(|i| {
        let at = object_offset(i, -WINDOW / 4, inner_step, outer_step);
        let prefix = if i < INNER { "inner_box" } else { "outer_box" };
        Ok((format!("{prefix}{i}"), box_at([at, at, 0.], [100.; 3])?))
    });
    named.collect()
}

/// Display every box with the context's default mode.
fn show_boxes(run: &mut SessionRun, boxes: &[(String, Shape)]) -> Result<(), Box<dyn Error>> {
    run.session()
        .without_updates(|session| -> Result<(), Failure> {
            for (name, shape) in boxes {
                session.display(name, shape, &DisplayRequest::default())?;
            }
            Ok(())
        })?;
    Ok(())
}

fn prepare_boxes(run: &mut SessionRun) -> Result<Vec<(String, Shape)>, Box<dyn Error>> {
    print_visible_count();
    println!("Start boxes generation...");
    let boxes = culling_boxes()?;
    println!("{OBJECTS} boxes generated.");
    println!();
    println!("Start displaying boxes without clipping...");
    show_boxes(run, &boxes)?;
    Ok(boxes)
}

fn boxes_with_culling(
    run: &mut SessionRun,
    boxes: &[(String, Shape)],
) -> Result<(), Box<dyn Error>> {
    let names: Vec<&str> = boxes.iter().map(|(name, _)| name.as_str()).collect();
    run.session().erase(&names)?;
    run.session().set_frustum_culling(true)?;
    println!("Start displaying boxes with clipping...");
    // Reuse each erased object with the source -noupdate command boundary.
    run.session()
        .without_updates(|session| -> Result<(), Failure> {
            for name in &names {
                session.show_erased(&[*name])?;
            }
            Ok(())
        })?;
    observe_culling_phase(run, BOX_COMPARISON[0], "bug24307_2_cull_on.png", "boxes")
}

/// `bug24307_2`: ten actual boxes, both measured phases and all four source dumps.
pub(crate) fn culled_boxes() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC24307 Objects clipping algorithm using BVH performance test: Simple boxes test",
        "========",
    ]);
    let mut run = culling_view()?;
    let boxes = prepare_boxes(&mut run)?;
    observe_culling_phase(
        &mut run,
        BOX_COMPARISON[1],
        "bug24307_2_cull_off.png",
        "boxes",
    )?;
    println!();
    boxes_with_culling(&mut run, &boxes)?;
    finish_culling(run, BOX_COMPARISON)
}

/// `bug27655`: a drawn sphere's triangulation made 70 percent transparent stays drawn over a
/// shaded box.
pub(crate) fn transparent_triangulation() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "CR27655: Visualization - AisTriangulation don't disappears after setting non-zero transparency",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let request = DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    };
    run.session()
        .display("b", &box_sized([10., 10., 10.])?, &request)?;
    println!("Compute Triangulation...");
    let sphere = vdraw_sphere_object([0.; 3], 100, 100., false)?;
    print_vdraw_sphere_info(&sphere)?;
    run.session().display_object("s", sphere.cast())?;
    run.session().set_transparency("s", 0.7)?;
    run.session().fit()?;
    run.dump_and_finish("bug27655_0.png", false)
}

/// The rendered structure count must print `expected`, else the case's error line.
fn expect_rendered_structures(
    run: &mut SessionRun,
    expected: &str,
    error: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().rendered_structures()? != expected {
        println!("{error}");
    }
    Ok(())
}

/// `bug31341`: 2000 nested wireframe boxes seen from the top; zoomed in 5 times, frustum
/// culling skips the 797 boxes outside the view, and zoomed out to 0.9 it skips none.
pub(crate) fn culled_by_presentation_range() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0031341: Visualization - Graphic3d_Layer::UpdateCulling() ignores Presentation range",
        "=============",
    ]);
    let boxes = (0..2000)
        .map(|i| {
            let size = f64::from(2000 - i);
            Ok((format!("b{i}"), box_sized([size; 3])?))
        })
        .collect::<Result<Vec<(String, Shape)>, Failure>>()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().set_auto_triangulation(false);
    let named: Vec<(&str, &Shape)> = boxes.iter().map(|(n, s)| (n.as_str(), s)).collect();
    let wireframe = DisplayRequest::default().with_display_mode(0);
    run.session().display_many(&named, &wireframe)?;
    run.session().fit()?;
    run.session().zoom(5.)?;
    expect_rendered_structures(
        &mut run,
        "1203",
        "Error: unexpected number of culled structures",
    )?;
    run.dump("bug31341_zoom.png")?;
    run.session().fit()?;
    run.session().zoom(0.9)?;
    expect_rendered_structures(&mut run, "2000", "Error: no culling is expected")?;
    run.dump_and_finish("bug31341.png", false)
}
