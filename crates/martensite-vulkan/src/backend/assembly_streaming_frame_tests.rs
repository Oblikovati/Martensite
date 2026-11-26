use super::*;
use crate::{RenderVertex, identity_clip};

fn part(x: f32, color: [f32; 4]) -> PrimitiveDraw {
    let points = [[-0.12, -0.2, 0.], [0.12, -0.2, 0.], [0., 0.2, 0.]];
    let mut clip = identity_clip();
    clip[12] = x;
    PrimitiveDraw::new(
        points.map(|point| RenderVertex::new(point, color)).to_vec(),
        vec![0, 1, 2],
        clip,
    )
    .unwrap()
}

#[test]
fn partial_layers_preserve_source_order_and_cover_empty_ranges() {
    let green = part(-0.6, [0., 1., 0., 1.]);
    let red = part(0.4, [1., 0., 0., 1.]);
    let layers = vec![
        crate::RenderLayer::new(vec![green.clone(), red.clone()], true, true, false),
        crate::RenderLayer::new(vec![green.clone()], true, true, true),
        crate::RenderLayer::new(vec![red.clone()], true, true, false),
    ];
    let frame = RenderFrame::with_layers(32, 24, [0., 0., 0., 1.], layers).unwrap();
    let filtered = partial_frame(&frame, [true, true, false, true].into_iter(), 3);
    assert_eq!(filtered.draws, [green, red.clone(), red]);
    assert_eq!(filtered.layers[0].draw_range(), 0..2);
    assert_eq!(filtered.layers[1].draw_range(), 2..2);
    assert_eq!(filtered.layers[1].depth_policy(), (true, true, true));
    assert_eq!(filtered.layers[2].draw_range(), 2..3);
}

