use super::{ContapSamplingContext, fake_surface::make_spline_surface};
use std::sync::{Arc, Barrier};

fn interleaved_samples(
    bounds: [f64; 4],
    spline: Option<[i32; 4]>,
    barrier: &Barrier,
) -> Vec<(u64, u64)> {
    let (surface, _) = make_spline_surface(bounds, None, spline);
    let mut context = ContapSamplingContext::new();
    let count = context.nb_sample_points(&surface).unwrap();
    barrier.wait();
    (1..=count)
        .map(|index| {
            let (u, v) = context.sample_point(&surface, index).unwrap();
            (u.to_bits(), v.to_bits())
        })
        .collect()
}

#[test]
fn barrier_interleaving_disjoint_contexts_keeps_their_serial_coordinates() {
    let cases = [
        ([1., 2., 9., 18.], None),
        ([101., 202., 109., 218.], Some([4, 3, 3, 3])),
    ];
    let serial: Vec<_> = cases
        .iter()
        .map(|&(bounds, spline)| interleaved_samples(bounds, spline, &Barrier::new(1)))
        .collect();
    for _ in 0..64 {
        let actual = parallel_samples(&cases);
        assert_eq!(actual, serial);
    }
}

type SamplingCase = ([f64; 4], Option<[i32; 4]>);
fn parallel_samples(cases: &[SamplingCase]) -> Vec<Vec<(u64, u64)>> {
    let barrier = Arc::new(Barrier::new(2));
    let workers: Vec<_> = cases
        .iter()
        .map(|&(bounds, spline)| {
            let barrier = barrier.clone();
            std::thread::spawn(move || interleaved_samples(bounds, spline, &barrier))
        })
        .collect();
    let actual: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    actual
}
