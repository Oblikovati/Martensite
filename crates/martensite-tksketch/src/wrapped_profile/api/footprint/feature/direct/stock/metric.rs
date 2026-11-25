//! Outward metric restriction of the native stock's world correspondence.
use super::super::contact::{COLLAR_GUARD, interval::I};
use super::{Checked, StockRefusal, StockWords, TAU, Word};

// The upper dyadic bound on 2*pi - binary64::TAU follows from Machin's
// identity. Subtracting the rounded full angles would lose the residual.
const PERIOD_RESIDUAL_UPPER: f64 = f64::from_bits(0x3cb1_a626_3314_5c07);

struct StockMetric {
    z: [f64; 2],
    radii: [f64; 2],
    cone_z: f64,
    cone_radius: f64,
    height: f64,
    sine: I,
    cosine_tail: I,
    line: [[f64; 3]; 2],
    vertices: [[f64; 3]; 2],
    limit: f64,
}

impl StockMetric {
    fn admit(source: &StockWords, sew: Word) -> Checked<Self> {
        let [a, b] = &source.ends;
        let z = [a.plane[0][2].get(), b.plane[0][2].get()];
        let radii = [a.radius.get(), b.radius.get()];
        let cone_radius = source.cone.radius.get();
        let height = source.seam.range[1].get();
        if !(z[0] < z[1] && 0. < radii[0] && radii[0] < radii[1] && 0. < cone_radius && 0. < height)
        {
            return Err(StockRefusal::Carrier);
        }
        if [a, b]
            .into_iter()
            .any(|rim| rim.range.map(Word::get) != [0., TAU])
        {
            return Err(StockRefusal::Band);
        }
        let [sine, cosine_tail] = trig_tails(source.cone.angle.get())?;
        let limit = raw_limit(source, sew)?;
        Ok(Self {
            z,
            radii,
            cone_z: source.cone.frame[0][2].get(),
            cone_radius,
            height,
            sine,
            cosine_tail,
            line: source.seam.source.map(|row| row.map(Word::get)),
            vertices: [a.vertex.map(Word::get), b.vertex.map(Word::get)],
            limit,
        })
    }

    fn cosine(&self) -> I {
        I::p(1.).add(self.cosine_tail)
    }

    fn check_contacts(&self) -> Checked<()> {
        let [base, dir] = self.line;
        if !(self.sine.lo > 0.
            && self.cosine().lo > 0.
            && dir[2] > 0.
            && base[0] > 0.
            && I::p(base[0]).add(I::p(self.height).mul(I::p(dir[0]))).lo > 0.)
        {
            return Err(StockRefusal::Contact);
        }
        for cap in 0..2 {
            self.disk_contact(cap)?;
            self.line_contact(cap)?;
        }
        Ok(())
    }

    fn disk_contact(&self, cap: usize) -> Checked<()> {
        let height = diff(self.z[cap], self.cone_z);
        let h = I::p(self.height);
        // Compare with the upper endpoint before dividing. Keeping cos(a)-1
        // separate preserves the sign when the disk lies within one ULP of it.
        let top = h.sub(height).add(h.mul(self.cosine_tail));
        if height.hi < 0. || top.hi < 0. {
            return Ok(());
        }
        if !(height.lo >= 0. && top.lo >= 0.) {
            return Err(StockRefusal::Contact);
        }
        let v = height.div(self.cosine()).ok_or(StockRefusal::Contact)?;
        let radius = diff(self.cone_radius, self.radii[cap]).add(self.sine.mul(v));
        if radius.lo > 0. || (radius.hi <= 0. && strictly_below(radius.abs(), self.limit)) {
            return Ok(());
        }
        Err(StockRefusal::Contact)
    }

    fn line_contact(&self, cap: usize) -> Checked<()> {
        // Use the nearby endpoint to avoid cancellation in the absolute
        // line parameter. The candidate lies exactly in the disk plane.
        let pivot = if cap == 0 { I::Z } else { I::p(self.height) };
        let target = self.vertices[cap];
        let [base, dir] = self.line;
        let gap = axial_error(base[2], dir[2], pivot, self.z[cap]);
        let delta = gap
            .negate()
            .div(I::p(dir[2]))
            .ok_or(StockRefusal::Contact)?;
        let [x, y, _] = self.line_error(pivot, target);
        let residue = [
            x.add(delta.mul(I::p(dir[0]))),
            y.add(delta.mul(I::p(dir[1]))),
            diff(self.z[cap], target[2]),
        ];
        if strictly_below(l1(residue), self.limit) {
            Ok(())
        } else {
            Err(StockRefusal::Contact)
        }
    }

    fn line_error(&self, v: I, target: [f64; 3]) -> [I; 3] {
        let [base, dir] = self.line;
        [
            diff(base[0], target[0]).add(v.mul(I::p(dir[0]))),
            diff(base[1], target[1]).add(v.mul(I::p(dir[1]))),
            axial_error(base[2], dir[2], v, target[2]),
        ]
    }

