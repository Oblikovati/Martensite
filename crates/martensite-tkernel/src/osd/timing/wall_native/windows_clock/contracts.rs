use super::{ClockQueries, seconds};
use std::{collections::VecDeque, sync::OnceLock};

#[derive(Debug, PartialEq)]
enum ClockCall {
    Frequency,
    Counter,
    Ticks,
}

struct ScriptedClockQueries {
    frequency: Option<i64>,
    counters: VecDeque<Option<i64>>,
    ticks: u64,
    calls: Vec<ClockCall>,
}

impl ScriptedClockQueries {
    fn new(frequency: Option<i64>, counters: &[Option<i64>], ticks: u64) -> Self {
        Self {
            frequency,
            counters: counters.iter().copied().collect(),
            ticks,
            calls: Vec::new(),
        }
    }
}

impl ClockQueries for ScriptedClockQueries {
    fn frequency(&mut self) -> Option<i64> {
        self.calls.push(ClockCall::Frequency);
        self.frequency
    }
    fn counter(&mut self) -> Option<i64> {
        self.calls.push(ClockCall::Counter);
        self.counters.pop_front().expect("unexpected counter query")
    }
    fn ticks(&mut self) -> u64 {
        self.calls.push(ClockCall::Ticks);
        self.ticks
    }
}

#[test]
fn successful_frequency_is_cached_without_tick_queries() {
    let cache = OnceLock::new();
    let mut source = ScriptedClockQueries::new(Some(2), &[Some(5), Some(7)], 99);
    assert_eq!(seconds(&cache, &mut source), 2.5);
    source.frequency = Some(100);
    assert_eq!(seconds(&cache, &mut source), 3.5);
    assert_eq!(
        source.calls,
        [ClockCall::Frequency, ClockCall::Counter, ClockCall::Counter]
    );
}

#[test]
fn failed_frequency_stays_cached_and_never_queries_counter() {
    let cache = OnceLock::new();
    let mut source = ScriptedClockQueries::new(None, &[], 1_250);
    assert_eq!(seconds(&cache, &mut source), 1.25);
    source.frequency = Some(2);
    source.ticks = 2_500;
    assert_eq!(seconds(&cache, &mut source), 2.5);
    assert_eq!(
        source.calls,
        [ClockCall::Frequency, ClockCall::Ticks, ClockCall::Ticks]
    );
}

#[test]
fn counter_failure_falls_back_without_invalidating_frequency() {
    let cache = OnceLock::new();
    let mut source = ScriptedClockQueries::new(Some(2), &[None, Some(9)], 500);
    assert_eq!(seconds(&cache, &mut source), 0.5);
    assert_eq!(seconds(&cache, &mut source), 4.5);
    assert_eq!(
        source.calls,
        [
            ClockCall::Frequency,
            ClockCall::Counter,
            ClockCall::Ticks,
            ClockCall::Counter
        ]
    );
}

#[test]
fn division_retains_zero_negative_and_rounding_behavior() {
    for (frequency, counter, expected) in [
        (0, 1, f64::INFINITY),
        (-2, 5, -2.5),
        (3, 1, f64::from_bits(0x3fd5_5555_5555_5555)),
        (1, 9_007_199_254_740_993, 9_007_199_254_740_992.0),
    ] {
        let mut source = ScriptedClockQueries::new(Some(frequency), &[Some(counter)], 99);
        assert_eq!(
            seconds(&OnceLock::new(), &mut source).to_bits(),
            expected.to_bits()
        );
        assert_eq!(source.calls, [ClockCall::Frequency, ClockCall::Counter]);
    }
}

#[test]
fn zero_divided_by_zero_is_not_replaced_by_fallback() {
    let mut source = ScriptedClockQueries::new(Some(0), &[Some(0)], 99);
    assert!(seconds(&OnceLock::new(), &mut source).is_nan());
    assert_eq!(source.calls, [ClockCall::Frequency, ClockCall::Counter]);
}
