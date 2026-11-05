//! BRepAlgo package probe operations, the Rust twin of the recorded capture
//! (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_same_index, tp_shapes};
use martensite_tkbool::brep_algo;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::geom_abs::Shape as Continuity;

/// The continuity of its enumeration value (C0..CN).
fn continuity_of(value: i32) -> Continuity {
    match value {
        1 => Continuity::G1,
        2 => Continuity::C1,
        3 => Continuity::G2,
        4 => Continuity::C2,
        5 => Continuity::C3,
        6 => Continuity::CN,
        _ => Continuity::C0,
    }
}

/// The program of a request, `None` (printed) when it fails.
fn program(output: &BufferedTokens, r: &mut Reader<'_>) -> Option<TpShapes> {
    match tp_shapes(r) {
        Ok(t) => Some(t),
        Err(error) => {
            failed(output, error);
            None
        }
    }
}

/// `ba_concat program wire option angtol`.
pub fn concat_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (index, option, tol) = (r.integer(), continuity_of(r.integer()), r.real());
    output.attempt(|| {
        shape(
            output,
            brep_algo::concatenate_wire(&topods::wire(&at(&t, index)?)?, option, tol)?.as_ref(),
        )
    });
    output.end();
}

/// `ba_concat_c0 program wire`.
pub fn concat_c0_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let index = r.integer();
    output.attempt(|| {
        shape(
            output,
            brep_algo::concatenate_wire_c0(&topods::wire(&at(&t, index)?)?)?.as_ref(),
        )
    });
    output.end();
}

/// `ba_convert program face angtol`.
pub fn convert_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (index, tol) = (r.integer(), r.real());
    let face = at(&t, index).and_then(|f| topods::face(&f));
    if let Ok(f) = &face {
        let wires: Vec<Shape> = Explorer::new(f.as_ref(), ShapeEnum::Wire, ShapeEnum::Shape)
            .map(|e| e.collect())
            .unwrap_or_default();
        for w in &wires {
            output.attempt(|| {
                shape(
                    output,
                    brep_algo::convert_wire(&topods::wire(w)?, tol, f)?.as_ref(),
                )
            });
        }
    }
    output.attempt(|| {
        shape(
            output,
            brep_algo::convert_face(&face.clone()?, tol)?.as_ref(),
        )
    });
    output.end();
}

/// `ba_valid program n arg... result closed geomctrl`.
pub fn valid_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let mut args: List<Shape> = List::default();
    for _ in 0..r.integer() {
        if let Ok(s) = at(&t, r.integer()) {
            args.append(s);
        }
    }
    let result = at(&t, r.integer()).unwrap_or_default();
    let (closed, geom) = (r.flag(), r.flag());
    output.attempt(|| brep_algo::is_valid(&result).map(|value| output.flag(value)));
    output.attempt(|| brep_algo::is_topologically_valid(&result).map(|value| output.flag(value)));
    output.attempt(|| {
        brep_algo::is_valid_result(&args, &result, closed, geom).map(|value| output.flag(value))
    });
    output.end();
}

/// `ba_face_restrictor program face proj control n wire...`.
pub fn face_restrictor_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (fi, proj, control, n) = (r.integer(), r.flag(), r.flag(), r.integer());
    let wires: Vec<i32> = (0..n).map(|_| r.integer()).collect();
    let mut fr = martensite_tkbool::brep_algo::BRepAlgoFaceRestrictor::new();
    let mut setup = || -> Result<(), Failure> {
        fr.init(&topods::face(&at(&t, fi)?)?, proj, control);
        for w in &wires {
            fr.add(&topods::wire(&at(&t, *w)?)?);
        }
        Ok(())
    };
    if let Err(error) = setup() {
        return failed(output, error);
    }
    output.attempt(|| fr.perform());
    output.flag(fr.is_done());
    let mut count = 0;
    output.attempt(|| {
        while fr.more() {
            shape(output, fr.current()?.as_ref())?;
            count += 1;
            fr.next()?;
        }
        Ok(())
    });
    output.int(count);
    output.end();
}

