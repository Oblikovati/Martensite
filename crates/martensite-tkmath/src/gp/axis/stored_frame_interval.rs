//! Outward binary64 arithmetic for the stored-frame contract (#432).
#[derive(Clone, Copy, Debug)]
pub(super) struct Bounds {
    pub lower: f64,
    pub upper: f64,
}
impl Bounds {
    pub fn point(value: f64) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }
    fn finite(lower: f64, upper: f64) -> Option<Self> {
        (lower.is_finite() && upper.is_finite() && lower <= upper).then_some(Self { lower, upper })
    }
    pub fn contains(self, value: f64) -> bool {
        self.lower <= value && value <= self.upper
    }
    pub fn negate(self) -> Self {
        Self {
            lower: -self.upper,
            upper: -self.lower,
        }
    }
    pub fn add(self, rhs: Self) -> Option<Self> {
        Self::finite(
            (self.lower + rhs.lower).next_down(),
            (self.upper + rhs.upper).next_up(),
        )
    }
    pub fn subtract(self, rhs: Self) -> Option<Self> {
        self.add(rhs.negate())
    }
    pub fn multiply(self, rhs: Self) -> Option<Self> {
        let products = [
            self.lower * rhs.lower,
            self.lower * rhs.upper,
            self.upper * rhs.lower,
            self.upper * rhs.upper,
        ];
        if products.iter().any(|value| !value.is_finite()) {
            return None;
        }
        let lower = products.into_iter().fold(f64::INFINITY, f64::min);
        let upper = products.into_iter().fold(f64::NEG_INFINITY, f64::max);
        Self::finite(lower.next_down(), upper.next_up())
    }
    pub fn divide_positive(self, rhs: Self) -> Option<Self> {
        if rhs.lower <= 0.0 {
            return None;
        }
        let reciprocal = Self::finite((1.0 / rhs.upper).next_down(), (1.0 / rhs.lower).next_up())?;
        self.multiply(reciprocal)
    }
    pub fn sqrt(self) -> Option<Self> {
        if self.lower < 0.0 {
            return None;
        }
        let lower = if self.lower == 0.0 {
            0.0
        } else {
            self.lower.sqrt().next_down()
        };
        let upper = self.upper.sqrt().next_up();
        let result = Self::finite(lower, upper)?;
        // Certify the brackets independently of the platform sqrt implementation.
        let below =
            lower == 0.0 || Self::point(lower).multiply(Self::point(lower))?.upper <= self.lower;
        let above = Self::point(upper).multiply(Self::point(upper))?.lower >= self.upper;
        (below && above).then_some(result)
    }
}
pub(super) type VectorBounds = [Bounds; 3];
pub(super) fn dot(left: VectorBounds, right: VectorBounds) -> Option<Bounds> {
    left[0]
        .multiply(right[0])?
        .add(left[1].multiply(right[1])?)?
        .add(left[2].multiply(right[2])?)
}
pub(super) fn cross(left: VectorBounds, right: VectorBounds) -> Option<VectorBounds> {
    let component = |j: usize, k: usize| {
        left[j]
            .multiply(right[k])?
            .subtract(left[k].multiply(right[j])?)
    };
    Some([component(1, 2)?, component(2, 0)?, component(0, 1)?])
}
pub(super) fn normalize(vector: VectorBounds) -> Option<VectorBounds> {
    let magnitude = dot(vector, vector)?.sqrt()?;
    Some([
        vector[0].divide_positive(magnitude)?,
        vector[1].divide_positive(magnitude)?,
        vector[2].divide_positive(magnitude)?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn outward_primitives_cover_exact_adjacent_representable_results() {
        let next = 1.0_f64.next_up();
        let product = Bounds::point(next).multiply(Bounds::point(next)).unwrap();
        let floor = next.next_up(); // (1 + 2^-52)^2 = 1 + 2^-51 + 2^-104.
        assert!(product.lower <= floor && product.upper > floor);
        let sum = Bounds::point(1.0)
            .add(Bounds::point(f64::EPSILON / 2.0))
            .unwrap();
        assert!(sum.lower <= 1.0 && sum.upper >= next);
        let difference = Bounds::point(next).subtract(Bounds::point(1.0)).unwrap();
        assert!(difference.contains(f64::EPSILON));
        let quotient = Bounds::point(1.0)
            .divide_positive(Bounds::point(3.0))
            .unwrap();
        assert!(quotient.lower * 3.0 <= 1.0 && quotient.upper * 3.0 >= 1.0);
    }
    #[test]
    fn bounds_handle_signed_zero_subnormals_and_refuse_unbounded_operations() {
        let tiny = f64::from_bits(1);
        let product = Bounds::point(tiny).multiply(Bounds::point(0.5)).unwrap();
        assert!(product.lower <= 0.0 && product.upper >= tiny);
        assert!(Bounds::point(-0.0).contains(0.0));
        assert_eq!(
            Bounds::point(-0.0).negate().lower.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(
            Bounds::point(f64::MAX)
                .multiply(Bounds::point(2.0))
                .is_none()
        );
        assert!(
            Bounds::point(1.0)
                .divide_positive(Bounds::point(0.0))
                .is_none()
        );
        assert!(
            Bounds::point(f64::INFINITY)
                .add(Bounds::point(1.0))
                .is_none()
        );
    }
    #[test]
    fn square_root_requires_independently_certified_brackets() {
        let root = Bounds::point(2.0).sqrt().unwrap();
        assert!(
            Bounds::point(root.lower)
                .multiply(Bounds::point(root.lower))
                .unwrap()
                .upper
                <= 2.0
        );
        assert!(
            Bounds::point(root.upper)
                .multiply(Bounds::point(root.upper))
                .unwrap()
                .lower
                >= 2.0
        );
        assert!(Bounds::point(-1.0).sqrt().is_none());
        assert!(Bounds::point(f64::from_bits(1)).sqrt().is_none());
    }
}

#[cfg(test)]
mod directed_oracle_tests {
    use super::Bounds;
    use rustc_apfloat::ieee::Double;
    use rustc_apfloat::{Float, Round};
    fn native(value: Double) -> f64 {
        f64::from_bits(value.to_bits() as u64)
    }
    fn assert_enclosure(bounds: Option<Bounds>, downward: Double, upward: Double) {
        if let Some(bounds) = bounds {
            assert!(
                bounds.lower <= native(downward),
                "lower {bounds:?} excludes {}",
                native(downward)
            );
            assert!(
                bounds.upper >= native(upward),
                "upper {bounds:?} excludes {}",
                native(upward)
            );
        }
    }
    #[test]
    fn primitives_enclose_independent_software_directed_rounding() {
        let values = [
            -f64::MAX,
            -1.0_f64.next_up(),
            -1.,
            -f64::MIN_POSITIVE,
            -f64::from_bits(1),
            -0.,
            0.,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            1.0_f64.next_down(),
            1.,
            1.0_f64.next_up(),
            3.,
            f64::MAX,
        ];
        for left in values {
            for right in values {
                let a = Double::from_bits(u128::from(left.to_bits()));
                let b = Double::from_bits(u128::from(right.to_bits()));
                let (down, up) = (Round::TowardNegative, Round::TowardPositive);
                assert_enclosure(
                    Bounds::point(left).add(Bounds::point(right)),
                    a.add_r(b, down).value,
                    a.add_r(b, up).value,
                );
                assert_enclosure(
                    Bounds::point(left).subtract(Bounds::point(right)),
                    a.sub_r(b, down).value,
                    a.sub_r(b, up).value,
                );
                assert_enclosure(
                    Bounds::point(left).multiply(Bounds::point(right)),
                    a.mul_r(b, down).value,
                    a.mul_r(b, up).value,
                );
                if right > 0.0 {
                    assert_enclosure(
                        Bounds::point(left).divide_positive(Bounds::point(right)),
                        a.div_r(b, down).value,
                        a.div_r(b, up).value,
                    );
                }
            }
        }
    }
}
