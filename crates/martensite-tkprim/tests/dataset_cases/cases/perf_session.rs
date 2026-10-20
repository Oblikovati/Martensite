//! `perf_session` case bodies.
use crate::support::{
    log,
    prim::box_sizes,
    printed::g17,
    session::Session,
    topology::{compound, trotate, ttranslate},
};
use martensite_tkbrep::topods::Shape;
use std::time::Instant;

/// The script's `replicate result shape nb cx cy cz rx ry rz angle tx ty tz`: `copy s a0`,
/// then each `a_i` a copy of `a_(i-1)` rotated and translated, and the compound of all.
/// `copy` of a shape variable shares the shape, as the variable copy does.
fn replicate(
    shape: &Shape,
    nb: usize,
    center: [f64; 3],
    axis: [f64; 3],
    angle: f64,
    translation: [f64; 3],
) -> Shape {
    let mut copies = vec![shape.clone()];
    for i in 1..nb {
        let rotated = trotate(&copies[i - 1], center, axis, angle, false);
        copies.push(ttranslate(&rotated, translation, false));
    }
    let refs: Vec<&Shape> = copies.iter().collect();
    compound(&refs)
}

/// 0028404: a deeply nested compound of about 93 000 located copies of a box is built
/// within the perf group's cpulimit (`autodisplay 0`: no variable is displayed).
pub fn bug28404(_s: &mut dyn Session) {
    log::puts("0028404: DRAW is slow");
    let start = Instant::now();
    let b = box_sizes(10., 10., 10.);
    let r1 = replicate(&b, 12, [0., 0., 0.], [0., 0., 1.], 30., [15., 20., 3.]);
    let r2 = replicate(&r1, 36, [50., -50., 0.], [1., 1., 0.], 10., [0., 0., 0.]);
    let r3 = replicate(
        &r2,
        18,
        [250., 250., 0.],
        [1., -1., 0.],
        20.,
        [100., 100., 20.],
    );
    let _r4 = replicate(
        &r3,
        12,
        [-300., -300., 0.],
        [0., 0., 1.],
        30.,
        [300., 300., 0.],
    );
    log::result(&format!(
        "COUNTER huge compound generation CPU: {}\n",
        g17(start.elapsed().as_secs_f64())
    ));
}
