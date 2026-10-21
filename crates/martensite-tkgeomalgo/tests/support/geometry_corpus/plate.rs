//! The Plate operations of the geometry corpus.
use super::common::{Reader, end, flag, int, real, show, xy, xyz};
use super::print::Output;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::Sequence;
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::plate::*;
use martensite_tkmath::gp::{Line3, Plane3, Xy, Xyz};

fn ppc(r: &mut Reader) -> PlatePinpointConstraint {
    let (u, v) = (r.real(), r.real());
    let p = r.xyz();
    let (u_order, v_order) = (r.integer(), r.integer());
    PlatePinpointConstraint::new(Xy::new(u, v), p, u_order, v_order)
}

fn print_ppc(output: &Output, c: &PlatePinpointConstraint) {
    xy(output, c.pnt2d());
    xyz(output, c.value());
    int(output, c.idu());
    int(output, c.idv());
}

fn print_lxyz(output: &Output, c: &PlateLinearXYZConstraint) {
    let k = c.coeff().expect("corpus coefficients");
    int(output, k.nb_rows() as i32);
    int(output, k.nb_columns() as i32);
    c.get_ppc()
        .expect("corpus constraints")
        .iter()
        .for_each(|value| print_ppc(output, value));
    k.as_slice().iter().for_each(|&v| real(output, v));
}

fn print_lsc(output: &Output, c: &PlateLinearScalarConstraint) {
    let k = c.coeff().expect("corpus coefficients");
    int(output, k.nb_rows() as i32);
    int(output, k.nb_columns() as i32);
    c.get_ppc()
        .expect("corpus constraints")
        .iter()
        .for_each(|value| print_ppc(output, value));
    k.as_slice().iter().for_each(|&v| xyz(output, v));
}

fn lxyz(r: &mut Reader) -> Result<PlateLinearXYZConstraint, Failure> {
    let (rows, cols) = (r.integer(), r.integer());
    let c = PlateLinearXYZConstraint::with_size(rows, cols)?;
    for j in 1..=cols {
        c.set_ppc(j, ppc(r))?;
    }
    for i in 1..=rows {
        for j in 1..=cols {
            c.set_coeff(i, j, r.real())?;
        }
    }
    Ok(c)
}

fn lsc(r: &mut Reader) -> Result<PlateLinearScalarConstraint, Failure> {
    let (rows, cols) = (r.integer(), r.integer());
    let c = PlateLinearScalarConstraint::with_size(rows, cols)?;
    for j in 1..=cols {
        c.set_ppc(j, ppc(r))?;
    }
    for i in 1..=rows {
        for j in 1..=cols {
            c.set_coeff(i, j, r.xyz())?;
        }
    }
    Ok(c)
}

/// The ga_plate_report block.
fn report(output: &Output, p: &PlatePlate, uv: &[f64]) {
    flag(output, p.is_done());
    int(output, p.continuity());
    show(output, p.uv_box(), |(a, b, c, d)| {
        [a, b, c, d]
            .into_iter()
            .for_each(|value| real(output, value))
    });
    for q in uv.chunks(2).filter(|c| c.len() == 2) {
        let q = Xy::new(q[0], q[1]);
        xyz(output, p.evaluate(q).expect("corpus evaluate"));
        for (u_order, v_order) in [(1, 0), (0, 1), (1, 1), (2, 0)] {
            xyz(
                output,
                p.evaluate_derivative(q, u_order, v_order)
                    .expect("corpus derivative"),
            );
        }
    }
    if p.is_done() {
        let coefs = p.coef_pol().expect("corpus coefficients");
        coefs
            .get()
            .expect("corpus array")
            .array2()
            .as_slice()
            .iter()
            .for_each(|&v| xyz(output, v));
    }
    let mut seq: Sequence<Xy> = Sequence::default();
    p.uv_constraints(&mut seq);
    int(output, seq.length());
}

