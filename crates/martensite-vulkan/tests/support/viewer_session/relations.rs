//! Relations between the selected sub-shapes (the interactive viewer's relation command) and
//! moved dimension and relation text (its move-dimension command with an explicit point).
//! Each relation's working plane comes from the selected geometry as the command builds it;
//! the symmetric one of three edges also takes the view point under the last cursor position.
use super::*;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge};
use martensite_tkernel::standard::exceptions::program_error;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve as _;
use martensite_tkg3d::geom::{GeomLine, GeomPlane};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_api::IntCS;
use martensite_tkgeombase::gc::MakePlane;
use martensite_tkmath::gp::{Direction3, Line3, Plane3, Point3, origin};
use martensite_tktopalgo::brep_extrema::{BRepExtremaExtCC, BRepExtremaExtFF};
use martensite_tkv3d::prs_dim::{
    PrsDimConcentricRelation, PrsDimDimensionMembers as _, PrsDimDimensionSource,
    PrsDimEqualDistanceRelation, PrsDimEqualRadiusRelation, PrsDimFixRelation,
    PrsDimIdenticRelation, PrsDimOffsetDimension, PrsDimParallelRelation,
    PrsDimPerpendicularRelation, PrsDimRelationMembers as _, PrsDimRelationSource,
    PrsDimSymmetricRelation,
};

/// The relation kinds the regressions use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationKind {
    /// Two circular edges.
    Concentric,
    /// Two parallel pairs of edges or vertices.
    EqualDistance,
    /// One or two circular edges.
    EqualRadius,
    /// One edge.
    Fix,
    /// Two coincident shapes.
    Identic,
    /// Two parallel faces.
    Offset,
    /// Two edges or two faces.
    Parallel,
    /// Two edges or two faces.
    Perpendicular,
    /// An axis edge and two edges or two vertices.
    Symmetric,
}

type Relation = Result<Handle<dyn AisInteractiveObject>, Failure>;

impl ViewerSession {
    /// Build the relation of the selected sub-shapes and display it
    /// as `name`; `session.display_relation("r", RelationKind::Concentric)?`.
    pub fn display_relation(&mut self, name: &str, kind: RelationKind) -> Result<(), Failure> {
        let shapes = self.selected_sub_shapes()?;
        let relation = match kind {
            RelationKind::Concentric => concentric(&shapes)?,
            RelationKind::EqualDistance => equal_distance(&shapes)?,
            RelationKind::EqualRadius => equal_radius(&shapes)?,
            RelationKind::Fix => fix(&shapes)?,
            RelationKind::Identic => identic(&shapes)?,
            RelationKind::Offset => offset(&shapes)?,
            RelationKind::Parallel | RelationKind::Perpendicular => pair_relation(kind, &shapes)?,
            RelationKind::Symmetric => symmetric(&shapes, self.point_at_cursor()?)?,
        };
        self.display_object(name, relation)
    }

    /// Move the text of the dimension or relation `name` to the integer
    /// point, then redisplay it; `session.move_dimension_text("diam", [15, 15, 15])?`.
    pub fn move_dimension_text(&self, name: &str, [x, y, z]: [i32; 3]) -> Result<(), Failure> {
        let object = self.named(name)?;
        let point = Point3::new(f64::from(x), f64::from(y), f64::from(z));
        if let Some(relation) = object.cast::<dyn PrsDimRelationSource>().get() {
            relation.set_position(&point);
        } else if let Some(dimension) = object.cast::<dyn PrsDimDimensionSource>().get() {
            dimension.set_text_position(&point);
        } else {
            return Err(program_error(&format!(
                "object {name:?}; expected a dimension or relation"
            )));
        }
        self.context.redisplay(&object, true, false)
    }

    /// `ViewerCommands::GetSelectedShapes`: the sub-shapes of the selected owners in order.
    pub(crate) fn selected_sub_shapes(&self) -> Result<Vec<Shape>, Failure> {
        let mut shapes = Vec::new();
        self.context.init_selected();
        while self.context.more_selected() {
            if self.context.has_selected_shape() {
                shapes.push(self.context.selected_shape()?);
            }
            self.context.next_selected();
        }
        Ok(shapes)
    }

