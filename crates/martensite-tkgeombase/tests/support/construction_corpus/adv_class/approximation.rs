fn fnv(text: &[u8]) -> u64 {
    text.iter().fold(1_469_598_103_934_665_603_u64, |h, ch| {
        (h ^ u64::from(*ch)).wrapping_mul(1_099_511_628_211)
    })
}

fn print_approx(transcript: &Output, a: &AdvApp2VarApproxAFunc2Var) -> Result<(), Failure> {
    flag(transcript, a.is_done());
    flag(transcript, a.has_result());
    int(transcript, a.u_degree());
    int(transcript, a.v_degree());
    for k in 1..=3 {
        int(transcript, a.num_sub_spaces(k)?);
    }
    real(transcript, a.max_error(3, 1)?);
    real(transcript, a.average_error(3, 1)?);
    real(transcript, a.u_front_error(3, 1)?);
    real(transcript, a.v_front_error(3, 1)?);
    real(transcript, a.crit_error(3, 1)?);
    flag(transcript, a.max_errors(1)?.is_none());
    flag(transcript, a.v_front_errors(2)?.is_none());
    attempt(transcript, || a.max_errors(4).map(|_| ()));
    attempt(transcript, || a.average_errors(0).map(|_| ()));
    attempt(transcript, || a.u_front_error(3, 2).map(|_| ()));
    attempt(transcript, || a.crit_error(1, 1).map(|_| ()));
    let s = a.surface(1)?;
    [s.u_degree(), s.v_degree(), s.nb_u_knots(), s.nb_v_knots()]
        .into_iter()
        .for_each(|value| int(transcript, value));
    for i in 1..=s.nb_u_knots() {
        real(transcript, s.u_knot(i)?);
        int(transcript, s.u_multiplicity(i)?);
    }
    for i in 1..=s.nb_v_knots() {
        real(transcript, s.v_knot(i)?);
        int(transcript, s.v_multiplicity(i)?);
    }
    int(transcript, s.nb_u_poles());
    int(transcript, s.nb_v_poles());
    for i in 1..=s.nb_u_poles() {
        for j in 1..=s.nb_v_poles() {
            pnt(transcript, s.pole(i, j)?);
        }
    }
    let mut text: Vec<u8> = Vec::new();
    a.dump(&mut text)
        .map_err(|e| Failure::with_message("Standard_Failure", &e.to_string()))?;
    transcript.write(format_args!("{} ", fnv(&text)));
    Ok(())
}

fn approx_body(transcript: &Output, r: &mut Reader) -> Result<(), Failure> {
    let func = Function(r.integer());
    let (fav, cu, cv, pr) = (r.integer(), r.integer(), r.integer(), r.integer());
    let (mu, mv, mp) = (r.integer(), r.integer(), r.integer());
    let d = [r.real(), r.real(), r.real(), r.real()];
    let (cut, ck, cr, cm) = (r.integer(), r.integer(), r.integer(), r.real());
    let mut rec: Array1<'static, f64> = Array1::with_bounds(1, 1).expect("bounds");
    rec.set_value(1, 0.5 * (d[0] + d[1])).expect("in range");
    let mut pref: Array1<'static, f64> = Array1::with_bounds(1, 2).expect("bounds");
    pref.set_value(1, 0.3).expect("in range");
    pref.set_value(2, 0.7).expect("in range");
    let (dicho, pref_cut) = (
        AdvApproxDichoCutting::new(),
        AdvApproxPrefAndRec::new(&rec, &pref, 5.)?,
    );
    let ch: &dyn AdvApproxCutting = if cut == 0 { &dicho } else { &pref_cut };
    let kinds = [
        AdvApp2VarCriterionType::Absolute,
        AdvApp2VarCriterionType::Relative,
    ];
    let reps = [
        AdvApp2VarCriterionRepartition::Regular,
        AdvApp2VarCriterionRepartition::Incremental,
    ];
    let crit = Criterion {
        max: cm,
        kind: kinds[usize::from(ck == 2)],
        repartition: reps[cr as usize],
    };
    let (t, tf) = (tol(1, 3), tof(1, 3));
    let (fav, cu, cv) = (
        ISO_TYPES[fav as usize],
        SHAPES[cu as usize],
        SHAPES[cv as usize],
    );
    let a = if ck == 0 {
        AdvApp2VarApproxAFunc2Var::new(
            0,
            0,
            1,
            None,
            None,
            t.as_ref(),
            None,
            None,
            tf.as_ref(),
            d[0],
            d[1],
            d[2],
            d[3],
            fav,
            cu,
            cv,
            pr,
            mu,
            mv,
            mp,
            &func,
            ch,
            ch,
        )?
    } else {
        AdvApp2VarApproxAFunc2Var::with_criterion(
            0,
            0,
            1,
            None,
            None,
            t.as_ref(),
            None,
            None,
            tf.as_ref(),
            d[0],
            d[1],
            d[2],
            d[3],
            fav,
            cu,
            cv,
            pr,
            mu,
            mv,
            mp,
            &func,
            &crit,
            ch,
            ch,
        )?
    };
    print_approx(transcript, &a)
}

/// aa_approx: function favor contU contV iprecis maxdegU maxdegV maxpatch u0 u1 v0 v1
/// cutkind critkind critrep critmax.
pub fn approx_case(transcript: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(transcript, || approx_body(transcript, &mut r));
    end(transcript);
}
