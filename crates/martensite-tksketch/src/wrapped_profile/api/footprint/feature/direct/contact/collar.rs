use super::{
    COLLAR_GUARD,
    interval::{I, hull, norm, smooth_trig},
};
mod admission;
mod bernstein;
use bernstein::Products;
use std::array::from_fn;

const MAX: usize = 10;
const PRODUCT: usize = 2 * MAX - 1;
const CONE_BASIS: [f64; 9] = [1., 0., 0., 0., 1., 0., 0., 0., 1.];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    Boss,
    Pocket,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Stock,
    Location,
    Carrier,
    Pcurve,
    Tolerance,
    Axis,
    ZeroSign,
    Sewing,
    Guard,
    RadialSign,
    DepthSign,
    Slant,
    Extent,
}

#[derive(Clone, Copy)]
pub struct StockSource {
    pub angle: f64,
    pub radius: f64,
    pub native_scale: f64,
    pub origin: [f64; 3],
    pub basis: [f64; 9],
    pub location: [f64; 3],
    pub location_basis: [f64; 9],
    pub sense: Sense,
}

#[derive(Clone, Copy)]
pub struct PolicySource<'a> {
    pub fuzzy: f64,
    pub sew: f64,
    pub class_maxima: [f64; 3],
    pub support_face_tolerances: &'a [f64],
}

// Inputs come from one non-rational-in-depth, degree-one ruled surface with
// a single native Bernstein span along the seam. The adapter checks the
// native basis, owner and orientation before exposing the two Euclidean rows.
#[derive(Clone, Copy)]
pub struct SideSource<'a> {
    pub normalized_domain: [f64; 4],
    pub affine: [f64; 13],
    pub first: &'a [[f64; 4]],
    pub last: &'a [[f64; 4]],
    pub seam_uv: &'a [[f64; 2]],
    pub seam_knots: &'a [f64],
    pub seam_tolerance: f64,
    pub side_tolerance: f64,
}

