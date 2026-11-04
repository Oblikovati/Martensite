//! Shapes with mirrored transformations: solids look inside
//! out, shells keep showing the faces they face the camera with, and reversed shapes flip back.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::SurfaceAspects;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;

/// The banner line shared by the three cases.
const OCC30630: &str =
    "0030630: Visualization - wrong visualization of shape with mirrored transformation";
const RULE: &str =
    "==================================================================================";

/// The mirror kept as the shape's location (forced, so the
/// negative determinant is accepted); `forced_mirror(&b, [0.; 3], [1., 0., 0.])?`.
fn forced_mirror(
    shape: &Shape,
    [x, y, z]: [f64; 3],
    [nx, ny, nz]: [f64; 3],
) -> Result<Shape, Failure> {
    let mut mirror = Transform3::default();
    mirror.set_mirror_plane(Axis2::new(
        Point3::new(x, y, z),
        Direction3::new(nx, ny, nz)?,
    )?);
    shape.moved(&Location::new(mirror), false)
}

/// Axonometric, rotated 3 about X, perspective, on a cleared view.
fn mirror_view() -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().rotate(3., 0., 0.)?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    Ok(run)
}

/// Display `name` shaded with red back faces.
fn show_red_backs(run: &mut SessionRun, name: &str, shape: &Shape) -> Result<(), Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display(name, shape, &shaded)?;
    let red_backs = SurfaceAspects {
        back_face_color: Some(Color::named(NamedColor::RED)),
        ..SurfaceAspects::default()
    };
    run.session().set_surface_aspects(name, &red_backs)?;
    Ok(())
}

/// The small blue box at `corner` that shows through an inside-out box.
fn show_blue_marker(
    run: &mut SessionRun,
    name: &str,
    corner: [f64; 3],
) -> Result<(), Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session()
        .display(name, &box_at(corner, [0.1; 3])?, &shaded)?;
    run.session()
        .set_color(name, Color::named(NamedColor::BLUE))?;
    Ok(())
}

/// Print `message` when pixel (x, y) is (`expected`) or is not (`!expected`) named `name`.
fn expect_named(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    (name, expected): (&str, bool),
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if (run.session().pixel_color_name(x, y)? == name) != expected {
        println!("{message}");
    }
    Ok(())
}

/// A unit box, optionally mirrored in the YZ plane and reversed, as its first shell
/// or whole.
fn unit_box(mirrored: bool, reversed: bool, shell: bool) -> Result<Shape, Box<dyn Error>> {
    let mut shape = box_sized([1.; 3])?;
    if mirrored {
        shape = forced_mirror(&shape, [0.; 3], [1., 0., 0.])?;
    }
    if reversed {
        shape = shape.oriented(Orientation::Reversed);
    }
    if shell {
        shape = all_distinct(&shape, ShapeEnum::Shell)
            .first()
            .cloned()
            .ok_or("explode s sh: expected a shell")?;
    }
    Ok(shape)
}

/// The three passes of `bug30630_1` and `bug30630_3`: a plain, a mirrored and a reversed
/// mirrored unit box (as a shell when `shell`), with the blue marker at `marker`.
fn mirror_passes(
    case: &'static [&'static str; 3],
    shell: bool,
    check: [(i32, i32); 1],
    marker: [f64; 3],
) -> Result<(), Box<dyn Error>> {
    let mut run = mirror_view()?;
    show_red_backs(&mut run, "b", &unit_box(false, false, shell)?)?;
    run.session().fit()?;
    run.dump(case[0])?;
    run.session().erase(&["b"])?;
    show_red_backs(&mut run, "a", &unit_box(true, false, shell)?)?;
    run.session().fit()?;
    show_blue_marker(&mut run, "aa", marker)?;
    run.dump(case[1])?;
    let (name, inverse) = if shell {
        (
            "RED2",
            "Error: shell of mirrored box should show back faces",
        )
    } else {
        ("BLUE2", "Error: mirrored box should look inside out")
    };
    expect_named(&mut run, check[0], (name, true), inverse)?;
    run.session().erase(&["a"])?;
    show_red_backs(&mut run, "c", &unit_box(true, true, shell)?)?;
    run.session().fit()?;
    run.dump(case[2])?;
    let flipped = if shell {
        "Error: shell of reversed mirrored box should show front faces"
    } else {
        "Error: reversed mirrored box should NOT look inside out"
    };
    expect_named(&mut run, check[0], (name, false), flipped)?;
    run.finish()
}

/// `bug30630_1`: shells of a box, of its mirror and of its reversed mirror show front, back and
/// front faces again.
pub(crate) fn mirrored_shells() -> Result<(), Box<dyn Error>> {
    banner(&[RULE, OCC30630]);
    banner(&[
        "Check that displaying of non-solid shapes (shells) is not affected by mirror transformation",
        RULE,
    ]);
    const DUMPS: [&str; 3] = [
        "bug30630_1_simple.png",
        "bug30630_1_mirror.png",
        "bug30630_1_mirrev.png",
    ];
    mirror_passes(&DUMPS, true, [(300, 200)], [-0.5, 0.5, 0.5])
}

/// `bug30630_3`: a mirrored solid looks inside out, and its reversed copy does not.
pub(crate) fn mirrored_solids() -> Result<(), Box<dyn Error>> {
    banner(&[RULE, OCC30630]);
    banner(&[
        "Check that solid with mirrored transformation is displayed as turned out",
        RULE,
    ]);
    const DUMPS: [&str; 3] = [
        "bug30630_3_simple.png",
        "bug30630_3_mirror.png",
        "bug30630_3_mirrev.png",
    ];
    mirror_passes(&DUMPS, false, [(195, 220)], [-0.5, 0.5, 0.5])
}

/// One `bug30630_2` pass: fit, dump `image`, then expect the blue marker at `pixel`.
fn dump_inside_out(
    run: &mut SessionRun,
    image: &'static str,
    pixel: (i32, i32),
    message: &str,
) -> Result<(), Box<dyn Error>> {
    run.session().fit()?;
    run.dump(image)?;
    expect_named(run, pixel, ("BLUE2", true), message)
}

/// `bug30630_2`: a mirrored box stays inside out when its location is rotated and when it is
/// replaced by a location mirror; the marker box follows the same locations.
pub(crate) fn mirrored_locations() -> Result<(), Box<dyn Error>> {
    banner(&[RULE, OCC30630]);
    banner(&[
        "Check that local transformation of visualizated object",
        "affects only position, not material place",
        RULE,
    ]);
    let mut run = mirror_view()?;
    let b = forced_mirror(&box_sized([1.; 3])?, [0.; 3], [0., 0., 1.])?;
    show_red_backs(&mut run, "b", &b)?;
    show_blue_marker(&mut run, "bb", [0.5, 0.5, -0.5])?;
    dump_inside_out(
        &mut run,
        "bug30630_2_simple.png",
        (195, 220),
        "Error: mirrored box should look inside out",
    )?;
    for name in ["b", "bb"] {
        run.session()
            .rotate_location(name, [0.; 3], [1., 0., 0.], 90.)?;
    }
    dump_inside_out(
        &mut run,
        "bug30630_2_rotate.png",
        (215, 230),
        "Error: rotated mirrored box should look inside out",
    )?;
    for name in ["b", "bb"] {
        run.session().reset_location(name)?;
        run.session()
            .mirror_location(name, [0., -0.5, 0.], [0., 1., 0.])?;
    }
    dump_inside_out(
        &mut run,
        "bug30630_2_mirror.png",
        (215, 230),
        "Error: mirrored mirrored box should look inside out",
    )?;
    run.finish()
}
