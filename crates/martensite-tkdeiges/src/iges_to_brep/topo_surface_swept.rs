//! Ruled surfaces, surfaces of revolution and tabulated cylinders.
use super::{IgesToBRep, IgesToBRepTopoSurface};
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{
    IgesGeomLine, IgesGeomRuledSurface, IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder,
};
use martensite_tkbool::brep_fill;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Shape, ShapeIterator, Wire};
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCurve, GeomLine, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution, GeomTrimmedCurve,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::bsplclib;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeRevol};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_analysis::ShapeAnalysis;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use std::f64::consts::PI;

const WHERE: &str = "IgesToBRepTopoSurface";

/// The curve of the one edge of a shape, trimmed to the edge; `None` for another count of edges.
fn extract_curve_3d(edges: &Shape) -> Result<Option<Handle<dyn GeomCurve>>, Failure> {
    let mut explorer = Explorer::new(edges, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut count = 0;
    let mut found = None;
    while explorer.more() {
        let edge = topods::edge(explorer.current())?;
        explorer.next()?;
        if edge.is_null() {
            continue;
        }
        count += 1;
        found = Some(BRepTool::curve(&edge)?);
    }
    let Some((curve, first, last)) = found else {
        return Ok(None);
    };
    let Some(live) = curve.get() else {
        return Ok(None);
    };
    if count != 1 {
        return Ok(None);
    }
    if first != live.first_parameter() || last != live.last_parameter() {
        return Ok(Some(
            Handle::new(GeomTrimmedCurve::new(&curve, first, last, true, true)?).cast(),
        ));
    }
    Ok(Some(curve))
}

/// A curve as a B-spline over `[0, 1]`, cut to a range.
fn reparametrized_bspline(
    curve: Handle<dyn GeomCurve>,
    first: f64,
    last: f64,
) -> Result<Handle<GeomBSplineCurve>, Failure> {
    let live = iges_live(&curve, WHERE)?;
    let bspline = curve.cast::<GeomBSplineCurve>();
    let result = if let Some(body) = bspline.get() {
        body.segment(first, last, precision::p_confusion())?;
        bspline.clone()
    } else {
        let bounded = if live.first_parameter() < first || live.last_parameter() > last {
            Handle::new(GeomTrimmedCurve::new(&curve, first, last, true, true)?).cast()
        } else {
            curve.clone()
        };
        geom_convert::curve_to_bspline_curve(&bounded, ParameterisationType::RationalC1)?
    };
    let body = iges_live(&result, WHERE)?;
    let mut knots = body.knots();
    bsplclib::reparametrize(0.0, 1.0, &mut knots)?;
    body.set_knots(&knots)?;
    Ok(result)
}

/// Puts a curve on an edge, over `[0, 1]`.
fn set_unit_curve(
    edge: &Edge,
    curve: &Handle<GeomBSplineCurve>,
    location: &Location,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    builder.update_edge_located_curve(edge, &curve.cast(), location, precision::confusion())?;
    builder.range(edge, 0.0, 1.0, false)
}

/// A line of an edge goes over `[0, 1]`: in IGES a line is parametrised so.
fn reparam_line_of_edge(edge: &Edge) -> Result<(), Failure> {
    let carried = BRepTool::curve_with_location(edge)?;
    let copy = iges_live(&carried.curve, WHERE)?
        .copy()?
        .cast::<dyn GeomCurve>();
    if copy.cast::<GeomLine>().is_null() {
        return Ok(());
    }
    let curve = reparametrized_bspline(copy, carried.first, carried.last)?;
    set_unit_curve(edge, &curve, &carried.location)
}

/// All curves not over `[0, 1]`.
fn reparam_edge_to_unit(edge: &Edge) -> Result<(), Failure> {
    let carried = BRepTool::curve_with_location(edge)?;
    let tolerance = precision::p_confusion();
    if carried.first.abs() <= tolerance && (carried.last - 1.0).abs() <= tolerance {
        return Ok(());
    }
    let copy = iges_live(&carried.curve, WHERE)?
        .copy()?
        .cast::<dyn GeomCurve>();
    let curve = reparametrized_bspline(copy, carried.first, carried.last)?;
    set_unit_curve(edge, &curve, &carried.location)
}

/// The directrix of a ruled surface as read: the count of edges, and its wire when it has several.
struct RuledDirectrix {
    shape: Shape,
    wire: Wire,
    nb_edges: i32,
}

fn wire_of_shape(shape: &Shape) -> Result<Wire, Failure> {
    let list = ShapeExtendWireData::new();
    list.add_shape(shape, 0)?;
    list.wire()
}

impl IgesToBRepTopoSurface {
    fn surface_warn_if_c0(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
    ) -> Result<(), Failure> {
        if shape.shape_type()? != ShapeEnum::Face {
            return Ok(());
        }
        let surface = BRepTool::surface(&topods::face(shape)?)?;
        if iges_live(&surface, WHERE)?.continuity() == Continuity::C0 {
            self.warn_key(start, "IGES_1250");
        }
        Ok(())
    }
    /// One directrix with its lines over `[0, 1]`; `None` after a fail.
    fn ruled_directrix(
        &self,
        start: &Handle<IgesGeomRuledSurface>,
        curve: &Handle<IgesDataIgesEntity>,
        tool: &mut super::IgesToBRepTopoCurve,
        reversed: bool,
    ) -> Result<Option<RuledDirectrix>, Failure> {
        let mut shape = tool.transfer_topo_curve(curve)?;
        if shape.is_null() {
            self.label_fail(
                start,
                curve,
                super::package::entity_type_name(curve).as_bytes(),
            )?;
            return Ok(None);
        }
        match shape.shape_type()? {
            ShapeEnum::Edge => {
                reparam_line_of_edge(&topods::edge(&shape)?)?;
                if reversed {
                    shape.reverse();
                }
                Ok(Some(RuledDirectrix {
                    shape,
                    wire: Wire::default(),
                    nb_edges: 1,
                }))
            }
            ShapeEnum::Wire => {
                let mut wire = topods::wire(&shape)?;
                let mut nb_edges = 0;
                let mut iterator = ShapeIterator::new(&wire, true, true);
                while iterator.more() {
                    reparam_line_of_edge(&topods::edge(iterator.value()?)?)?;
                    nb_edges += 1;
                    iterator.next();
                }
                if reversed {
                    // A reversed generatrix goes as a reversed wire.
                    let list = ShapeExtendWireData::new();
                    list.add_shape(&shape, 0)?;
                    list.reverse()?;
                    wire = list.wire()?;
                }
                Ok(Some(RuledDirectrix {
                    shape,
                    wire,
                    nb_edges,
                }))
            }
            _ => Ok(None),
        }
    }
    /// The face or the shell between two directrices.
    fn ruled_between(
        &self,
        start: &Handle<IgesGeomRuledSurface>,
        live: &IgesGeomRuledSurface,
        mut first: RuledDirectrix,
        mut second: RuledDirectrix,
    ) -> Result<Option<Shape>, Failure> {
        if first.nb_edges != second.nb_edges {
            if first.nb_edges == 1 {
                first.wire = wire_of_shape(&first.shape)?;
            } else if second.nb_edges == 1 {
                second.wire = wire_of_shape(&second.shape)?;
            }
            let container = ShapeAlgo::algo_container();
            let homogeneous = iges_live(&container, WHERE)?.homo_wires(
                &first.wire,
                &second.wire,
                live.is_ruled_by_parameter(),
            )?;
            let Some((wire_1, wire_2)) = homogeneous else {
                self.fail_key(start, "IGES_1255");
                return Ok(None);
            };
            return brep_fill::ruled_shell(&wire_1, &wire_2).map(|shell| Some(shell.into_shape()));
        }
        if first.nb_edges != 1 {
            return brep_fill::ruled_shell(&first.wire, &second.wire)
                .map(|shell| Some(shell.into_shape()));
        }
        let (edge_1, edge_2) = (topods::edge(&first.shape)?, topods::edge(&second.shape)?);
        reparam_edge_to_unit(&edge_1)?;
        reparam_edge_to_unit(&edge_2)?;
        let face = brep_fill::ruled_face(&edge_1, &edge_2)?;
        self.surface_warn_if_c0(&start.cast(), &face.clone().into_shape())?;
        Ok(Some(face.into_shape()))
    }
    /// The face or the shell of a ruled surface; `tool.transfer_ruled_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_ruled_surface(
        &mut self,
        start: &Handle<IgesGeomRuledSurface>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let mut tool = self.curve_tool();
        // The continuity of the file stays.
        tool.set_continuity(0);
        let (curve_1, curve_2) = (live.first_curve(), live.second_curve());
        if curve_1.is_null() || !IgesToBRep::is_topo_curve(&curve_1) {
            self.fail_key(start, "XSTEP_148");
            return Ok(Shape::default());
        }
        let Some(first) = self.ruled_directrix(start, &curve_1, &mut tool, false)? else {
            return Ok(Shape::default());
        };
        if curve_2.is_null() || !IgesToBRep::is_topo_curve(&curve_2) {
            self.fail_key(start, "XSTEP_149");
            return Ok(Shape::default());
        }
        let reversed = live.direction_flag() == 1;
        let Some(second) = self.ruled_directrix(start, &curve_2, &mut tool, reversed)? else {
            return Ok(Shape::default());
        };
        let Some(mut result) = self.ruled_between(start, live, first, second)? else {
            return Ok(Shape::default());
        };
        if result.is_null() {
            self.fail_key(start, "IGES_1255");
            return Ok(result);
        }
        self.moved_to_location(&start.cast(), &mut result)?;
        Ok(result)
    }
    /// The surface over a generatrix of one edge: a plain revolution would change the surface
    /// under its pcurves.
    fn revolution_face(
        generatrix: &Shape,
        axis: Axis1,
        angles: (f64, f64),
    ) -> Result<Shape, Failure> {
        let Some(curve) = extract_curve_3d(generatrix)? else {
            return Ok(Shape::default());
        };
        let live = iges_live(&curve, WHERE)?;
        let surface: Handle<dyn GeomSurface> =
            Handle::new(GeomSurfaceOfRevolution::new(&curve, axis)?).cast();
        let (start_angle, end_angle) = angles;
        let is_full = end_angle - start_angle > 2.0 * PI - precision::p_confusion();
        let maker = if is_full {
            BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?
        } else {
            BRepBuilderAPIMakeFace::from_surface_bounds(
                &surface,
                (
                    0.0,
                    end_angle - start_angle,
                    live.first_parameter(),
                    live.last_parameter(),
                ),
                precision::confusion(),
            )?
        };
        if !maker.is_done() {
            return Ok(Shape::default());
        }
        Ok(maker.face()?.into_shape())
    }
    /// The face or the shell of a surface of revolution, whose axis goes against the IGES one to
    /// keep the normal; `tool.transfer_surface_of_revolution(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_surface_of_revolution(
        &mut self,
        start: &Handle<IgesGeomSurfaceOfRevolution>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let entity = live.generatrix();
        let axis_line = live.axis_of_revolution();
        if entity.is_null() || !IgesToBRep::is_topo_curve(&entity) {
            self.fail_key(start, "XSTEP_153");
            return Ok(Shape::default());
        }
        let unit = self.get_unit_factor();
        let line = entity.cast::<IgesGeomLine>();
        if let Some(body) = line.get() {
            self.u_length = body.start_point().distance(&body.end_point()) * unit;
        }
        let Some(axis_body) = axis_line.get() else {
            self.fail_key(start, "XSTEP_152");
            return Ok(Shape::default());
        };
        let mut tool = self.curve_tool();
        // The continuity of the file stays.
        tool.set_continuity(0);
        let mut generatrix = tool.transfer_topo_curve(&entity)?;
        if generatrix.is_null() {
            self.label_fail(start, &entity, b"generatrix")?;
            return Ok(Shape::default());
        }
        let origin = Point3::new(0.0, 0.0, 0.0);
        let (pt1, pt2) = (
            axis_body.transformed_start_point()?.scaled(origin, unit),
            axis_body.transformed_end_point()?.scaled(origin, unit),
        );
        let axis = Axis1::new(pt1, Direction3::from_vector(Vector3::between(pt2, pt1))?);
        let start_angle = 2.0 * PI - live.end_angle();
        let end_angle = 2.0 * PI - live.start_angle();
        let mut delta = end_angle - start_angle;
        if delta > 2.0 * PI - precision::p_confusion() {
            // CKY 18-SEP-1996
            delta = 2.0 * PI;
        }
        // The 2D curves are translated by the start angle to be in phase with the shape.
        let mut phase = Transform3::default();
        phase.set_rotation(axis, start_angle)?;
        generatrix.move_by(&Location::new(phase), false)?;
        let mut result =
            Self::revolution_face(&generatrix, axis, (start_angle, end_angle)).unwrap_or_default();
        if result.is_null() {
            result = BRepPrimAPIMakeRevol::new(&generatrix, axis, delta, false)?
                .shape()?
                .clone();
        }
        self.surface_warn_if_c0(&start.cast(), &result)?;
        self.moved_to_location(&start.cast(), &mut result)?;
        Ok(result)
    }
    /// The extrusion surface over a directrix of one edge.
    fn extrusion_face(directrix: &Shape, direction: Vector3) -> Result<Shape, Failure> {
        let Some(curve) = extract_curve_3d(directrix)? else {
            return Ok(Shape::default());
        };
        let live = iges_live(&curve, WHERE)?;
        let axis = Direction3::from_vector(direction)?;
        let surface: Handle<dyn GeomSurface> =
            Handle::new(GeomSurfaceOfLinearExtrusion::new(&curve, axis)?).cast();
        let maker = BRepBuilderAPIMakeFace::from_surface_bounds(
            &surface,
            (
                live.first_parameter(),
                live.last_parameter(),
                0.0,
                direction.magnitude(),
            ),
            precision::confusion(),
        )?;
        if !maker.is_done() {
            return Ok(Shape::default());
        }
        Ok(maker.face()?.into_shape())
    }
    /// The face or the shell of a tabulated cylinder; `tool.transfer_tabulated_cylinder(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_tabulated_cylinder(
        &mut self,
        start: &Handle<IgesGeomTabulatedCylinder>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let entity = live.directrix();
        if entity.is_null() || !IgesToBRep::is_topo_curve(&entity) {
            self.fail_key(start, "XSTEP_153");
            return Ok(Shape::default());
        }
        let mut tool = self.curve_tool();
        // The continuity of the file stays.
        tool.set_continuity(0);
        let directrix = tool.transfer_topo_curve(&entity)?;
        if directrix.is_null() {
            self.label_fail(start, &entity, b"directrix")?;
            return Ok(Shape::default());
        }
        let (first_vertex, _) = ShapeAnalysis::find_bounds(&directrix)?;
        let pt1 = BRepTool::pnt(&first_vertex)?;
        let pt2 = live
            .end_point()
            .scaled(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor());
        self.u_length = pt1.distance(&pt2);
        if self.u_length < precision::confusion() {
            let mut message = Msg::new();
            message.set_text(b"Tabulated cylinder with zero length");
            self.send_fail(start, &message);
            return Ok(Shape::default());
        }
        let direction = Vector3::between(pt1, pt2);
        let mut result = Self::extrusion_face(&directrix, direction).unwrap_or_default();
        if result.is_null() {
            result = BRepPrimAPIMakePrism::new(&directrix, direction, false, true)?
                .shape()?
                .clone();
        }
        if result.shape_type()? == ShapeEnum::Face {
            let bounds = brep_tools::uv_bounds(&topods::face(&result)?)?;
            if bounds.3 <= precision::p_confusion() && bounds.2 < -precision::p_confusion() {
                self.u_length = -self.u_length;
                result.reverse();
            }
        }
        self.surface_warn_if_c0(&start.cast(), &result)?;
        self.moved_to_location(&start.cast(), &mut result)?;
        Ok(result)
    }
}
