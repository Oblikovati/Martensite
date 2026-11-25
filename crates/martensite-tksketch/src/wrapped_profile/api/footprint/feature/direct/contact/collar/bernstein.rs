//! Fixed degree Bernstein product coefficients for the directed collar.
use super::*;

pub(super) struct Products {
    weights: [[I; MAX]; PRODUCT],
}

impl Products {
    pub(super) fn new(n: usize) -> Self {
        Self {
            weights: from_fn(|k| {
                from_fn(|i| {
                    if k >= i && k - i <= n && i <= n {
                        let numer = (binomial(n, i) * binomial(n, k - i)) as f64;
                        let reciprocal = (f64::from(binomial(2 * n, k))).recip();
                        I::p(numer).mul(I {
                            lo: reciprocal.next_down(),
                            hi: reciprocal.next_up(),
                        })
                    } else {
                        I::Z
                    }
                })
            }),
        }
    }
    pub(super) fn product(&self, a: &[I], b: &[I]) -> [I; PRODUCT] {
        let n = a.len() - 1;
        from_fn(|k| {
            (k.saturating_sub(n)..=k.min(n)).fold(I::Z, |sum, i| {
                sum.add(self.weights[k][i].mul(a[i]).mul(b[k - i]))
            })
        })
    }
}

fn binomial(n: usize, k: usize) -> u32 {
    (0..k.min(n - k)).fold(1, |acc, i| acc * (n - i) as u32 / (i + 1) as u32)
}
