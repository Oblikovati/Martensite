//! BRepFillSectionLaw probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_lib_common::type_name;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shape_index, tp_shapes};
use martensite_tkbool::brep_fill::{BRepFillNSections, BRepFillSectionLaw, BRepFillShapeLaw};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::arrays::one_based;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::law::{LawFunction, LawLinear};
use martensite_tkmath::gp::Point3;

/// `bl_built_law_queries`.
fn built_law_queries(output: &BufferedTokens, law: &dyn BRepFillSectionLaw, param: f64) {
    let mut n: i32 = 0;
    output.attempt(|| {
        n = law.nb_law()?;
        output.int(n);
        Ok(())
    });
    for i in 1..=n + 1 {
        output.attempt(|| shape(output, &law.vertex(i, param)?.into_shape()));
    }
    for i in 0..=n {
        output.attempt(|| {
            output.real(law.vertex_tol(i, param)?);
            Ok(())
        });
        output.attempt(|| {
            output.int(law.continuity(i, 1.0e-2)? as i32);
            Ok(())
        });
    }
    output.attempt(|| {
        let c = law.concatened_law()?;
        type_name(output, &c);
        let Some(c) = c.get() else { return Ok(()) };
        let (np, nk, dg) = c.section_shape()?;
        output.int(np);
        output.int(nk);
        output.int(dg);
        let mut poles = one_based::<Point3>(np)?;
        let mut weights = one_based::<f64>(np)?;
        c.d0(param, &mut poles, &mut weights)?;
        for (p, w) in poles.iter().zip(weights.iter()) {
            output.pnt(*p);
            output.real(*w);
        }
        Ok(())
    });
}

/// `bl_section_law_queries`.
fn section_law_queries(
    output: &BufferedTokens,
    law: &dyn BRepFillSectionLaw,
    t: &TpShapes,
    build: bool,
    param: f64,
) {
    output.flag(law.is_vertex());
    output.flag(law.is_constant());
    output.flag(law.is_u_closed());
    output.flag(law.is_v_closed());
    output.flag(law.is_done());
    if build && law.is_done() {
        built_law_queries(output, law, param);
    }
    if law.is_done() {
        output.attempt(|| shape(output, &law.d0(param)?));
    }
    for s in &t.shapes {
        if s.is_null() || s.shape_type().ok() != Some(ShapeEnum::Edge) {
            continue;
        }
        output.attempt(|| {
            output.int(law.index_of_edge(s)?);
            Ok(())
        });
    }
}

/// The shape of the program at a table index.
fn at(t: &TpShapes, index: i32) -> Result<Shape, Failure> {
    t.shapes
        .get(index as usize)
        .cloned()
        .ok_or_else(|| Failure::out_of_range("replay_operations: shape index"))
}

/// The law of `bf_shape_law`.
fn shape_law(
    s: &Shape,
    build: bool,
    lin: &Handle<dyn LawFunction>,
) -> Result<BRepFillShapeLaw, Failure> {
    if s.shape_type()? == ShapeEnum::Vertex {
        return BRepFillShapeLaw::with_vertex(&topods::vertex(s)?, build);
    }
    if !lin.is_null() {
        return BRepFillShapeLaw::with_wire_and_law(&topods::wire(s)?, lin, build);
    }
    BRepFillShapeLaw::with_wire(&topods::wire(s)?, build)
}

/// `bf_shape_law program index build haslaw [pdeb valdeb pfin valfin] param`.
pub fn shape_law_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let index = r.integer();
    let (build, haslaw) = (r.flag(), r.flag());
    let mut lin: Handle<dyn LawFunction> = Handle::default();
    if haslaw {
        let law = LawLinear::new();
        let (a, b, c, d) = (r.real(), r.real(), r.real(), r.real());
        law.set(a, b, c, d);
        lin = Handle::new(law).cast::<dyn LawFunction>();
    }
    let param = r.real();
    let s = match at(&t, index) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let law = match shape_law(&s, build, &lin) {
        Ok(law) => law,
        Err(error) => return failed(output, error),
    };
    section_law_queries(output, &law, &t, build, param);
    if s.shape_type().ok() == Some(ShapeEnum::Wire) {
        let _ = topods::wire(&s).and_then(|w| law.init(&w));
        for _ in 0..=t.shapes.len() {
            let mut null = true;
            output.attempt(|| {
                let e = law.current_edge()?;
                output.int(tp_shape_index(&t, &e));
                null = e.is_null();
                Ok(())
            });
            if null {
                break;
            }
        }
    }
    output.end();
}

/// `bf_nsections program n (wire)... build param`.
pub fn nsections_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let n = r.integer();
    let mut sections = Vec::new();
    for _ in 0..n {
        match at(&t, r.integer()) {
            Ok(s) => sections.push(s),
            Err(error) => return failed(output, error),
        }
    }
    let (build, param) = (r.flag(), r.real());
    let law = match BRepFillNSections::new(&sections, build) {
        Ok(law) => law,
        Err(error) => return failed(output, error),
    };
    section_law_queries(output, &law, &t, build, param);
    output.end();
}
