// Kept in the parent operation namespace to preserve the shared fixtures and callback contracts.
pub fn advapprox_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut f = evaluator(&mut r);
    let (n1, n2, n3) = (r.integer(), r.integer(), r.integer());
    let (t1, t2, t3) = (
        tolerances(output, &mut r, n1),
        tolerances(output, &mut r, n2),
        tolerances(output, &mut r, n3),
    );
    let (first, last) = (r.real(), r.real());
    let cont = shape(r.integer());
    let (deg, seg) = (r.integer(), r.integer());
    let built = cutting(output, &mut r).and_then(|cut| match cut {
        Some(cut) => AdvApproxApproxAFunction::with_cutting(
            n1,
            n2,
            n3,
            t1.as_ref(),
            t2.as_ref(),
            t3.as_ref(),
            first,
            last,
            cont,
            deg,
            seg,
            &mut f,
            &*cut,
        ),
        None => AdvApproxApproxAFunction::new(
            n1,
            n2,
            n3,
            t1.as_ref(),
            t2.as_ref(),
            t3.as_ref(),
            first,
            last,
            cont,
            deg,
            seg,
            &mut f,
        ),
    });
    show(output, built, |a| result(output, &a));
    int(output, f.calls);
    end(output);
}

pub fn advapprox_static_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut f = evaluator(&mut r);
    let nss = r.integer();
    let dims_values: Vec<i32> = (0..nss).map(|_| r.integer()).collect();
    let mut dims: Array1<'static, i32> = Array1::with_bounds(1, nss.max(1)).unwrap();
    dims.as_mut_slice()[..dims_values.len()].copy_from_slice(&dims_values);
    let given = r.integer();
    let (first, last) = (r.real(), r.real());
    let (order, coeffs, seg) = (r.integer(), r.integer(), r.integer());
    let (tol, code) = (r.real(), r.integer());
    let tol_values: Vec<f64> = (1..=nss.max(1)).map(|i| tol * f64::from(i)).collect();
    let tols = table(output, &tol_values);
    let size = (seg * coeffs * given).max(1);
    let mut ncoeff: Array1<'static, i32> = Array1::with_bounds(1, seg.max(1)).unwrap();
    ncoeff.init(-7);
    let mut coef: Array1<'static, f64> = Array1::with_bounds(1, size).unwrap();
    let mut ints: Array1<'static, f64> = Array1::with_bounds(1, seg + 1).unwrap();
    let mut emax: Array1<'static, f64> = Array1::with_bounds(1, (seg * nss).max(1)).unwrap();
    let mut eavg: Array1<'static, f64> = Array1::with_bounds(1, (seg * nss).max(1)).unwrap();
    let mut run = || -> Result<(i32, i32), Failure> {
        let cut = cutting(output, &mut r)?;
        let dicho = AdvApproxDichoCutting::new();
        let tool: &dyn AdvApproxCutting = match &cut {
            Some(cut) => &**cut,
            None => &dicho,
        };
        let input = AdvApproxApproximationInput {
            total_dimension: given,
            total_num_ss: nss,
            local_dimension: &dims,
            first,
            last,
            continuity_order: order,
            num_max_coeffs: coeffs,
            max_segments: seg,
            tolerances: &tols,
            code_precis: code,
        };
        let mut output = AdvApproxApproximationOutput {
            num_curves: 0,
            num_coeff_per_curve: &mut ncoeff,
            coefficients: &mut coef,
            intervals: &mut ints,
            error_max: &mut emax,
            average_error: &mut eavg,
            error_code: 0,
        };
        approximation(&input, &mut f, tool, &mut output)?;
        Ok((output.num_curves, output.error_code))
    };
    let outcome = run();
    show(output, outcome, |(num, err)| {
        int(output, num);
        int(output, err);
        (1..=num).for_each(|i| int(output, ncoeff[i]));
        if num > 0 {
            (1..=num + 1).for_each(|i| real(output, ints[i]));
        }
        coef.as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        for i in 1..=num * nss {
            real(output, emax[i]);
            real(output, eavg[i]);
        }
    });
    int(output, f.calls);
    end(output);
}

pub fn advapprox_simple_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut f = evaluator(&mut r);
    let nss = r.integer();
    let dims_values: Vec<i32> = (0..nss).map(|_| r.integer()).collect();
    let td: i32 = dims_values.iter().sum();
    let mut dims: Array1<'static, i32> = Array1::with_bounds(1, nss).unwrap();
    dims.as_mut_slice().copy_from_slice(&dims_values);
    let cont = shape(r.integer());
    let (code, maxcoef) = (r.integer(), r.integer());
    let (first, last) = (r.real(), r.real());
    let (maxdeg, tol) = (r.integer(), r.real());
    let tols = table(
        output,
        &(1..=nss).map(|i| tol * f64::from(i)).collect::<Vec<f64>>(),
    );
    let run = |f: &mut Evaluator| -> Result<(), Failure> {
        let (mut ng, mut wd) = (0, 0);
        plib::jacobi_parameters(cont, maxcoef, code, &mut ng, &mut wd)?;
        int(output, ng);
        int(output, wd);
        let jac = JacobiPolynomial::new(wd, cont)?;
        let mut s = AdvApproxSimpleApprox::new(td, nss, cont, wd, ng, &jac, f)?;
        s.perform(&dims, &tols, first, last, maxdeg)?;
        flag(output, s.is_done());
        if !s.is_done() {
            return Ok(());
        }
        s.first_constr()
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        int(output, s.degree());
        s.coefficients()
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        s.last_constr()
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        for i in td..=s.som_tab().upper() {
            real(output, s.som_tab()[i]);
            real(output, s.dif_tab()[i]);
        }
        if ng % 2 == 1 {
            for i in 0..td {
                real(output, s.som_tab()[i]);
                real(output, s.dif_tab()[i]);
            }
        }
        for i in 1..=nss {
            real(output, s.max_error(i)?);
            real(output, s.average_error(i)?);
        }
        dump_text(output, |text| s.dump(text));
        s.perform(&dims, &tols, first, 0.5 * (first + last), maxdeg)?;
        flag(output, s.is_done());
        int(output, s.degree());
        real(output, s.max_error(1)?);
        Ok(())
    };
    if let Err(error) = run(&mut f) {
        replay_print!(output, "{} ", error.exception_type());
    }
    int(output, f.calls);
    end(output);
}

pub fn advapprox_cut_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (r.real(), r.real());
    let (accepted, c) = AdvApproxDichoCutting::new().value(a, b);
    flag(output, accepted);
    real(output, c);
    match cutting(output, &mut r) {
        Ok(Some(tool)) => {
            let (accepted, c) = tool.value(a, b);
            flag(output, accepted);
            real(output, c);
        }
        Ok(None) => {}
        Err(error) => replay_print!(output, "{} ", error.exception_type()),
    }
    end(output);
}
