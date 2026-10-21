//! Contap walking-line template specialization against native state transitions.
use super::common::{Reader, Sink, attempt, end, flag, int, pnt, real, show, xyz};
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::{
    int_surf::{IntSurfPathPoint, IntSurfPntOn2S},
    int_walk::TheIWLine,
};
use martensite_tkhlr::contap::ContapTheIWLineOfTheIWalking;
use martensite_tkmath::gp::Vector3;
fn pnt_on_2s(r: &mut Reader) -> IntSurfPntOn2S {
    let x = r.pnt();
    let (a, b, e, d) = (r.real(), r.real(), r.real(), r.real());
    let mut q = IntSurfPntOn2S::new();
    q.set_value_all(x, a, b, e, d);
    q
}
pub fn pnt2s(tokens: &mut Sink, p: &IntSurfPntOn2S) {
    pnt(tokens, p.value());
    let (a, b, c, d) = p.parameters();
    [a, b, c, d]
        .into_iter()
        .for_each(|value| real(tokens, value));
}
pub fn line_report(tokens: &mut Sink, l: &ContapTheIWLineOfTheIWalking) {
    let n = l.nb_points();
    int(tokens, n);
    for i in 1..=n {
        pnt2s(tokens, &l.value(i).expect("probe line point"));
    }
    flag(tokens, l.is_closed());
    flag(tokens, l.has_first_point());
    flag(tokens, l.has_last_point());
    line_endpoints(tokens, l);
    line_passings(tokens, l);
    line_tangent(tokens, l);
}

fn line_endpoints(tokens: &mut Sink, l: &ContapTheIWLineOfTheIWalking) {
    show(tokens, l.first_point_index(), int);
    show(tokens, l.first_point(), |tokens, p| pnt(tokens, p.value()));
    show(tokens, l.last_point_index(), int);
    show(tokens, l.last_point(), |tokens, p| pnt(tokens, p.value()));
}

fn line_passings(tokens: &mut Sink, l: &ContapTheIWLineOfTheIWalking) {
    int(tokens, l.nb_passing_point());
    for i in 1..=l.nb_passing_point() {
        let (a, b) = l.passing_point(i).expect("probe passing point");
        int(tokens, a);
        int(tokens, b);
    }
}

fn line_tangent(tokens: &mut Sink, l: &ContapTheIWLineOfTheIWalking) {
    let (v, idx) = l.tangent_vector();
    xyz(tokens, *v.xyz());
    int(tokens, idx);
    flag(tokens, l.is_tangent_at_begining());
    flag(tokens, l.is_tangent_at_end());
}

fn path_point_of(r: &mut Reader) -> IntSurfPathPoint {
    let x = r.pnt();
    let u = r.real();
    IntSurfPathPoint::with_value(x, u, r.real())
}

/// One command of `ga_ip_iwline_case`.
fn iwline_command(r: &mut Reader, l: &ContapTheIWLineOfTheIWalking, c: i32) -> Result<(), Failure> {
    match c {
        1..=6 => point_status_command(r, l, c),
        7..=10 => passing_tangent_command(r, l, c),
        11 => l.reverse(),
        _ => l.cut(r.integer())?,
    }
    Ok(())
}

fn point_status_command(r: &mut Reader, l: &ContapTheIWLineOfTheIWalking, c: i32) {
    match c {
        1 => l.add_point(&pnt_on_2s(r)),
        2 => {
            let cl = r.flag();
            l.add_status_first(cl, r.flag());
        }
        3 => first_status_point(r, l),
        4 => {
            let (cl, hf) = (r.flag(), r.flag());
            l.add_status_first_last(cl, hf, r.flag());
        }
        5 => l.add_status_last(r.flag()),
        _ => last_status_point(r, l),
    }
}

fn first_status_point(r: &mut Reader, l: &ContapTheIWLineOfTheIWalking) {
    let (cl, hf, k) = (r.flag(), r.flag(), r.integer());
    l.add_status_first_point(cl, hf, k, &path_point_of(r));
}

fn last_status_point(r: &mut Reader, l: &ContapTheIWLineOfTheIWalking) {
    let (hl, k) = (r.flag(), r.integer());
    l.add_status_last_point(hl, k, &path_point_of(r));
}

fn passing_tangent_command(r: &mut Reader, l: &ContapTheIWLineOfTheIWalking, c: i32) {
    match c {
        7 => l.add_index_passing(r.integer()),
        8 => {
            let v = Vector3::from(r.xyz());
            l.set_tangent_vector(v, r.integer());
        }
        9 => l.set_tangency_at_begining(r.flag()),
        _ => l.set_tangency_at_end(r.flag()),
    }
}

/// `ga_ip_iwline_case`.
pub fn iwline_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let l = ContapTheIWLineOfTheIWalking::with_inc_allocator();
    for _ in 0..r.integer() {
        let c = r.integer();
        attempt(tokens, |_| iwline_command(&mut r, &l, c));
    }
    line_report(tokens, &l);
    int(tokens, l.line().nb_points());
    attempt(tokens, |_| l.value(0).map(|_| ()));
    attempt(tokens, |_| l.value(l.nb_points() + 1).map(|_| ()));
    attempt(tokens, |_| l.passing_point(0).map(|_| ()));
    attempt(tokens, |_| {
        l.passing_point(l.nb_passing_point() + 1).map(|_| ())
    });
    end(tokens);
}
