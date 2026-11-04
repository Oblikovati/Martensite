//! Detection of planes built from three points, as the plane command builds them, by interior
//! and boundary sensitivity, under a box, and after the plane is moved and resized.
use super::prelude::*;
use crate::viewer_objects::point_object;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkgeombase::gc::MakePlane;
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::ais::{AisInteractiveObject, AisPlane};
use martensite_tkv3d::select3d::Select3DTypeOfSensitivity;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// The three corner points of every plane regression, named p1, p2 and p3.
const CORNERS: [(&str, [f64; 3]); 3] = [
    ("p1", [0., 0., 0.]),
    ("p2", [1., 0., 0.]),
    ("p3", [0., 1., 0.]),
];

/// A cleared 400x400 view showing the points p1, p2 and p3;
/// `corner_points()?`.
fn corner_points() -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::with_view_size(400, 400)?;
    run.session().clear()?;
    for (name, point) in CORNERS {
        run.session().display_object(name, point_object(point)?)?;
    }
    Ok(run)
}

/// The plane through the three points with the given
/// type of sensitivity, displayed; `show_plane(&mut run, "pl1", Interior)?`.
fn show_plane(
    run: &mut SessionRun,
    name: &str,
    sensitivity: Select3DTypeOfSensitivity,
) -> Result<Handle<AisPlane>, Box<dyn Error>> {
    let [a, b, c] = CORNERS.map(|(_, [x, y, z])| Point3::new(x, y, z));
    let geometry = MakePlane::through_points(a, b, c)?.value()?;
    let plane = AisPlane::new(&geometry, false)?;
    if let Some(object) = plane.get() {
        object.set_type_of_sensitivity(sensitivity);
    }
    let object: Handle<dyn AisInteractiveObject> = plane.clone().cast();
    run.session().display_object(name, object)?;
    Ok(plane)
}

/// Print `message` unless pixel (x, y) is near the linear colour `expected`.
fn expect_color(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    expected: [f64; 3],
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if !run.session().check_color(x, y, expected)? {
        println!("{message}");
    }
    Ok(())
}

/// Print `message` unless pixel (x, y) is named CYAN.
fn expect_cyan(run: &mut SessionRun, (x, y): (i32, i32), message: &str) -> Result<(), Failure> {
    if run.session().pixel_color_name(x, y)? != "CYAN" {
        println!("{message}");
    }
    Ok(())
}

/// `bug24420`: an interior-sensitive plane is detected inside, a boundary-sensitive one only
/// on its boundary.
pub(crate) fn plane_sensitivity_types() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR24420: Test for type of sensitivity of AisPlane",
    ]);
    banner(&["============", ""]);
    let mut run = corner_points()?;
    println!("Testing Select3DTOS_INTERIOR type of sensitivity:");
    show_plane(&mut run, "pl1", Select3DTypeOfSensitivity::Interior)?;
    run.session().fit()?;
    run.session().move_cursor(210, 210)?;
    let broken = "Error : Select3DSensitiveFace does not work properly with type of sensitivity";
    let interior = format!("{broken} Select3DTOS_INTERIOR!");
    expect_color(&mut run, (395, 200), [0., 1., 1.], &interior)?;
    run.session().erase(&["pl1"])?;
    println!("Testing Select3DTOS_BOUNDARY type of sensitivity:");
    show_plane(&mut run, "pl2", Select3DTypeOfSensitivity::Boundary)?;
    run.session().fit()?;
    run.session().move_cursor(210, 210)?;
    let boundary = format!("{broken} Select3DTOS_BOUNDARY!");
    expect_color(&mut run, (395, 200), [0.5, 0.8, 0.9], &boundary)?;
    run.session().move_cursor(395, 200)?;
    expect_color(&mut run, (395, 200), [0., 1., 1.], &boundary)?;
    run.dump("bug24420.png")?;
    run.finish()
}

