//! TopOpeBRepToolSolidClassifier, TopOpeBRepToolShapeClassifier, FSC_* and
//! TOOL::TggeomE probe operations, the Rust twin of
//! the recorded capture (the records are documented
//! there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::top_ope_brep_tool::{
    TopOpeBRepToolShapeClassifier, TopOpeBRepToolSolidClassifier, fsc_get_psc,
    fsc_get_psc_with_reference, fsc_state_eonface, fsc_state_ponface, tggeom_e,
};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Vector3;

/// The program of a request, `None` (printed) when it fails.
fn program(output: &BufferedTokens, r: &mut Reader<'_>) -> Option<TpShapes> {
    tp_shapes(r).map_err(|value| failed(output, value)).ok()
}

/// `tt_solid_classifier program shell solid tol n (x y z)...`.
pub fn solid_classifier_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (ish, iso, tol, n) = (r.integer(), r.integer(), r.real(), r.integer());
    let shapes = at(&t, ish)
        .and_then(|s| topods::shell(&s))
        .and_then(|sh| Ok((sh, topods::solid(&at(&t, iso)?)?)));
    let (sh, so) = match shapes {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let mut sc = TopOpeBRepToolSolidClassifier::new();
    for _ in 0..n {
        let p = r.pnt();
        output.attempt(|| {
            sc.classify_shell(&sh, p, tol)
                .map(|st| output.int(st as i32))
        });
        output.attempt(|| {
            sc.classify_solid(&so, p, tol)
                .map(|st| output.int(st as i32))
        });
        output.int(sc.state() as i32);
    }
    output.end();
}

/// `bl_tt_sc_result`.
fn sc_result(output: &BufferedTokens, sc: &TopOpeBRepToolShapeClassifier, st: State) {
    output.int(st as i32);
    output.int(sc.state() as i32);
    output.attempt(|| sc.p3d().map(|value| output.pnt(value)));
    output.attempt(|| sc.p2d().map(|value| output.pnt2d(value)));
}

/// The classifications of `tt_shape_classifier` once the shapes are read.
fn shape_classifications(
    output: &BufferedTokens,
    s: &Shape,
    rf: &Shape,
    sd: i32,
    avs: Option<&Shape>,
) {
    let mut sc = TopOpeBRepToolShapeClassifier::new();
    output.attempt(|| {
        let st = sc.state_shape_shape(s, rf, sd)?;
        {
            sc_result(output, &sc, st);
            Ok(())
        }
    });
    if let Some(avs) = avs {
        output.attempt(|| {
            let st = sc.state_shape_shape_avoiding(s, avs, rf)?;
            {
                sc_result(output, &sc, st);
                Ok(())
            }
        });
        let mut l = List::default();
        l.append(avs.clone());
        output.attempt(|| {
            let st = sc.state_shape_shape_avoiding_list(s, &l, rf)?;
            {
                sc_result(output, &sc, st);
                Ok(())
            }
        });
    }
    let null = Shape::default();
    output.attempt(|| {
        sc.set_same_domain(sd);
        sc.set_reference(rf)?;
        let st = sc.state_shape_reference(s, avs.unwrap_or(&null))?;
        {
            sc_result(output, &sc, st);
            Ok(())
        }
    });
}

/// `tt_shape_classifier program s ref samedomain avoid`.
pub fn shape_classifier_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (is, ir, sd, av) = (r.integer(), r.integer(), r.integer(), r.integer());
    let read = || -> Result<(Shape, Shape, Option<Shape>), Failure> {
        Ok((
            at(&t, is)?,
            at(&t, ir)?,
            if av >= 0 { Some(at(&t, av)?) } else { None },
        ))
    };
    match read() {
        Ok((s, rf, avs)) => shape_classifications(output, &s, &rf, sd, avs.as_ref()),
        Err(error) => return failed(output, error),
    }
    output.end();
}

/// `tt_fsc program e f t x y z`.
pub fn fsc_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (ie, iface, tt, p) = (r.integer(), r.integer(), r.real(), r.pnt());
    let run = || -> Result<(), Failure> {
        let (e, f) = (at(&t, ie)?, at(&t, iface)?);
        output.attempt(|| {
            fsc_get_psc(|psc| fsc_state_eonface(&e, tt, &f, psc)).map(|st| output.int(st as i32))
        });
        output.attempt(|| {
            fsc_get_psc(|psc| fsc_state_ponface(p, &f, psc)).map(|st| output.int(st as i32))
        });
        output.attempt(|| {
            fsc_get_psc_with_reference(&f, |sc| sc.state_shape_reference(&e, &Shape::default()))?
                .map(|st| output.int(st as i32))
        });
        Ok(())
    };
    output.attempt(run);
    output.end();
}

/// `tt_tggeom program e n par...`.
pub fn tggeom_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (ie, n) = (r.integer(), r.integer());
    let e = match at(&t, ie).and_then(|s| topods::edge(&s)) {
        Ok(e) => e,
        Err(error) => return failed(output, error),
    };
    for _ in 0..n {
        let par = r.real();
        output.attempt(|| {
            let tg = tggeom_e(par, &e)?;
            output.flag(tg.is_some());
            output.xyz(*tg.unwrap_or(Vector3::new(0., 0., 0.)).xyz());
            Ok(())
        });
    }
    output.end();
}