#[cfg(target_os = "linux")]
macro_rules! support_unit_tests {
    ($($tests:tt)*) => {};
}
#[cfg(target_os = "linux")]
#[path = "../../examples/support/native_window.rs"]
mod native_window;

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a mapped native X11 window"]
fn cold_native_parts_stream_then_match_complete_color_and_depth() {
    use super::super::resource_lifecycle_test_support::create_resource_renderer;
    use crate::{RenderBackend, RenderGuiSurfacePolicy};

    let mut renderer = create_resource_renderer();
    let native = native_window::NativeWindow::new(64, 64);
    let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
    let near = part(0., [0., 1., 0., 1.]);
    let far = part(-0.6, [1., 0., 0., 1.]);
    let mut occurrence = near.clone();
    occurrence.clip_transform[12] = 0.6;
    let mut frame = RenderFrame::new(
        64,
        64,
        [0., 0., 0., 1.],
        vec![far.clone(), near, occurrence],
    )
    .unwrap();
    frame.assembly_upload_budget = Some(stream_bytes(&far));
    let render = |renderer: &mut VulkanRenderer, frame: &RenderFrame, presenter: &mut _| {
        let ticket = renderer
            .render_window(
                frame,
                presenter,
                crate::driver_options::WindowFrameQueue::Unrestricted,
                RenderGuiSurfacePolicy::default(),
            )
            .unwrap();
        let capture = renderer.read_frame(&ticket).unwrap();
        (ticket.assembly_streaming, capture)
    };
    assert!(
        renderer
            .plan_assembly_streaming(&frame.clone().with_stencil_clear_value(1))
            .is_none()
    );
    let (first, partial) = render(&mut renderer, &frame, &mut presenter);
    let first = first.unwrap();
    assert_eq!((first.drawn, first.visible), (2, 3));
    assert!(first.uploaded_bytes > 0 && first.uploaded_bytes <= first.budget_bytes);
    assert!(partial.rgba.as_chunks::<4>().0.contains(&[0, 255, 0, 255]));
    assert!(!partial.rgba.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
    let (second, complete) = render(&mut renderer, &frame, &mut presenter);
    assert!(second.unwrap().complete());
    let (hot, _) = render(&mut renderer, &frame, &mut presenter);
    assert_eq!(hot.unwrap().uploaded_bytes, 0);

    frame.assembly_upload_budget = None;
    let (cancelled, ordinary) = render(&mut renderer, &frame, &mut presenter);
    assert!(cancelled.is_none());
    assert!(presenter.assembly_sources.is_empty());
    let baseline = renderer.render(&frame).unwrap();
    assert_eq!(complete.rgba, ordinary.rgba);
    assert_eq!(complete.depth, ordinary.depth);
    assert_eq!(complete.rgba, baseline.rgba);
    assert_eq!(complete.depth, baseline.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a mapped native X11 window"]
fn moved_focus_and_failed_redraw_use_current_geometry_on_retry() {
    use super::super::{
        resource_lifecycle_test_support::create_resource_renderer,
        window_gui_contract::WindowPresentationProbe,
    };
    use crate::{RenderError, RenderGuiSurfacePhase, RenderGuiSurfacePolicy};
    struct RejectBeforeSubmit;
    impl WindowPresentationProbe for RejectBeforeSubmit {
        fn observe(&mut self, phase: RenderGuiSurfacePhase) -> Result<(), RenderError> {
            if phase == RenderGuiSurfacePhase::BeforeSubmit {
                return Err(crate::error::rejected("test refusal", "a submitted redraw"));
            }
            Ok(())
        }
    }

    let mut renderer = create_resource_renderer();
    let native = native_window::NativeWindow::new(64, 64);
    let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
    let colors = [[1., 0., 0., 1.], [0., 1., 0., 1.], [0., 0., 1., 1.]];
    let draws = [-0.6, 0., 0.6]
        .into_iter()
        .zip(colors)
        .map(|(x, rgb)| part(x, rgb));
    let mut frame = RenderFrame::new(64, 64, [0., 0., 0., 1.], draws.collect()).unwrap();
    frame.assembly_upload_budget = Some(stream_bytes(&frame.draws[0]));
    let policy = RenderGuiSurfacePolicy::default();
    let first = renderer
        .render_window(
            &frame,
            &mut presenter,
            crate::driver_options::WindowFrameQueue::Unrestricted,
            policy,
        )
        .unwrap();
    assert_eq!(first.assembly_streaming.unwrap().drawn, 1);
    renderer.finish_frames().unwrap();

    // Shift both remaining cold parts so only blue is closest to the new focal point.
    frame.draws[0].clip_transform[12] = -0.7;
    frame.draws[1].clip_transform[12] = -0.4;
    frame.draws[2].clip_transform[12] = 0.12;
    let rejected = renderer.render_window_with_probe(
        &frame,
        &mut presenter,
        crate::driver_options::WindowFrameQueue::Unrestricted,
        policy,
        &mut RejectBeforeSubmit,
    );
    assert!(rejected.is_err());
    assert!(presenter.assembly_sources.is_empty());
    let retry = renderer
        .render_window(
            &frame,
            &mut presenter,
            crate::driver_options::WindowFrameQueue::Unrestricted,
            policy,
        )
        .unwrap();
    let progress = retry.assembly_streaming.unwrap();
    assert_eq!((progress.drawn, progress.visible), (2, 3));
    assert!(progress.uploaded_bytes <= progress.budget_bytes);
    let capture = renderer.read_frame(&retry).unwrap();
    assert!(capture.rgba.as_chunks::<4>().0.contains(&[0, 0, 255, 255]));
    assert!(!capture.rgba.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a mapped native X11 window"]
fn graphic3d_view_opts_in_without_changing_complete_redraw() {
    use crate::{
        Graphic3dRenderDriver, Graphic3dRenderView, RenderAssemblyStreaming, VulkanOptions,
    };
    use martensite_tkernel::standard::Handle;
    use martensite_tkmath::gp::{Direction3, Point3};
    use martensite_tkservice::graphic3d::*;
    use martensite_tkv3d::v3d::V3dViewer;
    use std::num::NonZeroU64;

    fn display_part(view: &Graphic3dRenderView, x: f64) -> Handle<Graphic3dStructure> {
        let structure =
            Graphic3dStructure::new(&view.structure_manager(), &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        let array = Handle::new(
            Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 0, 0)
                .unwrap(),
        );
        for (x, y) in [(x - 0.4, -0.4), (x + 0.4, -0.4), (x, 0.4)] {
            array.add_vertex_f64(x, y, 0.).unwrap();
        }
        let style = Handle::new(Graphic3dAspects::default());
        style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
        group.set_group_primitives_aspect(&style.cast());
        group.add_primitive_array(&array.upcast(), true);
        structure.calculate_bound_box();
        structure.display();
        structure
    }

    let driver = Graphic3dRenderDriver::new(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let native = native_window::NativeWindow::new(128, 128);
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = viewer.create_view().unwrap();
    view.set_immediate_update(false);
    view.set_window(&native.window.cast(), 0).unwrap();
    view.set_auto_z_fit_mode(false, 1.).unwrap();
    let camera = view.camera();
    camera.set_eye(&Point3::new(0., 0., 20.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_scale(10.);
    camera.set_z_range(1., 100.).unwrap();
    let rendered: Handle<Graphic3dRenderView> = view.get().unwrap().view().cast();
    let _parts = [display_part(&rendered, 0.), display_part(&rendered, 2.)];
    assert!(rendered.assembly_streaming_progress().unwrap().is_none());
    rendered.set_assembly_streaming(RenderAssemblyStreaming::VisibleBudgetedV2(
        NonZeroU64::new(144).unwrap(),
    ));
    rendered.try_redraw().unwrap();
    let first = rendered.assembly_streaming_progress().unwrap().unwrap();
    assert_eq!((first.drawn, first.visible), (1, 2));
    rendered.try_redraw().unwrap();
    assert!(
        rendered
            .assembly_streaming_progress()
            .unwrap()
            .unwrap()
            .complete()
    );
    let completed = rendered.capture().unwrap();
    rendered.set_assembly_streaming(RenderAssemblyStreaming::CompleteV1);
    rendered.try_redraw().unwrap();
    assert!(rendered.assembly_streaming_progress().unwrap().is_none());
    let ordinary = rendered.capture().unwrap();
    assert_eq!(completed.rgba, ordinary.rgba);
    assert_eq!(completed.depth, ordinary.depth);
    assert!(driver.validation_errors().unwrap().is_empty());
}
