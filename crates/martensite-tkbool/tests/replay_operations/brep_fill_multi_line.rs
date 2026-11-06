//! BRepFillMultiLine, BRepFillApproxSeewing and BRepFillTrimSurfaceTool probe
//! operations, the Rust twin of the recorded capture (the
//! records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::{
    BRepFillApproxSeewing, BRepFillMultiLine, BRepFillTrimSurfaceTool, MultiLineCurves,
};
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkernel::ncollection::Sequence;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeombase::app_cont::Function;
use martensite_tkmath::gp::Point3;

/// `bl_curve3d`.
pub fn curve3d(output: &BufferedTokens, c: &Handle<dyn GeomCurve>) -> Result<(), Failure> {
    let Some(curve) = c.get() else {
        output.int(-1);
        return Ok(());
    };
    let (f, l) = (curve.first_parameter(), curve.last_parameter());
    output.real(f);
    output.real(l);
    for u in [f, 0.5 * (f + l), l] {
        output.pnt(curve.value(u)?);
    }
    Ok(())
}

/// `bl_curve2d`.
pub fn curve2d(output: &BufferedTokens, c: &Handle<dyn Geom2dCurve>) -> Result<(), Failure> {
    let Some(curve) = c.get() else {
        output.int(-1);
        return Ok(());
    };
    let (f, l) = (curve.first_parameter(), curve.last_parameter());
    output.real(f);
    output.real(l);
    for u in [f, 0.5 * (f + l), l] {
        output.pnt2d(curve.value(u)?);
    }
    Ok(())
}

/// The three curves of a multi-line.
fn curves(output: &BufferedTokens, c: &MultiLineCurves) -> Result<(), Failure> {
    curve3d(output, &c.0)?;
    curve2d(output, &c.1)?;
    curve2d(output, &c.2)
}

/// The faces and edges of a request.
fn faces_and_edges(r: &mut Reader<'_>, t: &TpShapes) -> Result<(Face, Face, Edge, Edge), Failure> {
    let f1 = topods::face(&at(t, r.integer())?)?;
    let f2 = topods::face(&at(t, r.integer())?)?;
    let e1 = topods::edge(&at(t, r.integer())?)?;
    let e2 = topods::edge(&at(t, r.integer())?)?;
    Ok((f1, f2, e1, e2))
}

/// The p-curve of a program index.
fn pcurve_at(t: &TpShapes, index: i32) -> Handle<dyn Geom2dCurve> {
    t.pcurves.get(index as usize).cloned().unwrap_or_default()
}

/// The points, curves and approximation of a multi-line.
fn print_multi_line(output: &BufferedTokens, ml: &BRepFillMultiLine, n: i32) {
    output.flag(ml.is_particular_case());
    output.int(ml.continuity() as i32);
    let (u1, u2) = (ml.first_parameter(), ml.last_parameter());
    output.real(u1);
    output.real(u2);
    for i in 0..n {
        let u = u1 + f64::from(i) * (u2 - u1) / f64::from(n - 1);
        output.attempt(|| {
            output.pnt2d(ml.value_on_f1(u)?);
            output.pnt2d(ml.value_on_f2(u)?);
            output.pnt(ml.value_3d(u)?);
            Ok(())
        });
    }
    output.attempt(|| curves(output, &ml.curves()?));
    output.attempt(|| {
        let app = BRepFillApproxSeewing::with_multi_line(ml)?;
        output.flag(app.is_done());
        curve3d(output, app.curve()?)?;
        curve2d(output, app.curve_on_f1()?)?;
        curve2d(output, app.curve_on_f2()?)
    });
}

/// `bf_multi_line program f1 f2 e1 e2 inv1 inv2 bis n`.
pub fn multi_line_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let shapes = faces_and_edges(&mut r, &t);
    let (inv1, inv2) = (r.flag(), r.flag());
    let bis = pcurve_at(&t, r.integer());
    let n = r.integer();
    let ml = shapes
        .and_then(|(f1, f2, e1, e2)| BRepFillMultiLine::new(&f1, &f2, &e1, &e2, inv1, inv2, &bis));
    match ml {
        Ok(ml) => print_multi_line(output, &ml, n),
        Err(error) => return failed(output, error),
    }
    output.end();
}

/// `bf_trim_surface program bis f1 f2 e1 e2 inv1 inv2 ef1 ef2 x y pe u1 u2`.
pub fn trim_surface_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let bis = pcurve_at(&t, r.integer());
    let shapes = faces_and_edges(&mut r, &t);
    let (inv1, inv2) = (r.flag(), r.flag());
    let ef = (at(&t, r.integer()), at(&t, r.integer()));
    let p = r.pnt2d();
    let pe = at(&t, r.integer());
    let (u1, u2) = (r.real(), r.real());
    let Ok((f1, f2, e1, e2)) = shapes else {
        return output.end();
    };
    let tool = BRepFillTrimSurfaceTool::new(&bis, &f1, &f2, &e1, &e2, inv1, inv2);
    output.attempt(|| {
        let mut pts: Sequence<Point3> = Sequence::default();
        tool.intersect_with(
            &topods::edge(&ef.0.clone()?)?,
            &topods::edge(&ef.1.clone()?)?,
            &mut pts,
        )?;
        output.int(pts.len() as i32);
        for q in pts.iter() {
            output.pnt(*q);
        }
        Ok(())
    });
    output.attempt(|| tool.is_on_face(p).map(|value| output.flag(value)));
    output.attempt(|| {
        tool.proj_on(p, &topods::edge(&pe.clone()?)?)
            .map(|value| output.real(value))
    });
    output.attempt(|| {
        let (c, cont) = tool.project(u1, u2)?;
        output.int(cont as i32);
        curves(output, &c)
    });
    output.end();
}
