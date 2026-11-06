//! `TopoDSToStepMakeStepEdge`: an edge to an edge_curve, on a surface_curve when pcurves are
//! written.
use super::TopoDSToStepMakeStepVertex;
use super::{
    TopoDSToStepMakeEdgeError, TopoDSToStepRoot, TopoDSToStepTool, add_warning, step_model_of,
};
use crate::geom_to_step::{GeomToStepMakeCurve, GeomToStepMakeLine, array1_of, empty_step_name};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_new_array1;
use crate::step_geom::{
    StepGeomCurve, StepGeomPcurveOrSurface, StepGeomPreferredSurfaceCurveRepresentation,
    StepGeomSeamCurve, StepGeomSurfaceCurve,
};
use crate::step_shape::{
    StepShapeEdgeCurve, StepShapeTopologicalRepresentationItem, StepShapeVertex,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomLine};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};
use martensite_tkmath::gp::{Direction3, Point3, Vector3};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisCurve;
use martensite_tkxsbase::transfer::TransferFinderProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// The points of the polyline a 3D curve is built from when the edge has none.
const NBPT: usize = 21;

/// Make step edge.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeStepEdge {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: Handle<StepShapeTopologicalRepresentationItem>,
    my_error: TopoDSToStepMakeEdgeError,
}

/// `dpar`/`dU` made positive by whole periods.
fn positive_span(span: f64, period: f64) -> f64 {
    if span <= 0.0 {
        return span + (span.abs() / period).ceil() * period;
    }
    span
}

///: i4 abv 02 Sep 98: ProSTEP TR8 Motor.rle f3 & f62: check that edge participates twice in the
/// wires of the face before making it seam (else it can have two pcurves on the same surface
/// being shared by two faces on that surface). This fix is necessary because sharing of surfaces
/// is not preserved when writing faces to STEP (see TopoDSToStepMakeStepFace).
fn is_seam_of_current_face(edge: &Edge, tool: &TopoDSToStepTool) -> Result<bool, Failure> {
    if !BRepTool::is_closed_on_face(edge, tool.current_face())? {
        return Ok(false);
    }
    let mut count = 0;
    let mut exp = Explorer::new(tool.current_face(), ShapeEnum::Edge, ShapeEnum::Shape)?;
    while exp.more() {
        if edge.is_same(exp.current()) {
            count += 1;
        }
        exp.next()?;
    }
    Ok(count >= 2)
}

/// The two vertices of an edge, and their swap below when the range contradicts them.
struct EdgeEnds {
    first: Handle<StepShapeVertex>,
    last: Handle<StepShapeVertex>,
}

/// Special treatment is needed for very short edges based on periodic curves. Since edge in STEP
/// does not store its parametric range, parameters are computed on import by projecting vertices
/// on a curve, and for periodic curve this may lead to use of wrong part of the curve if end
/// vertices are too close to each other (often whole curve is taken instead of its very small
/// fragment).
fn treat_periodic_curve(
    curve: &mut Handle<dyn GeomCurve>,
    adaptor: &BRepAdaptorCurve,
    vertices: (&Vertex, &Vertex),
    ends: &mut EdgeEnds,
    tolerance: f64,
) -> Result<(), Failure> {
    let period = curve.period()?;
    let (first, last) = (adaptor.first_parameter()?, adaptor.last_parameter()?);
    let dpar = positive_span(last - first, period);
    // if range obtained from projection of vertices contradicts with range
    // of the edge tnen vertices are swapped to keep results correct after import
    // (see test de step_5 A1)
    let sac = ShapeAnalysisCurve;
    let (_, _, u1) = sac.project_full(curve, BRepTool::pnt(vertices.0)?, tolerance, false)?;
    let (_, _, u2) = sac.project_full(curve, BRepTool::pnt(vertices.1)?, tolerance, false)?;
    let du = positive_span(u2 - u1, period);
    let confusion = precision::p_confusion();
    if (du > confusion && du <= 0.1 * period && dpar > 0.5 * period)
        || (dpar > confusion && dpar <= 0.1 * period && du > 0.5 * period)
    {
        std::mem::swap(&mut ends.first, &mut ends.last);
    }
    // If vertices overlap, we cut only needed part of the BSpline curve.
    // Note that this method cannot be used for canonic curves
    // (STEP does not support trimmed curves in AIC 514).
    if curve.cast::<GeomBSplineCurve>().is_some()
        && is_small_overlapping_curve(adaptor, vertices)?
        && dpar > confusion
        && dpar <= 0.1 * period
    {
        let segment: Handle<GeomBSplineCurve> = curve.copy()?.cast();
        segment.segment(first, last, precision::p_confusion())?;
        *curve = segment.cast();
    }
    Ok(())
}

