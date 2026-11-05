//! TopOpeBRepBuildTools probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_same_index, tp_shapes};
use martensite_tkbool::top_ope_brep_build::{DataMapOfShapeState, TopOpeBRepBuildTools};
use martensite_tkbool::top_ope_brep_ds::MapOfShapeWithState;
use martensite_tkbool::top_ope_brep_tool::TopOpeBRepToolShapeClassifier;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkmath::gp::Vector3;

/// The program of a request, `None` (printed) when it fails.
fn program(output: &BufferedTokens, r: &mut Reader<'_>) -> Option<TpShapes> {
    tp_shapes(r).map_err(|value| failed(output, value)).ok()
}

/// A vector as its coordinates.
fn vec(output: &BufferedTokens, v: Vector3) {
    output.xyz(*v.xyz());
}

/// `tb_tools_geom program face edge edge2`.
pub fn geom_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (fi, ei, e2i) = (r.integer(), r.integer(), r.integer());
    let shapes = (|| Ok::<_, Failure>((at(&t, fi)?, at(&t, ei)?, at(&t, e2i)?)))();
    let (f, e, e2) = match shapes {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    output.attempt(|| {
        vec(
            output,
            TopOpeBRepBuildTools::get_normal_to_face_on_edge(
                &topods::face(&f)?,
                &topods::edge(&e)?,
            )?,
        );
        Ok(())
    });
    output.attempt(|| {
        vec(
            output,
            TopOpeBRepBuildTools::get_normal_in_nearest_point(
                &topods::face(&f)?,
                &topods::edge(&e)?,
            )?,
        );
        Ok(())
    });
    output.attempt(|| {
        let (ok, v) = TopOpeBRepBuildTools::get_tangent_to_edge(&topods::edge(&e)?)?;
        output.flag(ok);
        vec(output, v);
        Ok(())
    });
    output.attempt(|| {
        let (ok, v) = TopOpeBRepBuildTools::get_tangent_to_edge_edge(
            &topods::face(&f)?,
            &topods::edge(&e)?,
            &topods::edge(&e2)?,
        )?;
        output.flag(ok);
        vec(output, v);
        Ok(())
    });
    output.attempt(|| {
        output.flag(TopOpeBRepBuildTools::is_deg_edges_the_same(&e, &e2)?);
        Ok(())
    });
    output.attempt(|| {
        output.flag(TopOpeBRepBuildTools::check_face_closed2d(&topods::face(
            &f,
        )?)?);
        Ok(())
    });
    output.attempt(|| shape(output, &TopOpeBRepBuildTools::normalize_face(&f)?));
    output.end();
}

/// `bl_tb_dump_sws`.
fn dump_sws(output: &BufferedTokens, t: &TpShapes, m: &MapOfShapeWithState) -> Result<(), Failure> {
    output.int(m.extent() as i32);
    for i in 1..=m.extent() {
        output.int(tp_same_index(t, m.find_key(i)?));
        let sws = m.find_from_index(i)?;
        output.int(sws.state() as i32);
        output.flag(sws.is_splitted());
    }
    Ok(())
}

/// The PropagateState part of `tb_tools_states`.
fn propagate(
    output: &BufferedTokens,
    t: &TpShapes,
    s: &Shape,
    rf: &Shape,
    avoid: &MapOfShape,
) -> Result<(), Failure> {
    let mut sc = TopOpeBRepToolShapeClassifier::new();
    sc.set_reference(rf)?;
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(s, ShapeEnum::Face, &mut faces)?;
    let mut spl = DataMapOfShapeState::default();
    spl.bind(faces.find_key(1)?.clone(), State::In);
    let mut rest = IndexedMapOfShape::default();
    for f in faces.iter().skip(1) {
        rest.add(f.clone());
    }
    let mut m = MapOfShapeWithState::default();
    TopOpeBRepBuildTools::propagate_state(
        &spl,
        &rest,
        ShapeEnum::Edge,
        ShapeEnum::Face,
        &mut sc,
        &mut m,
        avoid,
    )?;
    TopOpeBRepBuildTools::propagate_state_for_wires(&rest, &mut m)?;
    dump_sws(output, t, &m)
}

/// `tb_tools_states program shape ref avoid`.
pub fn states_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (si, ri, av) = (r.integer(), r.integer(), r.integer());
    let shapes = (|| {
        Ok::<_, Failure>((
            at(&t, si)?,
            at(&t, ri)?,
            if av >= 0 { Some(at(&t, av)?) } else { None },
        ))
    })();
    let (s, rf, avs) = match shapes {
        Ok(x) => x,
        Err(error) => return failed(output, error),
    };
    let mut avoid = MapOfShape::default();
    if let Some(a) = avs {
        avoid.add(a);
    }
    output.attempt(|| {
        let mut sc = TopOpeBRepToolShapeClassifier::new();
        sc.set_reference(&rf)?;
        let mut m = MapOfShapeWithState::default();
        output.int(
            TopOpeBRepBuildTools::find_state_through_vertex(&s, &mut sc, &mut m, &avoid)? as i32,
        );
        dump_sws(output, &t, &m)
    });
    output.attempt(|| propagate(output, &t, &s, &rf, &avoid));
    output.end();
}

/// `tb_tools_tolerances program shape maxtol`.
pub fn tolerances_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (si, maxtol) = (r.integer(), r.real());
    let s = match at(&t, si) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    output.attempt(|| TopOpeBRepBuildTools::correct_tolerances(&s, maxtol));
    let (mut vs, mut es) = (IndexedMapOfShape::default(), IndexedMapOfShape::default());
    let maps = topexp::map_shapes_of_type(&s, ShapeEnum::Vertex, &mut vs)
        .and_then(|()| topexp::map_shapes_of_type(&s, ShapeEnum::Edge, &mut es));
    if let Err(error) = maps {
        return failed(output, error);
    }
    output.int(vs.extent() as i32);
    for v in vs.iter() {
        output.real(
            topods::vertex(v)
                .and_then(|v| BRepTool::vertex_tolerance(&v))
                .unwrap_or(f64::NAN),
        );
    }
    output.int(es.extent() as i32);
    for e in es.iter() {
        output.real(
            topods::edge(e)
                .and_then(|e| BRepTool::edge_tolerance(&e))
                .unwrap_or(f64::NAN),
        );
    }
    output.end();
}
