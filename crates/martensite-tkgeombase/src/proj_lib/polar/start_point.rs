// ComputeApproxOnPolarSurface::BuildInitialCurve2d

//! The projection of the first curve point on a Bezier, B-spline or offset surface.
//! Several exact solutions (a point on a pole or a seam) are resolved by the
//! direction the curve leaves in: the first later point with a single solution
//! gives it.
use super::ComputeApproxOnPolarSurface;
use super::initial::Sampling;
use crate::extrema::ExtPS;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Direction2, Point2, Vector2};

/// The exact solutions and the nearest candidate of the first projection.
struct FirstProjection {
    sols: Vec<Point2>,
    dist2_min: f64,
    good_value: i32,
}

/// The walk state: the last parameters, the current point, and `Dist2Min` as the
/// retained code leaves it for `myDist`.
pub(super) struct WalkStart {
    pub(super) u: f64,
    pub(super) v: f64,
    pub(super) dist2_min: f64,
}

// The first ExtPS, redone with PConfusion tolerances when no extremum is near.
fn first_extrema<'a, 'b>(s: &'b mut Sampling<'a>) -> Result<&'b ExtPS<'a>, Failure> {
    let point = s.pts[1];
    let mut min_sq_dist: f64 = f64::MAX;
    {
        let ext = s.global_extrema(point)?;
        if ext.is_done() {
            for i in 1..=ext.nb_ext()? {
                min_sq_dist = min_sq_dist.min_native(ext.square_distance(i)?);
            }
        }
    }
    if min_sq_dist > s.dist_tol3d2 {
        // Try to project with less tolerance; the reduced tolerances stay for the
        // rest of the construction.
        s.tol_u = s.tol_u.min(precision::p_confusion());
        s.tol_v = s.tol_v.min(precision::p_confusion());
        return s.reinit_global_extrema(point);
    }
    Ok(s.ext_ps.as_ref().expect("ext_ps initialized above"))
}

trait NativeMin {
    fn min_native(self, other: f64) -> f64;
}
impl NativeMin for f64 {
    // `if (aSqDist < aMinSqDist) aMinSqDist = aSqDist`.
    fn min_native(self, other: f64) -> f64 {
        if other < self { other } else { self }
    }
}

// The distinct exact solutions (within 1e-9) and the nearest extremum within the
// distance tolerance.
// The negated comparisons keep the retained positive tests, so a NaN takes the retained
// branch.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn collect(ext: &ExtPS<'_>, dist_tol3d2: f64) -> Result<FirstProjection, Failure> {
    let mut first = FirstProjection {
        sols: Vec::new(),
        dist2_min: 1.0e+200,
        good_value: 1,
    };
    for i in 1..=ext.nb_ext()? {
        let sq: f64 = ext.square_distance(i)?;
        if !(sq < dist_tol3d2) {
            continue;
        }
        if sq <= 1.0e-18 {
            let (u, v) = ext.point(i)?.parameter();
            let p2d: Point2 = Point2::new(u, v);
            if !first
                .sols
                .iter()
                .any(|sol| p2d.square_distance(*sol) <= 1.0e-18)
            {
                first.sols.push(p2d);
            }
        }
        if first.dist2_min > sq {
            first.dist2_min = sq;
            first.good_value = i;
        }
    }
    Ok(first)
}

impl ComputeApproxOnPolarSurface {
    /// The first point of the extremum chain; None when the first ExtPS gives no
    /// extremum (the chain is then not started).
    pub(super) fn start_point(
        &mut self,
        s: &mut Sampling<'_>,
    ) -> Result<Option<WalkStart>, Failure> {
        let (first, good) = {
            let dist_tol3d2 = s.dist_tol3d2;
            let ext = first_extrema(s)?;
            if !(ext.is_done() && ext.nb_ext()? >= 1) {
                return Ok(None);
            }
            let first: FirstProjection = collect(ext, dist_tol3d2)?;
            let good = if first.dist2_min <= dist_tol3d2 {
                Some(ext.point(first.good_value)?.parameter())
            } else {
                None
            };
            (first, good)
        };
        let mut start = WalkStart {
            u: 0.,
            v: 0.,
            dist2_min: first.dist2_min,
        };
        if let Some(good) = good {
            let uv: (f64, f64) = if first.sols.len() > 1 {
                choose_solution(s, &first.sols, good, &mut start.dist2_min)?
            } else {
                good
            };
            (start.u, start.v) = uv;
            s.pts2d[1] = Point2::new(uv.0, uv.1);
            self.proj_is_done = true;
        }
        Ok(Some(start))
    }
}

