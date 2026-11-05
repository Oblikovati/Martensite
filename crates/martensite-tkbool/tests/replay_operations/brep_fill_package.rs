//! BRepFill package probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill;
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Vector3;

/// The program of a request, or its failure printed and the line ended.
fn program(output: &BufferedTokens, r: &mut Reader<'_>) -> Option<TpShapes> {
    match tp_shapes(r) {
        Ok(t) => Some(t),
        Err(error) => {
            output.emit(format_args!("{} ", error.exception_type()));
            output.end();
            None
        }
    }
}

/// The shape of the program at a table index.
fn at(t: &TpShapes, index: i32) -> Result<Shape, Failure> {
    t.shapes
        .get(index as usize)
        .cloned()
        .ok_or_else(|| Failure::out_of_range("replay_operations: shape index"))
}

/// `bf_face program e1 e2`.
pub fn face_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (e1, e2) = (r.integer(), r.integer());
    output.attempt(|| {
        let face =
            brep_fill::ruled_face(&topods::edge(&at(&t, e1)?)?, &topods::edge(&at(&t, e2)?)?)?;
        shape(output, &face)
    });
    output.end();
}

/// `bf_shell program w1 w2`.
pub fn shell_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (w1, w2) = (r.integer(), r.integer());
    output.attempt(|| {
        let shell =
            brep_fill::ruled_shell(&topods::wire(&at(&t, w1)?)?, &topods::wire(&at(&t, w2)?)?)?;
        shape(output, &shell)
    });
    output.end();
}

/// `bf_axe program spine profile tol`.
pub fn axe_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (spine, profile, tol) = (r.integer(), r.integer(), r.real());
    output.attempt(|| {
        let (axe, on) = brep_fill::axe(&at(&t, spine)?, &topods::wire(&at(&t, profile)?)?, tol)?;
        output.pnt(axe.location());
        output.dir(axe.direction());
        output.dir(axe.x_direction());
        output.dir(axe.y_direction());
        output.flag(on);
        Ok(())
    });
    output.end();
}

/// `bf_search_origin program wire P Dir tol`.
pub fn search_origin_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let w = r.integer();
    let p = r.pnt();
    let xyz = r.xyz();
    let d = Vector3::new(xyz.x(), xyz.y(), xyz.z());
    let tol = r.real();
    output.attempt(|| {
        let mut wire = topods::wire(&at(&t, w)?)?;
        brep_fill::search_origin(&mut wire, p, d, tol)?;
        shape(output, &wire)
    });
    output.end();
}

/// `bf_compute_acr program wire`.
pub fn compute_acr_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let w = r.integer();
    output.attempt(|| {
        let wire = topods::wire(&at(&t, w)?)?;
        let mut n: i32 = 0;
        let mut explorer = WireExplorer::with_wire(&wire)?;
        while explorer.more() {
            n += 1;
            explorer.next()?;
        }
        output.int(n);
        let mut acr: Array1<'static, f64> =
            Array1::with_bounds(0, n).map_err(|_| Failure::out_of_range("acr"))?;
        brep_fill::compute_acr(&wire, &mut acr)?;
        for value in acr.iter() {
            output.real(*value);
        }
        Ok(())
    });
    output.end();
}

/// `bf_insert_acr program wire n (cut)... prec`.
pub fn insert_acr_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let w = r.integer();
    let n = r.integer();
    let mut cuts: Array1<'static, f64> = Array1::with_bounds(1, n).unwrap_or_default();
    for i in 1..=n {
        let _ = cuts.set_value(i, r.real());
    }
    let prec = r.real();
    output.attempt(|| {
        shape(
            output,
            &brep_fill::insert_acr(&topods::wire(&at(&t, w)?)?, &cuts, prec)?.into_shape(),
        )
    });
    output.end();
}
