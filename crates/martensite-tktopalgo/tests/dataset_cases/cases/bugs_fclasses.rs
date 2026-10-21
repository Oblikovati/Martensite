//! `bugs_fclasses` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::checkreal,
    commands::explode,
    distance::distmini,
    restore::restore_as,
    session::{Session, set, set_exploded},
    topology::vertex,
};
use martensite_tkmath::gp::Point3;

/// Distmini between the ellipse and two vertices on it is 0 (1e-12 absolute).
pub fn bug25757(s: &mut dyn Session) {
    let ellipse = restore_as(s, "bug25757_ellipse.brep", "ellipse").shape();
    let vertex1 = set(
        s,
        "vertex1",
        vertex(Point3::new(7.32050807568877, 3.999999999999999, 10.0)),
    );
    let vertex2 = set(
        s,
        "vertex2",
        vertex(Point3::new(7.32050807568877, 3.99999999999999, 10.0)),
    );
    let dist1 = distmini(s, "dv1", &vertex1, &ellipse, None).unwrap().value;
    println!("vertex1 distance = {dist1}");
    let dist2 = distmini(s, "dv2", &vertex2, &ellipse, None).unwrap().value;
    println!("vertex2 distance = {dist2}");
    checkreal("Distance 1", dist1, 0.0, 1.0e-12, 1.0e-2);
    checkreal("Distance 2", dist2, 0.0, 1.0e-12, 1.0e-2);
}

/// The distance between the two exploded sub-shapes is 1.0.
pub fn bug32960(s: &mut dyn Session) {
    let shape = restore_as(s, "bug32960.brep", "aShape").shape();
    let parts = set_exploded(s, "aShape", explode(&shape));
    let a_dist = distmini(s, "d", &parts[0], &parts[1], None).unwrap().value;
    checkreal("Distance value check", a_dist, 1.0, 1.0e-7, 1.0e-7);
}
