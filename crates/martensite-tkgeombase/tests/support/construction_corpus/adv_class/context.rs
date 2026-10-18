fn iso_index(iso_type: IsoType) -> i32 {
    ISO_TYPES
        .iter()
        .position(|t| *t == iso_type)
        .map_or(-1, |i| i as i32)
}

fn tol(n: i32, d: i32) -> Option<Array1<'static, f64>> {
    if n <= 0 {
        return None;
    }
    let mut t: Array1<'static, f64> = Array1::with_bounds(1, n).ok()?;
    for k in 1..=n {
        t.set_value(k, 1.0e-3 * f64::from(k) * f64::from(d)).ok()?;
    }
    Some(t)
}

fn tof(n: i32, d: i32) -> Option<Array2<'static, f64>> {
    if n <= 0 {
        return None;
    }
    let mut t: Array2<'static, f64> = Array2::with_bounds(1, n, 1, 4).ok()?;
    for k in 1..=n {
        for j in 1..=4 {
            let value = 1.0e-3 * f64::from(k) * f64::from(d) * (1. + f64::from(j) / 10.);
            t.set_value(k, j, value).ok()?;
        }
    }
    Some(t)
}

fn harray(transcript: &Output, a: Option<&Array1<'static, f64>>) {
    let Some(a) = a else {
        return int(transcript, -1);
    };
    int(transcript, a.length());
    a.as_slice().iter().for_each(|v| real(transcript, *v));
}

fn harray2(transcript: &Output, a: Option<&Array2<'static, f64>>) {
    let Some(a) = a else {
        return int(transcript, -1);
    };
    int(transcript, a.col_length());
    int(transcript, a.row_length());
    for i in a.lower_row()..=a.upper_row() {
        for j in a.lower_col()..=a.upper_col() {
            real(transcript, *a.value(i, j).expect("in range"));
        }
    }
}

fn head(transcript: &Output, a: &Array1<'static, f64>, n: i32) {
    a.as_slice()[..n.max(0) as usize]
        .iter()
        .for_each(|v| real(transcript, *v));
}

/// A Gauss table of `n` points without the zero-root row that mmapptt_ leaves
/// unwritten for an even count.
fn gauss(transcript: &Output, a: &Array1<'static, f64>, n: i32) {
    int(transcript, a.length());
    for (k, v) in (0..).zip(a.as_slice()) {
        if n % 2 == 1 || k % (n / 2 + 1) != 0 {
            real(transcript, *v);
        }
    }
}

/// A 3D context with one subspace, as the retained `aa_patch` builds it.
fn context_3d(
    f: i32,
    orders: [i32; 2],
    limits: [i32; 2],
    precis: i32,
) -> Result<AdvApp2VarContext, Failure> {
    let (t, tf) = (tol(1, 3), tof(1, 3));
    let tolerances = SubspaceTolerances {
        one_d: None,
        two_d: None,
        three_d: t.as_ref(),
    };
    let fronts = FrontierTolerances {
        one_d: None,
        two_d: None,
        three_d: tf.as_ref(),
    };
    AdvApp2VarContext::new(
        f,
        orders[0],
        orders[1],
        limits[0],
        limits[1],
        precis,
        [0, 0, 1],
        tolerances,
        fronts,
    )
}

fn context_body(transcript: &Output, r: &mut Reader) -> Result<(), Failure> {
    let (f, iu, iv, nu, nv, p) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let (n1, n2, n3) = (r.integer(), r.integer(), r.integer());
    let (t1, t2, t3) = (tol(n1, 1), tol(n2, 2), tol(n3, 3));
    let (f1, f2, f3) = (tof(n1, 1), tof(n2, 2), tof(n3, 3));
    let tolerances = SubspaceTolerances {
        one_d: t1.as_ref(),
        two_d: t2.as_ref(),
        three_d: t3.as_ref(),
    };
    let fronts = FrontierTolerances {
        one_d: f1.as_ref(),
        two_d: f2.as_ref(),
        three_d: f3.as_ref(),
    };
    let c = AdvApp2VarContext::new(f, iu, iv, nu, nv, p, [n1, n2, n3], tolerances, fronts)?;
    for v in [
        c.total_dimension(),
        c.total_number_ssp(),
        c.favor_iso(),
        c.u_order(),
        c.v_order(),
    ] {
        int(transcript, v);
    }
    for v in [c.u_limit(), c.v_limit(), c.u_jac_deg(), c.v_jac_deg()] {
        int(transcript, v);
    }
    harray(transcript, c.u_jac_max());
    harray(transcript, c.v_jac_max());
    let (ur, vr) = (c.u_roots().expect("roots"), c.v_roots().expect("roots"));
    int(transcript, ur.length());
    int(transcript, vr.length());
    head(transcript, ur, ur.length() - if iu > -1 { 2 } else { 0 });
    head(transcript, vr, vr.length() - if iv > -1 { 2 } else { 0 });
    let pu = ur.length() - if iu > -1 { 2 } else { 0 };
    let pv = vr.length() - if iv > -1 { 2 } else { 0 };
    gauss(transcript, c.u_gauss().expect("gauss"), pu);
    gauss(transcript, c.v_gauss().expect("gauss"), pv);
    harray(transcript, c.i_toler());
    harray2(transcript, c.f_toler());
    harray2(transcript, c.c_toler());
    let copy: AdvApp2VarContext = c.clone();
    int(transcript, copy.u_limit());
    harray(transcript, copy.i_toler());
    Ok(())
}

/// aa_context: ifav iu iv nlimu nlimv iprecis nb1 nb2 nb3.
pub fn context_case(transcript: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(transcript, || context_body(transcript, &mut r));
    end(transcript);
}
