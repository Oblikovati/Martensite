use super::*;
use crate::RenderPolygonOffset;

#[test]
fn uneven_buckets_cover_the_original_range_in_order() {
    let span = RenderLayerSpan {
        draw_range: 10..267,
        depth_test: true,
        depth_write: false,
        clear_depth: true,
        polygon_offset: RenderPolygonOffset::default(),
        stencil_boundaries: Vec::new(),
        placement_sources: Arc::from([]),
    };
    let jobs = bucket_spans(
        RecordingJob {
            layer: 3,
            span,
            finish_layer: true,
        },
        4,
    );
    assert_eq!(
        jobs.iter()
            .map(|job| job.span.draw_range.clone())
            .collect::<Vec<_>>(),
        [10..74, 74..138, 138..202, 202..267],
    );
    assert_eq!(
        jobs.iter()
            .map(|job| (job.layer, job.finish_layer))
            .collect::<Vec<_>>(),
        [(3, false), (3, false), (3, false), (3, true)],
    );
}

#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "lit-only-command-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "raytrace-input-diagnostics",
    feature = "raytrace-hit-diagnostics",
)))]
mod native {
    use super::*;

    #[test]
    fn secondaries_preserve_ordered_color_depth_and_layer_clears() {
        use crate::RenderLayer;
        let workers = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let mut renderer =
            super::super::super::resource_lifecycle_test_support::create_resource_renderer();
        for layers in [
            vec![RenderLayer::new(
                triangles(257, 0.1, 0.2),
                true,
                true,
                false,
            )],
            layered(false),
            layered(true),
        ] {
            let expected_buffers = if layers.len() == 1 { 4 } else { 2 };
            let frame = RenderFrame::with_layers(32, 24, [0.1, 0.2, 0.3, 1.], layers).unwrap();
            verify_render(&workers, &mut renderer, &frame, expected_buffers);
        }
        assert!(
            renderer.validation_errors().unwrap().is_empty(),
            "{:?}",
            renderer.validation_errors().unwrap()
        );
    }

    fn verify_render(
        workers: &rayon::ThreadPool,
        renderer: &mut super::super::super::VulkanRenderer,
        frame: &RenderFrame,
        expected_buffers: usize,
    ) {
        let (reference, inline_count) = capture(workers, renderer, frame, true);
        let (parallel, parallel_count) = capture(workers, renderer, frame, false);
        assert_eq!(inline_count, 0);
        assert_eq!(parallel_count, expected_buffers);
        assert_eq!(reference.rgba, parallel.rgba);
        assert!(
            reference
                .depth
                .iter()
                .map(|value| value.to_bits())
                .eq(parallel.depth.iter().map(|value| value.to_bits()))
        );
    }

    fn layered(clear_depth: bool) -> Vec<crate::RenderLayer> {
        use crate::RenderLayer;
        vec![
            RenderLayer::new(triangles(64, 0.1, 0.3), true, true, false),
            RenderLayer::new(triangles(64, 0.7, 1.0), true, true, clear_depth),
        ]
    }

    fn triangles(count: usize, depth: f32, end: f32) -> Vec<crate::PrimitiveDraw> {
        use crate::{PrimitiveDraw, RenderVertex, identity_clip};
        let corners = [[-0.8, -0.8, depth], [end, -0.8, depth], [-0.8, 0.8, depth]];
        (0..count)
            .map(|index| {
                let color = [0.9, (index % 17) as f32 / 16., depth, 1.];
                let vertices = corners.map(|point| RenderVertex::new(point, color));
                PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip()).unwrap()
            })
            .collect()
    }

    fn capture(
        workers: &rayon::ThreadPool,
        renderer: &mut super::super::super::VulkanRenderer,
        frame: &RenderFrame,
        inline: bool,
    ) -> (crate::FrameCapture, usize) {
        renderer
            .runtime
            .secondary_pools
            .disabled
            .store(inline, std::sync::atomic::Ordering::Relaxed);
        let resources = renderer.prepared_owned_frame(frame).unwrap();
        let resources = workers
            .install(|| renderer.complete_prepared_frame(frame, resources))
            .unwrap();
        (
            renderer.read_frame(&resources.readback_ticket()).unwrap(),
            resources.secondary_layers.commands.lock().unwrap().len(),
        )
    }
}
