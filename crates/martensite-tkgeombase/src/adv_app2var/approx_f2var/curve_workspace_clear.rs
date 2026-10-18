//! Model the one initialized trailing allocator flag without extending numeric storage.
use super::CurveProjectionFrame;

impl CurveProjectionFrame {
    pub(in crate::adv_app2var::approx_f2var) fn clear_coefficient_span(
        &mut self,
        offset: usize,
        count: i32,
    ) {
        if count <= 0 {
            return;
        }
        let (start, end) = self.coefficient_clear_bounds(offset, count);
        let payload_end = end.min(self.values.len());
        self.values[start..payload_end].fill(0.);
        //  SysBase::macrgfl_ initializes this scalar; madbtbk_ disables its release check.
        if end > self.values.len() {
            self.trailing_flag = 0.;
        }
    }

    fn coefficient_clear_bounds(&self, offset: usize, count: i32) -> (usize, usize) {
        let start = self.coefficient_start.checked_add(offset);
        let end = start.and_then(|start| start.checked_add(count as usize));
        assert!(
            end.is_some_and(|end| end <= self.values.len() + 1),
            "MMA2fnc: coefficient origin={}, offset={offset}, count={count}; expected numeric owner and one trailing flag within 0..{}",
            self.coefficient_start,
            self.values.len() + 1
        );
        (start.unwrap(), end.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> CurveProjectionFrame {
        CurveProjectionFrame::with_zone_starts([1, 2, 2, 3], vec![7.; 4])
    }

    #[test]
    fn payload_clear_preserves_initialized_flag_and_other_regions() {
        let mut frame = frame();
        frame.clear_coefficient_span(0, 1);
        assert_eq!(frame.values, [7., 7., 7., 0.]);
        assert_eq!(frame.trailing_flag.to_bits(), (-134744073_f64).to_bits());
    }

    #[test]
    fn flag_only_clear_keeps_numeric_allocation() {
        let mut frame = frame();
        let pointer = frame.values.as_ptr();
        frame.clear_coefficient_span(1, 1);
        assert_eq!(frame.trailing_flag.to_bits(), 0_f64.to_bits());
        let values = frame.into_values();
        assert_eq!(values, [7.; 4]);
        assert_eq!(values.as_ptr(), pointer);
    }

    #[test]
    fn spanning_clear_reaches_only_payload_tail_and_flag() {
        let mut frame = frame();
        frame.clear_coefficient_span(0, 2);
        assert_eq!(frame.values, [7., 7., 7., 0.]);
        assert_eq!(frame.trailing_flag.to_bits(), 0_f64.to_bits());
    }

    #[test]
    fn nonpositive_count_does_not_form_or_access_a_slot() {
        let mut frame = frame();
        frame.clear_coefficient_span(usize::MAX, 0);
        frame.clear_coefficient_span(usize::MAX, -1);
        assert_eq!(frame.values, [7.; 4]);
        assert_eq!(frame.trailing_flag.to_bits(), (-134744073_f64).to_bits());
    }

    #[test]
    fn requests_past_the_single_flag_are_rejected_without_stores() {
        let mut frame = frame();
        for (offset, count) in [(0, 3), (2, 1), (usize::MAX, 1)] {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                frame.clear_coefficient_span(offset, count);
            }));
            assert!(result.is_err());
            assert_eq!(frame.values, [7.; 4]);
            assert_eq!(frame.trailing_flag.to_bits(), (-134744073_f64).to_bits());
        }
    }
}
