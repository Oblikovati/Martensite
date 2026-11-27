use super::*;
use martensite_tkservice::aspect::AspectInteriorStyle;

#[test]
fn the_unbound_context_answers_the_native_initial_values() {
    let state = RenderContextState::default();
    assert!(!state.is_valid());
    assert_eq!(state.viewport(), [0; 4]);
    assert_eq!(
        state.face_culling(),
        Graphic3dTypeOfBackfacingModel::DoubleSided
    );
    assert_eq!((state.resolution(), state.line_width_scale()), (72, 1.));
    let mut stream = DumpStream::default();
    state.dump_json(&mut stream, -1).unwrap();
    assert!(
        stream
            .text()
            .starts_with("\"className\": \"GlContext\", \"AnisoMax\": 1, \"MaxTexDim\": 1024"),
        "{}",
        stream.text()
    );
}

#[test]
fn set_resolution_scales_resolution_ratio_and_line_width_as_native() {
    let mut state = RenderContextState::default();
    state.set_resolution(96, 96. / 72., 2.);
    assert_eq!(state.resolution(), 192);
    assert_eq!(state.resolution_ratio(), 96. / 72. * 2.);
    assert_eq!(state.line_width_scale(), 3.);
    assert_eq!(state.render_scale_inv(), 0.5);
    state.resize_viewport([129, 64]);
    assert_eq!(state.viewport(), [0, 0, 129, 64]);
    // ResizeViewport truncates `side * (1 / scale)`.
    assert_eq!(state.virtual_viewport(), [0, 0, 64, 32]);
}

#[test]
fn applied_aspects_resolve_culling_offset_and_coverage() {
    let mut state = RenderContextState {
        allow_alpha_to_coverage: true,
        ..RenderContextState::default()
    };
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_interior_style(AspectInteriorStyle::Solid);
    aspect.set_alpha_mode(Graphic3dAlphaMode::Mask, 0.5);
    let aspects = aspect.clone().cast::<dyn Graphic3dAspectsSource>();
    state.apply_aspects(&aspects, true);
    // Masked surfaces stay double-sided under the automatic rule.
    assert_eq!(
        state.face_culling(),
        Graphic3dTypeOfBackfacingModel::DoubleSided
    );
    assert!(state.sample_alpha_to_coverage());
    aspect.set_alpha_mode(Graphic3dAlphaMode::Opaque, 0.5);
    state.apply_aspects(&aspects, true);
    assert_eq!(
        state.face_culling(),
        Graphic3dTypeOfBackfacingModel::BackCulled
    );
    assert!(!state.sample_alpha_to_coverage());
    state.view_backfacing = Graphic3dTypeOfBackfacingModel::FrontCulled;
    state.apply_aspects(&aspects, false);
    assert_eq!(
        state.face_culling(),
        Graphic3dTypeOfBackfacingModel::FrontCulled
    );
}
