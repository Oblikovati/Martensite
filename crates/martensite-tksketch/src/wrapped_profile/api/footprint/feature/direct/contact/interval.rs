#[derive(Clone, Copy, Debug)]
pub struct I {
    pub lo: f64,
    pub hi: f64,
}
impl I {
    pub const Z: Self = Self::p(0.);
    pub const fn p(x: f64) -> Self {
        Self { lo: x, hi: x }
    }
    pub fn add(self, r: Self) -> Self {
        if self.lo == 0. && self.hi == 0. {
            return r;
        }
        if r.lo == 0. && r.hi == 0. {
            return self;
        }
        Self {
            lo: (self.lo + r.lo).next_down(),
            hi: (self.hi + r.hi).next_up(),
        }
    }
    pub fn negate(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }
    pub fn sub(self, r: Self) -> Self {
        self.add(r.negate())
    }
    pub fn mul(self, r: Self) -> Self {
        if (self.lo == 0. && self.hi == 0.) || (r.lo == 0. && r.hi == 0.) {
            return Self::Z;
        }
        let v = [
            self.lo * r.lo,
            self.lo * r.hi,
            self.hi * r.lo,
            self.hi * r.hi,
        ];
        Self {
            lo: v.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: v.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }
    pub fn sq(self) -> Self {
        let lo = self.lo.abs().min(self.hi.abs());
        let hi = self.abs();
        Self {
            lo: if self.lo <= 0. && self.hi >= 0. {
                0.
            } else {
                (lo * lo).next_down().max(0.)
            },
            hi: (hi * hi).next_up(),
        }
    }
    pub fn inv(self) -> Option<Self> {
        (self.lo > 0.).then(|| Self {
            lo: self.hi.recip().next_down(),
            hi: self.lo.recip().next_up(),
        })
    }
    pub fn div(self, r: Self) -> Option<Self> {
        Some(self.mul(r.inv()?))
    }
    pub fn abs(self) -> f64 {
        self.lo.abs().max(self.hi.abs())
    }
    pub fn sqrt(self) -> Option<Self> {
        if self.lo < 0. || !self.hi.is_finite() {
            return None;
        }
        if self.hi == 0. {
            return Some(Self::Z);
        }
        let mut a = self.lo.sqrt();
        let mut b = self.hi.sqrt();
        for _ in 0..4 {
            if a > 0. {
                a = a.next_down();
            }
            b = b.next_up();
            if (a == 0. || Self::p(a).sq().hi <= self.lo) && Self::p(b).sq().lo >= self.hi {
                return Some(Self { lo: a, hi: b });
            }
        }
        None
    }
    pub fn hull(self, b: Self) -> Self {
        Self {
            lo: self.lo.min(b.lo),
            hi: self.hi.max(b.hi),
        }
    }
}
pub fn hull(row: &[I]) -> I {
    row[1..].iter().fold(row[0], |acc, &x| acc.hull(x))
}
pub fn dot(a: [I; 3], b: [I; 3]) -> I {
    a.into_iter().zip(b).fold(I::Z, |d, (x, y)| d.add(x.mul(y)))
}
pub fn norm(a: [I; 3]) -> Option<I> {
    let mut s = a.into_iter().fold(I::Z, |d, x| d.add(x.sq()));
    s.lo = s.lo.max(0.);
    s.sqrt()
}
pub fn smooth_trig(raw: f64) -> Option<[I; 2]> {
    if !(0. ..=3.).contains(&raw) {
        return None;
    }
    let x = I::p(raw);
    let unit = x.sq().negate();
    let mut values = [I::p(1.), x];
    let mut term = values;
    for k in 1..=25 {
        for parity in 0..=1 {
            let n = (2 * k + parity) as f64;
            term[parity] = term[parity].mul(unit).div(I::p(n * (n - 1.)))?;
            if k <= 24 {
                values[parity] = values[parity].add(term[parity]);
            } else {
                let error = term[parity].abs();
                values[parity] = values[parity].add(I {
                    lo: -error,
                    hi: error,
                });
            }
        }
    }
    Some(values)
}
