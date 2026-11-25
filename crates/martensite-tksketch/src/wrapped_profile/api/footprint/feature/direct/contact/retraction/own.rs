use super::interval::{I, dot, hull, norm};
use super::{AXIAL_MAX, Issuer, Refusal, Result, SideSource, finite};
use std::array::from_fn;

#[derive(Clone, Copy)]
pub struct OwnLimits {
    pub broad_comparator_upper: f64,
    pub minimum_signed_depth_rate: f64,
    pub maximum_speed: f64,
    pub maximum_u_anchor_pad: f64,
    pub xyz_anchor_pad: f64,
    pub sew_upper: f64,
    pub vertex_guard: f64,
    pub continuation_floor: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OwnProof {
    pub shape_tangent_lower: f64,
    pub native_parameter_difference_upper: f64,
    pub ratio_upper: f64,
    pub margin_lower: f64,
    pub guarded_core_upper: f64,
}

impl Issuer {
    pub fn own(&self, side: SideSource<'_>, p: OwnLimits) -> Result<OwnProof> {
        side.check()?;
        p.check()?;
        let scale = I::p(7.)
            .div(I::p(11.))
            .ok_or(Refusal::Policy)?
            .mul(I::p(self.0.radius))
            .lo;
        let sine = I::p(self.0.trig[1].hi);
        let max_radius = I::p(self.0.radius).add(I::p(AXIAL_MAX).mul(sine));
        let shape = shape_bounds(side, I::p(scale), sine, max_radius)?;
        let gamma = I::p(p.minimum_signed_depth_rate)
            .sub(I::p(p.maximum_speed).mul(I::p(p.maximum_u_anchor_pad)));
        let sigma = gamma
            .mul(I::p(shape.tangent))
            .div(I::p(p.maximum_speed))
            .ok_or(Refusal::Slopes)?;
        if sigma.lo <= 0. {
            return Err(Refusal::Slopes);
        }
        let budget = I::p(p.broad_comparator_upper);
        let span = budget
            .add(I::p(p.xyz_anchor_pad))
            .div(I::p(shape.tangent))
            .ok_or(Refusal::Slopes)?;
        let ratio = shape
            .curvature
            .mul(span)
            .add(shape.jumps)
            .div(I::p(sigma.lo))
            .ok_or(Refusal::Slopes)?;
        let margin = monotone(ratio)?;
        let core = own_core(p, ratio, margin)?;
        Ok(OwnProof {
            shape_tangent_lower: shape.tangent,
            native_parameter_difference_upper: span.hi,
            ratio_upper: ratio.hi,
            margin_lower: margin.lo,
            guarded_core_upper: core,
        })
    }
}

impl OwnLimits {
    fn check(self) -> Result<()> {
        if [
            self.broad_comparator_upper,
            self.minimum_signed_depth_rate,
            self.maximum_speed,
            self.continuation_floor,
        ]
        .into_iter()
        .any(|x| !(x.is_finite() && x > 0. && x <= 1024.))
            || [
                self.maximum_u_anchor_pad,
                self.xyz_anchor_pad,
                self.sew_upper,
                self.vertex_guard,
            ]
            .into_iter()
            .any(|x| !finite(x, 0., 1.))
            || self.sew_upper > self.xyz_anchor_pad
        {
            return Err(Refusal::Policy);
        }
        Ok(())
    }
}

fn monotone(ratio: I) -> Result<I> {
    let q = I::p(ratio.hi);
    let margin = I::p(1.).sub(I::p(2.).mul(q)).sub(q.sq());
    if !(q.lo >= 0. && margin.lo > 0. && margin.hi.is_finite()) {
        return Err(Refusal::Curvature);
    }
    Ok(margin)
}

fn own_core(p: OwnLimits, ratio: I, margin: I) -> Result<f64> {
    let g = I::p(p.vertex_guard);
    // Only the span-dependent part needs scaling. Scaling the whole
    // proven perturbation ratio conservatively includes it.
    let factor = I::p(1.).add(
        g.div(I::p(p.broad_comparator_upper))
            .ok_or(Refusal::Center)?,
    );
    let q = I::p(ratio.mul(factor).hi);
    let mu = I::p(monotone(q)?.lo.min(margin.lo));
    let residual = I::p(p.sew_upper).add(g);
    let scaled = I::p(1.).add(q).sq().div(mu).ok_or(Refusal::Center)?;
    let center = residual.mul(I::p(1.).add(scaled));
    if center.hi.partial_cmp(&p.continuation_floor) != Some(std::cmp::Ordering::Less) {
        return Err(Refusal::Center);
    }
    Ok(center.hi)
}

struct Shape {
    tangent: f64,
    curvature: I,
    jumps: I,
}

fn shape_bounds(side: SideSource<'_>, scale: I, sine: I, radius: I) -> Result<Shape> {
    let a = side.uv[0];
    let b = side.uv[side.uv.len() - 1];
    let direction = [
        I::p(b[0]).sub(I::p(a[0])).mul(scale),
        I::p(b[1]).sub(I::p(a[1])),
        I::Z,
    ];
    let length = norm(direction).ok_or(Refusal::Chord)?;
    if length.lo <= 0. {
        return Err(Refusal::Chord);
    }
    let mut tmin = f64::INFINITY;
    let mut curvature = I::Z;
    let mut jumps = I::Z;
    let mut preceding: Option<[I; 2]> = None;
    for (i, k) in side.knots.windows(2).enumerate() {
        let du = I::p(k[1]).sub(I::p(k[0]));
        let poles = side.uv[3 * i..3 * i + 4].as_chunks::<4>().0[0];
        let slopes = slopes(poles, du)?;
        let local = slopes
            .iter()
            .map(|p| dot(direction, [p[0].mul(scale), p[1], I::Z]))
            .map(|dot| {
                dot.div(I::p(length.hi))
                    .ok_or(Refusal::Slopes)
                    .map(|x| x.lo)
            })
            .try_fold(f64::INFINITY, |a, b| Ok(a.min(b?)))?;
        tmin = tmin.min(local);
        curvature = curvature.hull(span_curvature(slopes, du, sine, radius)?);
        if let Some(old) = preceding {
            let jump = [
                slopes[0][0].sub(old[0]).mul(radius),
                slopes[0][1].sub(old[1]),
                I::Z,
            ];
            jumps = jumps.add(norm(jump).ok_or(Refusal::Slopes)?);
        }
        preceding = Some(slopes[2]);
    }
    if !(tmin > 0. && tmin.is_finite()) {
        return Err(Refusal::Slopes);
    }
    Ok(Shape {
        tangent: tmin,
        curvature,
        jumps,
    })
}

fn slopes(poles: [[f64; 2]; 4], width: I) -> Result<[[I; 2]; 3]> {
    let factor = I::p(3.).div(width).ok_or(Refusal::Slopes)?;
    Ok(from_fn(|i| {
        from_fn(|a| I::p(poles[i + 1][a]).sub(I::p(poles[i][a])).mul(factor))
    }))
}

fn span_curvature(slopes: [[I; 2]; 3], width: I, sine: I, radius: I) -> Result<I> {
    let factor = I::p(2.).div(width).ok_or(Refusal::Slopes)?;
    let speed = |a| I::p(hull(&slopes.map(|d| d[a])).abs());
    let accel = |a: usize| {
        I::p(
            slopes[1][a]
                .sub(slopes[0][a])
                .hull(slopes[2][a].sub(slopes[1][a]))
                .mul(factor)
                .abs(),
        )
    };
    let u = speed(0);
    let v = speed(1);
    Ok(accel(1)
        .add(I::p(2.).mul(sine).mul(u).mul(v))
        .add(radius.mul(u.sq().add(accel(0)))))
}
