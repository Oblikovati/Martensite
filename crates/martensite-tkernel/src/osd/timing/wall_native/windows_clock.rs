//! Keep caching and fallback ordering testable without inducing native failures (#89).
#![forbid(unsafe_code)]
use std::sync::OnceLock;

pub(super) trait ClockQueries {
    fn frequency(&mut self) -> Option<i64>;
    fn counter(&mut self) -> Option<i64>;
    fn ticks(&mut self) -> u64;
}

pub(super) fn seconds(cache: &OnceLock<Option<i64>>, source: &mut impl ClockQueries) -> f64 {
    let frequency = *cache.get_or_init(|| source.frequency());
    if let Some(frequency) = frequency
        && let Some(counter) = source.counter()
    {
        return counter as f64 / frequency as f64;
    }
    0.001 * source.ticks() as f64
}

#[cfg(test)]
mod contracts;
