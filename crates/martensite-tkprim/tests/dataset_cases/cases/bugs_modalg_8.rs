//! `bugs_modalg_8` case bodies (the input of bug_gh469 is an issue attachment in the
//! test-data submodule).
use crate::support::{
    bounding::{Bounding, bounding},
    curves::{shape_set, transform_copy},
    log,
    prim::psphere,
    restore::restore_as,
    session::Session,
    topology::dump_shape,
};
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};

/// GH469: BRepBndLib::AddOptimal of the restored compound gives a box, not a void one,
/// and raises no exception.
pub fn bug_gh469(s: &mut dyn Session) {
    log::puts("=========================================================================");
    log::puts("GH469: Bounding box is void when using BRepBndLib::AddOptimal");
    log::puts("=========================================================================");
    log::puts("");
    let result = restore_as(s, "bug_gh469.brep", "result").shape();
    bounding(
        s,
        Bounding {
            optimal: true,
            ..Bounding::of(&result)
        },
    );
}

/// 0033591: a sphere rotated then translated with `-copy` keeps one surface (the old one
/// is not left in the shape).
pub fn bug33591(s: &mut dyn Session) {
    log::puts("========================");
    log::puts(
        "0033591: Modeling Algorithms - Regression: old surface is not removed after translation or rotation with geometry copying",
    );
    log::puts("========================");
    log::puts("");
    let sphere = shape_set(s, "Sphere_1", psphere(80.));
    let mut rotation = Transform3::default();
    rotation
        .set_rotation(
            Axis1::new(Point3::default(), Direction3::new(0., 1., 0.).unwrap()),
            90. * (std::f64::consts::PI / 180.0),
        )
        .unwrap();
    let sphere = shape_set(s, "Sphere_1", transform_copy(&sphere, rotation));
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(0., 0., 200.));
    let sphere = shape_set(s, "Sphere_1", transform_copy(&sphere, translation));
    let dump_output = dump_shape("Sphere_1", &sphere);
    log::result(&dump_output);
    let num_surfaces = dump_output
        .split("Dump of ")
        .skip(1)
        .find_map(|rest| {
            let (number, tail) = rest.split_once(' ')?;
            tail.starts_with("surfaces")
                .then(|| number.parse::<usize>().ok())
                .flatten()
        })
        .expect("Dump of N surfaces");
    if num_surfaces != 1 {
        log::puts("Error: The number of surfaces must be 1");
    }
}