    /// Read selected sub-shapes only when their owner is the named source object.
    /// Example: `session.selected_sub_shapes_on("b")?`.
    pub(crate) fn selected_sub_shapes_on(&self, name: &str) -> Result<Vec<Shape>, Failure> {
        let expected = self.named(name)?;
        selected_shapes_for_object(&self.context, &expected, name)
    }

    /// `Get3DPointAtMousePosition()`: the point under the last cursor position in the plane of
    /// the view through its target point, or the origin without an intersection.
    fn point_at_cursor(&self) -> Result<Point3, Failure> {
        let view = self.view().view_base();
        let (xv, yv, zv) = view.proj();
        let (xat, yat, zat) = view.at();
        let plane = Plane3::new(Point3::new(xat, yat, zat), Direction3::new(xv, yv, zv)?)?;
        let cursor = self.controller.last_mouse_position();
        let p = view.convert_with_proj(cursor.x(), cursor.y())?;
        let line = Line3::new(
            Point3::new(p[0], p[1], p[2]),
            Direction3::new(p[3], p[4], p[5])?,
        );
        let curve = Handle::new(GeomLine::from_lin(line)).cast();
        let surface = Handle::new(GeomPlane::from_pln(plane)).cast();
        let inter = IntCS::with_curve(&curve, &surface)?;
        if !inter.is_done() || inter.nb_points()? == 0 {
            return Ok(origin());
        }
        inter.point(1)
    }
}

fn wrong(count: usize, expected: &str) -> Failure {
    program_error(&format!("{count} selected shapes; expected {expected}"))
}

fn require_type(shapes: &[Shape], kind: ShapeEnum) -> Result<(), Failure> {
    for shape in shapes {
        if shape.shape_type()? != kind {
            return Err(program_error(&format!(
                "selected {:?}; expected {kind:?} shapes",
                shape.shape_type()?
            )));
        }
    }
    Ok(())
}

fn selected_shapes_for_object(
    context: &AisInteractiveContext,
    expected: &Handle<dyn AisInteractiveObject>,
    name: &str,
) -> Result<Vec<Shape>, Failure> {
    let mut shapes = Vec::new();
    context.init_selected();
    while context.more_selected() {
        shapes.push(selected_shape_for_object(context, expected, name)?);
        context.next_selected();
    }
    Ok(shapes)
}

fn selected_shape_for_object(
    context: &AisInteractiveContext,
    expected: &Handle<dyn AisInteractiveObject>,
    name: &str,
) -> Result<Shape, Failure> {
    if context.selected_interactive()?.object_address() != expected.object_address() {
        return Err(program_error(&format!(
            "selected owner for {name:?}; expected exact named interactive object"
        )));
    }
    if !context.has_selected_shape() {
        return Err(program_error(&format!(
            "selected owner for {name:?}; expected an exact selected sub-shape"
        )));
    }
    context.selected_shape()
}

/// `GC_MakePlane(A, B, C).Value()`.
fn plane(a: Point3, b: Point3, c: Point3) -> Result<Handle<GeomPlane>, Failure> {
    MakePlane::through_points(a, b, c)?.value()
}

fn curve_value(edge: &Edge, u: f64) -> Result<Point3, Failure> {
    BRepAdaptorCurve::with_edge(edge)?.value(u)
}

/// The point moved by 5 along each axis, as the command completes a plane.
fn offset5(p: Point3) -> Point3 {
    Point3::new(p.x() + 5.0, p.y() + 5.0, p.z() + 5.0)
}

fn first_edge(face: &Shape) -> Result<Edge, Failure> {
    let explorer = Explorer::new(
        topods::face(face)?.as_ref(),
        ShapeEnum::Edge,
        ShapeEnum::Shape,
    )?;
    topods::edge(explorer.current())
}

fn vertex_point(shape: &Shape) -> Result<Point3, Failure> {
    BRepTool::pnt(&topods::vertex(shape)?)
}

fn first_vertex_point(edge: &Edge) -> Result<Point3, Failure> {
    BRepTool::pnt(&topexp::vertices(edge, false).0)
}

/// Two edges must be parallel; other shape pairs always pass (`IsParallel`).
fn require_parallel(a: &Shape, b: &Shape) -> Result<(), Failure> {
    let edges = a.shape_type()? == ShapeEnum::Edge && b.shape_type()? == ShapeEnum::Edge;
    if edges
        && !BRepExtremaExtCC::with_edges(&topods::edge(a)?, &topods::edge(b)?)?.is_parallel()?
    {
        return Err(program_error("non parallel edges; expected parallel edges"));
    }
    Ok(())
}

