//! `TopoDSToStepWireframeBuilder`: the edges of a shape to trimmed curves.
//! :S4134: abv 10 Mar 99: the methods below moved from package TopoDSToGBWire. :j1 modified by abv
//! 22 Oct 98: CSR BUC60401: unused parts of code dropped; fixed trimming of circles and ellipses
//! (radians used instead of degrees).
use super::{TopoDSToStepBuilderError, TopoDSToStepRoot, TopoDSToStepShapeMap, TopoDSToStepTool};
use crate::geom_to_step::{
    GeomToStepMakeCartesianPoint, GeomToStepMakeCurve, GeomToStepMakeLine, array1_of,
    empty_step_name, shared_array1,
};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCartesianPoint, StepGeomCurve, StepGeomTrimmedCurve, StepGeomTrimmingPreference,
    StepGeomTrimmingSelect,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, ShapeIterator, Vertex};
use martensite_tkernel::ncollection::HSequence;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomLine, GeomPlane, GeomTrimmedCurve};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Direction3, Point3, Vector3};

const NBPT: usize = 23;

/// The items of the result: `HSequence<Handle<`StandardTransient`>>`.
pub type TopoDSToStepWireframeItems = Handle<HSequence<Handle<StandardTransient>>>;

/// Wireframe builder.
#[derive(Clone, Default)]
pub struct TopoDSToStepWireframeBuilder {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: TopoDSToStepWireframeItems,
    my_error: TopoDSToStepBuilderError,
}

/// `make_trimmed_curve`.
fn make_trimmed_curve(
    curve: &Handle<StepGeomCurve>,
    points: (
        &Handle<StepGeomCartesianPoint>,
        &Handle<StepGeomCartesianPoint>,
    ),
    trims: (f64, f64),
    sense: bool,
) -> Result<Handle<StepGeomCurve>, Failure> {
    let trim_of = |point: &Handle<StepGeomCartesianPoint>, trim: f64| -> Result<_, Failure> {
        let mut by_point = StepGeomTrimmingSelect::new();
        by_point.set_value(&point.cast())?;
        let mut by_parameter = StepGeomTrimmingSelect::new();
        by_parameter.set_parameter_value(trim)?;
        Ok(shared_array1(vec![by_point, by_parameter]))
    };
    let trimmed = Handle::new(StepGeomTrimmedCurve::new());
    trimmed.init(
        &empty_step_name(),
        curve,
        &trim_of(points.0, trims.0)?,
        &trim_of(points.1, trims.1)?,
        sense,
        StepGeomTrimmingPreference::TpParameter,
    );
    Ok(trimmed.cast())
}

/// The end vertices of an edge and the cartesian points they trim at.
#[derive(Default)]
struct EdgeTrims {
    first: Vertex,
    last: Vertex,
    point1: Handle<StepGeomCartesianPoint>,
    point2: Handle<StepGeomCartesianPoint>,
}

fn edge_trims(edge: &Edge, factors: &StepDataFactors) -> Result<EdgeTrims, Failure> {
    let mut trims = EdgeTrims::default();
    let mut exp = Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while exp.more() {
        let vertex = topods::vertex(exp.value())?;
        let point = BRepTool::pnt(&vertex)?;
        let made = GeomToStepMakeCartesianPoint::from_pnt(&point, factors.length_factor());
        if vertex.orientation() == Orientation::Forward {
            trims.first = vertex.clone();
            // 1.point for trimming
            trims.point1 = made.value()?;
        }
        if vertex.orientation() == Orientation::Reversed {
            trims.last = vertex;
            // 2.point for trimming
            trims.point2 = made.value()?;
        }
        exp.next()?;
    }
    Ok(trims)
}

