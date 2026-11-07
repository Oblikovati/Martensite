//! The pcurve of an edge in the parameters an IGES surface has.
use super::br_wire::origin_of;
use super::{BRepToIgesBRWire, BRepToIgesOriginMap};
use crate::geom2d_to_iges::Geom2dToIgesGeom2dCurve;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use martensite_cmath::sin;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomConicalSurface, GeomCylindricalSurface, GeomLine, GeomOffsetSurface,
    GeomPlane, GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface,
    GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution, GeomSweptSurface, GeomToroidalSurface,
    GeomTrimmedCurve,
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Axis2d, Direction2, Point2, Transform2, Vector2};
use martensite_tkshhealing::shape_analysis;
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use std::f64::consts::PI;

const WHERE: &str = "BRepToIgesBRWire";

/// The kinds of surface the conventions depend on.
struct SurfaceKinds {
    is_cylinder: bool,
    is_cone: bool,
    is_sphere: bool,
    is_torus: bool,
    is_revolution: bool,
    is_extrusion: bool,
    is_plane: bool,
}

impl SurfaceKinds {
    fn of(surface: &Handle<dyn GeomSurface>) -> Self {
        Self {
            is_cylinder: surface.cast::<GeomCylindricalSurface>().is_some(),
            is_cone: surface.cast::<GeomConicalSurface>().is_some(),
            is_sphere: surface.cast::<GeomSphericalSurface>().is_some(),
            is_torus: surface.cast::<GeomToroidalSurface>().is_some(),
            is_revolution: surface.cast::<GeomSurfaceOfRevolution>().is_some(),
            is_extrusion: surface.cast::<GeomSurfaceOfLinearExtrusion>().is_some(),
            is_plane: surface.cast::<GeomPlane>().is_some(),
        }
    }
}

/// The surface under a trim and an offset, as the entity of the surface has it.
fn underlying_surface(surface: &Handle<dyn GeomSurface>) -> Handle<dyn GeomSurface> {
    let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
    let basis = trimmed.get().map_or_else(
        || surface.clone(),
        GeomRectangularTrimmedSurface::basis_surface,
    );
    let offset = surface.cast::<GeomOffsetSurface>();
    offset.get().map_or(basis, GeomOffsetSurface::basis_surface)
}

/// A revolution of a line has its V along the line, from the start of its range.
fn revolves_a_line(surface: &Handle<dyn GeomSurface>) -> bool {
    let revolution = surface.cast::<GeomSurfaceOfRevolution>();
    let Some(live) = revolution.get() else {
        return false;
    };
    let curve = live.basis_curve();
    let trimmed = curve.cast::<GeomTrimmedCurve>();
    let basis = trimmed
        .get()
        .map_or_else(|| curve.clone(), GeomTrimmedCurve::basis_curve);
    basis.cast::<GeomLine>().is_some()
}

/// The shift that takes a pcurve from the bounds of a periodic B-spline to those of the face.
fn periodic_shift(
    surface: &Handle<dyn GeomSurface>,
    (u_first, v_first): (f64, f64),
) -> Result<(f64, f64), Failure> {
    let bspline = surface.cast::<GeomBSplineSurface>();
    let Some(live) = bspline.get() else {
        return Ok((0.0, 0.0));
    };
    let bounds = iges_live(surface, WHERE)?.bounds();
    let tolerance = precision::p_confusion();
    let u_shift = if live.is_u_periodic() && (u_first - bounds.0).abs() > tolerance {
        shape_analysis::adjust_to_period(u_first, bounds.0, bounds.1)
    } else {
        0.0
    };
    let v_shift = if live.is_v_periodic() && (v_first - bounds.2).abs() > tolerance {
        shape_analysis::adjust_to_period(v_first, bounds.2, bounds.3)
    } else {
        0.0
    };
    Ok((u_shift, v_shift))
}

/// Swaps U and V and goes against V over a period.
fn swap_parameters(curve: &dyn Geom2dCurve) -> Result<(), Failure> {
    let origin = Point2::new(0.0, 0.0);
    curve.mirror_axis(Axis2d::new(origin, Direction2::new(1.0, 1.0)?))?;
    curve.mirror_axis(Axis2d::new(origin, Direction2::new(1.0, 0.0)?))?;
    curve.translate(Vector2::new(0.0, 2.0 * PI))
}