// Several exact solutions: when they are farther apart than the parametric
// tolerance, the one the curve leaves from towards the next resolvable point.
fn choose_solution(
    s: &mut Sampling<'_>,
    sols: &[Point2],
    good: (f64, f64),
    dist2_min: &mut f64,
) -> Result<(f64, f64), Failure> {
    let mut dist2_max: f64 = -1.0e+200;
    for (i, p1) in sols.iter().enumerate() {
        for p2 in &sols[i + 1..] {
            dist2_max = dist2_max.max_native(p1.square_distance(*p2));
        }
    }
    let mut max_t2: f64 = s.tol_u.max(s.tol_v);
    max_t2 *= max_t2;
    if dist2_max <= max_t2 {
        return Ok(good);
    }
    let Some((t_pp, pp)) = single_solution_sample(s, dist2_min)? else {
        return Ok(good);
    };
    if t_pp == s.nb_of_pnts {
        return Ok(good);
    }
    let Some(pn) = next_projection(s, t_pp, dist2_min)? else {
        return Ok(good);
    };
    let at_v: Direction2 = Direction2::from_vector(Vector2::between(pp, pn))?;
    for p1 in sols {
        let as_v: Direction2 = Direction2::from_vector(Vector2::between(*p1, pp))?;
        if as_v.dot(at_v) > 0. {
            return Ok((p1.x(), p1.y()));
        }
    }
    Ok(good)
}

trait NativeMax {
    fn max_native(self, other: f64) -> f64;
}
impl NativeMax for f64 {
    // `if (aDist2 > Dist2Max) Dist2Max = aDist2`.
    fn max_native(self, other: f64) -> f64 {
        if other > self { other } else { self }
    }
}

// The first of five samples (at fifths of the curve) with exactly one extremum
// within the distance tolerance, and its projection.
fn single_solution_sample(
    s: &mut Sampling<'_>,
    dist2_min: &mut f64,
) -> Result<Option<(i32, Point2)>, Failure> {
    let nb_of_pnts = s.nb_of_pnts;
    let dist_tol3d2 = s.dist_tol3d2;
    for i in 1..=5 {
        let i_t: i32 = 1 + (nb_of_pnts - 1) / 5 * i;
        let pt = s.pts[i_t];
        let ext = s.global_extrema(pt)?;
        *dist2_min = 1.0e+200;
        let (mut nb_ext_ok, mut ind_ext): (i32, i32) = (0, 0);
        if ext.is_done() && ext.nb_ext()? >= 1 {
            for j in 1..=ext.nb_ext()? {
                let sq: f64 = ext.square_distance(j)?;
                if sq < dist_tol3d2 {
                    nb_ext_ok += 1;
                    if sq < *dist2_min {
                        (*dist2_min, ind_ext) = (sq, j);
                    }
                }
            }
        }
        if nb_ext_ok == 1 {
            let (u, v) = ext.point(ind_ext)?.parameter();
            return Ok(Some((i_t, Point2::new(u, v))));
        }
    }
    Ok(None)
}

// The projection of the first later sample with an extremum within the tolerance.
fn next_projection(
    s: &mut Sampling<'_>,
    t_pp: i32,
    dist2_min: &mut f64,
) -> Result<Option<Point2>, Failure> {
    let nb_of_pnts = s.nb_of_pnts;
    let dist_tol3d2 = s.dist_tol3d2;
    for j in t_pp + 1..=nb_of_pnts {
        let pt = s.pts[j];
        let ext = s.global_extrema(pt)?;
        *dist2_min = f64::MAX;
        if !(ext.is_done() && ext.nb_ext()? >= 1) {
            continue;
        }
        let mut ind_ext: i32 = 0;
        for i in 1..=ext.nb_ext()? {
            let sq: f64 = ext.square_distance(i)?;
            if sq < dist_tol3d2 && sq < *dist2_min {
                (*dist2_min, ind_ext) = (sq, i);
            }
        }
        if ind_ext > 0 {
            let (u, v) = ext.point(ind_ext)?.parameter();
            return Ok(Some(Point2::new(u, v)));
        }
    }
    Ok(None)
}
