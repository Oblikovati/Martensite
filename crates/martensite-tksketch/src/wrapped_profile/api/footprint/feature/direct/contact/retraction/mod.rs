//! Numeric evidence for native conical attachment retractions.
mod capture;
use super::interval;
mod own;

use interval::{I, dot, norm, smooth_trig};
pub use own::OwnLimits;

const MAX: usize = 10;
const AXIAL_MAX: f64 = 30.;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Source,
    Window,
    Anchor,
    Linear,
    Curvature,
    Center,
    Chord,
    Slopes,
    Policy,
}
pub type Result<T> = std::result::Result<T, Refusal>;

#[derive(Clone, Copy)]
pub struct ConeSource {
    pub angle: f64,
    pub radius: f64,
    pub origin_z: f64,
}

#[derive(Clone, Copy)]
pub struct SideSource<'a> {
    pub affine: [f64; 13],
    pub first: &'a [[f64; 4]],
    pub last: &'a [[f64; 4]],
    pub knots: &'a [f64],
    pub uv: &'a [[f64; 2]],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endpoint {
    First,
    Last,
}

impl Endpoint {
    fn index(self, index: usize, len: usize) -> usize {
        match self {
            Self::First => index,
            Self::Last => len - 1 - index,
        }
    }
}

#[derive(Clone, Copy)]
pub struct IncidentEnd<'a> {
    pub side: SideSource<'a>,
    pub piece_index: usize,
    pub end: Endpoint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArmProof {
    pub linear_min_lower: f64,
    pub linear_sum_upper: f64,
    pub quadratic_upper: f64,
    pub derivative_remainder_upper: f64,
    pub anchor_upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectedProof {
    pub mixed: ArmProof,
    pub mixed_margin_lower: f64,
    pub mixed_guarded_core_upper: f64,
    pub native_margin_lower: f64,
}

#[derive(Clone, Copy)]
pub struct IncidentLimits {
    pub window_fraction: f64,
    pub depth_upper: f64,
    pub vertex_guard: f64,
    pub continuation_floor: f64,
}

struct Datum<'a> {
    cut: I,
    location: [f64; 13],
    first: &'a [[f64; 4]],
    last: &'a [[f64; 4]],
    pcurve: [[f64; 2]; 4],
    end: Endpoint,
}

pub struct Issuer(Stock);
struct Stock {
    radius: f64,
    z: f64,
    trig: [I; 2],
}
#[derive(Clone, Copy)]
struct Geometry {
    origin: [I; 3],
    v: [I; 3],
    k: f64,
}
#[derive(Clone, Copy)]
struct Arm {
    native: Geometry,
    g: [I; 3],
    gp: f64,
    gpp: f64,
}

impl Issuer {
    pub fn new(source: ConeSource) -> Result<Self> {
        if !finite(source.angle, 0.001, 1.5)
            || !finite(source.radius, 0.01, 1024.)
            || !finite(source.origin_z, -1024., 1024.)
        {
            return Err(Refusal::Source);
        }
        let trig = smooth_trig(source.angle).ok_or(Refusal::Source)?;
        Ok(Self(Stock {
            radius: source.radius,
            z: source.origin_z,
            trig,
        }))
    }

    pub fn incident(
        &self,
        ends: [IncidentEnd<'_>; 2],
        policy: IncidentLimits,
    ) -> Result<[DirectedProof; 2]> {
        policy.check()?;
        let [a, b] = [
            ends[0].datum(policy.window_fraction)?,
            ends[1].datum(policy.window_fraction)?,
        ];
        let a0 = a.first[a.end.index(0, a.first.len())];
        let b0 = b.first[b.end.index(0, b.first.len())];
        if a.location != b.location || a0[..3] != b0[..3] || a.pcurve[0] != b.pcurve[0] {
            return Err(Refusal::Anchor);
        }
        let [arm_a, arm_b] = [capture::arm(&a)?, capture::arm(&b)?];
        let [img_a, img_b] = [
            capture::cone_image(&self.0, &a, policy.window_fraction)?,
            capture::cone_image(&self.0, &b, policy.window_fraction)?,
        ];
        Ok([
            directed(arm_a, arm_b, img_b, policy)?,
            directed(arm_b, arm_a, img_a, policy)?,
        ])
    }
}

impl IncidentLimits {
    fn check(self) -> Result<()> {
        if !(self.window_fraction > 0. && self.window_fraction < 1.)
            || !(self.depth_upper > 0. && self.depth_upper < 1.)
            || !finite(self.vertex_guard, 0., 1.)
            || !(self.continuation_floor > 0. && self.continuation_floor <= 1.)
        {
            return Err(Refusal::Window);
        }
        Ok(())
    }
}

impl<'a> IncidentEnd<'a> {
    fn datum(self, width: f64) -> Result<Datum<'a>> {
        self.side.check()?;
        let n = self.side.knots.len() - 1;
        if self.piece_index != self.end.index(0, n) {
            return Err(Refusal::Window);
        }
        let p = self.piece_index;
        let span = I::p(self.side.knots[p + 1]).sub(I::p(self.side.knots[p]));
        let cut = if n == 1 {
            I::p(width)
        } else {
            span.mul(I::p(width))
        };
        if !(cut.lo > 0. && cut.hi <= 1.) {
            return Err(Refusal::Window);
        }
        let pcurve = std::array::from_fn(|i| self.side.uv[3 * p + self.end.index(i, 4)]);
        Ok(Datum {
            cut,
            location: self.side.affine,
            first: self.side.first,
            last: self.side.last,
            pcurve,
            end: self.end,
        })
    }
}

