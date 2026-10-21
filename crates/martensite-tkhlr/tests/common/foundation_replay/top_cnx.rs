//! TopBas and TopCnx: the
//! interference record and the edge-face transition.
use super::common::{Reader, Sink, end, int, real};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkhlr::top_bas::TestInterference;
use martensite_tkhlr::top_cnx::EdgeFaceTransition;

fn orientation(r: &mut Reader) -> Orientation {
    Orientation::ALL[r.integer() as usize]
}

/// `mode I B O T BT [x n]`: see `hl_tb_interference_case`.
pub fn interference_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let (mode, inter, bound) = (r.integer(), r.real(), r.integer());
    let (o, t, bt) = (
        orientation(&mut r),
        orientation(&mut r),
        orientation(&mut r),
    );
    let mut record = TestInterference::with_values(inter, bound, o, t, bt);
    if mode == 0 {
        record = TestInterference::new();
        record.set_intersection(inter);
        record.set_boundary(bound);
        record.set_orientation(o);
        record.set_transition(t);
        record.set_boundary_transition(bt);
    }
    if mode == 2 {
        let (x, n) = (r.real(), r.integer());
        *record.change_intersection() = x;
        *record.change_boundary() += n;
    }
    real(out, record.intersection());
    int(out, record.boundary());
    int(out, record.orientation() as i32);
    int(out, record.transition() as i32);
    int(out, record.boundary_transition() as i32);
    end(out);
}

fn print_transition(out: &mut Sink, transition: &EdgeFaceTransition) {
    int(out, transition.transition() as i32);
    int(out, transition.boundary_transition() as i32);
}

fn run_command(r: &mut Reader, transition: &mut EdgeFaceTransition) {
    match r.integer() {
        0 => {
            let (t, n) = (r.dir(), r.dir());
            transition.reset(t, n, r.real());
        }
        1 => transition.reset_line(r.dir()),
        _ => {
            let (tole, t, n, c) = (r.real(), r.dir(), r.dir(), r.real());
            let (o, tr, bt) = (orientation(r), orientation(r), orientation(r));
            transition.add_interference(tole, t, n, c, o, tr, bt);
        }
    }
}

/// `n commands`: see `hl_tc_transition_case`.
pub fn transition_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut transition = EdgeFaceTransition::new();
    print_transition(out, &transition);
    for _ in 0..r.integer() {
        run_command(&mut r, &mut transition);
        print_transition(out, &transition);
    }
    end(out);
}
