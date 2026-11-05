//! BRepAlgoNormalProjection and BRepProjProjection probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_algo::BRepAlgoNormalProjection;
use martensite_tkbool::brep_proj::BRepProjProjection;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Point3};

/// `bl_program_index`: the index of a shape among the program shapes, -1 for none.
fn program_index(t: &TpShapes, s: &Shape) -> i32 {
    t.shapes
        .iter()
        .position(|p| p.is_same(s))
        .map_or(-1, |i| i as i32)
}

/// The edges of the projection.
fn projected_edges(s: &Shape) -> Vec<Shape> {
    Explorer::new(s, ShapeEnum::Edge, ShapeEnum::Shape)
        .map(|e| e.collect())
        .unwrap_or_default()
}

/// The projection of the request, its edges added; the edges.
fn projection_of(
    r: &mut Reader<'_>,
    t: &TpShapes,
) -> Result<(BRepAlgoNormalProjection, Vec<Shape>), Failure> {
    let mut np = BRepAlgoNormalProjection::with_shape(&at(t, r.integer())?);
    let mut edges = Vec::new();
    for _ in 0..r.integer() {
        let e = at(t, r.integer())?;
        np.add(&e)?;
        edges.push(e);
    }
    np.set_max_distance(r.real());
    np.compute3d(r.flag());
    np.set_limit(r.flag());
    Ok((np, edges))
}

/// `ba_normal_projection program shape n (edge)... maxdist with3d limit`.
pub fn normal_projection_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let (np, edges) =
        match projection_of(&mut r, &t).and_then(|(mut np, e)| np.build().map(|()| (np, e))) {
            Ok(v) => v,
            Err(error) => return failed(output, error),
        };
    output.flag(np.is_done());
    output.attempt(|| shape(output, np.projection()));
    for e in projected_edges(np.projection()) {
        output.attempt(|| {
            np.ancestor(&topods::edge(&e)?)
                .map(|a| output.int(program_index(&t, a)))
        });
        output.attempt(|| {
            np.couple(&topods::edge(&e)?)
                .map(|c| output.int(program_index(&t, c)))
        });
    }
    for e in &edges {
        output.attempt(|| np.generated(e).map(|g| output.int(g.len() as i32)));
    }
    output.attempt(|| {
        let mut wires: List<Shape> = List::default();
        output.flag(np.build_wire(&mut wires)?);
        output.int(wires.len() as i32);
        Ok(())
    });
    output.end();
}

/// `bp_projection program wire shape mode x y z`.
pub fn projection_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let (wi, si, mode) = (r.integer(), r.integer(), r.integer());
    let (x, y, z) = (r.real(), r.real(), r.real());
    let built = (|| {
        let (w, s) = (at(&t, wi)?, at(&t, si)?);
        if mode == 0 {
            BRepProjProjection::along(&w, &s, Direction3::new(x, y, z)?)
        } else {
            BRepProjProjection::from_point(&w, &s, Point3::new(x, y, z))
        }
    })();
    let mut p = match built {
        Ok(p) => p,
        Err(error) => return failed(output, error),
    };
    output.flag(p.is_done());
    output.attempt(|| shape(output, p.shape().as_ref()));
    let mut n = 0;
    p.init();
    while p.more() {
        n += 1;
        p.next();
    }
    output.int(n);
    p.init();
    while p.more() {
        output.attempt(|| shape(output, p.current()?.as_ref()));
        p.next();
    }
    output.end();
}
