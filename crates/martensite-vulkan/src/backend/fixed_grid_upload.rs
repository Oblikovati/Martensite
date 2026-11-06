//! Coverage admission changes only the private upload copy, never semantic input ownership.
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, RenderPointRasterBehavior};
use ash::vk;

pub(super) fn selected_point_draw(
    behavior: RenderPointRasterBehavior,
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if behavior == RenderPointRasterBehavior::FixedGridSprites12V4 {
        return super::fixed_grid_sprite::selected_draw(draw, samples);
    }
    select_plain(behavior, draw, samples)
}
pub(super) fn select_plain(
    behavior: RenderPointRasterBehavior,
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if !plain_candidate(behavior, draw, samples) {
        return Ok(None);
    }
    validate_plain(behavior, draw)?;
    let mut selected = draw.clone();
    selected.fixed_grid_point = true;
    selected.fixed_grid_point_unit12 = behavior == RenderPointRasterBehavior::FixedGridUnit12V3;
    Ok(Some(selected))
}
fn plain_candidate(
    behavior: RenderPointRasterBehavior,
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> bool {
    behavior != RenderPointRasterBehavior::HardwareQuadV1
        && samples == vk::SampleCountFlags::TYPE_1
        && draw.topology() == PrimitiveTopology::Points
        && !draw.polygon_points
        && draw
            .point_style
            .as_ref()
            .is_some_and(|s| s.color_policy == crate::point_style::PointColorPolicy::Plain)
}
fn validate_plain(
    behavior: RenderPointRasterBehavior,
    draw: &PrimitiveDraw,
) -> Result<(), RenderError> {
    if draw.user_program.is_some() {
        return Err(RenderError::rejected(
            "registered point program",
            "established fixed-grid instance vertex semantics",
        ));
    }
    let size = draw.point_style.as_ref().unwrap().size;
    if behavior == RenderPointRasterBehavior::FixedGridUnit12V3 && size != 1.0 {
        return Err(RenderError::rejected(
            size,
            "requested point size exactly 1.0 for FixedGridUnit12V3",
        ));
    }
    crate::fixed_grid_point::fixed_grid_point_extent(size)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plain_point() -> PrimitiveDraw {
        let mut style = crate::PointStyle::new(3.5, [1, 1], vec![[1.; 4]]).unwrap();
        style.color_policy = crate::point_style::PointColorPolicy::Plain;
        PrimitiveDraw::points(
            vec![crate::RenderVertex::new([0.; 3], [1.; 4])],
            vec![0, 0],
            crate::identity_clip(),
            style,
        )
        .unwrap()
    }
    #[test]
    fn explicit_fixed_grid_selection_preserves_inputs_and_requested_size() {
        let source = plain_point();
        let before = source.clone();
        let selected = selected_point_draw(
            RenderPointRasterBehavior::FixedGridAliasedV2,
            &source,
            vk::SampleCountFlags::TYPE_1,
        )
        .unwrap()
        .unwrap();
        assert!(selected.fixed_grid_point);
        assert!(!selected.fixed_grid_point_unit12);
        assert_eq!(selected.indices, vec![0, 0]);
        assert_eq!(selected.point_style.unwrap().size, 3.5);
        assert_eq!(source, before);
    }
    #[test]
    fn unit12_mode_accepts_only_one_pixel_plain_single_sample_points() {
        let mut source = plain_point();
        source.point_style.as_mut().unwrap().size = 1.;
        let before = source.clone();
        let selected = selected_point_draw(
            RenderPointRasterBehavior::FixedGridUnit12V3,
            &source,
            vk::SampleCountFlags::TYPE_1,
        )
        .unwrap()
        .unwrap();
        assert!(selected.fixed_grid_point && selected.fixed_grid_point_unit12);
        assert_eq!(source, before);

        source.point_style.as_mut().unwrap().size = 1.25;
        let error = selected_point_draw(
            RenderPointRasterBehavior::FixedGridUnit12V3,
            &source,
            vk::SampleCountFlags::TYPE_1,
        )
        .unwrap_err();
        assert!(error.to_string().contains("1.25"));
    }
    #[test]
    fn registered_point_program_refuses_before_instance_publication() {
        use crate::user_program::*;
        let library = RenderUserProgramLibrary::default();
        register_samples(&library).unwrap();
        let modules = library.modules(&phong_shading_program()).unwrap();
        let mut source = plain_point();
        source.user_program = Some(std::sync::Arc::new(UserProgramDraw::new(
            modules,
            Default::default(),
        )));
        let before = source.clone();
        let result = selected_point_draw(
            RenderPointRasterBehavior::FixedGridAliasedV2,
            &source,
            vk::SampleCountFlags::TYPE_1,
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("instance vertex semantics")
        );
        assert_eq!(source, before);
    }
    #[test]
    fn hardware_sprite_msaa_and_polygon_point_paths_are_unchanged() {
        let source = plain_point();
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::HardwareQuadV1,
                &source,
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridAliasedV2,
                &source,
                vk::SampleCountFlags::TYPE_4
            )
            .unwrap()
            .is_none()
        );
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridUnit12V3,
                &source,
                vk::SampleCountFlags::TYPE_4
            )
            .unwrap()
            .is_none()
        );
        let mut sprite = source.clone();
        sprite.point_style.as_mut().unwrap().color_policy =
            crate::point_style::PointColorPolicy::AlphaSprite;
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridAliasedV2,
                &sprite,
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridUnit12V3,
                &sprite,
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
        let mut polygon = source;
        polygon.polygon_points = true;
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridAliasedV2,
                &polygon,
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
        assert!(
            selected_point_draw(
                RenderPointRasterBehavior::FixedGridUnit12V3,
                &polygon,
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
    }
}
