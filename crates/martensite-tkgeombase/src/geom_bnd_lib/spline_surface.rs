//! `BezierSurface` and `BSplineSurface`: pole-hull boxes, the
//! grid fallback and the optimal box.
use super::live;
use super::patch::Patch;
use super::sampling::{compute_nb_u_samples_full, compute_nb_v_samples_full};
use super::spline::compute_pole_index_range;
use super::surface_sampling::{grid_box, surface_box_optimal};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, exceptions};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomBezierSurface, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkmath::bnd::BoundingBox3;

/// A `GeomAdaptorSurface` on a typed surface handle; `surface_adaptor_of(&bezier)?`.
pub(super) fn surface_adaptor_of<T: GeomSurface + RuntimeTyped>(
    surface: &Handle<T>,
) -> Result<GeomAdaptorSurface, Failure> {
    GeomAdaptorSurface::with_surface(&surface.cast::<dyn GeomSurface>())
}

// The grid fallback of both classes: the FULL-range sample counts on the patch (the
// ranged counts are OtherSurface's).
fn full_count_grid(
    adaptor: &GeomAdaptorSurface,
    patch: Patch,
    tolerance: f64,
) -> Result<BoundingBox3, Failure> {
    let adaptor: &dyn Adaptor3dSurface = adaptor;
    let counts: (i32, i32) = (
        compute_nb_u_samples_full(adaptor)?,
        compute_nb_v_samples_full(adaptor)?,
    );
    grid_box(adaptor, patch, counts, tolerance)
}

fn optimal(
    adaptor: &GeomAdaptorSurface,
    patch: Patch,
    tolerance: f64,
) -> Result<BoundingBox3, Failure> {
    surface_box_optimal(adaptor, patch, tolerance)
}

/// `BezierSurface`: the box of a `BezierSurface` patch.
pub struct BezierSurface {
    geom: Handle<GeomBezierSurface>,
}

impl BezierSurface {
    /// The evaluator of `surface`; `BezierSurface::new(handle)`.
    pub fn new(surface: Handle<GeomBezierSurface>) -> Self {
        Self { geom: surface }
    }
    /// `Geometry()`; `evaluator.geometry()`.
    pub fn geometry(&self) -> &Handle<GeomBezierSurface> {
        &self.geom
    }
    /// `Box(Tol)`: the hull of all poles; `evaluator.bounding_box(0.0)?`.
    pub fn bounding_box(&self, tolerance: f64) -> Result<BoundingBox3, Failure> {
        let mut bounds: BoundingBox3 = BoundingBox3::default();
        let poles = live(&self.geom, "GeomBndLib_BezierSurface")?.poles();
        poles.iter().for_each(|&pole| bounds.add_point(pole));
        bounds.enlarge(tolerance);
        Ok(bounds)
    }
    /// `Box(UMin, UMax, VMin, VMax, Tol)`: the pole hull for the natural bounds (within
    /// `Parametric(Confusion)`), otherwise the grid of the full-range sample counts;
    /// `evaluator.bounding_box_range(0.0, 0.5, 0.0, 1.0, 0.0)?`.
    pub fn bounding_box_range(
        &self,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        let tol: f64 = precision::parametric(precision::confusion());
        let (u1, u2, v1, v2) = live(&self.geom, "GeomBndLib_BezierSurface")?.bounds();
        if (u_min - u1).abs() <= tol
            && (v_min - v1).abs() <= tol
            && (u_max - u2).abs() <= tol
            && (v_max - v2).abs() <= tol
        {
            return self.bounding_box(tolerance);
        }
        full_count_grid(
            &surface_adaptor_of(&self.geom)?,
            Patch {
                u_min,
                u_max,
                v_min,
                v_max,
            },
            tolerance,
        )
    }
    /// `BoxOptimal(Tol)`: the natural bounds; `evaluator.bounding_box_optimal(0.0)?`.
    pub fn bounding_box_optimal(&self, tolerance: f64) -> Result<BoundingBox3, Failure> {
        let (u1, u2, v1, v2) = live(&self.geom, "GeomBndLib_BezierSurface")?.bounds();
        self.bounding_box_optimal_range(u1, u2, v1, v2, tolerance)
    }
    /// `BoxOptimal(UMin, UMax, VMin, VMax, Tol)`: the optimal surface box;
    /// `evaluator.bounding_box_optimal_range(0.0, 1.0, 0.0, 1.0, 0.0)?`.
    pub fn bounding_box_optimal_range(
        &self,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        optimal(
            &surface_adaptor_of(&self.geom)?,
            Patch {
                u_min,
                u_max,
                v_min,
                v_max,
            },
            tolerance,
        )
    }
}

/// `BSplineSurface`: the box of a `BSplineSurface` patch.
pub struct BSplineSurface {
    geom: Handle<GeomBSplineSurface>,
}

// The pole index range of one direction: all poles unless the patch is strictly inside
// the bounds, then `ComputePolesIndexes`.
struct PoleSpan<'a> {
    knots: martensite_tkernel::ncollection::Array1<'a, f64>,
    mults: martensite_tkernel::ncollection::Array1<'a, i32>,
    degree: i32,
    nb_poles: i32,
    periodic: bool,
}

