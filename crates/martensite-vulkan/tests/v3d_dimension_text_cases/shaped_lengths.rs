//! Lengths between a face and a point, edge or arc (`bug26035_2`, `bug26056`).
use super::prelude::*;
use crate::primitive_shapes::box_at;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape as _,
};

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn direction([x, y, z]: [f64; 3]) -> Result<Direction3, Failure> {
    Direction3::new(x, y, z)
}

/// `bug26035_2`: a length between a box face and a point on the ZOX plane is refused, and
/// the face alone is dumped.
/// Example: `face_point_length()?`.
pub(crate) fn face_point_length() -> Outcome {
    dimension_banner(&[
        "CR26035: E x c e p t i o n when trying to draw dimension between face and point",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let face = face_and_point(&mut run)?;
    refused_face_point_dimension(&mut run, &face);
    run.session().fit()?;
    run.dump("bug26035_2.png")?;
    run.finish()
}

/// `mkedge name curve [first last]`: an edge of the curve, bounded or whole.
fn curve_edge(curve: Handle<dyn GeomCurve>, range: Option<(f64, f64)>) -> Result<Shape, Failure> {
    let maker = match range {
        Some((first, last)) => BRepBuilderAPIMakeEdge::from_curve_params(&curve, first, last)?,
        None => BRepBuilderAPIMakeEdge::from_curve(&curve)?,
    };
    Ok(maker.edge()?.as_ref().clone())
}

fn line_edge(origin: [f64; 3], along: [f64; 3], range: (f64, f64)) -> Result<Shape, Failure> {
    let line = GeomLine::from_point(point(origin), direction(along)?);
    curve_edge(Handle::new(line).cast(), Some(range))
}

fn circle_edge(
    centre: [f64; 3],
    normal: [f64; 3],
    range: Option<(f64, f64)>,
) -> Result<Shape, Failure> {
    let circle = GeomCircle::new(Axis2::new(point(centre), direction(normal)?)?, 20.)?;
    curve_edge(Handle::new(circle).cast(), range)
}

/// `plane aPlane 0 0 50 0 0 1; mkface aFace aPlane -100 100 -100 100`.
fn square_face() -> Result<Shape, Failure> {
    let frame = Axis3::from_axis2(Axis2::new(point([0., 0., 50.]), direction([0., 0., 1.])?)?);
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    let bounds = (-100., 100., -100., 100.);
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&plane, bounds, precision::confusion())?;
    Ok(maker.shape()?.clone())
}

/// `bug26056`: lengths between a face and an edge (both orders), a circle and an arc, with
/// shaded 3d text of height 15.
/// Example: `face_edge_lengths()?`.
pub(crate) fn face_edge_lengths() -> Outcome {
    dimension_banner(&[
        "CR26056",
        "AisLengthDimension can not build dimension for face-edge or edge-face",
        "Tests case of edge-face and face-edge input geometry for dimension",
    ]);
    let face = square_face()?;
    let edges = measured_face_edges()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    display_face_edges(&mut run, &face, &edges)?;
    measure_face_edges(&mut run, &face, &edges)?;
    run.session().fit()?;
    run.dump("bug26056.png")?;
    run.finish()
}

fn face_and_point(run: &mut SessionRun) -> Result<Shape, Box<dyn Error>> {
    let b = box_at([-10., -10., -10.], [20., 20., 20.])?;
    let face = crate::boolean_typed_explode::all_distinct(&b, ShapeEnum::Face)[0].clone();
    run.session().display("b_1", &face, &Default::default())?;
    display_point(run, "p", [10., 0., 0.])?;
    Ok(face)
}

fn refused_face_point_dimension(run: &mut SessionRun, face: &Shape) {
    let shapes = [Measured::Shape(face), Measured::Object("p")];
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Zox),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &shapes)
    };
    // The original catches the refusal; its message is the expected output.
    if let Err(refused) = display_dimension(run.session(), "len", &request) {
        println!("{refused}");
    }
}

fn measured_face_edges() -> Result<[Shape; 4], Failure> {
    Ok([
        line_edge([0., -100., 0.], [1., 0., 0.], (-100., 100.))?,
        line_edge([0., 0., 100.], [1., 1., 0.1], (-150., 150.))?,
        circle_edge([30., -30., 100.], [0.3, -0.3, 1.], None)?,
        circle_edge([-130., -30., 100.], [0.3, -0.3, 1.], Some((30., 180.)))?,
    ])
}

fn display_face_edges(run: &mut SessionRun, face: &Shape, edges: &[Shape; 4]) -> Outcome {
    let session = run.session();
    session.update_session(|session| {
        for (name, shape) in ["anEdge", "anEdge2", "anEdge3", "anEdge4"]
            .into_iter()
            .zip(edges)
            .chain([("aFace", face)])
        {
            session.display(name, shape, &Default::default())?;
        }
        Ok(())
    })?;
    Ok(())
}

fn measure_face_edges(run: &mut SessionRun, face: &Shape, edges: &[Shape; 4]) -> Outcome {
    for (index, edge) in edges.iter().enumerate() {
        let pair = if index == 0 {
            [Measured::Shape(edge), Measured::Shape(face)]
        } else {
            [Measured::Shape(face), Measured::Shape(edge)]
        };
        let request = DimensionRequest {
            text_3d: Some(true),
            style: DimensionStyle {
                text_height: Some(15.),
                text_shaded: Some(true),
                ..DimensionStyle::default()
            },
            ..DimensionRequest::new(DimensionKind::Length, &pair)
        };
        display_dimension(run.session(), &format!("aDim{}", index + 1), &request)?;
    }
    Ok(())
}