fn concentric(shapes: &[Shape]) -> Relation {
    let [a, b] = shapes else {
        return Err(wrong(shapes.len(), "2 circular edges"));
    };
    require_type(shapes, ShapeEnum::Edge)?;
    let curve = BRepAdaptorCurve::with_edge(&topods::edge(a)?)?;
    let centre = curve.circle()?.location();
    let plane = plane(centre, curve.value(0.25)?, curve.value(0.75)?)?;
    Ok(PrsDimConcentricRelation::new(a, b, &plane)?.cast())
}

/// The plane points of an equal distance from its first pair.
fn equal_distance_points(first: &Shape, second: &Shape) -> Result<[Point3; 3], Failure> {
    let second_is_edge = second.shape_type()? == ShapeEnum::Edge;
    if first.shape_type()? == ShapeEnum::Edge {
        let (va, vb) = topexp::vertices(&topods::edge(first)?, false);
        let c = match second_is_edge {
            true => first_vertex_point(&topods::edge(second)?)?,
            false => vertex_point(second)?,
        };
        return Ok([BRepTool::pnt(&va)?, BRepTool::pnt(&vb)?, c]);
    }
    let a = vertex_point(first)?;
    if second_is_edge {
        let (vb, vc) = topexp::vertices(&topods::edge(second)?, false);
        return Ok([a, BRepTool::pnt(&vb)?, BRepTool::pnt(&vc)?]);
    }
    let b = vertex_point(second)?;
    Ok([a, b, offset5(b)])
}

fn equal_distance(shapes: &[Shape]) -> Relation {
    let [a, b, c, d] = shapes else {
        return Err(wrong(shapes.len(), "4 shapes (two parallel pairs)"));
    };
    require_parallel(a, b)?;
    require_parallel(c, d)?;
    let [p, q, r] = equal_distance_points(a, b)?;
    Ok(PrsDimEqualDistanceRelation::new(a, b, c, d, &plane(p, q, r)?).cast())
}

fn equal_radius(shapes: &[Shape]) -> Relation {
    let (Some(first), Some(last), 1..=2) = (shapes.first(), shapes.last(), shapes.len()) else {
        return Err(wrong(shapes.len(), "1 or 2 circular edges"));
    };
    require_type(shapes, ShapeEnum::Edge)?;
    let (e1, e2) = (topods::edge(first)?, topods::edge(last)?);
    let [p, q, r] = [0.1, 0.5, 0.9].map(|u| curve_value(&e1, u));
    Ok(PrsDimEqualRadiusRelation::new(&e1, &e2, &plane(p?, q?, r?)?).cast())
}

fn fix(shapes: &[Shape]) -> Relation {
    let [shape] = shapes else {
        return Err(wrong(shapes.len(), "1 edge"));
    };
    require_type(shapes, ShapeEnum::Edge)?;
    let edge = topods::edge(shape)?;
    let [a, b, d] = [0.1, 0.5, 0.9].map(|u| curve_value(&edge, u));
    let plane = plane(a?, d?, offset5(b?))?;
    Ok(PrsDimFixRelation::new(edge.as_ref(), &plane, None).cast())
}

/// The plane points of an identic relation from its two shapes.
fn identic_points(a: &Shape, b: &Shape) -> Result<[Point3; 3], Failure> {
    match (a.shape_type()?, b.shape_type()?) {
        (ShapeEnum::Edge, _) => identic_edge_points(&topods::edge(a)?),
        (ShapeEnum::Vertex, ShapeEnum::Edge) => identic_edge_points(&topods::edge(b)?),
        (ShapeEnum::Vertex, ShapeEnum::Face) => identic_face_points(b),
        (ShapeEnum::Vertex, _) => {
            let q = vertex_point(b)?;
            Ok([vertex_point(a)?, q, offset5(q)])
        }
        _ => identic_face_points(a),
    }
}

fn identic(shapes: &[Shape]) -> Relation {
    let [a, b] = shapes else {
        return Err(wrong(shapes.len(), "2 shapes"));
    };
    let [p, q, r] = identic_points(a, b)?;
    Ok(PrsDimIdenticRelation::new(a, b, &plane(p, q, r)?).cast())
}