/// `order aniso pponly npp ppc... nxyz lxyz... nsc lsc... uv...`: the ga_plate_solve_case plate.
pub fn solve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ord, aniso, pponly) = (r.integer(), r.real(), r.flag());
    let mut p = PlatePlate::new();
    for _ in 0..r.integer() {
        p.load_pinpoint(&ppc(&mut r));
    }
    for _ in 0..r.integer() {
        p.load_linear_xyz(&lxyz(&mut r).expect("corpus lxyz"))
            .expect("corpus load");
    }
    for _ in 0..r.integer() {
        p.load_linear_scalar(&lsc(&mut r).expect("corpus lsc"))
            .expect("corpus load");
    }
    let uv = r.reals();
    if let Err(error) = p.solve_ti(ord, aniso, &ProgressRange::default()) {
        output.write(format_args!("{} ", error.exception_type()));
    }
    if pponly {
        p.set_polynomial_part_only(true);
    }
    report(output, &p, &uv);
    report(output, &p.clone(), &uv);
    let mut a = PlatePlate::new();
    a.load_pinpoint(&PlatePinpointConstraint::new(
        Xy::new(9.0, 9.0),
        Xyz::new(1.0, 1.0, 1.0),
        0,
        0,
    ));
    a.copy_from(&p);
    report(output, &a, &uv);
    end(output);
}

fn d1(r: &mut Reader) -> PlateD1 {
    let a = r.xyz();
    PlateD1::new(a, r.xyz())
}

fn d2(r: &mut Reader) -> PlateD2 {
    let (a, b) = (r.xyz(), r.xyz());
    PlateD2::new(a, b, r.xyz())
}

fn d3(r: &mut Reader) -> PlateD3 {
    let (a, b, c) = (r.xyz(), r.xyz(), r.xyz());
    PlateD3::new(a, b, c, r.xyz())
}

/// The ga_plate_solve_with plate: the constraint, three corner values, order 3.
fn solve_with(
    output: &Output,
    load: impl FnOnce(&mut PlatePlate) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let mut p = PlatePlate::new();
    load(&mut p)?;
    let corners = [
        (Xy::new(0.0, 0.0), Xyz::new(0.0, 0.0, 0.0)),
        (Xy::new(1.0, 0.0), Xyz::new(1.0, 0.0, 0.5)),
        (Xy::new(0.0, 1.0), Xyz::new(0.0, 1.0, -0.5)),
    ];
    for (uv, value) in corners {
        p.load_pinpoint(&PlatePinpointConstraint::new(uv, value, 0, 0));
    }
    if let Err(error) = p.solve_ti(3, 1.0, &ProgressRange::default()) {
        output.write(format_args!("{} ", error.exception_type()));
    }
    report(output, &p, &[0.3, 0.4]);
    Ok(())
}

fn gtoc(output: &Output, c: &PlateGtoCConstraint) -> Result<(), Failure> {
    int(output, c.nb_ppc());
    for i in 0..9 {
        print_ppc(output, &c.get_ppc(i)?);
    }
    xyz(output, c.d1_surf_init().du());
    xyz(output, c.d1_surf_init().dv());
    solve_with(output, |p| p.load_gtoc(c))
}

fn free(output: &Output, c: &PlateFreeGtoCConstraint) -> Result<(), Failure> {
    int(output, c.nb_ppc());
    for i in 0..5 {
        print_ppc(output, &c.get_ppc(i)?);
    }
    int(output, c.nb_lsc());
    for i in 0..c.nb_lsc() {
        print_lsc(output, c.lsc(i)?);
    }
    solve_with(output, |p| p.load_free_gtoc(c))
}

