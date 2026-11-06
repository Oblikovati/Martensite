//! `StepToTopoDSTranslatePolyLoop`: a poly_loop of a faceted B-rep to a wire of straight edges.
use super::{
    StepToTopoDSPointPair, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslatePolyLoopError,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_shape::StepShapePolyLoop;
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Vertex, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::geom::{GeomLine, GeomPlane, GeomPoint, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3, Vector2, Vector3};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisSurface;

/// The vertex bound to a point, made and bound when it is new.
fn bound_vertex(
    tool: &mut StepToTopoDSTool,
    point: &Handle<StepGeomCartesianPoint>,
    position: Point3,
) -> Result<Vertex, Failure> {
    if let Some(found) = tool.find_vertex(point) {
        return Ok(found.clone());
    }
    let mut vertex = Vertex::default();
    BRepBuilder::default().make_vertex_at(&mut vertex, position, precision::confusion())?;
    tool.bind_vertex(point, &vertex);
    Ok(vertex)
}

/// The straight edge between two new or known vertices.
fn straight_edge(from: Point3, to: Point3, v1: &Vertex, v2: &Vertex) -> Result<Edge, Failure> {
    let builder = BRepBuilder::default();
    let vector = Vector3::between(from, to);
    let line = GeomLine::from_point(from, Direction3::from_vector(vector)?);
    let curve: Handle<dyn martensite_tkg3d::geom::GeomCurve> = Handle::new(line).cast();
    let mut edge = Edge::default();
    builder.make_edge_with_curve(&mut edge, &curve, precision::confusion())?;
    let first = v1.oriented(Orientation::Forward);
    let last = v2.oriented(Orientation::Reversed);
    builder.add(&mut edge, &first)?;
    builder.add(&mut edge, &last)?;
    builder.update_vertex_on_edge(&topods::vertex(&first)?, 0.0, &edge, 0.0)?;
    builder.update_vertex_on_edge(&topods::vertex(&last)?, vector.magnitude(), &edge, 0.0)?;
    Ok(edge)
}

/// The line of an edge in the parameters of the face, oriented as the edge.
fn edge_pcurve(edge: &Edge, uv1: Point2, uv2: Point2) -> Result<Handle<Geom2dLine>, Failure> {
    let (start, end) = if edge.orientation() == Orientation::Forward {
        (uv1, uv2)
    } else {
        (uv2, uv1)
    };
    let direction = Direction2::from_vector(Vector2::between(start, end))?;
    Ok(Handle::new(Geom2dLine::from_point(start, direction)))
}

/// Translate poly loop.
#[derive(Default)]
pub struct StepToTopoDSTranslatePolyLoop {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslatePolyLoopError,
}

impl StepToTopoDSTranslatePolyLoop {
    /// `StepToTopoDSTranslatePolyLoop()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslatePolyLoop(PL, T, S, F, factors)`.
    pub fn with_loop(
        poly_loop: &Handle<StepShapePolyLoop>,
        tool: &mut StepToTopoDSTool,
        surface: &Handle<dyn GeomSurface>,
        face: &Face,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(poly_loop, tool, surface, face, factors)?;
        Ok(made)
    }
    fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslatePolyLoopError::Done;
        self.root.done = true;
    }
    /// The points of the loop, closed by its first point.
    fn closed_points(
        live: &StepShapePolyLoop,
    ) -> Result<Vec<Handle<StepGeomCartesianPoint>>, Failure> {
        let count = live.nb_polygon();
        let mut points = Vec::with_capacity(count as usize + 1);
        for rank in 1..=count {
            points.push(live.polygon_value(rank)?);
        }
        points.push(live.polygon_value(1)?);
        Ok(points)
    }
    /// The position of a point; a point that cannot be made is the null dereference.
    fn position(
        point: &Handle<StepGeomCartesianPoint>,
        factors: &StepDataFactors,
    ) -> Result<Point3, Failure> {
        let made = StepToGeom::make_cartesian_point(point, factors)?;
        Ok(step_live(&made, "StepToTopoDSTranslatePolyLoop")?.pnt())
    }
    /// `init(PL, tool, GeomSurf, TopoFace, factors)`.
    pub fn init(
        &mut self,
        poly_loop: &Handle<StepShapePolyLoop>,
        tool: &mut StepToTopoDSTool,
        surface: &Handle<dyn GeomSurface>,
        face: &Face,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if tool.is_bound(&poly_loop.cast()) {
            let found = topods::wire(tool.find(&poly_loop.cast())?)?;
            self.finish(found.into_shape());
            return Ok(());
        }
        let live = step_live(poly_loop, "StepToTopoDSTranslatePolyLoop")?;
        if surface.cast::<GeomPlane>().is_null() {
            let process = tool.transient_process();
            let entity: Handle<StandardTransient> = poly_loop.cast();
            step_live(&process, "StepToTopoDSTranslatePolyLoop")?.add_fail_text(
                &entity,
                "Surface not planar in a FacetedBRep !",
                "",
            );
        }
        let analysis = ShapeAnalysisSurface::new(surface)?;
        let points = Self::closed_points(live)?;
        let builder = BRepBuilder::default();
        let mut p1 = points[0].clone();
        let mut position1 = Self::position(&p1, factors)?;
        let mut v1 = bound_vertex(tool, &p1, position1)?;
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        for p2 in &points[1..] {
            if p1 == *p2 {
                continue; // peut arriver (KK) CKY 9-DEC-1997
            }
            let pair = StepToTopoDSPointPair::new(&p1, p2);
            let position2 = Self::position(p2, factors)?;
            let known_edge = tool.find_edge(&pair).cloned();
            let is_bound = known_edge.is_some();
            let (edge, v2) = match known_edge {
                None => {
                    let v2 = bound_vertex(tool, p2, position2)?;
                    (straight_edge(position1, position2, &v1, &v2)?, v2)
                }
                Some(mut found) => {
                    // Il faut qu en finale l edge soit vue
                    // - via sa premiere face, orientation combinee = celle de cette premiere face
                    // - via sa deuxieme face, orientation combinee INVERSE de la precedente
                    if face.orientation() == Orientation::Forward {
                        found.reverse();
                    }
                    let v2 = tool.find_vertex(p2).cloned().ok_or_else(|| {
                        martensite_tkernel::standard::exceptions::no_such_object(
                            "StepToTopoDSTool::FindVertex: the point is not bound",
                        )
                    })?;
                    (found, v2)
                }
            };
            let uv1 = analysis.value_of_uv(position1, self.root.precision())?;
            let uv2 = analysis.value_of_uv(position2, self.root.precision())?;
            let pcurve: Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve> =
                edge_pcurve(&edge, uv1, uv2)?.cast();
            builder.update_edge_pcurve_on_face(&edge, &pcurve, face, 0.0)?;
            // pour le binding: cumul des orientations !
            if !is_bound {
                let mut for_binding = edge.clone();
                for_binding.set_orientation(face.orientation());
                tool.bind_edge(&pair, &for_binding);
            }
            if !edge.is_null() {
                builder.add(&mut wire, &edge)?;
            }
            p1 = p2.clone();
            position1 = position2;
            v1 = v2;
        }
        wire.set_closed(BRepTool::is_closed(&wire)?)?;
        let shape = wire.into_shape();
        tool.bind(&poly_loop.cast(), &shape)?;
        self.finish(shape);
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure without a result.
    pub fn value(&self) -> Result<&Shape, Failure> {
        self.root
            .check_done("StepToTopoDSTranslatePolyLoop::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslatePolyLoopError {
        self.my_error
    }
}
