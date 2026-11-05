//! BRepFillPipe probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::{failed, indexed, list};
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::tp_shapes;
use martensite_tkbool::brep_fill::BRepFillPipe;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_fill::GeomFillTrihedron;

/// The trihedron of a `GeomFillTrihedron` value.
fn trihedron_of(value: i32) -> GeomFillTrihedron {
    use GeomFillTrihedron::*;
    [
        IsCorrectedFrenet,
        IsFixed,
        IsFrenet,
        IsConstantNormal,
        IsDarboux,
        IsGuideAC,
        IsGuidePlan,
        IsGuideACWithContact,
        IsGuidePlanWithContact,
        IsDiscreteTrihedron,
    ]
    .get(value as usize)
    .copied()
    .unwrap_or_default()
}

/// The sub-shapes of a kind in TopExpExplorer order.
fn explored(s: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    Explorer::new(s, kind, ShapeEnum::Shape)
        .map(|e| e.collect())
        .unwrap_or_default()
}

/// The queries of the built pipe.
fn pipe_queries(
    output: &BufferedTokens,
    pipe: &mut BRepFillPipe,
    spine: &Shape,
    profile: &Shape,
    all: &[Shape],
    (point, tables): (martensite_tkmath::gp::Point3, bool),
) {
    output.attempt(|| shape(output, pipe.shape()));
    output.attempt(|| {
        output.real(pipe.error_on_surface());
        Ok(())
    });
    output.attempt(|| shape(output, pipe.first_shape()));
    output.attempt(|| shape(output, pipe.last_shape()));
    let mut map = IndexedMapOfShape::default();
    if !pipe.shape().is_null() {
        topexp::map_shapes(pipe.shape(), &mut map, true, true);
    }
    for s in all {
        output.attempt(|| list(output, &map, &pipe.generated(s)));
    }
    let table_spine: Vec<Shape> = if tables {
        explored(spine, ShapeEnum::Edge)
    } else {
        Vec::new()
    };
    for es in table_spine {
        for ep in explored(profile, ShapeEnum::Edge) {
            output.attempt(|| {
                indexed(
                    output,
                    &map,
                    pipe.face(&topods::edge(&es)?, &topods::edge(&ep)?)?
                        .as_ref(),
                )
            });
        }
        for ev in explored(profile, ShapeEnum::Vertex) {
            output.attempt(|| {
                indexed(
                    output,
                    &map,
                    pipe.edge(&topods::edge(&es)?, &topods::vertex(&ev)?)?
                        .as_ref(),
                )
            });
        }
    }
    let section_spine: Vec<Shape> = if tables {
        explored(spine, ShapeEnum::Vertex)
    } else {
        Vec::new()
    };
    for ev in section_spine {
        output.attempt(|| shape(output, &pipe.section(&topods::vertex(&ev)?)?));
    }
    output.attempt(|| shape(output, pipe.pipe_line(point)?.as_ref()));
}

/// `bf_pipe program spine profile mode c1 x y z tables`.
pub fn pipe_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let built =
        (|| -> Result<(Shape, Shape, BRepFillPipe, (martensite_tkmath::gp::Point3, bool)), Failure> {
            let spine = at(&t, r.integer())?;
            let profile = at(&t, r.integer())?;
            let mode = trihedron_of(r.integer());
            let c1 = r.flag();
            let point = r.pnt();
            let tables = r.flag();
            let pipe =
                BRepFillPipe::with_profile(&topods::wire(&spine)?, &profile, mode, c1, false)?;
            Ok((spine, profile, pipe, (point, tables)))
        })();
    match built {
        Ok((spine, profile, mut pipe, point)) => {
            pipe_queries(output, &mut pipe, &spine, &profile, &t.shapes, point)
        }
        Err(error) => return failed(output, error),
    }
    output.end();
}
