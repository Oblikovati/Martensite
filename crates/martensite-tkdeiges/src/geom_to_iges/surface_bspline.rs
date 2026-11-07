//! B-spline and Bezier surfaces into the rational B-spline surface entity (type 128).
use super::GeomToIgesGeomSurface;
use super::curve::EntityResult;
use super::curve_bspline::shared_array;
use super::surface::SurfaceRange;
use crate::iges_data::{iges_array2_set, iges_live, iges_new_array2};
use crate::iges_geom::IgesGeomBSplineSurface;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomBezierSurface, GeomSurface};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::gp::Xyz;
use martensite_tkshhealing::shape_analysis;

const WHERE: &str = "GeomToIgesGeomSurface";

/// One parametric range of a surface after the fix of its bounds.
struct FixedRange {
    lower: f64,
    upper: f64,
    shift: f64,
}

/// A range clipped to the bounds of a direction that is not periodic.
fn clipped_range(first: f64, last: f64, lower: f64, upper: f64) -> FixedRange {
    FixedRange {
        lower: first.max(lower),
        upper: if upper < last { upper } else { last },
        shift: 0.0,
    }
}

/// A range shifted into the period of a periodic direction, at most one period long.
fn periodic_range(first: f64, last: f64, lower: f64, upper: f64) -> FixedRange {
    let tolerance = precision::p_confusion();
    let begin = if (first - lower).abs() < tolerance {
        lower
    } else {
        first
    };
    let end = if (last - upper).abs() < tolerance {
        upper
    } else {
        last
    };
    let shift = shape_analysis::adjust_to_period(begin, lower, upper);
    let (begin, mut end) = (begin + shift, end + shift);
    if end - begin > upper - lower {
        end = begin + (upper - lower);
    }
    FixedRange {
        lower: begin,
        upper: end,
        shift,
    }
}

fn fixed_range(periodic: bool, first: f64, last: f64, bounds: (f64, f64)) -> FixedRange {
    if periodic {
        periodic_range(first, last, bounds.0, bounds.1)
    } else {
        clipped_range(first, last, bounds.0, bounds.1)
    }
}

/// Whether a range ends in another period than it starts, so the origin has to move there.
fn needs_new_origin(given_last: f64, fixed: &FixedRange, bounds: (f64, f64)) -> bool {
    let last_shift = shape_analysis::adjust_to_period(given_last, bounds.0, bounds.1);
    (fixed.shift - last_shift).abs() > precision::p_confusion()
}

/// Sets aside the periodicity of a copy, moving its origin when the range asks for it.
fn unperiodic_surface(
    surface: &GeomBSplineSurface,
    (periodic_u, periodic_v): (bool, bool),
    origins: (Option<f64>, Option<f64>),
) -> Result<(), Failure> {
    if periodic_u {
        if let Some(lower) = origins.0 {
            let (left, _) = surface.locate_u(lower, precision::p_confusion(), false)?;
            surface.set_u_origin(left)?;
        }
        surface.set_u_not_periodic()?;
    }
    if periodic_v {
        if let Some(lower) = origins.1 {
            let (left, _) = surface.locate_v(lower, precision::p_confusion(), false)?;
            surface.set_v_origin(left)?;
        }
        surface.set_v_not_periodic()?;
    }
    Ok(())
}

impl GeomToIgesGeomSurface {
    fn scaled_poles(&self, surface: &GeomBSplineSurface) -> Result<Handle<HArray2<Xyz>>, Failure> {
        let (index_u, index_v) = (surface.nb_u_poles() - 1, surface.nb_v_poles() - 1);
        let poles = iges_new_array2(0, index_u, 0, index_v)?;
        for row in 0..=index_u {
            for column in 0..=index_v {
                let pole = surface.pole(row + 1, column + 1)?;
                iges_array2_set(&poles, row, column, self.in_unit(pole.xyz()), WHERE)?;
            }
        }
        Ok(poles)
    }
    fn surface_weights(surface: &GeomBSplineSurface) -> Result<Handle<HArray2<f64>>, Failure> {
        let (index_u, index_v) = (surface.nb_u_poles() - 1, surface.nb_v_poles() - 1);
        let weights = iges_new_array2(0, index_u, 0, index_v)?;
        let rational = surface.is_u_rational() || surface.is_v_rational();
        for row in 0..=index_u {
            for column in 0..=index_v {
                let weight = if rational {
                    surface.weight(row + 1, column + 1)?
                } else {
                    1.0
                };
                iges_array2_set(&weights, row, column, weight, WHERE)?;
            }
        }
        Ok(weights)
    }
    /// The B-spline surface entity, with the knots a surface without periodicity has;
    /// `tool.transfer_bspline_surface(&surface, (0.0, 1.0, 0.0, 1.0))?`.
    ///
    /// # Errors
    /// A failure of the reading of the surface.
    pub fn transfer_bspline_surface(
        &mut self,
        start: &Handle<GeomBSplineSurface>,
        (first_u, last_u, first_v, last_v): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(original) = start.get() else {
            return Ok(Handle::default());
        };
        let periodic = (original.is_u_periodic(), original.is_v_periodic());
        let surface = GeomBSplineSurface::copy_of(original)?;
        let bounds = surface.bounds();
        let bounds_u = (bounds.0, bounds.1);
        let bounds_v = (bounds.2, bounds.3);
        let fixed_u = fixed_range(periodic.0, first_u, last_u, bounds_u);
        let fixed_v = fixed_range(periodic.1, first_v, last_v, bounds_v);
        let origin_u =
            (periodic.0 && needs_new_origin(last_u, &fixed_u, bounds_u)).then_some(fixed_u.lower);
        let origin_v =
            (periodic.1 && needs_new_origin(last_v, &fixed_v, bounds_v)).then_some(fixed_v.lower);
        unperiodic_surface(&surface, periodic, (origin_u, origin_v))?;
        let knots = (
            self.knot_array(surface.u_knot_sequence().as_slice(), surface.u_degree())?,
            self.knot_array(surface.v_knot_sequence().as_slice(), surface.v_degree())?,
        );
        let entity = IgesGeomBSplineSurface::new();
        entity.init(
            surface.nb_u_poles() - 1,
            surface.nb_v_poles() - 1,
            surface.u_degree(),
            surface.v_degree(),
            surface.is_u_closed()?,
            surface.is_v_closed()?,
            !(surface.is_u_rational() || surface.is_v_rational()),
            periodic.0,
            periodic.1,
            &knots.0,
            &knots.1,
            &Self::surface_weights(&surface)?,
            &self.scaled_poles(&surface)?,
            fixed_u.lower,
            fixed_u.upper,
            fixed_v.lower,
            fixed_v.upper,
        )?;
        Ok(Handle::new(entity).cast())
    }
    fn knot_array(&self, knots: &[f64], degree: i32) -> Result<Handle<HArray1<f64>>, Failure> {
        shared_array(knots, -degree)
    }
    /// The B-spline surface entity of a whole Bezier surface; the range does not apply;
    /// `tool.transfer_bezier_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the conversion.
    pub fn transfer_bezier_surface(&mut self, start: &Handle<GeomBezierSurface>) -> EntityResult {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let bspline = geom_convert::surface_to_bspline_surface(&start.cast())?;
        let bounds = iges_live(&bspline, WHERE)?.bounds();
        self.transfer_bspline_surface(&bspline, (bounds.0, bounds.1, bounds.2, bounds.3))
    }
}
