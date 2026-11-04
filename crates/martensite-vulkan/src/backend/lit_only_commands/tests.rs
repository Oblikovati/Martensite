use super::*;
use crate::{PrimitiveDraw, RenderVertex, identity_clip};

struct NamedFinalCommand {
    issued: usize,
    decisions: Vec<bool>,
}
impl FinalDrawCommand for NamedFinalCommand {
    fn issue_draw(&mut self) {
        self.issued += 1;
    }
    fn record_decision(&mut self, retained: bool) {
        self.decisions.push(retained);
    }
}
#[test]
fn default_mode_records_every_command_without_control_manifest() {
    let mut commands = NamedFinalCommand {
        issued: 0,
        decisions: vec![],
    };
    for lit in [false, true, false] {
        LitOnlyCommands::Normal.issue(lit, &mut commands);
    }
    assert_eq!(commands.issued, 3);
    assert!(commands.decisions.is_empty());
}
#[test]
fn control_emits_only_lit_but_records_each_actual_decision() {
    let mut commands = NamedFinalCommand {
        issued: 0,
        decisions: vec![],
    };
    for lit in [false, true, false, true] {
        LitOnlyCommands::PhongTriangles.issue(lit, &mut commands);
    }
    assert_eq!(commands.issued, 2);
    assert_eq!(commands.decisions, [false, true, false, true]);
}
#[test]
fn setting_is_literal_and_failures_remain_typed() {
    assert_eq!(
        LitOnlyCommands::requested(None).unwrap(),
        LitOnlyCommands::Normal
    );
    assert_eq!(
        LitOnlyCommands::requested(Some(OsStr::new("1"))).unwrap(),
        LitOnlyCommands::PhongTriangles
    );
    for value in ["", "0", "true", "lit-only"] {
        let failure = LitOnlyCommands::requested(Some(OsStr::new(value))).unwrap_err();
        assert_typed_control_failure(failure);
    }
}
fn assert_typed_control_failure(failure: LitOnlyFailure) {
    let error = RenderError::from(failure.clone());
    assert_eq!(error.lit_only_failure(), Some(&failure));
    assert_eq!(
        std::error::Error::source(&error)
            .unwrap()
            .downcast_ref::<LitOnlyFailure>(),
        Some(&failure)
    );
}

fn point(mask: f32, source_alpha: f32) -> PrimitiveDraw {
    let mut draw = PrimitiveDraw::points(
        vec![RenderVertex::new([0.; 3], [1., 1., 1., source_alpha])],
        vec![0],
        identity_clip(),
        crate::PointStyle::new(1., [1, 1], vec![[1., 1., 1., mask]]).unwrap(),
    )
    .unwrap();
    draw.point_style.as_mut().unwrap().color_policy =
        crate::point_style::PointColorPolicy::AlphaSprite;
    draw
}
fn accepted(draw: PrimitiveDraw) -> Result<(), RenderError> {
    let frame = RenderFrame::new(2, 2, [0.; 4], vec![draw]).unwrap();
    LitOnlyCommands::PhongTriangles.validate_frame(&frame, RenderStencilState::default())
}
#[test]
fn only_binary_nearest_alpha_sprite_with_opaque_source_is_admitted() {
    for alpha in [0., 1.] {
        accepted(point(alpha, 1.)).unwrap();
    }
    for alpha in [0.25, 0.5, 0.999] {
        assert!(accepted(point(alpha, 1.)).is_err());
    }
    assert!(accepted(point(1., 0.5)).is_err());
    let mut filtered = point(1., 1.);
    filtered.surface_texture_binding_only = true;
    assert!(accepted(filtered).is_err());
    let mut other_policy = point(1., 1.);
    other_policy.point_style.as_mut().unwrap().color_policy =
        crate::point_style::PointColorPolicy::Modulated;
    assert!(accepted(other_policy).is_err());
}
#[test]
fn user_stencil_blending_and_oit_are_rejected_without_affecting_disabled_path() {
    let mut draw = point(1., 1.);
    draw.stencil_rect = true;
    let frame = RenderFrame::new(2, 2, [0.; 4], vec![draw]).unwrap();
    assert!(
        LitOnlyCommands::PhongTriangles
            .validate_frame(&frame, RenderStencilState::default())
            .is_err()
    );
    LitOnlyCommands::Normal
        .validate_frame(&frame, RenderStencilState::default())
        .unwrap();
    let mut blend = point(1., 1.);
    blend.surface_blend = true;
    assert!(accepted(blend).is_err());
}
#[test]
fn oit_control_is_refused() {
    let frame = RenderFrame::new(2, 2, [0.; 4], vec![point(1., 1.)])
        .unwrap()
        .with_transparency(crate::RenderTransparency::Weighted { depth_factor: 0. })
        .unwrap();
    assert!(
        LitOnlyCommands::PhongTriangles
            .validate_frame(&frame, RenderStencilState::default())
            .is_err()
    );
}