/// The line or the polyline standing for an edge without a 3D curve. Null when the line is shorter
/// than `Precision::confusion`.
fn constructed_curve(
    adaptor: &BRepAdaptorCurve,
    face: &Face,
    trims: &EdgeTrims,
    factors: &StepDataFactors,
) -> Result<Handle<StepGeomCurve>, Failure> {
    let mut on_plane = false;
    if !face.is_null() {
        let pcurve = BRepTool::curve_on_face(&adaptor.edge(), face)?;
        let surface = BRepTool::surface(face)?;
        on_plane =
            surface.cast::<GeomPlane>().is_some() && pcurve.pcurve.cast::<Geom2dLine>().is_some();
    }
    let (first, last) = (adaptor.first_parameter()?, adaptor.last_parameter()?);
    // to be modified: cf and cl are the topological trimming parameter
    // these are computed after ! (U1 and U2) -> cf and cl instead
    if on_plane {
        let (p1, p2) = (adaptor.value(first)?, adaptor.value(last)?);
        let vector = Vector3::between(p1, p2);
        let length = vector.magnitude();
        if length < precision::confusion() {
            return Ok(Handle::default());
        }
        let line = Handle::new(GeomLine::from_point(p1, Direction3::from_vector(vector)?));
        let made = GeomToStepMakeLine::from_geom(&line, factors)?.value()?;
        return make_trimmed_curve(
            &made.cast(),
            (&trims.point1, &trims.point2),
            (0.0, length),
            true,
        );
    }
    let spline = polyline_through(adaptor, trims, (first, last))?;
    GeomToStepMakeCurve::from_geom(&spline.cast(), factors)?.value()
}