/// The vertices coincide and the whole curve stays within one vertex tolerance.
fn is_small_overlapping_curve(
    adaptor: &BRepAdaptorCurve,
    vertices: (&Vertex, &Vertex),
) -> Result<bool, Failure> {
    let (first, last) = (adaptor.first_parameter()?, adaptor.last_parameter()?);
    let p11 = adaptor.value(first)?;
    let p12 = adaptor.value(last)?;
    let pm = adaptor.value((first + last) * 0.5)?;
    let dist_max = p11
        .distance(&pm)
        .max(p12.distance(&pm))
        .max(p11.distance(&p12));
    let small = dist_max <= BRepTool::vertex_tolerance(vertices.0)?
        || dist_max <= BRepTool::vertex_tolerance(vertices.1)?;
    Ok(brep_tools::compare_vertices(vertices.0, vertices.1)? && small)
}

/// A 3D curve is constructed for an edge that only has curves on surfaces.
fn construct_curve(
    adaptor: &BRepAdaptorCurve,
    tool: &TopoDSToStepTool,
    factors: &StepDataFactors,
) -> Result<Handle<StepGeomCurve>, Failure> {
    let surface = BRepAdaptorSurface::with_face(tool.current_face(), true)?;
    let (u1, u2) = (adaptor.first_parameter()?, adaptor.last_parameter()?);
    if surface.get_type()? == SurfaceType::Plane && adaptor.get_type()? == CurveType::Line {
        let (p1, p2) = (adaptor.value(u1)?, adaptor.value(u2)?);
        let direction = Direction3::from_vector(Vector3::between(p1, p2))?;
        let line = Handle::new(GeomLine::from_point(p1, direction));
        return Ok(GeomToStepMakeLine::from_geom(&line, factors)?
            .value()?
            .cast());
    }
    // To Be Optimized: create an approximated BSpline
    //                   using the spline through points
    let bspline = polyline_through(adaptor, u1, u2)?;
    GeomToStepMakeCurve::from_geom(&bspline.cast(), factors)?.value()
}

/// The degree 1 B-spline through `NBPT` points of the adaptor.
fn polyline_through(
    adaptor: &BRepAdaptorCurve,
    u1: f64,
    u2: f64,
) -> Result<Handle<GeomBSplineCurve>, Failure> {
    let mut points: Vec<Point3> = Vec::with_capacity(NBPT);
    let mut knots: Vec<f64> = Vec::with_capacity(NBPT);
    for index in 0..NBPT {
        let u = u1 + index as f64 * (u2 - u1) / (NBPT - 1) as f64;
        points.push(adaptor.value(u)?);
        knots.push(u);
    }
    let mut mults = vec![1; NBPT];
    mults[0] = 2;
    mults[NBPT - 1] = 2;
    GeomBSplineCurve::new(
        &array1_of(points),
        &array1_of(knots),
        &array1_of(mults),
        1,
        false,
    )
    .map(Handle::new)
}

/// The surface_curve or seam_curve on a 3D curve; its two pcurves come with the faces.
fn on_surface_curve(
    curve: &Handle<StepGeomCurve>,
    seam: bool,
) -> Result<Handle<StepGeomCurve>, Failure> {
    let geom = step_new_array1::<StepGeomPcurveOrSurface>(1, 2)?;
    let name = empty_step_name();
    let representation = StepGeomPreferredSurfaceCurveRepresentation::PscrPcurveS1;
    if seam {
        let entity = Handle::new(StepGeomSeamCurve::new());
        entity.init(&name, curve, &geom, representation);
        return Ok(entity.cast());
    }
    let entity = Handle::new(StepGeomSurfaceCurve::new());
    entity.init(&name, curve, &geom, representation);
    Ok(entity.cast())
}

