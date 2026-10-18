//! The Rust twin of the implementation.
use super::common::{Reader, attempt, end};
use super::geom_convert::{curve as print_curve, curve2d as print_curve2d};
use super::geometry_record_fixtures::{ax3, curve, surface};
use super::print::{Output, real};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomPlane, GeomSurface, SurfaceBounds};
use martensite_tkgeombase::geom_proj_lib as gpl;

/// The fields of a `gpl_curve2d` request before the curve and the surface.
struct Curve2dRequest {
    variant: i32,
    range: (f64, f64),
    bounds: SurfaceBounds,
    tol: f64,
}

// The overload of `request.variant`, and the tolerance reached when it has one.
fn curve2d_of(
    request: &Curve2dRequest,
    c: &Handle<dyn GeomCurve>,
    s: &Handle<dyn GeomSurface>,
) -> Result<(Handle<dyn Geom2dCurve>, Option<f64>), Failure> {
    let (range, bounds, tol) = (request.range, request.bounds, request.tol);
    Ok(match request.variant {
        0 => with_tol(gpl::curve2d_range_bounds_tolerance(
            c, range, s, bounds, tol,
        )?),
        1 => with_tol(gpl::curve2d_range_tolerance(c, range, s, tol)?),
        2 => (gpl::curve2d_range(c, range, s)?, None),
        3 => (gpl::curve2d(c, s)?, None),
        4 => (gpl::curve2d_bounds(c, s, bounds)?, None),
        _ => with_tol(gpl::curve2d_bounds_tolerance(c, s, bounds, tol)?),
    })
}

fn with_tol(result: gpl::Curve2dWithTolerance) -> (Handle<dyn Geom2dCurve>, Option<f64>) {
    (result.0, Some(result.1))
}

fn read_curve2d_request(r: &mut Reader<'_>) -> Curve2dRequest {
    let variant = r.integer();
    let range = (r.real(), r.real());
    let bounds = (r.real(), r.real(), r.real(), r.real());
    Curve2dRequest {
        variant,
        range,
        bounds,
        tol: r.real(),
    }
}

/// gb_gpl_curve2d_case.
pub fn curve2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let request = read_curve2d_request(&mut r);
    attempt(output, || {
        let (c, s) = (curve(&mut r)?, surface(&mut r)?);
        let (projected, reached) = curve2d_of(&request, &c, &s)?;
        print_curve2d(output, &projected);
        reached.into_iter().for_each(|value| real(output, value));
        Ok(())
    });
    end(output);
}

/// gb_gpl_project_case.
pub fn project_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let (c, s) = (curve(&mut r)?, surface(&mut r)?);
        print_curve(output, &gpl::project(&c, &s)?);
        Ok(())
    });
    end(output);
}

/// gb_gpl_on_plane_case.
pub fn on_plane_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let keep = r.flag();
    let (pl, d) = (ax3(&mut r), r.dir());
    attempt(output, || {
        let c = curve(&mut r)?;
        let plane: Handle<GeomPlane> = Handle::new(GeomPlane::new(pl));
        print_curve(output, &gpl::project_on_plane(&c, &plane, d, keep)?);
        Ok(())
    });
    end(output);
}