    fn endpoint_bounds(&self, cap: usize, v: f64, angular: I) -> [f64; 5] {
        let v = I::p(v);
        let ideal = [self.radii[cap], 0., self.z[cap]];
        let cone_ideal = [
            diff(self.cone_radius, ideal[0]).add(v.mul(self.sine)),
            I::Z,
            diff(self.cone_z, ideal[2])
                .add(v)
                .add(v.mul(self.cosine_tail)),
        ];
        let [base, dir] = self.line;
        let cone_line = [
            diff(self.cone_radius, base[0]).add(v.mul(self.sine.sub(I::p(dir[0])))),
            self.line_error(v, [0.; 3])[1],
            diff(self.cone_z, base[2]).add(v.mul(diff(1., dir[2]).add(self.cosine_tail))),
        ];
        let ideal_vertex = std::array::from_fn(|i| diff(ideal[i], self.vertices[cap][i]));
        [
            I::p(l1(cone_ideal)).add(angular).hi,
            I::p(l1(cone_line)).add(angular).hi,
            l1(self.line_error(v, ideal)),
            l1(self.line_error(v, self.vertices[cap])),
            I::p(l1(ideal_vertex))
                .add(I::p(ideal[0]).mul(I::p(PERIOD_RESIDUAL_UPPER)))
                .hi,
        ]
    }

    fn bound(&self) -> Checked<f64> {
        self.check_contacts()?;
        let tip = I::p(self.cone_radius).add(I::p(self.height).mul(self.sine));
        let angular = I::p(tip.hi.max(self.radii[1])).mul(I::p(PERIOD_RESIDUAL_UPPER));
        let upper = self
            .endpoint_bounds(0, 0., angular)
            .into_iter()
            .chain(self.endpoint_bounds(1, self.height, angular))
            .fold(0., f64::max);
        if strictly_below(upper, self.limit) && upper <= COLLAR_GUARD {
            Ok(upper)
        } else {
            Err(StockRefusal::Limit)
        }
    }
}

fn strictly_below(value: f64, limit: f64) -> bool {
    value.is_finite() && value < limit
}

fn axial_error(base: f64, slope: f64, v: I, target: f64) -> I {
    diff(base, target).add(v).add(v.mul(diff(slope, 1.)))
}

fn diff(a: f64, b: f64) -> I {
    let large = a.abs().max(b.abs());
    let small = a.abs().min(b.abs());
    // Sterbenz subtraction avoids an unnecessary ULP around an already
    // exact end-height or radius difference.
    if a == b
        || (a.is_sign_positive() == b.is_sign_positive()
            && large >= 2. * f64::MIN_POSITIVE
            && small >= large * 0.5)
    {
        I::p(a - b)
    } else {
        I::p(a).sub(I::p(b))
    }
}

fn l1(values: [I; 3]) -> f64 {
    if values
        .iter()
        .any(|x| !x.lo.is_finite() || !x.hi.is_finite())
    {
        return f64::INFINITY;
    }
    values
        .into_iter()
        .fold(I::Z, |sum, x| sum.add(I::p(x.abs())))
        .hi
}

fn raw_limit(source: &StockWords, sew: Word) -> Checked<f64> {
    let [a, b] = &source.ends;
    let limit = [
        source.cone.tolerance,
        a.plane_tol,
        b.plane_tol,
        a.edge_tol,
        b.edge_tol,
        source.seam.edge_tol,
        a.vertex_tol,
        b.vertex_tol,
        sew,
    ]
    .into_iter()
    .map(Word::get)
    .fold(f64::INFINITY, f64::min);
    if limit.is_finite() && limit > 0. {
        Ok(limit)
    } else {
        Err(StockRefusal::Limit)
    }
}

fn trig_tails(angle: f64) -> Checked<[I; 2]> {
    if !(angle > 0. && angle < 1.) {
        return Err(StockRefusal::Carrier);
    }
    let a = I::p(angle);
    let step = a.sq().negate();
    // Terms decrease for the admitted angle. Keep the cosine correction
    // independent of its leading unit term to bound tiny stock end gaps.
    Ok([series(a, 1, step)?, series(step.mul(I::p(0.5)), 2, step)?])
}

fn series(first: I, power: u32, step: I) -> Checked<I> {
    let mut term = first;
    let mut sum = term;
    for j in 1..10 {
        term = series_term(term, step, power + 2 * j)?;
        sum = sum.add(term);
    }
    Ok(sum.hull(sum.add(series_term(term, step, power + 20)?)))
}

fn series_term(term: I, step: I, power: u32) -> Checked<I> {
    let power = f64::from(power);
    term.mul(step)
        .div(I::p(power * (power - 1.)))
        .ok_or(StockRefusal::Carrier)
}

pub(super) fn prove(source: &StockWords, sew: Word) -> Checked<f64> {
    StockMetric::admit(source, sew)?.bound()
}