impl SideSource<'_> {
    fn check(self) -> Result<()> {
        if self.affine[0] != 1.
            || self.affine.iter().any(|&v| !finite(v, -1024., 1024.))
            || !(2..=MAX).contains(&self.first.len())
            || self.first.len() != self.last.len()
            || !(2..=128).contains(&self.knots.len())
            || self.knots.first() != Some(&0.)
            || self.knots.last() != Some(&1.)
            || self
                .knots
                .windows(2)
                .any(|w| w[0].partial_cmp(&w[1]) != Some(std::cmp::Ordering::Less))
            || self.uv.len() != 3 * (self.knots.len() - 1) + 1
            || self
                .uv
                .iter()
                .any(|[u, v]| !finite(*u, 0., 3.) || !finite(*v, 0., AXIAL_MAX))
        {
            return Err(Refusal::Source);
        }
        for (a, b) in self.first.iter().zip(self.last) {
            if a[3].to_bits() != b[3].to_bits()
                || !finite(a[3], 0.001, 1024.)
                || a[..3]
                    .iter()
                    .chain(&b[..3])
                    .any(|&v| !finite(v, -1024., 1024.))
            {
                return Err(Refusal::Source);
            }
        }
        Ok(())
    }
}

fn finite(x: f64, lo: f64, hi: f64) -> bool {
    (lo..=hi).contains(&x)
}
fn positive(x: I) -> Result<I> {
    if x.lo > 0. && x.hi.is_finite() {
        Ok(x)
    } else {
        Err(Refusal::Linear)
    }
}
fn vector_norm(x: [I; 3]) -> Result<I> {
    norm(x).ok_or(Refusal::Linear)
}
fn ray_min_sq(a: [I; 3], b: [I; 3]) -> Result<I> {
    let [aa, bb, ab] = [dot(a, a), dot(b, b), dot(a, b)];
    let opt = ab.div(positive(bb)?).ok_or(Refusal::Linear)?;
    let opt = I {
        lo: opt.lo.clamp(0., 1.),
        hi: opt.hi.clamp(0., 1.),
    };
    positive(aa.sub(I::p(2.).mul(opt).mul(ab)).add(opt.sq().mul(bb)))
}

fn proof(source: Arm, target: Geometry, depth: f64, embedding: bool) -> Result<ArmProof> {
    let ruling = source.g.map(|value| value.mul(I::p(depth)));
    let k = I::p(source.native.k)
        .add(I::p(target.k))
        .add(I::p(depth).mul(I::p(source.gpp).add(I::p(2.).mul(I::p(source.gp)))));
    let tangent = ray_min_sq(source.native.v, target.v)?
        .lo
        .min(ray_min_sq(target.v, source.native.v)?.lo);
    let radial = dot(ruling, ruling).lo;
    let cross = I::p(dot(ruling, source.native.v).abs()).add(I::p(dot(ruling, target.v).abs()));
    let m = positive(I::p(tangent.min(radial)).sub(cross))?
        .sqrt()
        .ok_or(Refusal::Linear)?;
    let l = vector_norm(source.native.v)?
        .add(vector_norm(target.v)?)
        .add(vector_norm(ruling)?);
    let origin = std::array::from_fn(|i| source.native.origin[i].sub(target.origin[i]));
    let error = if embedding {
        0.
    } else {
        vector_norm(origin)?.hi
    };
    // The intervals already enclose the exact native and cone tangents.
    // Because the proof takes extrema over them, it needs no tangent midpoint.
    Ok(ArmProof {
        linear_min_lower: m.lo,
        linear_sum_upper: l.hi,
        quadratic_upper: I::p(0.5).mul(k).hi,
        derivative_remainder_upper: k.hi,
        anchor_upper: error,
    })
}

impl ArmProof {
    fn margin(self) -> Result<I> {
        let m = I::p(self.linear_min_lower);
        let q = I::p(self.quadratic_upper);
        let r = I::p(self.derivative_remainder_upper);
        if I::p(2.).mul(m).lo < q.add(r).hi {
            return Err(Refusal::Curvature);
        }
        positive(m.sq().sub(m.mul(q.add(r))).sub(q.mul(r))).map_err(|_| Refusal::Curvature)
    }
    fn core(self, margin: I, guard: f64) -> Result<f64> {
        let l = I::p(self.linear_sum_upper);
        let q = I::p(self.quadratic_upper);
        let r = I::p(self.derivative_remainder_upper);
        let e = I::p(self.anchor_upper).add(I::p(guard));
        let radius = I::p(2.)
            .mul(e)
            .mul(l.add(r))
            .div(margin)
            .ok_or(Refusal::Center)?;
        if radius.hi >= 1. {
            return Err(Refusal::Center);
        }
        Ok(e.add(l.add(q).mul(radius)).hi)
    }
}

fn directed(
    source: Arm,
    other: Arm,
    target: Geometry,
    policy: IncidentLimits,
) -> Result<DirectedProof> {
    let native = proof(source, other.native, policy.depth_upper, true)?;
    let native_margin_lower = native.margin()?.lo;
    let mixed = proof(source, target, policy.depth_upper, false)?;
    let margin = mixed.margin()?;
    let mixed_guarded_core_upper = mixed.core(I::p(margin.lo), policy.vertex_guard)?;
    if mixed_guarded_core_upper.partial_cmp(&policy.continuation_floor)
        != Some(std::cmp::Ordering::Less)
    {
        return Err(Refusal::Center);
    }
    Ok(DirectedProof {
        mixed,
        mixed_margin_lower: margin.lo,
        mixed_guarded_core_upper,
        native_margin_lower,
    })
}
