//! Differences of `RenderDeviceWork` totals, as the views and the parity harness take them.
use super::{RenderDeviceWork, RenderWorkTimes};
use std::time::Duration;

fn totals(seed: u64) -> RenderDeviceWork {
    RenderDeviceWork {
        redraws: seed,
        buffers_created: 2 * seed,
        geometry_buffers_created: 10 * seed,
        images_created: 3 * seed,
        memory_allocations: 4 * seed,
        pipelines_created: 5 * seed,
        descriptor_sets_allocated: 6 * seed,
        submissions: 7 * seed,
        uploaded_bytes: 8 * seed,
        geometry_uploaded_bytes: 11 * seed,
        read_back_bytes: 9 * seed,
        times: RenderWorkTimes {
            redraw: Duration::from_micros(10 * seed),
            scene: Duration::from_micros(seed),
            upload: Duration::from_micros(2 * seed),
            recording: Duration::from_micros(3 * seed),
            submission: Duration::from_micros(4 * seed),
            waiting: Duration::from_micros(5 * seed),
            readback: Duration::from_micros(6 * seed),
        },
    }
}

#[test]
fn since_subtracts_every_count_and_phase() {
    assert_eq!(totals(5).since(&totals(2)), totals(3));
}

#[test]
fn since_an_unrelated_later_total_saturates_at_zero() {
    assert_eq!(totals(1).since(&totals(4)), RenderDeviceWork::default());
}
