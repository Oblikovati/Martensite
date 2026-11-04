//! A plane datum given a minimum on-screen size next to a large box.
use super::prelude::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomElementarySurface as _, GeomPlane};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tkv3d::ais::{AisInteractiveObject, AisPlane, AisPoint};
use martensite_tkv3d::select3d::Select3DTypeOfSensitivity;

/// `bug32091`: a 100 x 100 plane of minimum size 100 stays visible zoomed out and panned.
pub(crate) fn plane_minimum_size() -> Outcome {
    banner(&[
        "============",
        "CR32091: Visualization - possibility to redefine Graphic3d_TransformPers",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display_trihedron("t1")?;
    let b1 = box_sized([500., 500., 500.])?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display("b1", &b1, &request)?;
    let points = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    for (name, point) in ["p1", "p2", "p3"].into_iter().zip(points) {
        display_point(&mut run, name, point)?;
    }
    let plane = three_point_plane(points)?;
    plane.set_type_of_sensitivity(Select3DTypeOfSensitivity::Interior);
    run.session().display_object("pl1", plane.clone().cast())?;
    run.session().fit()?;
    resize_plane(&mut run, &plane)?;
    run.session().fit()?;
    run.dump("bug32091_0.png")?;
    run.session().zoom(8.)?;
    run.session().pan(1100, 600)?;
    run.dump("bug32091_1.png")?;
    run.session().fit()?;
    run.session().zoom(0.2)?;
    run.dump("bug32091_2.png")?;
    run.finish()
}

/// Display a point named `name`.
fn display_point(run: &mut SessionRun, name: &str, [x, y, z]: [f64; 3]) -> Outcome {
    let point = Handle::new(GeomCartesianPoint::new(x, y, z));
    let object: Handle<dyn AisInteractiveObject> = AisPoint::new(&point.cast())?.cast();
    run.session().display_object(name, object)?;
    Ok(())
}

/// The plane GC_MakePlane builds through three points, located at
/// the first, with normal (p2 - p1) x (p3 - p1) and X direction p2 - p1.
fn three_point_plane([a, b, c]: [[f64; 3]; 3]) -> Result<Handle<AisPlane>, Box<dyn Error>> {
    let (v1, v2) = (
        [b[0] - a[0], b[1] - a[1], b[2] - a[2]],
        [c[0] - a[0], c[1] - a[1], c[2] - a[2]],
    );
    let normal = [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];
    let frame = Axis3::new(
        Point3::new(a[0], a[1], a[2]),
        Direction3::new(normal[0], normal[1], normal[2])?,
        Direction3::new(v1[0], v1[1], v1[2])?,
    )?;
    Ok(AisPlane::new(&Handle::new(GeomPlane::new(frame)), false)?)
}

/// Set size 100x100 and minimum size 100, then update: the plane keeps its
/// centre and normal, takes the size and minimum size, and is updated twice.
fn resize_plane(run: &mut SessionRun, plane: &Handle<AisPlane>) -> Outcome {
    let value = plane.get().expect("a new plane is never null");
    let center = value.center();
    let normal = value
        .component()
        .get()
        .expect("a plane has a component")
        .axis()
        .direction();
    value.set_center(center);
    value.set_component(&Handle::new(GeomPlane::from_point(center, normal)?));
    value.set_size_xy(100., 100.)?;
    value.set_minimum_size(100.)?;
    let object: Handle<dyn AisInteractiveObject> = plane.clone().cast();
    run.session()
        .update_context(|context| context.update(&object, true))?;
    run.session()
        .update_context(|context| context.update(&object, false))?;
    Ok(())
}
