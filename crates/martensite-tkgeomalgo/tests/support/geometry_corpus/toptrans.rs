//! The TopTrans operations of the geometry corpus.
use super::common::{Reader, attempt, end, int};
use super::print::Output;
use martensite_tkg3d::top_abs::{Orientation, State};
use martensite_tkgeomalgo::toptrans::{CurveTransition, SurfaceTransition, get_after, get_before};

fn orientation(r: &mut Reader) -> Orientation {
    Orientation::ALL[r.integer() as usize]
}
fn states(output: &Output, before: State, after: State) {
    int(output, before as i32);
    int(output, after as i32);
}

/// Commands on one curve transition, the states after each.
pub fn curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut transition = CurveTransition::new();
    for _ in 0..r.integer() {
        match r.integer() {
            0 => {
                let (t, n) = (r.dir(), r.dir());
                transition.reset(t, n, r.real());
            }
            1 => transition.reset_line(r.dir()),
            _ => {
                let (tole, t, n, c) = (r.real(), r.dir(), r.dir(), r.real());
                let s = orientation(&mut r);
                transition.compare(tole, t, n, c, s, orientation(&mut r));
            }
        }
        states(output, transition.state_before(), transition.state_after());
    }
    end(output);
}

fn surface_command(
    output: &Output,
    r: &mut Reader,
    transition: &mut SurfaceTransition,
    command: i32,
) {
    if command == 0 {
        let (t, n) = (r.dir(), r.dir());
        return attempt(output, || transition.reset_line(t, n));
    }
    if command == 1 {
        let (t, n, mx, mn, cx, cn) = (r.dir(), r.dir(), r.dir(), r.dir(), r.real(), r.real());
        return attempt(output, || transition.reset(t, n, mx, mn, cx, cn));
    }
    let (tole, n) = (r.real(), r.dir());
    if command == 2 {
        let (s, o) = (orientation(r), orientation(r));
        return attempt(output, || transition.compare_plane(tole, n, s, o));
    }
    let (mx, mn, cx, cn) = (r.dir(), r.dir(), r.real(), r.real());
    let (s, o) = (orientation(r), orientation(r));
    attempt(output, || transition.compare(tole, n, mx, mn, cx, cn, s, o));
}

/// Commands on one surface transition, the exception and the states after each.
pub fn surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut transition = SurfaceTransition::new();
    for _ in 0..r.integer() {
        let command = r.integer();
        surface_command(output, &mut r, &mut transition, command);
        states(output, transition.state_before(), transition.state_after());
    }
    end(output);
}

/// `orientation`: the static state tables.
pub fn states_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let o = orientation(&mut r);
    states(output, get_before(o), get_after(o));
    end(output);
}
