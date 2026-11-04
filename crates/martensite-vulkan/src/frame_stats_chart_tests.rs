//! Native updateChart numbers: label format, stacking and the corner offsets.
use super::*;

#[test]
fn time_labels_use_whole_milliseconds_and_one_decimal_rate() {
    assert_eq!(format_time_ms(0.05), "50 ms (20.0 FPS)");
    assert_eq!(format_time_ms(0.0), "0 ms (0.0 FPS)");
}

#[test]
fn cpu_timers_stack_and_the_elapsed_bar_starts_at_zero() {
    let heights = stacked_heights([0.01, 0.01, 0.0, 0.05], 0.1, 100.0);
    assert_eq!(heights[0], [0.0, 10.0]);
    assert_eq!(heights[1], [10.0, 20.0]);
    assert_eq!(heights[2], [20.0, 20.0]);
    assert_eq!(heights[3], [20.0, 50.0]);
}

#[test]
fn right_corners_place_the_chart_left_of_the_anchor() {
    assert_eq!(
        chart_offset(Some(Corner::RIGHT_UPPER), [160, 36]),
        [-160, 0]
    );
    assert_eq!(chart_offset(Some(Corner::LEFT_LOWER), [160, 36]), [0, 36]);
    assert_eq!(chart_offset(None, [160, 36]), [0, 0]);
}
