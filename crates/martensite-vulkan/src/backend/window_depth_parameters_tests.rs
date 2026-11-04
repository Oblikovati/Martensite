use super::*;
fn selected() -> RenderWindowDepthBehavior {
    RenderWindowDepthBehavior::UnsnappedWindowDepthV2 {
        reference_grid: RenderViewportGridPrecision::Fractional12,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    }
}
fn viewport() -> vk::Viewport {
    vk::Viewport {
        x: 0.,
        y: 409.,
        width: 409.,
        height: -409.,
        min_depth: 0.,
        max_depth: 1.,
    }
}

#[test]
fn default_does_not_validate_or_change_old_viewport_inputs() {
    let invalid = vk::Viewport {
        width: f32::NAN,
        ..viewport()
    };
    assert_eq!(
        WindowDepthPipelineParameters::selected(Default::default(), invalid, false),
        Ok(None)
    );
}

#[test]
fn exact_viewport_grid_and_clip_mapping_are_in_the_cache_and_specialization() {
    let parameters = WindowDepthPipelineParameters::selected(selected(), viewport(), false)
        .unwrap()
        .unwrap();
    assert_eq!(parameters.viewport().height, -409.);
    assert_eq!(
        parameters.grids(),
        [
            RenderViewportGridPrecision::Fractional12,
            RenderViewportGridPrecision::Fractional8
        ]
    );
    assert_eq!(&parameters.specialization_words()[4..], &[12, 8, 0]);
    assert_eq!(
        parameters.cache_words()[4..6],
        [0_f32.to_bits(), 1_f32.to_bits()]
    );
    assert!(!parameters.negative_one_to_one());
    let gl = WindowDepthPipelineParameters::selected(selected(), viewport(), true)
        .unwrap()
        .unwrap();
    assert_ne!(parameters.cache_words(), gl.cache_words());
    assert_eq!(gl.specialization_words()[6], 1);
}

#[test]
fn changed_range_nonfinite_or_unbound_actual_viewport_refuses() {
    let parameters = WindowDepthPipelineParameters::selected(selected(), viewport(), false)
        .unwrap()
        .unwrap();
    assert!(parameters.require_actual_viewport(viewport()).is_ok());
    let other = vk::Viewport {
        width: 410.,
        ..viewport()
    };
    assert!(parameters.require_actual_viewport(other).is_err());
    for invalid in [
        vk::Viewport {
            width: 0.,
            ..viewport()
        },
        vk::Viewport {
            max_depth: 0.5,
            ..viewport()
        },
    ] {
        assert!(WindowDepthPipelineParameters::selected(selected(), invalid, false).is_err());
    }
}

#[test]
fn direct_fragment_and_geometry_replay_have_distinct_pipeline_cache_identity() {
    let old = WindowDepthPipelineParameters::selected(selected(), viewport(), false)
        .unwrap()
        .unwrap();
    let v3 = RenderWindowDepthBehavior::ReferenceWindowDepthV3 {
        reference_grid: RenderViewportGridPrecision::Fractional12,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    };
    let new = WindowDepthPipelineParameters::selected(v3, viewport(), false)
        .unwrap()
        .unwrap();
    assert!(!old.fragment_depth());
    assert!(new.fragment_depth());
    assert_ne!(old.cache_words(), new.cache_words());
    assert_eq!(old.specialization_words(), new.specialization_words());
}

#[test]
fn v5_is_disjoint_from_v3_even_with_identical_viewport_and_grids() {
    let viewport = vk::Viewport {
        width: 409.,
        height: 409.,
        max_depth: 1.,
        ..Default::default()
    };
    let grids = (
        crate::RenderViewportGridPrecision::Fractional12,
        crate::RenderViewportGridPrecision::Fractional8,
    );
    let v3 = WindowDepthPipelineParameters::selected(
        RenderWindowDepthBehavior::ReferenceWindowDepthV3 {
            reference_grid: grids.0,
            raster_grid: grids.1,
        },
        viewport,
        false,
    )
    .unwrap()
    .unwrap();
    let v5 = WindowDepthPipelineParameters::selected(
        RenderWindowDepthBehavior::Float64ReferenceWindowDepthV5 {
            reference_grid: grids.0,
            raster_grid: grids.1,
        },
        viewport,
        false,
    )
    .unwrap()
    .unwrap();
    assert_ne!(v3.cache_words(), v5.cache_words());
    assert_eq!([v3.cache_words()[9], v5.cache_words()[9]], [1, 5]);
    assert!(v5.float64());
    assert!(v5.fragment_depth());
}

#[test]
fn clipped_fan_parameters_are_separate_from_both_earlier_reference_operations() {
    let behavior = RenderWindowDepthBehavior::ClippedFanFramebufferV4 {
        fan_grid: RenderViewportGridPrecision::Fractional8,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    };
    let v4 = WindowDepthPipelineParameters::selected(behavior, viewport(), false)
        .unwrap()
        .unwrap();
    let v2 = WindowDepthPipelineParameters::selected(selected(), viewport(), false)
        .unwrap()
        .unwrap();
    assert!(v4.fragment_depth() && v4.clipped_fan());
    assert_eq!(v4.cache_words()[9], 2);
    assert_ne!(v4.cache_words(), v2.cache_words());
    assert_eq!(&v4.specialization_words()[4..], &[8, 8, 0]);
}
#[test]
fn shared_incidence_version_has_its_own_cache_tag_and_requires_explicit_q12_over_q8() {
    let mut view = viewport();
    view.y = 0.;
    view.height = 409.;
    let v6 = RenderWindowDepthBehavior::ClippedIncidenceFramebufferV6 {
        fan_grid: RenderViewportGridPrecision::Fractional12,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    };
    let actual = WindowDepthPipelineParameters::selected(v6, view, false)
        .unwrap()
        .unwrap();
    assert!(actual.shared_incidence() && actual.clipped_fan() && actual.fragment_depth());
    assert_eq!(actual.cache_words()[9], 3);
    let old = RenderWindowDepthBehavior::ClippedFanFramebufferV4 {
        fan_grid: RenderViewportGridPrecision::Fractional12,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    };
    assert_eq!(
        WindowDepthPipelineParameters::selected(old, view, false)
            .unwrap()
            .unwrap()
            .cache_words()[9],
        2
    );
    let unsupported = RenderWindowDepthBehavior::ClippedIncidenceFramebufferV6 {
        fan_grid: RenderViewportGridPrecision::Fractional8,
        raster_grid: RenderViewportGridPrecision::Fractional8,
    };
    assert!(WindowDepthPipelineParameters::selected(unsupported, view, false).is_err());
    assert_eq!(v6.behavior_version(), 6);
}