/// `bug24564`: a box with wide edges over a plane stays detectable from the top and in
/// axonometry.
pub(crate) fn box_over_plane() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR24564: Test for sensitivity of box located over AisPlane",
    ]);
    banner(&["============", ""]);
    let mut run = corner_points()?;
    show_plane(&mut run, "pl1", Select3DTypeOfSensitivity::Interior)?;
    let b = box_sized([50., 50., 50.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    println!("Testing box selection over the plane in orthogonal projection:");
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    for name in ["p1", "p2", "p3", "pl1", "b"] {
        run.session().set_line_width(name, 5.)?;
    }
    run.session().move_cursor(200, 200)?;
    expect_cyan(&mut run, (200, 200), "Error : The box is not selectable!")?;
    println!("Testing box selection over the plane in axonometry projection:");
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session().move_cursor(200, 200)?;
    expect_cyan(&mut run, (199, 200), "Error : The box is not selectable!")?;
    run.dump("bug24564.png")?;
    run.finish()
}

/// Centre the plane at `center` with the same normal and
/// the given size, drop a minimum size, then update its presentation and selection with a
/// redraw; `change_plane(&run, &plane, [-150., 150., 0.], [10., 10.])?`.
fn change_plane(
    run: &mut SessionRun,
    plane: &Handle<AisPlane>,
    [x, y, z]: [f64; 3],
    [sx, sy]: [f64; 2],
) -> Result<(), Box<dyn Error>> {
    let value = plane.get().ok_or("a displayed plane")?;
    let normal = value.component().get().ok_or("a plane component")?.pln();
    let center = Point3::new(x, y, z);
    value.set_center(center);
    let component = GeomPlane::from_point(center, normal.axis().direction())?;
    value.set_component(&Handle::new(component));
    value.set_size_xy(sx, sy)?;
    if value.has_minimum_size() {
        value.unset_minimum_size();
    }
    let object: Handle<dyn AisInteractiveObject> = plane.clone().cast();
    run.session().context().update(&object, true)?;
    Ok(())
}

/// Update the object without a redraw, update the
/// viewer, then set the camera depth range and redraw.
fn update_then_z_range(
    run: &mut SessionRun,
    plane: &Handle<AisPlane>,
    [near, far]: [f64; 2],
) -> Result<(), Box<dyn Error>> {
    let object: Handle<dyn AisInteractiveObject> = plane.clone().cast();
    run.session()
        .update_context(|context| context.update(&object, false))?;
    run.session().update_view(|view| {
        let camera = view.camera();
        camera.get().map_or(Ok(()), |c| c.set_z_range(near, far))
    })?;
    Ok(())
}

/// `bug24569`: the plane's sensitive entity follows the plane after it is moved and resized.
pub(crate) fn changed_plane_sensitivity() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR24569: Test for conversion of sensitives by AisInteractiveContext::Update",
    ]);
    banner(&["============", ""]);
    let mut run = corner_points()?;
    let plane = show_plane(&mut run, "pl1", Select3DTypeOfSensitivity::Interior)?;
    run.session().fit()?;
    run.session().move_cursor(200, 256)?;
    let initial = "Error : Selection of AisPlane does not work initially!";
    expect_color(&mut run, (395, 200), [0., 1., 1.], initial)?;
    println!("Testing of plane sensitivity update:");
    change_plane(&mut run, &plane, [-150., 150., 0.], [10., 10.])?;
    update_then_z_range(&mut run, &plane, [230.3627074066607, 409.91874112463427])?;
    run.session().move_cursor(200, 240)?;
    let stale = "Error : Sensitive entity of AisPlane is not updated!";
    expect_color(&mut run, (199, 23), [0., 1., 0.], stale)?;
    run.session().move_cursor(200, 28)?;
    let wrong = "Error : Sensitive entity of AisPlane is incorrect after update!";
    expect_color(&mut run, (199, 23), [0., 1., 1.], wrong)?;
    run.dump("bug24569.png")?;
    run.finish()
}
