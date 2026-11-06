use super::*;

fn params(atomic: bool) -> Graphic3dRenderingParams {
    let mut params = Graphic3dRenderingParams::new().unwrap();
    params.ray_tracing.adaptive_screen_sampling = true;
    params.ray_tracing.adaptive_screen_sampling_atomic = atomic;
    params
}

/// A 100x70 view in 32-pixel tiles (4x3 tiles) with 64 offset tiles (8x8 shrunk map).
fn adaptive(atomic: bool) -> PathtraceAdaptive {
    let mut params = params(atomic);
    params.ray_tracing.ray_tracing_tile_size = 32;
    params.ray_tracing.nb_ray_tracing_tiles = 64;
    pathtrace_adaptive(&params).unwrap()
}

#[test]
fn admission_clears_only_the_unsupported_flags() {
    let mut supported = params(true);
    assert!(!admit_adaptive_sampling(&mut supported, (true, true)));
    assert!(supported.ray_tracing.adaptive_screen_sampling_atomic);
    let mut without_atomics = params(true);
    assert!(admit_adaptive_sampling(&mut without_atomics, (true, false)));
    assert!(without_atomics.ray_tracing.adaptive_screen_sampling);
    assert!(!without_atomics.ray_tracing.adaptive_screen_sampling_atomic);
    let mut without_adaptive = params(false);
    assert!(admit_adaptive_sampling(
        &mut without_adaptive,
        (false, false)
    ));
    assert!(!without_adaptive.ray_tracing.adaptive_screen_sampling);
    let mut off = Graphic3dRenderingParams::new().unwrap();
    assert!(!admit_adaptive_sampling(&mut off, (false, false)));
    assert_eq!(pathtrace_adaptive(&off), None);
}

#[test]
fn the_first_frame_samples_every_tile_once_in_one_pass() {
    let (mut sampling, mut random) = (AdaptiveSampling::default(), BullardGenerator::default());
    let mut frame = adaptive(false);
    sampling
        .prepare(&mut frame, ([100, 70], 0), &mut random)
        .unwrap();
    let passes = frame.passes.unwrap();
    assert_eq!(passes.tiles, [4, 3]);
    assert_eq!(passes.tile_samples[0], 32 * 32);
    assert_eq!(passes.tile_samples[11], 4 * 6);
    assert_eq!((passes.offset_tiles, passes.viewport), ([4, 3], [100, 70]));
    let mut expected = BullardGenerator::default();
    assert_eq!(passes.seeds, vec![(expected.next_int() >> 2) as i32]);
}

#[test]
fn later_frames_redistribute_the_shrunk_offsets_from_the_published_error() {
    let (mut sampling, mut random) = (AdaptiveSampling::default(), BullardGenerator::default());
    for (frames, atomic) in [(0, false), (1, false), (2, false)] {
        let mut frame = adaptive(atomic);
        sampling
            .prepare(&mut frame, ([100, 70], frames), &mut random)
            .unwrap();
        let passes = frame.passes.unwrap();
        // All error in the first tile: the shrunk map redirects every offset there.
        let mut errors = vec![0; 12];
        errors[0] = 1000;
        passes.feedback.publish(errors);
        if frames < 2 {
            assert_eq!(passes.seeds.len(), 1, "frame {frames}");
            continue;
        }
        assert_eq!(passes.offset_tiles, [8, 8]);
        assert_eq!(passes.tile_samples[0], 64 * 32 * 32);
        assert_eq!(passes.seeds.len(), 64);
    }
}

#[test]
fn the_atomic_variant_draws_one_pass_over_the_offset_viewport() {
    let (mut sampling, mut random) = (AdaptiveSampling::default(), BullardGenerator::default());
    for frames in 0..3 {
        let mut frame = adaptive(true);
        sampling
            .prepare(&mut frame, ([100, 70], frames), &mut random)
            .unwrap();
        let passes = frame.passes.unwrap();
        assert_eq!(passes.seeds.len(), 1);
        let expected = if frames > 1 { [256, 256] } else { [128, 96] };
        assert_eq!(passes.viewport, expected, "frame {frames}");
        assert_eq!(
            passes.offsets.len(),
            (passes.offset_tiles[0] * passes.offset_tiles[1]) as usize
        );
        passes.feedback.publish(vec![1; 12]);
    }
}

#[test]
fn feedback_compares_by_identity() {
    let feedback = VisualErrorFeedback::default();
    assert_eq!(feedback, feedback.clone());
    assert_ne!(feedback, VisualErrorFeedback::default());
    feedback.publish(vec![3]);
    assert_eq!(feedback.take(), Some(vec![3]));
    assert_eq!(feedback.take(), None);
}
