//! Datum objects built from displayed points (circle, plane, triangle, segment), shown after
//! erasing the points and fitting; the open view is dumped as the 3D check screenshot.
use super::prelude::*;
use crate::viewer_objects::{
    circle_object, circle_through, point_object, segment_object, triangle_object,
};
use martensite_tkernel::standard::Handle;
use martensite_tkgeombase::gc::MakePlane;
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::ais::{AisInteractiveObject, AisPlane};

/// Show the named points, then `object` as `name`; erase the points, fit, report OK and dump
/// `image`, with the OCC22554 banner first.
fn datum_from_points(
    points: &[(&str, [f64; 3])],
    name: &str,
    object: Handle<dyn AisInteractiveObject>,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(&["============", "OCC22554", "============", ""]);
    let mut run = SessionRun::new()?;
    for (point, xyz) in points {
        run.session().display_object(point, point_object(*xyz)?)?;
    }
    run.session().display_object(name, object)?;
    let names: Vec<&str> = points.iter().map(|(point, _)| *point).collect();
    run.session().erase(&names)?;
    run.session().fit()?;
    println!("OK OCC22554");
    // `checkview -screenshot -3d` dumps the open view.
    run.dump(image)?;
    run.finish()
}

const P3: [[f64; 3]; 3] = [[0., 0., 0.], [5., 5., 0.], [10., 0., 0.]];

/// `bug22554_1`: an outline circle through three points.
pub(crate) fn erased_points_circle() -> Result<(), Box<dyn Error>> {
    let circle = circle_object(&circle_through(P3)?, false, None);
    let points = [("p1", P3[0]), ("p2", P3[1]), ("p3", P3[2])];
    datum_from_points(&points, "result", circle, "bug22554_1.png")
}

/// `bug22554_2`: the plane through three points.
pub(crate) fn erased_points_plane() -> Result<(), Box<dyn Error>> {
    let xyz = [[0., 0., 0.], [30., 0., 0.], [30., 30., 0.]];
    let [a, b, c] = xyz.map(|[x, y, z]| Point3::new(x, y, z));
    let plane = MakePlane::through_points(a, b, c)?.value()?;
    let object = AisPlane::new(&plane, false)?.cast();
    let points = [("p1", xyz[0]), ("p2", xyz[1]), ("p3", xyz[2])];
    datum_from_points(&points, "result", object, "bug22554_2.png")
}

/// `bug22554_3`: the triangle of three points.
pub(crate) fn erased_points_triangle() -> Result<(), Box<dyn Error>> {
    let xyz = [[0., 0., 0.], [50., 0., 0.], [25., 40., 0.]];
    let points = [("p1", xyz[0]), ("p2", xyz[1]), ("p3", xyz[2])];
    datum_from_points(&points, "result", triangle_object(xyz)?, "bug22554_3.png")
}

/// `bug22554_4`: the segment of two points.
pub(crate) fn erased_points_segment() -> Result<(), Box<dyn Error>> {
    let xyz = [[0., 0., 0.], [50., 0., 0.]];
    let points = [("p1", xyz[0]), ("p2", xyz[1])];
    datum_from_points(&points, "segment", segment_object(xyz)?, "bug22554_4.png")
}
