//! TopOpeBRepBuild face chain probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_same_index, tp_shapes};
use martensite_tkbool::top_ope_brep_build::{
    ShapeToShapeMap, TopOpeBRepBuildBlockBuilder, TopOpeBRepBuildBuilder,
    TopOpeBRepBuildCompositeClassifier, TopOpeBRepBuildFaceBuilder, TopOpeBRepBuildShapeSet,
    TopOpeBRepBuildWireEdgeClassifier, TopOpeBRepBuildWireEdgeSet, TopOpeBRepBuildWireToFace,
};
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbool::top_ope_brep_tool::TopOpeBRepToolOutCurveType;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{IndexedMap, List};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{ShapeEnum, State};

/// The program of a request, `None` (printed) when it fails.
fn program(output: &BufferedTokens, r: &mut Reader<'_>) -> Option<TpShapes> {
    tp_shapes(r).map_err(|value| failed(output, value)).ok()
}

/// A state as its TopAbsState value.
fn put_state(output: &BufferedTokens, st: State) -> Result<(), Failure> {
    output.int(st as i32);
    Ok(())
}

/// The count and each face of a list.
fn faces(output: &BufferedTokens, lf: &List<Shape>) -> Result<(), Failure> {
    output.int(lf.len() as i32);
    lf.iter()
        .try_for_each(|shape_value| shape(output, shape_value))
}

/// `tb_wire_to_face program face n wire...`.
pub fn wire_to_face_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let face_index = r.integer();
    let n = r.integer();
    let wires: Vec<i32> = (0..n).map(|_| r.integer()).collect();
    output.attempt(|| {
        let mut wtf = TopOpeBRepBuildWireToFace::new();
        for w in &wires {
            wtf.add_wire(&topods::wire(&at(&t, *w)?)?);
        }
        let mut lf = List::default();
        wtf.make_faces(&topods::face(&at(&t, face_index)?)?, &mut lf)?;
        faces(output, &lf)
    });
    output.end();
}

/// The edges of the current new wire of `bl_tb_dump_face_builder`.
fn dump_new_wire(
    output: &BufferedTokens,
    fb: &mut TopOpeBRepBuildFaceBuilder,
    t: &TpShapes,
) -> Result<(), Failure> {
    output.int(fb.init_edge()?);
    while fb.more_edge() {
        output.int(tp_same_index(t, fb.edge()?));
        output.int(fb.edge()?.orientation() as i32);
        fb.next_edge()?;
    }
    output.int(-3);
    Ok(())
}

/// `bl_tb_dump_face_builder`.
fn dump_face_builder(
    output: &BufferedTokens,
    fb: &mut TopOpeBRepBuildFaceBuilder,
    t: &TpShapes,
) -> Result<(), Failure> {
    output.int(fb.init_face());
    while fb.more_face() {
        output.int(fb.init_wire());
        while fb.more_wire() {
            let old = fb.is_old_wire()?;
            output.flag(old);
            if old {
                output.int(tp_same_index(t, &fb.old_wire()?));
            } else {
                dump_new_wire(output, fb, t)?;
            }
            fb.next_wire();
        }
        fb.next_face();
    }
    Ok(())
}

/// The wire edge set of `tb_face_builder`.
fn wire_edge_set(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
    f: &Shape,
) -> Result<TopOpeBRepBuildWireEdgeSet, Failure> {
    let mut wes = TopOpeBRepBuildWireEdgeSet::new(f)?;
    let n = r.integer();
    for _ in 0..n {
        let kind = r.integer();
        let s = at(t, r.integer())?;
        output.attempt(|| match kind {
            0 => wes.add_shape(&s),
            1 => wes.add_start_element(&s),
            _ => wes.add_element(&s),
        });
    }
    Ok(wes)
}

/// The detections of `tb_face_builder`.
fn detections(output: &BufferedTokens, fb: &mut TopOpeBRepBuildFaceBuilder, t: &TpShapes) {
    output.attempt(|| {
        let mut me: IndexedMap<Shape, ShapeMapHasher> = IndexedMap::default();
        fb.detect_pseudo_internal_edge(&mut me)?;
        output.int(me.len() as i32);
        (1..=me.len()).try_for_each(|i| {
            output.int(tp_same_index(t, me.find_key(i)?));
            Ok(())
        })
    });
    output.attempt(|| {
        let (mut vv, mut v1e) = (ShapeToShapeMap::default(), ShapeToShapeMap::default());
        fb.detect_unclosed_wire(&mut vv, &mut v1e)?;
        output.int(vv.len() as i32);
        output.int(v1e.len() as i32);
        Ok(())
    });
}

/// `tb_face_builder program face force n (kind shape)...`.
pub fn face_builder_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let f = match at(&t, r.integer()) {
        Ok(f) => f,
        Err(error) => return failed(output, error),
    };
    let force = r.flag();
    let mut fb = TopOpeBRepBuildFaceBuilder::new();
    let built = wire_edge_set(output, &mut r, &t, &f)
        .and_then(|mut wes| fb.init_face_builder(&mut wes, &f, force));
    if let Err(error) = built {
        return failed(output, error);
    }
    output.attempt(|| dump_face_builder(output, &mut fb, &t));
    detections(output, &mut fb, &t);
    output.attempt(|| dump_face_builder(output, &mut fb, &t));
    output.attempt(|| {
        let bt = TopOpeBRepDSBuildTool::with_type(TopOpeBRepToolOutCurveType::Approx);
        let mut lf = List::default();
        TopOpeBRepBuildBuilder::new(&bt).make_faces(&f, &mut fb, &mut lf)?;
        faces(output, &lf)
    });
    output.end();
}

/// The edges of a shape.
fn edges_of(s: &Shape) -> Result<Vec<Shape>, Failure> {
    Ok(Explorer::new(s, ShapeEnum::Edge, ShapeEnum::Shape)?.collect())
}

/// `tb_wec program face s1 s2`.
pub fn wec_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(t) = program(output, &mut r) else {
        return;
    };
    let (fi, i1, i2) = (r.integer(), r.integer(), r.integer());
    let bb = TopOpeBRepBuildBlockBuilder::new();
    let run = || -> Result<(), Failure> {
        let (f, s1, s2) = (at(&t, fi)?, at(&t, i1)?, at(&t, i2)?);
        let mut wec = TopOpeBRepBuildWireEdgeClassifier::new(&f, &bb)?;
        output.attempt(|| put_state(output, wec.compare_shapes(&s1, &s2)?));
        output.attempt(|| {
            let first = edges_of(&s1)?.into_iter().next().ok_or_else(|| {
                Failure::with_type(
                    "Standard_NoSuchObject",
                    "TopExp_Explorer::Current: no edge",
                    "TopExp_Explorer",
                )
            })?;
            put_state(output, wec.compare_element_to_shape(&first, &s2)?)
        });
        output.attempt(|| {
            wec.reset_shape(&s1)?;
            for e in edges_of(&s2)? {
                output.flag(wec.compare_element(&e)?);
            }
            put_state(output, wec.state())
        });
        Ok(())
    };
    output.attempt(run);
    output.end();
}