impl TopoDSToStepMakeStepEdge {
    /// `TopoDSToStepMakeStepEdge()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepMakeStepEdge(E, T, FP, theLocalFactors)`.
    pub fn with_edge(
        edge: &Edge,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(edge, tool, fp, factors)?;
        Ok(made)
    }
    fn finish(&mut self, result: Handle<StepShapeTopologicalRepresentationItem>) {
        self.my_error = TopoDSToStepMakeEdgeError::EdgeDone;
        self.my_result = result;
        self.root.done = true;
    }
    fn fail(
        &mut self,
        error: TopoDSToStepMakeEdgeError,
        edge: &Edge,
        fp: &Handle<TransferFinderProcess>,
        message: &str,
    ) -> Result<(), Failure> {
        add_warning(fp, edge, message)?;
        self.my_error = error;
        self.root.done = false;
        Ok(())
    }
    /// The entity the shared maps already hold for this edge.
    fn known_edge(
        edge: &Edge,
        tool: &TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
    ) -> Result<Option<Handle<StepShapeTopologicalRepresentationItem>>, Failure> {
        // [BEGIN] Processing non-manifold topology (ssv; 11.11.2010)
        if step_model_of(fp)?.internal_parameters().write_nonmanifold {
            let mapper = TransferBRep::shape_mapper(fp, edge)?;
            let mut known = Handle::<StepShapeEdgeCurve>::default();
            let atype = StepShapeEdgeCurve::static_type();
            if fp.find_typed_transient_as(&mapper.cast(), atype, &mut known) {
                // Non-manifold topology detected
                return Ok(Some(known.cast()));
            }
        }
        // [END] Processing non-manifold topology (ssv; 11.11.2010)
        if tool.is_bound(edge) {
            return Ok(Some(tool.find(edge)?));
        }
        Ok(None)
    }
    /// `init(edge, tool, FP, local_factors)`: the edge is given with its relative orientation
    /// (i.e. in the wire).
    pub fn init(
        &mut self,
        edge: &Edge,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        tool.set_current_edge(edge)?;
        if let Some(known) = Self::known_edge(edge, tool, fp)? {
            self.finish(known);
            return Ok(());
        }
        let is_seam = is_seam_of_current_face(edge, tool)?;
        if matches!(
            edge.orientation(),
            Orientation::Internal | Orientation::External
        ) {
            let text = " Edge(internal/external) from Non Manifold Topology";
            return self.fail(TopoDSToStepMakeEdgeError::NonManifoldEdge, edge, fp, text);
        }
        let (v_first, v_last) = topexp::vertices(edge, false);
        let mut ends = self.translate_vertices(edge, (&v_first, &v_last), tool, fp, factors)?;
        let Some(ends) = ends.as_mut() else {
            return Ok(());
        };
        let geometry = self.translate_curve(edge, (&v_first, &v_last), ends, tool, factors)?;
        // ---------------------------------------------------------
        // Warning: if the edge is connected aGeom->Length = 2
        //           otherwise = 1;
        //           and enumeration is pscrPcurveS2 or pscrPcurveS1
        // This is corrected in the Write File phases !
        // ---------------------------------------------------------
        //:abv 25.01.00 CAX-IF TRJ3
        // if PcurveMode is 1 (default), make surface_curve instead of simple 3d curve
        let geometry = if tool.pcurve_mode() {
            on_surface_curve(&geometry, is_seam)?
        } else {
            geometry
        };
        let edge_curve = Handle::new(StepShapeEdgeCurve::new());
        edge_curve.init(&empty_step_name(), &ends.first, &ends.last, &geometry, true);
        tool.bind(edge, &edge_curve.cast());
        self.finish(edge_curve.cast());
        Ok(())
    }
    /// The two end vertices; `None` when one has no entity, with the failure recorded.
    fn translate_vertices(
        &mut self,
        edge: &Edge,
        vertices: (&Vertex, &Vertex),
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Option<EdgeEnds>, Failure> {
        let mut maker = TopoDSToStepMakeStepVertex::new();
        maker.init(vertices.0, tool, fp, factors)?;
        if !maker.is_done() {
            let text = " First Vertex of Edge not mapped";
            self.fail(TopoDSToStepMakeEdgeError::EdgeOther, edge, fp, text)?;
            return Ok(None);
        }
        let first: Handle<StepShapeVertex> = maker.value()?.cast();
        maker.init(vertices.1, tool, fp, factors)?;
        if !maker.is_done() {
            let text = " Last Vertex of Edge not mapped";
            self.fail(TopoDSToStepMakeEdgeError::EdgeOther, edge, fp, text)?;
            return Ok(None);
        }
        Ok(Some(EdgeEnds {
            first,
            last: maker.value()?.cast(),
        }))
    }
    /// The 3D representation of the edge.
    fn translate_curve(
        &self,
        edge: &Edge,
        vertices: (&Vertex, &Vertex),
        ends: &mut EdgeEnds,
        tool: &TopoDSToStepTool,
        factors: &StepDataFactors,
    ) -> Result<Handle<StepGeomCurve>, Failure> {
        let adaptor = BRepAdaptorCurve::with_edge(edge)?;
        let stored = adaptor.transformed_curve().geom_curve();
        let Some(live) = stored.get() else {
            return construct_curve(&adaptor, tool, factors);
        };
        let mut curve: Handle<dyn GeomCurve> = live.copy()?.cast();
        curve.transform(adaptor.transformed_curve().trsf())?;
        if curve.is_periodic() {
            treat_periodic_curve(&mut curve, &adaptor, vertices, ends, self.root.tolerance())?;
        }
        GeomToStepMakeCurve::from_geom(&curve, factors)?.value()
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepShapeTopologicalRepresentationItem>, Failure> {
        self.root.check_done("TopoDSToStepMakeStepEdge::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepMakeEdgeError {
        self.my_error
    }
}
