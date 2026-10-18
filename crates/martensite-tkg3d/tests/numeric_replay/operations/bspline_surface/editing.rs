// Kept in the parent operation namespace to preserve the shared fixtures and callback contracts.
fn read_reals(r: &mut Reader) -> Array1<'static, f64> {
    let (lower, n) = (r.integer(), r.integer());
    let mut a = Array1::<f64>::with_bounds(lower, lower + n - 1).unwrap();
    for k in lower..lower + n {
        a[k] = r.real();
    }
    a
}
fn read_ints(r: &mut Reader) -> Array1<'static, i32> {
    let (lower, n) = (r.integer(), r.integer());
    let mut a = Array1::<i32>::with_bounds(lower, lower + n - 1).unwrap();
    for k in lower..lower + n {
        a[k] = r.integer();
    }
    a
}
fn read_pnts(r: &mut Reader) -> Array1<'static, Point3> {
    let (lower, n) = (r.integer(), r.integer());
    let mut a = Array1::<Point3>::with_bounds(lower, lower + n - 1).unwrap();
    for k in lower..lower + n {
        a[k] = r.pnt();
    }
    a
}

// Periodicity, origin, reversal, degree and knot edits (codes 0 to 31).
fn knot_edit(
    output: &Output,
    s: &GeomBSplineSurface,
    r: &mut Reader,
    op: i32,
    u: bool,
) -> Result<(), Failure> {
    match op {
        0 => s.exchange_uv(),
        2 | 3 => {
            if u {
                s.set_u_periodic()
            } else {
                s.set_v_periodic()
            }
        }
        4 | 5 => {
            let i = r.integer();
            if u {
                s.set_u_origin(i)
            } else {
                s.set_v_origin(i)
            }
        }
        6 | 7 => {
            if u {
                s.set_u_not_periodic()
            } else {
                s.set_v_not_periodic()
            }
        }
        8 | 9 => {
            if u {
                s.u_reverse()
            } else {
                s.v_reverse()
            }
        }
        10 => {
            let (a, b) = (r.integer(), r.integer());
            s.increase_degree(a, b)
        }
        12 | 13 => {
            let (k, m) = (read_reals(r), read_ints(r));
            let (t, add) = (r.real(), r.flag());
            if u {
                s.insert_u_knots(&k, &m, t, add)
            } else {
                s.insert_v_knots(&k, &m, t, add)
            }
        }
        14 | 15 => {
            let (i, m, t) = (r.integer(), r.integer(), r.real());
            let removed = if u {
                s.remove_u_knot(i, m, t)
            } else {
                s.remove_v_knot(i, m, t)
            };
            removed.map(|value| flag(output, value))
        }
        _ => multiplicity_edit(output, s, r, op, u),
    }
}

fn multiplicity_edit(
    output: &Output,
    s: &GeomBSplineSurface,
    r: &mut Reader,
    op: i32,
    u: bool,
) -> Result<(), Failure> {
    match op {
        16 | 17 => {
            let (i, m) = (r.integer(), r.integer());
            if u {
                s.increase_u_multiplicity(i, m)
            } else {
                s.increase_v_multiplicity(i, m)
            }
        }
        18 | 19 => {
            let (a, b, m) = (r.integer(), r.integer(), r.integer());
            if u {
                s.increase_u_multiplicity_range(a, b, m)
            } else {
                s.increase_v_multiplicity_range(a, b, m)
            }
        }
        20 | 21 => {
            let (a, b, st) = (r.integer(), r.integer(), r.integer());
            if u {
                s.increment_u_multiplicity(a, b, st)
            } else {
                s.increment_v_multiplicity(a, b, st)
            }
        }
        22 | 23 => {
            let (x, m, t, add) = (r.real(), r.integer(), r.real(), r.flag());
            if u {
                s.insert_u_knot(x, m, t, add)
            } else {
                s.insert_v_knot(x, m, t, add)
            }
        }
        _ => knot_value_edit(output, s, r, op, u),
    }
}

fn knot_value_edit(
    output: &Output,
    s: &GeomBSplineSurface,
    r: &mut Reader,
    op: i32,
    u: bool,
) -> Result<(), Failure> {
    match op {
        24 | 25 => {
            let p: Vec<f64> = (0..6).map(|_| r.real()).collect();
            let (a, b, c) = ((p[0], p[1]), (p[2], p[3]), (p[4], p[5]));
            if u {
                s.segment(a, b, c)
            } else {
                s.check_and_segment(a, b, c)
            }
        }
        26 | 27 => {
            let (i, x) = (r.integer(), r.real());
            if u {
                s.set_u_knot(i, x)
            } else {
                s.set_v_knot(i, x)
            }
        }
        28 | 29 => {
            let k = read_reals(r);
            if u {
                s.set_u_knots(&k)
            } else {
                s.set_v_knots(&k)
            }
        }
        30 | 31 => {
            let (i, x, m) = (r.integer(), r.real(), r.integer());
            if u {
                s.set_u_knot_with_multiplicity(i, x, m)
            } else {
                s.set_v_knot_with_multiplicity(i, x, m)
            }
        }
        _ => pole_edit(output, s, r, op, u),
    }
}

// Pole and weight edits, MovePoint and Transform (codes 32 to 46).
fn pole_edit(
    output: &Output,
    s: &GeomBSplineSurface,
    r: &mut Reader,
    op: i32,
    u: bool,
) -> Result<(), Failure> {
    match op {
        32 => {
            let (i, j, p) = (r.integer(), r.integer(), r.pnt());
            s.set_pole(i, j, p)
        }
        34 => {
            let (i, j, p, w) = (r.integer(), r.integer(), r.pnt(), r.real());
            s.set_pole_weighted(i, j, p, w)
        }
        36 | 37 => {
            let (i, p) = (r.integer(), read_pnts(r));
            if u {
                s.set_pole_row(i, &p)
            } else {
                s.set_pole_col(i, &p)
            }
        }
        38 | 39 => {
            let (i, p, w) = (r.integer(), read_pnts(r), read_reals(r));
            if u {
                s.set_pole_row_weighted(i, &p, &w)
            } else {
                s.set_pole_col_weighted(i, &p, &w)
            }
        }
        40 => {
            let (i, j, w) = (r.integer(), r.integer(), r.real());
            s.set_weight(i, j, w)
        }
        42 | 43 => {
            let (i, w) = (r.integer(), read_reals(r));
            if u {
                s.set_weight_row(i, &w)
            } else {
                s.set_weight_col(i, &w)
            }
        }
        44 => {
            let (a, b, p) = (r.real(), r.real(), r.pnt());
            let range = MoveRange {
                u_index1: r.integer(),
                u_index2: r.integer(),
                v_index1: r.integer(),
                v_index2: r.integer(),
            };
            let (f1, f2, f3, f4) = s.move_point(a, b, p, range)?;
            [f1, f2, f3, f4]
                .into_iter()
                .for_each(|value| int(output, value));
            Ok(())
        }
        46 => s.transform(trsf(r)),
        _ => panic!("unknown edit {op}"),
    }
}

pub fn bspline_surface_edit_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(base) = super::common::built_surface(output, &mut r) else {
        return;
    };
    let s: Handle<GeomBSplineSurface> = base.cast();
    let uv = r.reals();
    let n = r.integer();
    for _ in 0..n {
        let op = r.integer();
        attempt(output, || knot_edit(output, &s, &mut r, op, op % 2 == 0));
        state(output, &s);
        surface_summary(output, &*s, &uv);
    }
    dump(output, &*s, -1);
    end(output);
}
