use super::*;
use crate::fragment_alpha_feedback::FragmentAlphaFeedback;
use crate::stencil_feedback::StencilFeedback;

fn frame() -> RenderFrame {
    RenderFrame::new(2, 2, [0.; 4], vec![]).unwrap()
}

#[test]
fn an_unsubmitted_frame_puts_back_the_feedback_of_its_view() {
    let (alpha, stencil) = (FragmentAlphaFeedback::default(), StencilFeedback::default());
    let mut handed = frame();
    handed.attach_fragment_alpha(&alpha);
    handed.attach_stencil_context(&stencil);
    let rollback = HandoffRollback::before(&handed, Default::default(), Default::default());
    let context = handed.fragment_alpha_context.as_ref().unwrap();
    context.commit(LegacyFragmentAlpha(Some(0.5)));
    let mut changed = handed.stencil_state();
    changed.enabled = !changed.enabled;
    handed.stencil_context.as_ref().unwrap().commit(changed);
    let (mut renderer_alpha, mut renderer_stencil) = (Default::default(), Default::default());
    rollback.undo(&mut renderer_alpha, &mut renderer_stencil);
    let mut next = frame();
    next.attach_fragment_alpha(&alpha);
    next.attach_stencil_context(&stencil);
    assert_eq!(
        next.initial_fragment_alpha(Default::default()),
        LegacyFragmentAlpha(None)
    );
    assert_eq!(next.stencil_state(), RenderStencilState::default());
}

#[test]
fn a_frame_without_a_view_context_puts_back_the_renderer_state() {
    let previous = LegacyFragmentAlpha(Some(0.25));
    let rollback = HandoffRollback::before(&frame(), previous, RenderStencilState::default());
    let mut alpha = LegacyFragmentAlpha(Some(0.75));
    let mut stencil = RenderStencilState {
        enabled: true,
        ..Default::default()
    };
    rollback.undo(&mut alpha, &mut stencil);
    assert_eq!(alpha, previous);
    assert_eq!(stencil, RenderStencilState::default());
}
