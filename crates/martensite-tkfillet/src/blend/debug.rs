//! The process-wide debug switches of the blend algorithms.
use std::sync::atomic::{AtomicBool, Ordering};

static TRACE_DRAW_SECT: AtomicBool = AtomicBool::new(false);
static CONTEXT_NO_TEST_DEFL: AtomicBool = AtomicBool::new(false);

/// `BlendSettraceDRAWSECT(b)`: whether the approximated fillet sections are traced;
/// `set_trace_draw_sect(false)`.
pub fn set_trace_draw_sect(value: bool) {
    TRACE_DRAW_SECT.store(value, Ordering::Relaxed);
}

/// `BlendGettraceDRAWSECT()`; `get_trace_draw_sect()`.
pub fn get_trace_draw_sect() -> bool {
    TRACE_DRAW_SECT.load(Ordering::Relaxed)
}

/// `BlendSetcontextNOTESTDEFL(b)`: whether the walking skips the deflection test;
/// `set_context_no_test_defl(false)`.
pub fn set_context_no_test_defl(value: bool) {
    CONTEXT_NO_TEST_DEFL.store(value, Ordering::Relaxed);
}

/// `BlendGetcontextNOTESTDEFL()`: false unless set (the native default);
/// `get_context_no_test_defl()`.
pub fn get_context_no_test_defl() -> bool {
    CONTEXT_NO_TEST_DEFL.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switches_round_trip_and_default_to_false() {
        assert!(!get_context_no_test_defl());
        set_trace_draw_sect(true);
        assert!(get_trace_draw_sect());
        set_trace_draw_sect(false);
        assert!(!get_trace_draw_sect());
    }
}