impl PoleSpan<'_> {
    fn range(
        &self,
        (min, max): (f64, f64),
        (first, last): (f64, f64),
    ) -> Result<(i32, i32), Failure> {
        if min > first || max < last {
            return compute_pole_index_range(
                &self.knots,
                &self.mults,
                self.degree,
                (min, max),
                self.nb_poles,
                self.periodic,
            );
        }
        Ok((1, self.nb_poles))
    }
}

impl BSplineSurface {
    /// The evaluator of `surface`; `BSplineSurface::new(handle)`.
    pub fn new(surface: Handle<GeomBSplineSurface>) -> Self {
        Self { geom: surface }
    }
    /// `Geometry()`; `evaluator.geometry()`.
    pub fn geometry(&self) -> &Handle<GeomBSplineSurface> {
        &self.geom
    }
    fn surface(&self) -> Result<&GeomBSplineSurface, Failure> {
        live(&self.geom, "GeomBndLib_BSplineSurface")
    }
    /// `Box(Tol)`: the natural bounds; `evaluator.bounding_box(0.0)?`.
    pub fn bounding_box(&self, tolerance: f64) -> Result<BoundingBox3, Failure> {
        let (u1, u2, v1, v2) = self.surface()?.bounds();
        self.bounding_box_range(u1, u2, v1, v2, tolerance)
    }
    /// `Box(UMin, UMax, VMin, VMax, Tol)`: inside the bounds (within
    /// `Parametric(Confusion)`), the hull of the poles covering the patch (periodic
    /// indices wrap); outside, the grid of the full-range sample counts;
    /// `evaluator.bounding_box_range(0.0, 0.5, 0.0, 1.0, 0.0)?`.
    pub fn bounding_box_range(
        &self,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        let tol: f64 = precision::parametric(precision::confusion());
        let (u1, u2, v1, v2) = self.surface()?.bounds();
        let outside: bool =
            (u_min - u1) < -tol || (v_min - v1) < -tol || (u_max - u2) > tol || (v_max - v2) > tol;
        let patch: Patch = Patch {
            u_min,
            u_max,
            v_min,
            v_max,
        };
        if outside {
            return full_count_grid(&surface_adaptor_of(&self.geom)?, patch, tolerance);
        }
        self.pole_box(patch, (u1, u2, v1, v2), tolerance)
    }
    fn pole_box(
        &self,
        patch: Patch,
        (u1, u2, v1, v2): (f64, f64, f64, f64),
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        let surface = self.surface()?;
        let nb_u: i32 = surface.nb_u_poles();
        let nb_v: i32 = surface.nb_v_poles();
        let full_u: bool = !(patch.u_min > u1 || patch.u_max < u2);
        let full_v: bool = !(patch.v_min > v1 || patch.v_max < v2);
        let poles = surface.poles();
        let mut bounds: BoundingBox3 = BoundingBox3::default();
        if full_u && full_v {
            poles.iter().for_each(|&pole| bounds.add_point(pole));
            bounds.enlarge(tolerance);
            return Ok(bounds);
        }
        let u_periodic: bool = surface.is_u_periodic();
        let v_periodic: bool = surface.is_v_periodic();
        let (i_min, i_max) = if full_u {
            (1, nb_u)
        } else {
            let u = PoleSpan {
                knots: surface.u_knots(),
                mults: surface.u_multiplicities(),
                degree: surface.u_degree(),
                nb_poles: nb_u,
                periodic: u_periodic,
            };
            u.range((patch.u_min, patch.u_max), (u1, u2))?
        };
        let (j_min, j_max) = if full_v {
            (1, nb_v)
        } else {
            let v = PoleSpan {
                knots: surface.v_knots(),
                mults: surface.v_multiplicities(),
                degree: surface.v_degree(),
                nb_poles: nb_v,
                periodic: v_periodic,
            };
            v.range((patch.v_min, patch.v_max), (v1, v2))?
        };
        let wrap = |index: i32, nb: i32, periodic: bool| -> i32 {
            if periodic && index > nb {
                index - nb
            } else {
                index
            }
        };
        for i in i_min..=i_max {
            let wi: i32 = wrap(i, nb_u, u_periodic);
            for j in j_min..=j_max {
                // A pole index still past the array after one wrap is index-range failure.
                let pole = poles
                    .value(wi, wrap(j, nb_v, v_periodic))
                    .map_err(|e| exceptions::out_of_range(&format!("pole ({i}, {j}): {e}")))?;
                bounds.add_point(*pole);
            }
        }
        bounds.enlarge(tolerance);
        Ok(bounds)
    }
    /// `BoxOptimal(Tol)`: the natural bounds; `evaluator.bounding_box_optimal(0.0)?`.
    pub fn bounding_box_optimal(&self, tolerance: f64) -> Result<BoundingBox3, Failure> {
        let (u1, u2, v1, v2) = self.surface()?.bounds();
        self.bounding_box_optimal_range(u1, u2, v1, v2, tolerance)
    }
    /// `BoxOptimal(UMin, UMax, VMin, VMax, Tol)`: the optimal surface box;
    /// `evaluator.bounding_box_optimal_range(0.0, 1.0, 0.0, 1.0, 0.0)?`.
    pub fn bounding_box_optimal_range(
        &self,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        optimal(
            &surface_adaptor_of(&self.geom)?,
            Patch {
                u_min,
                u_max,
                v_min,
                v_max,
            },
            tolerance,
        )
    }
}