/// An offset of two parallel faces, labelled with their distance rounded to 0.1.
fn offset(shapes: &[Shape]) -> Relation {
    let [a, b] = shapes else {
        return Err(wrong(shapes.len(), "2 parallel faces"));
    };
    require_type(shapes, ShapeEnum::Face)?;
    let (f1, f2) = (topods::face(a)?, topods::face(b)?);
    let delta = BRepExtremaExtFF::with_faces(&f1, &f2)?;
    if !delta.is_parallel()? {
        return Err(program_error("faces not parallel; expected parallel faces"));
    }
    let distance = (delta.square_distance(1)?.sqrt() * 10.0).round() / 10.0;
    let text = ExtendedString::from_utf8("offset=").cat(ExtendedString::from_real(distance));
    Ok(PrsDimOffsetDimension::new(f1.as_ref(), f2.as_ref(), distance, &text).cast())
}

/// A parallel or perpendicular relation of two edges or two faces; the plane passes through
/// 0.1 and 0.9 of the first edge and 0.5 of the second (a face's first edge).
fn pair_relation(kind: RelationKind, shapes: &[Shape]) -> Relation {
    let [a, b] = shapes else {
        return Err(wrong(shapes.len(), "2 edges or 2 faces"));
    };
    let is_edge = a.shape_type()? == ShapeEnum::Edge;
    require_pair_parallel(kind, a, b, is_edge)?;
    let (ea, eb) = match is_edge {
        true => (topods::edge(a)?, topods::edge(b)?),
        false => (first_edge(a)?, first_edge(b)?),
    };
    let plane = plane(
        curve_value(&ea, 0.1)?,
        curve_value(&ea, 0.9)?,
        curve_value(&eb, 0.5)?,
    )?;
    Ok(match kind {
        RelationKind::Parallel => PrsDimParallelRelation::new(a, b, &plane).cast(),
        _ => PrsDimPerpendicularRelation::new(a, b, is_edge.then_some(&plane)).cast(),
    })
}

/// A symmetry of an edge and two parallel edges (plane through their first vertices and the
/// point under the cursor) or two vertices (plane through the axis's first vertex and both).
fn symmetric(shapes: &[Shape], cursor: Point3) -> Relation {
    let [axis, b, c] = shapes else {
        return Err(wrong(shapes.len(), "an edge and 2 edges or 2 vertices"));
    };
    let edge = topods::edge(axis)?;
    if b.shape_type()? == ShapeEnum::Edge {
        require_parallel(axis, b)?;
        require_parallel(axis, c)?;
        let (eb, ec) = (topods::edge(b)?, topods::edge(c)?);
        let plane = plane(first_vertex_point(&eb)?, first_vertex_point(&ec)?, cursor)?;
        return Ok(PrsDimSymmetricRelation::new(edge.as_ref(), b, c, &plane)?.cast());
    }
    let (pb, pc) = (vertex_point(b)?, vertex_point(c)?);
    let plane = plane(first_vertex_point(&edge)?, pb, pc)?;
    Ok(PrsDimSymmetricRelation::new(edge.as_ref(), b, c, &plane)?.cast())
}

fn identic_edge_points(edge: &Edge) -> Result<[Point3; 3], Failure> {
    let q = curve_value(edge, 0.9)?;
    Ok([curve_value(edge, 0.1)?, q, offset5(q)])
}

fn identic_face_points(face: &Shape) -> Result<[Point3; 3], Failure> {
    let edge = first_edge(face)?;
    let [p, q, r] = [0.1, 0.5, 0.9].map(|u| curve_value(&edge, u));
    Ok([p?, q?, r?])
}

fn require_pair_parallel(
    kind: RelationKind,
    a: &Shape,
    b: &Shape,
    is_edge: bool,
) -> Result<(), Failure> {
    if kind == RelationKind::Parallel && !is_edge {
        let faces = BRepExtremaExtFF::with_faces(&topods::face(a)?, &topods::face(b)?)?;
        if !faces.is_parallel()? {
            return Err(program_error("faces not parallel; expected parallel faces"));
        }
    } else if kind == RelationKind::Parallel {
        require_parallel(a, b)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "relation_command_contract.rs"]
mod relation_command_contract;
