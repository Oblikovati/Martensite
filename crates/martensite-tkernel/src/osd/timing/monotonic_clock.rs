//! Portable monotonic instant and Unix timestamp clock (#631).
//!
//! Standard `std::time::Instant::now` and `std::time::SystemTime::now` panic on
//! `wasm32-unknown-unknown`. Every kernel and exchange crate reads monotonic
//! durations and Unix epoch timestamps through this module instead.

use crate::standard::Failure;
use std::time::Duration;

/// A monotonic timestamp that works on native targets (`std::time::Instant`)
/// and browser WebAssembly (`performance.now()`, falling back to `Date.now()`).
///
/// ```
/// use martensite_tkernel::osd::MonotonicInstant;
/// let start = MonotonicInstant::now();
/// assert!(start.elapsed().as_nanos() < u128::MAX);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonotonicInstant {
    #[cfg(not(target_arch = "wasm32"))]
    inner: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    inner: Duration,
}

impl MonotonicInstant {
    /// Captures the current monotonic timestamp.
    ///
    /// ```
    /// let now = martensite_tkernel::osd::MonotonicInstant::now();
    /// assert_eq!(now.saturating_duration_since(now), std::time::Duration::ZERO);
    /// ```
    #[allow(clippy::disallowed_methods)]
    pub fn now() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                inner: std::time::Instant::now(),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                inner: wasm_monotonic_duration(),
            }
        }
    }

    /// Returns the elapsed duration since `self` was captured.
    ///
    /// ```
    /// let start = martensite_tkernel::osd::MonotonicInstant::now();
    /// let _elapsed = start.elapsed();
    /// ```
    pub fn elapsed(&self) -> Duration {
        Self::now().saturating_duration_since(*self)
    }

    /// Returns the duration elapsed from `earlier` to `self`, saturating at zero.
    ///
    /// ```
    /// let t0 = martensite_tkernel::osd::MonotonicInstant::now();
    /// let t1 = martensite_tkernel::osd::MonotonicInstant::now();
    /// assert!(t1.saturating_duration_since(t0) <= t1.elapsed() + std::time::Duration::from_secs(1));
    /// ```
    pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inner.saturating_duration_since(earlier.inner)
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.inner.saturating_sub(earlier.inner)
        }
    }

    /// Returns the duration elapsed from `earlier` to `self`, or `None` if `earlier > self`.
    ///
    /// ```
    /// let t0 = martensite_tkernel::osd::MonotonicInstant::now();
    /// assert_eq!(t0.checked_duration_since(t0), Some(std::time::Duration::ZERO));
    /// ```
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inner.checked_duration_since(earlier.inner)
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.inner.checked_sub(earlier.inner)
        }
    }

    /// Returns the duration elapsed from `earlier` to `self`, saturating at zero.
    ///
    /// ```
    /// let t0 = martensite_tkernel::osd::MonotonicInstant::now();
    /// assert_eq!(t0.duration_since(t0), std::time::Duration::ZERO);
    /// ```
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.saturating_duration_since(earlier)
    }

    /// Adds `duration` to this instant, returning `None` on overflow.
    ///
    /// ```
    /// use std::time::Duration;
    /// let t0 = martensite_tkernel::osd::MonotonicInstant::now();
    /// assert!(t0.checked_add(Duration::from_millis(1)).is_some());
    /// ```
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.inner.checked_add(duration).map(|inner| Self { inner })
    }

    /// Subtracts `duration` from this instant, returning `None` on underflow.
    ///
    /// ```
    /// use std::time::Duration;
    /// let t0 = martensite_tkernel::osd::MonotonicInstant::now();
    /// let _ = t0.checked_sub(Duration::ZERO);
    /// ```
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.inner.checked_sub(duration).map(|inner| Self { inner })
    }
}

/// Returns the duration elapsed since `1970-01-01T00:00:00Z` (Unix epoch).
///
/// Uses `std::time::SystemTime::now()` on native targets and `js_sys::Date::now()`
/// on `wasm32`. Fails with `Standard_ProgramError` if the host clock reports a
/// timestamp before 1970-01-01 or a non-finite millisecond count.
///
/// ```
/// let since_epoch = martensite_tkernel::osd::unix_timestamp().unwrap();
/// assert!(since_epoch.as_secs() > 0);
/// ```
#[allow(clippy::disallowed_methods)]
pub fn unix_timestamp() -> Result<Duration, Failure> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| {
                crate::standard::exceptions::program_error(&format!(
                    "system clock is before 1970-01-01: {error:?}"
                ))
            })
    }
    #[cfg(target_arch = "wasm32")]
    {
        let millis: f64 = js_sys::Date::now();
        if !millis.is_finite() || millis < 0.0 {
            return Err(crate::standard::exceptions::program_error(&format!(
                "browser clock reported {millis:?} ms; expected a finite non-negative epoch offset"
            )));
        }
        Ok(millis_to_duration(millis))
    }
}

#[cfg(target_arch = "wasm32")]
fn millis_to_duration(millis: f64) -> Duration {
    let clamped: f64 = if millis.is_finite() && millis > 0.0 {
        millis
    } else {
        0.0
    };
    let whole_secs: u64 = (clamped / 1_000.0).floor() as u64;
    let rem_nanos: u32 = ((clamped % 1_000.0) * 1_000_000.0)
        .round()
        .clamp(0.0, 999_999_999.0) as u32;
    Duration::new(whole_secs, rem_nanos)
}

#[cfg(target_arch = "wasm32")]
fn wasm_performance_millis() -> Option<f64> {
    use wasm_bindgen::JsCast;
    let global = js_sys::global();
    let perf =
        js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("performance")).ok()?;
    let perf: web_sys::Performance = perf.dyn_into().ok()?;
    let ms: f64 = perf.now();
    ms.is_finite().then_some(ms)
}

#[cfg(target_arch = "wasm32")]
fn wasm_monotonic_duration() -> Duration {
    let millis: f64 = wasm_performance_millis().unwrap_or_else(js_sys::Date::now);
    millis_to_duration(millis)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn wasm_wall_seconds() -> f64 {
    use std::sync::OnceLock;
    static START_MILLIS: OnceLock<f64> = OnceLock::new();
    let now_ms: f64 = wasm_performance_millis().unwrap_or_else(js_sys::Date::now);
    let start_ms: f64 = *START_MILLIS.get_or_init(|| now_ms);
    ((now_ms - start_ms) * 1e-3).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::{MonotonicInstant, unix_timestamp};
    use std::time::Duration;

    #[test]
    fn monotonic_instant_is_non_decreasing() {
        let first = MonotonicInstant::now();
        let second = MonotonicInstant::now();
        assert!(second >= first);
        assert!(second.checked_duration_since(first).is_some());
        assert_eq!(first.saturating_duration_since(second), Duration::ZERO);
    }

    #[test]
    fn unix_timestamp_is_after_year_2020() {
        let now = unix_timestamp().expect("host clock should be after 1970");
        assert!(now.as_secs() > 1_577_836_800);
    }
}
