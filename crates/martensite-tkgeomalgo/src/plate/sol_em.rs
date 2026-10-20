//! The two kernels of the plate: `SolEm`, the derivatives of the fundamental solution
//! of the iterated Laplacian, and `Polm`, the derivatives of the monomials.
use super::PlatePlate;
use super::sol_em_terms::{SolEmTerms, duv};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::errors;
use martensite_tkmath::gp::Xy;

/// The `(Uold, Vold, U2, R, L)` members that `SolEm` updates through a `const_cast`:
/// the last scaled point, `U * U`, `R = U2 + V * V` and `ln R`.
#[derive(Clone, Copy, Debug)]
pub(super) struct SolEmCache {
    u_old: f64,
    v_old: f64,
    u2: f64,
    r: f64,
    l: f64,
}

impl Default for SolEmCache {
    /// The constructor values: `Uold = Vold = 1e20` and null `U2`, `R`, `L`.
    fn default() -> Self {
        Self {
            u_old: 1.0e20,
            v_old: 1.0e20,
            u2: 0.0,
            r: 0.0,
            l: 0.0,
        }
    }
}

impl SolEmCache {
    /// The cached `(U2, R, L)` of `(u, v)`, updating the cache on a new point; `None`
    /// when `R < 1e-20`. A fresh cache holds `(1e20, 1e20)` with `R = 0`, so a first
    /// call at exactly that point returns `None`, as natively.
    fn terms(&mut self, u: f64, v: f64) -> Option<(f64, f64, f64)> {
        if u == self.u_old && v == self.v_old {
            if self.r < 1.0e-20 {
                return None;
            }
        } else {
            self.u_old = u;
            self.v_old = v;
            self.u2 = u * u;
            self.r = self.u2 + v * v;
            if self.r < 1.0e-20 {
                return None;
            }
            self.l = martensite_cmath::log(self.r);
        }
        Some((self.u2, self.r, self.l))
    }
}

/// `R^expo` by repeated products (`1 / R^-expo` for a negative exponent), as the native
/// code computes it instead of `pow`.
fn integer_power(r: f64, expo: i32) -> f64 {
    if expo == 0 {
        return 1.0;
    }
    let mut pr: f64 = r;
    for _ in 1..expo.abs() {
        pr *= r;
    }
    if expo < 0 { 1.0 / pr } else { pr }
}

impl PlatePlate {
    /// `SolEm`: the `(u_order, v_order)` derivative of the fundamental solution at `point2d`,
    /// scaled by `ddu[u_order] * ddv[v_order]`; the case table is symmetric, so `v_order > u_order` swaps
    /// the coordinates.
    ///
    /// Standard_OutOfRange for a derivative order outside `0..=9`, where the native
    /// `ddu`/`ddv` read is undefined.
    pub(super) fn sol_em(&self, point2d: Xy, u_order: i32, v_order: i32) -> Result<f64, Failure> {
        let (ddu1, ddv1) = (self.ddu[1], self.ddv[1]);
        let (big_iu, big_iv, u, v) = if v_order > u_order {
            (v_order, u_order, point2d.y() * ddv1, point2d.x() * ddu1)
        } else {
            (u_order, v_order, point2d.x() * ddu1, point2d.y() * ddv1)
        };
        let cached = self
            .sol_em_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .terms(u, v);
        let Some((u2, r, l)) = cached else {
            return Ok(0.0);
        };
        let (ddu_iu, ddv_iv) = (self.scale_u(u_order)?, self.scale_v(v_order)?);
        let m: i32 = self.order;
        let terms = SolEmTerms {
            u,
            v,
            u2,
            r,
            l,
            pr: integer_power(r, m - 1 - big_iu - big_iv),
            m: f64::from(m),
            mm1: f64::from(m - 1),
        };
        Ok(duv(big_iu, big_iv, &terms) * ddu_iu * ddv_iv)
    }

    /// `ddu[u_order]`, Standard_OutOfRange outside the ten scale factors.
    pub(super) fn scale_u(&self, u_order: i32) -> Result<f64, Failure> {
        scale_factor(&self.ddu, u_order, "U")
    }

    /// `ddv[v_order]`, Standard_OutOfRange outside the ten scale factors.
    pub(super) fn scale_v(&self, v_order: i32) -> Result<f64, Failure> {
        scale_factor(&self.ddv, v_order, "V")
    }

    /// `Polm`: the `(idu, idv)` derivative of the monomial `u^u_order v^v_order` at `point2d`,
    /// scaled by `ddu[u_order] * ddv[v_order]` (which the native comment says improves the
    /// conditioning).
    pub(super) fn polm(
        &self,
        point2d: Xy,
        u_order: i32,
        v_order: i32,
        idu: i32,
        idv: i32,
    ) -> Result<f64, Failure> {
        if idu > u_order || idv > v_order {
            return Ok(0.0);
        }
        let (u, v) = (point2d.x(), point2d.y());
        let mut value: f64 = 1.0;
        let degu: i32 = u_order - idu;
        for _ in 0..degu {
            value *= u;
        }
        for i in (degu + 1..=u_order).rev() {
            value *= f64::from(i);
        }
        let degv: i32 = v_order - idv;
        for _ in 0..degv {
            value *= v;
        }
        for i in (degv + 1..=v_order).rev() {
            value *= f64::from(i);
        }
        Ok(value * self.scale_u(u_order)? * self.scale_v(v_order)?)
    }
}

fn scale_factor(factors: &[f64; 10], order: i32, direction: &str) -> Result<f64, Failure> {
    let slot = usize::try_from(order).ok().and_then(|i| factors.get(i));
    slot.copied().ok_or_else(|| {
        errors::out_of_range(&format!(
            "Plate_Plate: derivative order {order} in {direction}; expected in [0, 9]"
        ))
    })
}