impl BRepToIgesBRWire {
    /// A copy of the pcurve, shifted to the origin of the entity of the surface.
    fn shifted_pcurve(
        curve: &Handle<dyn Geom2dCurve>,
        surface: &Handle<dyn GeomSurface>,
        (u_first, v_first): (f64, f64),
        need_shift: bool,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let live = iges_live(curve, WHERE)?;
        let mut placed: Handle<dyn Geom2dCurve> = if need_shift {
            let mut shift = Transform2::default();
            shift.set_translation_between(Point2::new(0.0, 0.0), Point2::new(0.0, -v_first));
            live.transformed(shift)?.cast()
        } else {
            live.copy()?.cast()
        };
        let (u_shift, v_shift) = periodic_shift(surface, (u_first, v_first))?;
        let tolerance = precision::p_confusion();
        if u_shift.abs() > tolerance || v_shift.abs() > tolerance {
            let mut shift = Transform2::default();
            shift.set_translation_between(Point2::new(0.0, 0.0), Point2::new(u_shift, v_shift));
            placed = iges_live(&placed, WHERE)?.transformed(shift)?.cast();
        }
        Ok(placed)
    }
    /// The pcurve of an edge as an entity, null on a plane, without pcurves, or for a degenerated
    /// edge out of BRep mode; `tool.transfer_edge_on_face(&edge, &face, &origins, 1.0, false)?`.
    ///
    /// # Errors
    /// A failure of the curves of the edge or of the binding of the result.
    pub fn transfer_edge_on_face(
        &self,
        edge: &Edge,
        face: &Face,
        origins: &BRepToIgesOriginMap,
        length: f64,
        b_rep_mode: bool,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if edge.is_null()
            || !self.get_p_curve_mode()
            || (!b_rep_mode && BRepTool::degenerated(edge)?)
        {
            return Ok(Handle::default());
        }
        let carried = BRepTool::curve_on_face(edge, face)?;
        let mut result = Handle::default();
        if carried.pcurve.is_some() {
            let (surface, _) = BRepTool::surface_with_location(face)?;
            if surface.cast::<GeomPlane>().is_some() {
                return Ok(Handle::default());
            }
            result = self.pcurve_entity(
                edge,
                (&carried.pcurve, carried.first, carried.last),
                (&surface, brep_tools::uv_bounds(face)?),
                length,
                b_rep_mode,
            )?;
        }
        self.set_shape_result(&origin_of(edge, origins)?, &result.cast())?;
        Ok(result)
    }
    fn pcurve_entity(
        &self,
        edge: &Edge,
        (pcurve, first, last): (&Handle<dyn Geom2dCurve>, f64, f64),
        (located, bounds): (&Handle<dyn GeomSurface>, brep_tools::UVBounds),
        length: f64,
        b_rep_mode: bool,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let analytic = !self.get_convert_surface_mode() && b_rep_mode;
        let surface = underlying_surface(located);
        let kinds = SurfaceKinds::of(&surface);
        let need_shift =
            (!analytic && (kinds.is_cylinder || kinds.is_cone)) || revolves_a_line(&surface);
        let curve = Self::shifted_pcurve(pcurve, &surface, (bounds.0, bounds.2), need_shift)?;
        let live = iges_live(&curve, WHERE)?;
        if !analytic && (kinds.is_cylinder || kinds.is_cone || kinds.is_sphere) {
            swap_parameters(live)?;
        }
        if kinds.is_revolution || kinds.is_torus {
            swap_parameters(live)?;
        }
        let (mut first, mut last) = (first, last);
        if analytic && (kinds.is_sphere || kinds.is_torus) {
            let mut scale = Transform2::default();
            scale.set_scale(Point2::new(0.0, 0.0), 180.0 / PI);
            live.transform(scale)?;
            first = live.transformed_parameter(first, scale);
            last = live.transformed_parameter(last, scale);
        }
        if analytic && kinds.is_cone {
            let cone = surface.cast::<GeomConicalSurface>();
            let body = iges_live(&cone, WHERE)?;
            if body.semi_angle() < 0.0 {
                let apex = 2.0 * body.ref_radius() / sin(body.semi_angle());
                live.translate(Vector2::new(0.0, apex))?;
            }
        }
        let scaled_length = if analytic && (kinds.is_cylinder || kinds.is_cone) {
            PI / 180.0
        } else {
            length
        };
        let (mut curve, mut first, mut last) = self.scaled_pcurve(
            &curve,
            (&surface, &kinds, bounds),
            scaled_length,
            b_rep_mode,
            (first, last),
        )?;
        if edge.orientation() == Orientation::Reversed {
            let body = iges_live(&curve, WHERE)?;
            let reversed = (
                body.reversed_parameter(last)?,
                body.reversed_parameter(first)?,
            );
            body.reverse()?;
            (first, last) = reversed;
            curve = curve.clone();
        }
        let mut tool = Geom2dToIgesGeom2dCurve::new();
        tool.set_model(&self.get_model())?;
        tool.transfer_2d_curve(&curve, first, last)
    }
    /// The scale of the parameters of the entity of the surface.
    fn scaled_pcurve(
        &self,
        curve: &Handle<dyn Geom2dCurve>,
        (surface, kinds, bounds): (
            &Handle<dyn GeomSurface>,
            &SurfaceKinds,
            brep_tools::UVBounds,
        ),
        length: f64,
        b_rep_mode: bool,
        (first, last): (f64, f64),
    ) -> Result<(Handle<dyn Geom2dCurve>, f64, f64), Failure> {
        let mut trans = Transform2::default();
        let origin = Point2::new(0.0, 0.0);
        let mut u_fact = 1.0;
        let surface_bounds = iges_live(surface, WHERE)?.bounds();
        let span_u = surface_bounds.1 - surface_bounds.0;
        let span_v = bounds.3 - bounds.2;
        if b_rep_mode && kinds.is_plane {
            trans.set_scale(origin, 1.0 / self.get_unit());
        }
        if kinds.is_extrusion {
            trans.set_scale(origin, 1.0 / span_v);
            u_fact = span_v / span_u;
        }
        if kinds.is_cylinder || kinds.is_cone || kinds.is_revolution {
            u_fact = 1.0 / length;
        }
        let made = ShapeBuildEdge.transform_pcurve(curve, trans, u_fact, first, last)?;
        if !kinds.is_extrusion {
            return Ok(made);
        }
        let mut shift = Transform2::default();
        shift.set_translation(Vector2::new(-surface_bounds.0 / span_u, -bounds.2 / span_v));
        ShapeBuildEdge.transform_pcurve(&made.0, shift, 1.0, made.1, made.2)
    }
}