/// The degree 1 B-spline through `NBPT` points, ending on the vertices.
fn polyline_through(
    adaptor: &BRepAdaptorCurve,
    trims: &EdgeTrims,
    range: (f64, f64),
) -> Result<Handle<GeomBSplineCurve>, Failure> {
    let mut points: Vec<Point3> = Vec::with_capacity(NBPT);
    let mut knots: Vec<f64> = Vec::with_capacity(NBPT);
    for index in 0..NBPT {
        let u = range.0 + index as f64 * (range.1 - range.0) / (NBPT - 1) as f64;
        points.push(adaptor.value(u)?);
        knots.push(u);
    }
    points[0] = BRepTool::pnt(&trims.first)?;
    points[NBPT - 1] = BRepTool::pnt(&trims.last)?;
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

/// The edge on its 3D curve: `None` when `GeomToStepMakeCurve` refused it, the free curve when
/// the edge is infinite at both ends and has no vertex.
fn stored_curve(
    edge: &Edge,
    adaptor: &BRepAdaptorCurve,
    trims: &mut EdgeTrims,
    factors: &StepDataFactors,
) -> Result<Option<(Handle<StepGeomCurve>, bool)>, Failure> {
    let (stored, first, last) = BRepTool::curve(edge)?;
    let basis = match stored.cast::<GeomTrimmedCurve>().get() {
        Some(trimmed) => trimmed.basis_curve(),
        None => stored,
    };
    let maker = GeomToStepMakeCurve::from_geom(&basis, factors)?;
    if !maker.is_done() {
        return Ok(None);
    }
    let made = maker.value()?;
    // trim the curve
    let (trim1, trim2) = (adaptor.first_parameter()?, adaptor.last_parameter()?);
    if trims.first.is_null()
        && trims.last.is_null()
        && precision::is_infinite(first)
        && precision::is_infinite(last)
    {
        return Ok(Some((made, true)));
    }
    let length_factor = factors.length_factor();
    if trims.first.is_null() {
        let point = adaptor.value(first)?;
        trims.point1 = GeomToStepMakeCartesianPoint::from_pnt(&point, length_factor).value()?;
    }
    if trims.last.is_null() {
        let point = adaptor.value(last)?;
        trims.point2 = GeomToStepMakeCartesianPoint::from_pnt(&point, length_factor).value()?;
    }
    let points = (&trims.point1, &trims.point2);
    Ok(Some((
        make_trimmed_curve(&made, points, (trim1, trim2), true)?,
        false,
    )))
}

impl TopoDSToStepWireframeBuilder {
    /// `TopoDSToStepWireframeBuilder()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepWireframeBuilder(aShape, aTool, theLocalFactors)`.
    pub fn with_shape(
        shape: &Shape,
        tool: &mut TopoDSToStepTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(shape, tool, factors)?;
        Ok(made)
    }
    /// `init(shape, T, local_factors)`: the tool is not used.
    pub fn init(
        &mut self,
        shape: &Shape,
        _tool: &mut TopoDSToStepTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let items: TopoDSToStepWireframeItems = Handle::new(HSequence::default());
        let mut map = TopoDSToStepShapeMap::default();
        self.root.done = self.get_trimmed_curve_from_shape(shape, &mut map, &items, factors)?;
        self.my_result = items;
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepBuilderError {
        self.my_error
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<TopoDSToStepWireframeItems, Failure> {
        self.root
            .check_done("TopoDSToStepWireframeBuilder::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `get_trimmed_curve_from_edge(edge, face, map, curve_list, local_factors)`.
    pub fn get_trimmed_curve_from_edge(
        &self,
        edge: &Edge,
        face: &Face,
        map: &mut TopoDSToStepShapeMap,
        curve_list: &TopoDSToStepWireframeItems,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        if matches!(
            edge.orientation(),
            Orientation::Internal | Orientation::External
        ) {
            return Ok(false);
        }
        let forward = topods::edge(&edge.oriented(Orientation::Forward))?;
        // Only a null object failure is caught; a null edge gives it.
        let adaptor = match BRepAdaptorCurve::with_edge(&forward) {
            Ok(adaptor) => adaptor,
            Err(failure) if failure.exception_type() == "Standard_NullObject" => return Ok(false),
            Err(failure) => return Err(failure),
        };
        let mut trims = edge_trims(&forward, factors)?;
        let items = step_live(curve_list, "TopoDSToStepWireframeBuilder")?;
        let (stored, _, _) = BRepTool::curve(&forward)?;
        let curve = if stored.is_some() {
            let Some((curve, free)) = stored_curve(&forward, &adaptor, &mut trims, factors)? else {
                return Ok(false);
            };
            if free {
                items.append(curve.cast());
                return Ok(true);
            }
            curve
        } else {
            constructed_curve(&adaptor, face, &trims, factors)?
        };
        // Takes what the map has for the edge first, and overwrites it on each path but
        // the short line, where it stays.
        let curve = match (curve.is_null(), map.seek(&forward)) {
            (true, Some(known)) => known.cast::<StepGeomCurve>(),
            _ => curve,
        };
        if curve.is_null() {
            return Ok(false);
        }
        map.bind(forward.into_shape(), curve.cast());
        items.append(curve.cast());
        Ok(true)
    }
    /// `get_trimmed_curve_from_face(face, map, curve_list, local_factors)`.
    pub fn get_trimmed_curve_from_face(
        &self,
        face: &Face,
        map: &mut TopoDSToStepShapeMap,
        curve_list: &TopoDSToStepWireframeItems,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let mut result = false;
        let mut exp = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while exp.more() {
            let edge = topods::edge(exp.current())?;
            if self.get_trimmed_curve_from_edge(&edge, face, map, curve_list, factors)? {
                result = true;
            }
            exp.next()?;
        }
        Ok(result)
    }
    /// `get_trimmed_curve_from_shape(shape, map, curve_list, local_factors)`.
    pub fn get_trimmed_curve_from_shape(
        &self,
        shape: &Shape,
        map: &mut TopoDSToStepShapeMap,
        curve_list: &TopoDSToStepWireframeItems,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        match shape.shape_type()? {
            ShapeEnum::Edge => {
                let edge = topods::edge(shape)?;
                self.get_trimmed_curve_from_edge(&edge, &Face::default(), map, curve_list, factors)
            }
            ShapeEnum::Wire => {
                let mut result = false;
                let mut exp = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
                while exp.more() {
                    let edge = topods::edge(exp.current())?;
                    if self.get_trimmed_curve_from_edge(
                        &edge,
                        &Face::default(),
                        map,
                        curve_list,
                        factors,
                    )? {
                        result = true;
                    }
                    exp.next()?;
                }
                Ok(result)
            }
            ShapeEnum::Face => {
                self.get_trimmed_curve_from_face(&topods::face(shape)?, map, curve_list, factors)
            }
            ShapeEnum::Shell => {
                let mut result = false;
                let mut children = ShapeIterator::new(shape, true, true);
                while children.more() {
                    let face = topods::face(children.value()?)?;
                    if self.get_trimmed_curve_from_face(&face, map, curve_list, factors)? {
                        result = true;
                    }
                    children.next();
                }
                Ok(result)
            }
            ShapeEnum::Solid | ShapeEnum::Compound => {
                self.trimmed_curves_of_children(shape, map, curve_list, factors)
            }
            _ => Ok(false),
        }
    }
    /// A solid goes through its shells, a compound through all its children.
    fn trimmed_curves_of_children(
        &self,
        shape: &Shape,
        map: &mut TopoDSToStepShapeMap,
        curve_list: &TopoDSToStepWireframeItems,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let only_shells = shape.shape_type()? == ShapeEnum::Solid;
        let mut result = false;
        let mut children = ShapeIterator::new(shape, true, true);
        while children.more() {
            let child = children.value()?.clone();
            children.next();
            if only_shells && child.shape_type()? != ShapeEnum::Shell {
                continue;
            }
            if self.get_trimmed_curve_from_shape(&child, map, curve_list, factors)? {
                result = true;
            }
        }
        Ok(result)
    }
}
