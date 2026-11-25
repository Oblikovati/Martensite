// SPDX-License-Identifier: GPL-2.0-only
//! Directed intervals for literal decoded coefficients and positive denominators.
#[derive(Clone, Copy)]
pub(super) struct Range(pub(super) [f64; 2]);

impl Range {
    pub(super) fn point(value: f64) -> Self {
        Self([value; 2])
    }

    fn wide() -> Self {
        Self([f64::NEG_INFINITY, f64::INFINITY])
    }

    pub(super) fn finite(self) -> bool {
        let [lo, hi] = self.0;
        lo.is_finite() && hi.is_finite() && lo <= hi
    }

    fn rounded(lo: f64, hi: f64) -> Self {
        if lo.is_nan() || hi.is_nan() {
            return Self::wide();
        }
        Self([lo.next_down(), hi.next_up()])
    }

    pub(super) fn difference(self, other: Self) -> Self {
        Self::rounded(self.0[0] - other.0[1], self.0[1] - other.0[0])
    }

    pub(super) fn sum(self, other: Self) -> Self {
        Self::rounded(self.0[0] + other.0[0], self.0[1] + other.0[1])
    }

    pub(super) fn scale(self, scalar: f64) -> Self {
        self.product(Self::point(scalar))
    }

    pub(super) fn product(self, other: Self) -> Self {
        if !self.finite() || !other.finite() {
            return Self::wide();
        }
        let products = self.0.into_iter().flat_map(|a| other.0.map(|b| a * b));
        let [lo, hi] = products.fold([f64::INFINITY, f64::NEG_INFINITY], |[lo, hi], x| {
            [lo.min(x), hi.max(x)]
        });
        Self::rounded(lo, hi)
    }

    pub(super) fn divide(self, denominator: Self) -> Self {
        if !denominator.finite() || denominator.0[0] <= 0. {
            return Self::wide();
        }
        self.product(Self::rounded(1. / denominator.0[1], 1. / denominator.0[0]))
    }

    pub(super) fn magnitude(self) -> f64 {
        self.0[0].abs().max(self.0[1].abs())
    }
}

pub(super) fn derivative<const N: usize>(controls: &[[Range; N]]) -> Vec<[Range; N]> {
    if controls.len() <= 1 {
        return vec![[Range::point(0.); N]];
    }
    controls
        .windows(2)
        .map(|pair| {
            std::array::from_fn(|i| {
                pair[1][i]
                    .difference(pair[0][i])
                    .scale((controls.len() - 1) as f64)
            })
        })
        .collect()
}

pub(super) fn hull<const N: usize>(controls: &[[Range; N]]) -> [Range; N] {
    controls
        .iter()
        .copied()
        .reduce(|a, b| {
            std::array::from_fn(|i| Range([a[i].0[0].min(b[i].0[0]), a[i].0[1].max(b[i].0[1])]))
        })
        .unwrap_or([Range::wide(); N])
}

pub(super) fn prefix<const N: usize>(
    mut controls: Vec<[Range; N]>,
    end: f64,
    reverse: bool,
) -> Vec<[Range; N]> {
    if reverse {
        controls.reverse();
    }
    let Some(&first) = controls.first() else {
        return controls;
    };
    let a = Range::point(1.).difference(Range::point(end));
    let b = Range::point(end);
    let mut left = vec![first];
    for count in (1..controls.len()).rev() {
        for i in 0..count {
            controls[i] = std::array::from_fn(|j| {
                controls[i][j].product(a).sum(controls[i + 1][j].product(b))
            });
        }
        left.push(controls[0]);
    }
    left
}

pub(super) fn add(a: f64, b: f64) -> f64 {
    (a + b).next_up()
}

pub(super) fn mul(a: f64, b: f64) -> f64 {
    (a * b).next_up()
}

pub(super) fn positive(number: f64) -> bool {
    number.is_finite() && number > 0.
}
