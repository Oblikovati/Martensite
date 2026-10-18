//! Original request-local analytic values, derivatives, jumps and periods.
use super::common::Reader;
use martensite_tkernel::ncollection::Array1;
use martensite_tkgeombase::app_cont::{Function, FunctionBase};
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};

/// GbAccFunction: coordinate d is `a0 + a1 u + a2 u^2 + a3 sin(a4 u)` (plus the jumps
/// at the ends), reduced into [0, P) when periodic of P.
pub struct AccFunction {
    base: FunctionBase,
    u0: f64,
    u1: f64,
    jump0: f64,
    jump1: f64,
    d1_ok: bool,
    coef: Vec<f64>,
    periods: Vec<f64>,
}

impl AccFunction {
    pub fn read(r: &mut Reader<'_>) -> Self {
        let (u0, u1, nb3d, nb2d) = (r.real(), r.real(), r.integer(), r.integer());
        let (d1_ok, jump0, jump1) = (r.flag(), r.real(), r.real());
        let dims = (3 * nb3d + 2 * nb2d).max(0) as usize;
        let mut periods = vec![0.0; dims];
        for _ in 0..r.integer() {
            let d = r.integer() as usize;
            periods[d - 1] = r.real();
        }
        let coef = (0..5 * dims).map(|_| r.real()).collect();
        let mut base = FunctionBase::new();
        base.set_number_of_points(nb3d, nb2d);
        Self {
            base,
            u0,
            u1,
            jump0,
            jump1,
            d1_ok,
            coef,
            periods,
        }
    }
    pub fn coord(&self, d: i32, u: f64) -> f64 {
        let a = &self.coef[5 * (d as usize - 1)..];
        let mut v = a[0] + a[1] * u + a[2] * u * u + a[3] * (a[4] * u).sin();
        if u == self.u0 {
            v += self.jump0;
        }
        if u == self.u1 {
            v += self.jump1;
        }
        let period = self.periods[d as usize - 1];
        if period > 0. {
            v -= period * (v / period).floor();
        }
        v
    }
    pub fn deriv(&self, d: i32, u: f64) -> f64 {
        let a = &self.coef[5 * (d as usize - 1)..];
        a[1] + 2. * a[2] * u + a[3] * a[4] * (a[4] * u).cos()
    }
}

impl Function for AccFunction {
    fn base(&self) -> &FunctionBase {
        &self.base
    }
    fn first_parameter(&self) -> f64 {
        self.u0
    }
    fn last_parameter(&self) -> f64 {
        self.u1
    }
    fn value(&self, u: f64, p2d: &mut Array1<'_, Point2>, pnt: &mut Array1<'_, Point3>) -> bool {
        let mut d = 1;
        for j in 1..=self.get_nb_of_3d_points() {
            pnt[j] = Point3::new(self.coord(d, u), self.coord(d + 1, u), self.coord(d + 2, u));
            d += 3;
        }
        for j in 1..=self.get_nb_of_2d_points() {
            p2d[j] = Point2::new(self.coord(d, u), self.coord(d + 1, u));
            d += 2;
        }
        true
    }
    fn d1(&self, u: f64, v2d: &mut Array1<'_, Vector2>, v: &mut Array1<'_, Vector3>) -> bool {
        let mut d = 1;
        for j in 1..=self.get_nb_of_3d_points() {
            v[j] = Vector3::new(self.deriv(d, u), self.deriv(d + 1, u), self.deriv(d + 2, u));
            d += 3;
        }
        for j in 1..=self.get_nb_of_2d_points() {
            v2d[j] = Vector2::new(self.deriv(d, u), self.deriv(d + 1, u));
            d += 2;
        }
        self.d1_ok
    }
    fn period_information(&self, dim_idx: i32) -> (bool, f64) {
        let period = self.periods[dim_idx as usize - 1];
        (period > 0., period)
    }
}
