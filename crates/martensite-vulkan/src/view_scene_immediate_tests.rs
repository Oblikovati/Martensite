use crate::immediate_elision::{ImmediateViewInputs, MainElision};
use crate::immediate_main::eligible_start;
use crate::immediate_redraw_fixture::{SceneExtractionOnly, display_sphere, lit_sphere_view};
use crate::{Graphic3dRenderView, RenderFrame};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dClipPlaneSequenceSource;
use martensite_tkv3d::{ais::AisInteractiveContext, v3d::V3dViewer};

fn draws_text(frame: &RenderFrame, start: usize) -> String {
    let first = frame.layers[start].draw_range.start;
    format!("{:#?}", &frame.draws[first..])
}

#[test]
fn the_immediate_frame_holds_the_immediate_draws_of_the_whole_frame() {
    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 4096).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let v3d = lit_sphere_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    display_sphere(&context);
    v3d.auto_z_fit().unwrap();
    context.move_to(64, 64, &v3d, false).unwrap();
    assert!(context.has_detected());
    let view: Handle<Graphic3dRenderView> = v3d.get().unwrap().view().cast();
    let view = view.get().unwrap();
    let clipping = Handle::<dyn Graphic3dClipPlaneSequenceSource>::default();
    let extras = Default::default();
    let whole = crate::view_scene::frame_for_view(view, [128, 128], &clipping, extras).unwrap();
    let start = eligible_start(&whole).unwrap();
    let inputs = ImmediateViewInputs::capture(view, [128, 128]);
    let elision = MainElision::capture(0, inputs, &whole, start);
    let frame =
        super::immediate_frame_for_view(view, [128, 128], &clipping, extras, &elision).unwrap();
    assert_eq!(frame.immediate_start, Some(start));
    assert!(
        frame.layers[..start]
            .iter()
            .all(|span| span.draw_range.is_empty())
    );
    assert!(
        whole.layers[..start]
            .iter()
            .any(|span| !span.draw_range.is_empty())
    );
    assert!(
        frame.layers[start..]
            .iter()
            .any(|span| !span.draw_range.is_empty())
    );
    assert_eq!(draws_text(&frame, start), draws_text(&whole, start));
}