fn constraint(output: &Output, r: &mut Reader, kind: i32, uv: Xy) -> Result<(), Failure> {
    match kind {
        0 | 1 => {
            let (o, d) = (r.pnt(), r.dir());
            let (u_order, v_order) = (r.integer(), r.integer());
            if kind == 0 {
                let c = PlateLineConstraint::new(uv, &Line3::new(o, d), u_order, v_order)?;
                print_lsc(output, c.lsc());
                return solve_with(output, |p| p.load_line(&c));
            }
            let c = PlatePlaneConstraint::new(uv, &Plane3::new(o, d)?, u_order, v_order)?;
            print_lsc(output, c.lsc());
            solve_with(output, |p| p.load_plane(&c))
        }
        2 => {
            let mut s: Sequence<Xy> = Sequence::default();
            for q in r.reals().chunks(2).filter(|c| c.len() == 2) {
                s.append(Xy::new(q[0], q[1]));
            }
            let c = PlateGlobalTranslationConstraint::new(&s)?;
            print_lxyz(output, c.lxyzc());
            solve_with(output, |p| p.load_global_translation(&c))
        }
        3 => {
            let mut s: Sequence<PlatePinpointConstraint> = Sequence::default();
            for _ in 0..r.integer() {
                s.append(ppc(r));
            }
            let c = PlateSampledCurveConstraint::new(&s, r.integer())?;
            print_lxyz(output, c.lxyzc());
            solve_with(output, |p| p.load_sampled_curve(&c))
        }
        4..=9 => gtoc_kind(output, r, kind, uv),
        _ => free_kind(output, r, kind, uv),
    }
}

fn gtoc_kind(output: &Output, r: &mut Reader, kind: i32, uv: Xy) -> Result<(), Failure> {
    let (s1, t1) = (d1(r), d1(r));
    if kind <= 5 {
        let np = r.xyz();
        let c = if kind == 4 {
            PlateGtoCConstraint::g1(uv, &s1, &t1)
        } else {
            PlateGtoCConstraint::g1_projected(uv, &s1, &t1, np)
        };
        return gtoc(output, &c);
    }
    let (s2, t2) = (d2(r), d2(r));
    if kind <= 7 {
        let np = r.xyz();
        let c = if kind == 6 {
            PlateGtoCConstraint::g2(uv, &s1, &t1, &s2, &t2)?
        } else {
            PlateGtoCConstraint::g2_projected(uv, (&s1, &t1), (&s2, &t2), np)?
        };
        return gtoc(output, &c);
    }
    let (s3, t3) = (d3(r), d3(r));
    let np = r.xyz();
    let c = if kind == 8 {
        PlateGtoCConstraint::g3(uv, (&s1, &t1), (&s2, &t2), (&s3, &t3))?
    } else {
        PlateGtoCConstraint::g3_projected(uv, (&s1, &t1), (&s2, &t2), (&s3, &t3), np)?
    };
    gtoc(output, &c)
}

fn free_kind(output: &Output, r: &mut Reader, kind: i32, uv: Xy) -> Result<(), Failure> {
    let (s1, t1) = (d1(r), d1(r));
    if kind == 10 {
        let (load, o) = (r.real(), r.integer());
        return free(output, &PlateFreeGtoCConstraint::g1(uv, &s1, &t1, load, o)?);
    }
    let (s2, t2) = (d2(r), d2(r));
    if kind == 11 {
        let (load, o) = (r.real(), r.integer());
        return free(
            output,
            &PlateFreeGtoCConstraint::g2(uv, (&s1, &t1), (&s2, &t2), load, o)?,
        );
    }
    let (s3, t3) = (d3(r), d3(r));
    let (load, o) = (r.real(), r.integer());
    free(
        output,
        &PlateFreeGtoCConstraint::g3(uv, (&s1, &t1), (&s2, &t2), (&s3, &t3), load, o)?,
    )
}

/// `kind args...`: the ga_plate_constraint_case constraints.
pub fn constraint_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    let uv = Xy::new(r.real(), r.real());
    if let Err(error) = constraint(output, &mut r, kind, uv) {
        output.write(format_args!("{} ", error.exception_type()));
    }
    end(output);
}