fn phong_triangle() -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 3];
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], identity_clip()).unwrap();
    draw.lighting = Some(
        crate::RenderLighting::new(
            vec![[0., 0., 1.]; 3],
            crate::RenderMaterial::default(),
            identity_clip(),
            identity_clip(),
            false,
            crate::RenderShading::Phong,
        )
        .unwrap(),
    );
    draw
}
#[test]
fn opaque_phong_is_retained_and_fractional_material_is_refused() {
    accepted(phong_triangle()).unwrap();
    let mut draw = phong_triangle();
    draw.lighting.as_mut().unwrap().material.common[0].diffuse[3] = 0.5;
    assert!(accepted(draw).is_err());
}
#[test]
fn actual_registered_user_program_is_refused() {
    let library = crate::user_program::RenderUserProgramLibrary::default();
    crate::user_program::register_samples(&library).unwrap();
    let program = crate::user_program::phong_shading_program();
    let modules = library.modules(&program).unwrap();
    let mut draw = phong_triangle();
    draw.user_program = Some(std::sync::Arc::new(
        crate::user_program::UserProgramDraw::new(
            modules,
            crate::user_program::UserUniformScene::default(),
        ),
    ));
    assert!(accepted(draw).is_err());
}
#[test]
fn multisampling_is_refused() {
    let base = RenderFrame::new(2, 2, [0.; 4], vec![phong_triangle()]).unwrap();
    let mut samples = base.clone();
    samples.raster.requested_samples = 2;
    for frame in [samples] {
        assert!(
            LitOnlyCommands::PhongTriangles
                .validate_frame(&frame, RenderStencilState::default())
                .is_err()
        );
    }
}
#[test]
fn active_stencil_control_is_refused() {
    let base = RenderFrame::new(2, 2, [0.; 4], vec![phong_triangle()]).unwrap();
    let stencil = RenderStencilState {
        enabled: true,
        ..Default::default()
    };
    assert!(
        LitOnlyCommands::PhongTriangles
            .validate_frame(&base, stencil)
            .is_err()
    );
}

#[test]
fn sdk_requested_coverage_stays_inactive_in_single_sample_base_pipeline() {
    let params = martensite_tkservice::graphic3d::Graphic3dRenderingParams::new().unwrap();
    let mut frame = RenderFrame::new(2, 2, [0.; 4], vec![point(1., 1.)]).unwrap();
    frame.raster =
        crate::transparency::RenderRasterSettings::from_parameters(&params, false).unwrap();
    assert!(frame.raster.alpha_to_coverage);
    assert_eq!(frame.raster.requested_samples, 0);
    LitOnlyCommands::PhongTriangles
        .validate_frame(&frame, RenderStencilState::default())
        .unwrap();
}

#[test]
fn alternate_surface_and_compatibility_routes_are_refused() {
    let mut back = point(1., 1.);
    back.unlit_back_color = Some([1., 1., 1., 0.25]);
    let mut cutoff = point(1., 1.);
    cutoff.surface_cutoff = Some(0.5);
    let mut compatibility = point(1., 1.);
    compatibility.compatibility_color_target = true;
    let mut fixed = point(1., 1.);
    fixed.fixed_function_lighting = true;
    for draw in [back, cutoff, compatibility, fixed] {
        assert!(accepted(draw).is_err());
    }
}

#[test]
fn frame_manifest_binds_actual_counts_and_device_subpixel_precision() {
    let row = frame_manifest(10, 3, 1, 8);
    assert!(row.contains("\"uploaded_draws\":3"));
    assert!(row.contains("\"expected_retained\":1"));
    assert!(row.contains("\"expected_skipped\":2"));
    assert!(row.contains("\"subpixel_precision_bits\":8"));
    assert!(row.contains("\"qualification\":false"));
}