/// `bl_shape_list`: `n (index orientation)...` (orientation -1 as is).
fn shape_list(r: &mut Reader<'_>, t: &TpShapes) -> Result<List<Shape>, Failure> {
    let n = r.integer();
    let mut l = List::default();
    for _ in 0..n {
        let (index, o) = (r.integer(), r.integer());
        let s = at(t, index)?;
        l.append(if o >= 0 {
            s.oriented(Orientation::ALL[o as usize])
        } else {
            s
        });
    }
    Ok(l)
}

/// `bl_ba_pieces`.
fn pieces(output: &BufferedTokens, ne: &List<Shape>, t: &TpShapes) -> Result<(), Failure> {
    output.int(ne.len() as i32);
    for s in ne.iter() {
        let e = topods::edge(s)?;
        let (f, l) = martensite_tkbrep::brep::BRepTool::range(&e)?;
        output.real(f);
        output.real(l);
        output.int(s.orientation() as i32);
        let (v1, v2) = martensite_tkbrep::topexp::vertices(&e, false);
        output.int(tp_same_index(t, v1.as_ref()));
        output.int(tp_same_index(t, v2.as_ref()));
    }
    Ok(())
}

/// `ba_cut_edge program face edge list`.
pub fn cut_edge_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let read = |r: &mut Reader<'_>| -> Result<(Shape, Shape, List<Shape>), Failure> {
        let (f, e) = (at(&t, r.integer())?, at(&t, r.integer())?);
        Ok((f, e, shape_list(r, &t)?))
    };
    let (f, e, lv) = match read(&mut r) {
        Ok(x) => x,
        Err(error) => return failed(output, error),
    };
    let mut lp = brep_algo::BRepAlgoLoop::new();
    output.attempt(|| {
        lp.init(&topods::face(&f)?);
        let mut ne = List::default();
        lp.cut_edge(&topods::edge(&e)?, &lv, &mut ne)?;
        pieces(output, &ne, &t)
    });
    output.end();
}

/// The loop of `ba_loop` with its edges read; the cut edges.
fn loop_with_edges(
    r: &mut Reader<'_>,
    t: &TpShapes,
    lp: &mut brep_algo::BRepAlgoLoop,
) -> Result<Vec<Shape>, Failure> {
    lp.init(&topods::face(&at(t, r.integer())?)?);
    let n = r.integer();
    let mut cut = Vec::new();
    for _ in 0..n {
        let e = at(t, r.integer())?;
        let lv = shape_list(r, t)?;
        lp.add_edge(&topods::edge(&e)?, &lv);
        cut.push(e);
    }
    lp.add_const_edges(&shape_list(r, t)?);
    lp.set_tol_conf(r.real());
    Ok(cut)
}

/// The count and each shape of a list.
fn shapes(output: &BufferedTokens, l: &List<Shape>) -> Result<(), Failure> {
    output.int(l.len() as i32);
    l.iter()
        .try_for_each(|shape_value| shape(output, shape_value))
}

/// `ba_loop program face n (edge list)... consts tolconf other`.
pub fn loop_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let mut lp = brep_algo::BRepAlgoLoop::new();
    let cut = match loop_with_edges(&mut r, &t, &mut lp) {
        Ok(cut) => cut,
        Err(error) => return failed(output, error),
    };
    let other = r.integer();
    output.real(lp.get_tol_conf());
    output.attempt(|| lp.perform());
    output.attempt(|| shapes(output, lp.new_wires()));
    output.attempt(|| lp.wires_to_faces());
    output.attempt(|| shapes(output, lp.new_faces()));
    for e in &cut {
        output.attempt(|| pieces(output, lp.new_edges(&topods::edge(e)?)?, &t));
    }
    output.attempt(|| lp.new_edges(&topods::edge(&at(&t, other)?)?).map(|_| ()));
    let mut vv = brep_algo::VertexSubstitutes::default();
    lp.get_vertices_for_substitute(&mut vv);
    output.int(vv.len() as i32);
    output.end();
}