#[derive(Clone, Copy)]
pub struct UpstreamEvidence {
    pub first_layer_lower: f64,
    pub same_parameter_gap_upper: f64,
    pub guarded_sources_normal_upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub envelope_upper: f64,
    pub normalized_depth_upper: f64,
    pub from_zero_edge_upper: f64,
    pub from_support_curve_upper: f64,
    pub uv_from_zero_projection_upper: [f64; 2],
    pub uv_from_support_curve_upper: [f64; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Receipt {
    signed_derivative_lower: f64,
    motion_upper: [f64; 3],
    face: Bounds,
    all: Bounds,
}

impl Receipt {
    pub fn signed_derivative_lower(&self) -> f64 {
        self.signed_derivative_lower
    }
    pub fn motion_upper(&self) -> [f64; 3] {
        self.motion_upper
    }
    pub fn face(&self) -> &Bounds {
        &self.face
    }
    pub fn all(&self) -> &Bounds {
        &self.all
    }
}

pub struct Issuer {
    stock: StockSource,
    trig: [I; 2],
    fuzzy: f64,
    sew: f64,
    face_max: f64,
    edge_max: f64,
    stock_face_max: f64,
    guarded_broad_upper: f64,
    products: [Products; MAX - 1],
}

fn finite_range(x: f64, lo: f64, hi: f64) -> bool {
    (lo..=hi).contains(&x)
}

fn positive(x: I, failure: Refusal) -> Result<I, Refusal> {
    if x.lo > 0. && x.hi.is_finite() {
        Ok(x)
    } else {
        Err(failure)
    }
}

pub(super) fn native_comparator_upper(a: f64, b: f64, fuzzy: f64) -> f64 {
    let analytic_upper = I::p(a).add(I::p(b)).add(I::p(fuzzy)).hi;
    let split_native = (a + fuzzy / 2.) + (b + fuzzy / 2.);
    analytic_upper.max(split_native)
}

impl Issuer {
    pub fn new(stock: &StockSource, policy: PolicySource<'_>) -> Result<Self, Refusal> {
        if stock.native_scale != 1.
            || stock.location != [0.; 3]
            || stock.basis != CONE_BASIS
            || stock.location_basis != CONE_BASIS
            || stock.origin[..2] != [0., 0.]
            || !finite_range(stock.origin[2], -1024., 1024.)
            || !finite_range(stock.radius, 0.01, 1024.)
            || !finite_range(stock.angle, 0.01, 1.5)
        {
            return Err(Refusal::Stock);
        }
        let trig = smooth_trig(stock.angle).ok_or(Refusal::Stock)?;
        if trig.iter().any(|v| v.lo <= 0.) {
            return Err(Refusal::Stock);
        }
        if policy.support_face_tolerances.is_empty()
            || policy
                .class_maxima
                .iter()
                .chain(policy.support_face_tolerances)
                .any(|&x| !finite_range(x, 0., 1.))
            || !finite_range(policy.fuzzy, 0., 1.)
            || !finite_range(policy.sew, f64::MIN_POSITIVE, 1.)
        {
            return Err(Refusal::Tolerance);
        }
        let [_, edge_max, face_max] = policy.class_maxima;
        let stock_face_max = policy
            .support_face_tolerances
            .iter()
            .copied()
            .fold(0., f64::max);
        if stock_face_max > face_max {
            return Err(Refusal::Tolerance);
        }
        let maximum = policy.class_maxima.into_iter().fold(0., f64::max);
        let broad_comparator_upper = native_comparator_upper(maximum, maximum, policy.fuzzy);
        let guarded_broad_upper = I::p(broad_comparator_upper).add(I::p(COLLAR_GUARD)).hi;
        Ok(Self {
            stock: *stock,
            trig,
            fuzzy: policy.fuzzy,
            sew: policy.sew,
            face_max,
            edge_max,
            stock_face_max,
            guarded_broad_upper,
            products: from_fn(|i| Products::new(i + 1)),
        })
    }

    pub fn stock_point_normal_upper(&self, point: [f64; 3]) -> Result<f64, Refusal> {
        if point.iter().any(|&x| !finite_range(x, -1024., 1024.)) {
            return Err(Refusal::Guard);
        }
        let [x, y, z] = point.map(I::p);
        let rho = norm([x, y, I::Z]).ok_or(Refusal::Axis)?;
        let [c, s] = self.trig;
        let radial = rho.sub(I::p(self.stock.radius)).mul(c);
        let axial = z.sub(I::p(self.stock.origin[2])).mul(s);
        Ok(radial.sub(axial).abs())
    }

    // The live adapter validates a sealed exact signed-carrier receipt on these
    // very source rows and native owners before returning its first-layer bound.
    pub(crate) fn issue(
        &self,
        side: &SideSource<'_>,
        source_proof: impl FnOnce(&StockSource, &SideSource<'_>) -> Option<UpstreamEvidence>,
    ) -> Result<Receipt, Refusal> {
        side.check(self)?;
        let proof = source_proof(&self.stock, side).ok_or(Refusal::ZeroSign)?;
        if !(proof.first_layer_lower.is_finite() && proof.first_layer_lower > 0.) {
            return Err(Refusal::ZeroSign);
        }
        if !finite_range(proof.same_parameter_gap_upper, 0., self.sew) {
            return Err(Refusal::Sewing);
        }
        if !finite_range(proof.guarded_sources_normal_upper, 0., COLLAR_GUARD) {
            return Err(Refusal::Guard);
        }
        let carrier = Carrier::new(side);
        let factors = &self.products[side.first.len() - 2];
        let weighted = carrier.weighted();
        let (rho, radial_sign, rough) = self.rough_derivative(&carrier)?;
        let (refined, motion) = self.refine(&carrier, &weighted, factors, rho, radial_sign)?;
        let lower = rough.max(refined);
        let face_upper =
            native_comparator_upper(self.stock_face_max, side.side_tolerance, self.fuzzy);
        let face = self.extent(side, face_upper, lower, motion, rho)?;
        let all = self.extent(side, self.guarded_broad_upper, lower, motion, rho)?;
        Ok(Receipt {
            signed_derivative_lower: lower,
            motion_upper: motion,
            face,
            all,
        })
    }
}

#[derive(Clone)]
struct Carrier {
    hull: [I; 3],
    motion: [I; 3],
    speed: f64,
    first_xy: [[I; MAX]; 2],
    delta: [[I; MAX]; 3],
    w: [I; MAX],
    n: usize,
}

struct Weighted {
    w: [I; MAX],
    first_xy: [[I; MAX]; 2],
    delta: [[I; MAX]; 3],
}

fn transform(loc: [f64; 13], vector: [I; 3], translate: bool) -> [I; 3] {
    from_fn(|i| {
        let center = if translate { I::p(loc[i + 1]) } else { I::Z };
        (0..3).fold(center, |sum, j| {
            sum.add(I::p(loc[4 + 3 * i + j]).mul(vector[j]))
        })
    })
}

impl Carrier {
    fn new(side: &SideSource<'_>) -> Self {
        let mut first = [[I::Z; MAX]; 3];
        let mut last = [[I::Z; MAX]; 3];
        let mut delta = [[I::Z; MAX]; 3];
        let mut w = [I::Z; MAX];
        let mut speed = f64::NEG_INFINITY;
        let n = side.first.len();
        for (j, (a, b)) in side.first.iter().zip(side.last).enumerate() {
            let a0 = transform(side.affine, from_fn(|i| I::p(a[i])), true);
            let b0 = transform(side.affine, from_fn(|i| I::p(b[i])), true);
            let d = transform(side.affine, from_fn(|i| I::p(b[i]).sub(I::p(a[i]))), false);
            for i in 0..3 {
                first[i][j] = a0[i];
                last[i][j] = b0[i];
                delta[i][j] = d[i];
            }
            speed = speed.max(norm(d).map_or(f64::INFINITY, |i| i.hi));
            w[j] = I::p(a[3]);
        }
        let box3 = from_fn(|i| hull(&first[i][..n]).hull(hull(&last[i][..n])));
        let motion = from_fn(|i| hull(&delta[i][..n]));
        Self {
            hull: box3,
            motion,
            speed,
            first_xy: [first[0], first[1]],
            delta,
            w,
            n,
        }
    }

    fn weighted(&self) -> Weighted {
        let weight = |v: [I; MAX]| {
            from_fn(|j| {
                if j < self.n {
                    v[j].mul(self.w[j])
                } else {
                    I::Z
                }
            })
        };
        Weighted {
            w: self.w,
            first_xy: self.first_xy.map(weight),
            delta: self.delta.map(weight),
        }
    }
}

fn quotient(net: &[I; PRODUCT], den: &[I; PRODUCT], n: usize) -> Result<I, Refusal> {
    let mut output = net[0].div(den[0]).ok_or(Refusal::Carrier)?;
    for i in 1..2 * n - 1 {
        output = output.hull(net[i].div(den[i]).ok_or(Refusal::Carrier)?);
    }
    Ok(output)
}

impl Issuer {
    fn directed(&self, x: I) -> f64 {
        match self.stock.sense {
            Sense::Boss => x.lo,
            Sense::Pocket => -x.hi,
        }
    }

    fn rough_derivative(&self, net: &Carrier) -> Result<(I, bool, f64), Refusal> {
        let [x, y, _] = net.hull;
        if y.lo <= 0. {
            return Err(Refusal::Axis);
        }
        let rho = x.sq().add(y.sq()).sqrt().ok_or(Refusal::Axis)?;
        let [dx, dy, dz] = net.motion;
        let radial = x.mul(dx).add(y.mul(dy)).div(rho).ok_or(Refusal::Axis)?;
        if radial.lo <= 0. && radial.hi >= 0. {
            return Err(Refusal::RadialSign);
        }
        let [c, s] = self.trig;
        let rough = self.directed(c.mul(radial).sub(s.mul(dz)));
        if rough <= 0. {
            return Err(Refusal::DepthSign);
        }
        Ok((rho, radial.lo > 0., rough))
    }

    fn refine(
        &self,
        net: &Carrier,
        w: &Weighted,
        p: &Products,
        rho: I,
        outward: bool,
    ) -> Result<(f64, [f64; 3]), Refusal> {
        let n = net.n;
        let [x, y] = w.first_xy;
        let [dx, dy, dz] = w.delta;
        let product = |a: &[I; MAX], b: &[I; MAX]| p.product(&a[..n], &b[..n]);
        let den = product(&w.w, &w.w);
        let xdy = product(&x, &dy);
        let ydx = product(&y, &dx);
        let xy = from_fn(|i| xdy[i].sub(ydx[i]));
        let cross = quotient(&xy, &den, n)?;
        let rho2 = rho.sq();
        let angle = I::p(cross.abs())
            .div(I::p(rho2.lo))
            .ok_or(Refusal::Axis)?
            .hi;
        let cross_radial = cross.sq().div(rho2).ok_or(Refusal::Axis)?;
        let xx = product(&dx, &dx);
        let yy = product(&dy, &dy);
        let xy2 = from_fn(|i| xx[i].add(yy[i]));
        let radial2 = quotient(&xy2, &den, n)?.sub(cross_radial);
        let magnitude = positive(radial2, Refusal::RadialSign)?
            .sqrt()
            .ok_or(Refusal::RadialSign)?;
        let radial = if outward {
            magnitude
        } else {
            magnitude.negate()
        };
        let [c, s] = self.trig;
        let signed = self.directed(c.mul(radial).sub(s.mul(net.motion[2])));
        let slant = self.slant(p, net, &den, xy2, &dz, [cross_radial, radial])?;
        Ok((signed, [net.speed, angle, slant]))
    }

    fn slant(
        &self,
        p: &Products,
        net: &Carrier,
        den: &[I; PRODUCT],
        xy2: [I; PRODUCT],
        dz: &[I; MAX],
        [cross_radial, radial]: [I; 2],
    ) -> Result<f64, Refusal> {
        let [c, s] = self.trig;
        let zz = p.product(&dz[..net.n], &dz[..net.n]);
        let csq = c.sq();
        let ssq = s.sq();
        let terms = from_fn(|i| ssq.mul(xy2[i]).sub(csq.mul(zz[i])));
        let num = quotient(&terms, den, net.n)?.sub(ssq.mul(cross_radial));
        let sum = s.mul(radial).sub(c.mul(net.motion[2]));
        let abslow = sum.lo.max(-sum.hi);
        let result = I::p(num.abs())
            .div(positive(I::p(abslow), Refusal::Slant)?)
            .ok_or(Refusal::Slant)?;
        Ok(result.hi)
    }

    fn extent(
        &self,
        side: &SideSource<'_>,
        envelope_upper: f64,
        slope: f64,
        motion: [f64; 3],
        rho: I,
    ) -> Result<Bounds, Refusal> {
        let t = I::p(envelope_upper)
            .div(I::p(slope))
            .ok_or(Refusal::Extent)?;
        if !(t.lo > 0. && t.hi < 1.) || motion.iter().any(|x| !x.is_finite()) {
            return Err(Refusal::Extent);
        }
        let travel = motion.map(|x| I::p(x).mul(t));
        let sew = I::p(self.sew);
        let u = sew.div(I::p(rho.lo).sub(sew)).ok_or(Refusal::Extent)?;
        let padding = [travel[1].add(u).hi, travel[2].add(sew).hi];
        let chart = [6., 30.];
        if side.seam_uv.iter().any(|p| {
            (0..2).any(|i| {
                let offset = I::p(padding[i]);
                I::p(p[i]).sub(offset).lo <= 0. || I::p(p[i]).add(offset).hi >= chart[i]
            })
        }) {
            return Err(Refusal::Extent);
        }
        Ok(Bounds {
            envelope_upper,
            normalized_depth_upper: t.hi,
            from_zero_edge_upper: travel[0].hi,
            from_support_curve_upper: travel[0].add(sew).hi,
            uv_from_zero_projection_upper: [travel[1].hi, travel[2].hi],
            uv_from_support_curve_upper: padding,
        })
    }
}
