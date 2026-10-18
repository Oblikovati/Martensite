/// The probe criterion: 2 * max error + 1e-4 * coefficient count, satisfied under the bound.
struct Criterion {
    max: f64,
    kind: AdvApp2VarCriterionType,
    repartition: AdvApp2VarCriterionRepartition,
}

impl AdvApp2VarCriterion for Criterion {
    fn value(&self, p: &AdvApp2VarPatch, _c: &AdvApp2VarContext) -> Result<(), Failure> {
        let errors = p
            .max_errors()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "no errors"))?;
        let count = f64::from(p.nb_coeff_in_u()) * f64::from(p.nb_coeff_in_v());
        p.set_crit_value(2. * *errors.value(1).expect("one subspace") + 1.0e-4 * count);
        Ok(())
    }
    fn is_satisfied(&self, p: &AdvApp2VarPatch) -> bool {
        p.crit_value() < self.max
    }
    fn max_value(&self) -> f64 {
        self.max
    }
    fn criterion_type(&self) -> AdvApp2VarCriterionType {
        self.kind
    }
    fn repartition(&self) -> AdvApp2VarCriterionRepartition {
        self.repartition
    }
}

fn print_patch(
    transcript: &Output,
    p: &AdvApp2VarPatch,
    c: &AdvApp2VarContext,
    crit: &Criterion,
) -> Result<(), Failure> {
    flag(transcript, p.is_discretised());
    flag(transcript, p.is_approximated());
    flag(transcript, p.has_result());
    int(transcript, p.cut_sense());
    int(transcript, p.cut_sense_with(crit, 3));
    int(transcript, p.nb_coeff_in_u());
    int(transcript, p.nb_coeff_in_v());
    if !p.has_result() {
        return Ok(());
    }
    harray(transcript, p.max_errors().as_ref());
    harray(transcript, p.average_errors().as_ref());
    harray2(transcript, p.iso_errors().as_ref());
    head(
        transcript,
        &p.coefficients(1, c)?.expect("equation"),
        c.u_limit() * c.v_limit() * 3,
    );
    let poles = p.poles(1, c)?;
    int(transcript, poles.col_length());
    int(transcript, poles.row_length());
    for i in poles.lower_row()..=poles.upper_row() {
        for j in poles.lower_col()..=poles.upper_col() {
            pnt(transcript, *poles.value(i, j).expect("in range"));
        }
    }
    attempt(transcript, || p.coefficients(2, c).map(|_| ()));
    Ok(())
}

fn approximate_isos(
    transcript: &Output,
    g: &mut Grid,
    c: &AdvApp2VarContext,
    f: &Function,
    d: [f64; 4],
    orders: [i32; 2],
) -> Result<bool, Failure> {
    let (n1, n2) = (
        AdvApp2VarNode::with_orders(orders[0], orders[1]),
        AdvApp2VarNode::with_orders(orders[0], orders[1]),
    );
    while let Some((iso, i1, i2)) = g.frame.first_not_approx() {
        let (a, b) = (
            g.frame.first_node(iso.iso_type(), i1, i2),
            g.frame.last_node(iso.iso_type(), i1, i2),
        );
        n1.assign(g.frame.node(a)?);
        n2.assign(g.frame.node(b)?);
        iso.make_approx(c, d[0], d[1], d[2], d[3], f, &n1, &n2)?;
        print_iso(transcript, &iso);
        int(transcript, iso.nb_coeff());
        // An iso without result leaves null tables that Discretise dereferences natively.
        if !iso.has_result() {
            int(transcript, -9);
            return Ok(false);
        }
        harray(transcript, iso.polynom().as_ref());
        harray(transcript, iso.som_tab().as_ref());
        harray(transcript, iso.dif_tab().as_ref());
        harray2(transcript, iso.max_errors().as_ref());
        harray2(transcript, iso.moy_errors().as_ref());
        iso.overwrite_approx();
        g.frame.change_iso(i1, i2, &iso)?;
        g.frame.node(a)?.assign(&n1);
        g.frame.node(b)?.assign(&n2);
        for u in 0..=orders[0] {
            for v in 0..=orders[1] {
                pnt(transcript, n1.point(u, v)?);
                real(transcript, n1.error(u, v)?);
                pnt(transcript, n2.point(u, v)?);
            }
        }
    }
    Ok(true)
}

fn patch_body(transcript: &Output, r: &mut Reader) -> Result<(), Failure> {
    let func = Function(r.integer());
    let (f, iu, iv, nu, nv, pr) = (
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let d = [r.real(), r.real(), r.real(), r.real()];
    let numdec = r.integer();
    let crit = Criterion {
        max: r.real(),
        kind: AdvApp2VarCriterionType::Relative,
        repartition: AdvApp2VarCriterionRepartition::Incremental,
    };
    let c = context_3d(f, [iu, iv], [nu, nv], pr)?;
    let mut g = Grid::new(iu, iv, d)?;
    g.cut(r)?;
    if !approximate_isos(transcript, &mut g, &c, &func, d, [iu, iv])? {
        return Ok(());
    }
    for i in 1..=g.net.nb_patch() {
        let p = g.net.change_patch(i)?;
        p.discretise(&c, &g.frame, &func)?;
        if p.is_discretised() {
            p.make_approx(&c, &g.frame, numdec)?;
            if p.has_result() {
                crit.value(p, &c)?;
                real(transcript, p.crit_value());
            }
        }
        print_patch(transcript, p, &c, &crit)?;
    }
    Ok(())
}

/// aa_patch: function ifav iu iv ndu ndv iprecis u0 u1 v0 v1 numdec critmax cuts.
pub fn patch_case(transcript: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(transcript, || patch_body(transcript, &mut r));
    end(transcript);
}
