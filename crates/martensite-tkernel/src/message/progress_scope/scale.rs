//! The local-to-global mapping of a progress scope: next, close and value.
use super::{super::ProgressRange, ProgressScope};
use crate::precision;

impl ProgressScope<'_> {
    /// Advances the local value by `step` and returns the range covering it; an
    /// inactive scope, a non-positive step or a step mapping to no global advance
    /// returns an unbound range; `func(scope.next(1.))`.
    pub fn next(&self, step: f64) -> ProgressRange<'_> {
        if self.is_active() && step > 0. {
            let value = self.local_value();
            let current = self.local_to_global(value);
            // The sum is stored first, then mapped to the global scale.
            self.set_local_value(value + step);
            let next = self.local_to_global(value + step);
            let delta = next - current;
            if delta > 0. {
                return ProgressRange::new(self, self.start + current, delta);
            }
        }
        ProgressRange::default()
    }
    /// Advances the indicator to the end of the scope and disarms it; the local value
    /// becomes `max` (or `precision::infinite()` for infinite scopes); `scope.close()`.
    ///
    /// A parent that is no longer armed is not
    /// checked.
    pub fn close(&self) {
        if !self.is_active() {
            return;
        }
        let current = self.local_to_global(self.local_value());
        self.set_local_value(if self.infinite {
            precision::infinite()
        } else {
            self.max
        });
        let delta = self.portion - current;
        if delta > 0. {
            self.increment_indicator(delta);
        }
        self.disarm();
    }
    /// The current local value: the global position mapped into `[0, max]` and rounded
    /// up, `max` (or infinite) once the scope end is within `precision::confusion()`;
    /// `scope.value()`.
    pub fn value(&self) -> f64 {
        let end = if self.infinite {
            precision::infinite()
        } else {
            self.max
        };
        let Some(progress) = self.progress.as_ref().filter(|_| self.is_active()) else {
            return end;
        };
        let reached = progress.position() - self.start;
        if reached <= 0. {
            return 0.;
        }
        let distance = self.portion - reached;
        if distance <= precision::confusion() {
            return end;
        }
        let span = if self.infinite {
            distance
        } else {
            self.portion
        };
        (self.max * reached / span - precision::confusion()).ceil()
    }
    /// Linear for finite scopes (snapping to the portion when
    /// within `real_small()` of max), hyperbolic `x / (1 + x)` for infinite ones.
    fn local_to_global(&self, value: f64) -> f64 {
        if value <= 0. {
            return 0.;
        }
        if !self.infinite {
            // `real_small()` is DBL_MIN.
            if self.max - value < f64::MIN_POSITIVE {
                return self.portion;
            }
            return self.portion * value / self.max;
        }
        let x = value / self.max;
        self.portion * x / (1. + x)
    }
}
