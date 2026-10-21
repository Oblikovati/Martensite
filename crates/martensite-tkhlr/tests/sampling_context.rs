//! Explicit sampling bounds preserve arithmetic without sharing historical state.
use martensite_tkhlr::contap::{ContapHContTool, ContapSamplingContext};
#[path = "sampling_context/concurrency.rs"]
mod concurrency;
#[path = "sampling_context/differential.rs"]
mod differential;
#[path = "sampling_context/failure_order.rs"]
mod failure_order;
#[path = "sampling_context/fake_surface.rs"]
mod fake_surface;
#[path = "sampling_context/ranges.rs"]
mod ranges;

#[test]
fn owned_context_is_send_sync_and_clones_independently() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ContapSamplingContext>();
    let mut original = ContapSamplingContext::new();
    let (surface, _) = fake_surface::make_surface([1., 2., 9., 18.], None);
    original.nb_sample_points(&surface).unwrap();
    let copy = original.clone();
    let (changed, _) = fake_surface::make_surface([0., 0., 4., 8.], None);
    original.nb_sample_points(&changed).unwrap();
    assert_eq!(copy.bounds(), [1., 2., 9., 18.]);
    assert_eq!(original.bounds(), [0., 0., 4., 8.]);
}
