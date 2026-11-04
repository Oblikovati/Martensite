#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raster_v3_clipped_unlit_line_uses_special_pipeline_selection() {
        let mut draw = PrimitiveDraw::segments(
            vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 2],
            vec![0, 1],
            crate::identity_clip(),
            crate::LineStyle::new(2., 0xffff, 1).unwrap(),
        )
        .unwrap();
        draw.clipping = Some(
            crate::RenderClipping::new(vec![vec![[0., 1., 0., -0.5]]], crate::identity_clip())
                .unwrap(),
        );
        assert!(draw.lighting.is_none() && !draw.surface_blend);
        assert_eq!(draw.topology(), crate::PrimitiveTopology::Lines);
        assert!(requires_special_pipeline(&draw, true));
        assert!(!requires_special_pipeline(&draw, false));
    }
    #[test]
    fn world_clip_triangle_and_line_variants_have_distinct_cache_entries() {
        let draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        assert_ne!(
            world_clip_cache_key(&draw, true, false, false),
            world_clip_cache_key(&draw, false, true, false)
        );
        assert_ne!(
            world_clip_cache_key(&draw, false, false, false),
            world_clip_cache_key(&draw, false, true, false)
        );
        assert_ne!(
            world_clip_cache_key(&draw, false, false, false),
            world_clip_cache_key(&draw, false, false, true)
        );
    }
    #[test]
    fn grid12_fragment_clipping_selects_the_descriptor_bearing_clipped_variant() {
        let mut kind = PipelineVariant::from_index(1, false);
        apply_grid12_world_clip(&mut kind, true);
        assert!(kind.world_clip_grid12);
        assert!(kind.clipped);
        let mut legacy = PipelineVariant::from_index(1, false);
        apply_grid12_world_clip(&mut legacy, false);
        assert!(!legacy.world_clip_grid12);
        assert!(!legacy.clipped);
    }
    #[test]
    fn clipped_phong_triangle_already_uses_a_special_descriptor_pipeline() {
        let mut draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
            [-0.75, -0.75, 0.0],
            [0.75, -0.75, 0.0],
            [-0.75, 0.75, 0.0],
        ]);
        draw.clipping = Some(
            crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
                .unwrap(),
        );
        assert!(requires_special_pipeline(&draw, false));
        let mut kind = special_variant(&draw, false);
        apply_grid12_world_clip(&mut kind, true);
        assert!(kind.clipped);
    }
    #[test]
    fn mixed_clipped_triangle_and_line_select_only_triangle_grid12_fragment_path() {
        let clipping =
            crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
                .unwrap();
        let mut triangle = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
            [-0.75, -0.75, 0.0],
            [0.75, -0.75, 0.0],
            [-0.75, 0.75, 0.0],
        ]);
        triangle.clipping = Some(clipping.clone());
        let mut line = PrimitiveDraw::segments(
            vec![
                crate::RenderVertex::new([-0.5, 0.0, 0.0], [1.0; 4]),
                crate::RenderVertex::new([0.5, 0.0, 0.0], [1.0; 4]),
            ],
            vec![0, 1],
            crate::identity_clip(),
            crate::LineStyle::new(1.0, 0xffff, 1).unwrap(),
        )
        .unwrap();
        line.clipping = Some(clipping);
        let behaviors = [
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V1,
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
        ];
        for behavior in behaviors {
            assert!(
                super::super::world_clip_interpolation_upload::selects_grid12_fragment(
                    &triangle, behavior,
                )
            );
            assert!(
                !super::super::world_clip_interpolation_upload::selects_grid12_fragment(
                    &line, behavior,
                )
            );
        }
        let triangle_selected =
            super::super::world_clip_interpolation_upload::selects_grid12_fragment(
                &triangle,
                crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            );
        let line_selected = super::super::world_clip_interpolation_upload::selects_grid12_fragment(
            &line,
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
        );
        let mut triangle_kind = special_variant(&triangle, false);
        let mut line_kind = special_variant(&line, false);
        apply_grid12_world_clip(&mut triangle_kind, triangle_selected);
        apply_grid12_world_clip(&mut line_kind, line_selected);

        assert!(triangle_selected);
        assert!(triangle_kind.world_clip_grid12 && triangle_kind.clipped);
        assert!(!line_selected);
        assert!(!line_kind.world_clip_grid12 && line_kind.clipped);
        let legacy_line = [
            include_bytes!("../../shaders/lighting.line.clipped.vert.spv").as_slice(),
            include_bytes!("../../shaders/lighting.line.clipped.frag.spv").as_slice(),
        ];
        assert_eq!(
            super::super::pipeline_shader_sources::lighting_shaders(true, false, true),
            legacy_line
        );
    }
    #[test]
    fn unlit_side_color_uses_surface_family_without_aliasing_legacy_cache_key() {
        let mut draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        let legacy = special_variant_key(&draw);
        draw.unlit_back_color = Some([0., 1., 0., 0.25]);
        let kind = special_variant(&draw, false);
        assert!(kind.surface_alpha && !kind.surface_texture && !kind.lighting);
        assert_eq!(special_variant_key(&draw) - legacy, 1048576);
        draw.surface_cutoff = Some(0.5);
        assert_eq!(special_variant_key(&draw) - legacy, 1048832);
    }
    #[test]
    fn encoded_surface_alias_is_distinct_without_requiring_glyph_logic_op() {
        let mut draw = encoded_incomplete_draw();
        let kind = special_variant(&draw, true);
        assert!(kind.encoded_surface && !kind.text_logic_xor);
        assert!(crate::text_logic::require_text_xor(kind.text_logic_xor, false).is_ok());
        let encoded = special_variant_key(&draw);
        draw.surface_texture_unavailable
            .as_mut()
            .unwrap()
            .fixed_environment = false;
        assert_ne!(encoded, special_variant_key(&draw));
        assert!(!encoded_surface_attachment(&draw));
        draw.surface_texture_unavailable
            .as_mut()
            .unwrap()
            .fixed_environment = true;
        draw.surface_texture_binding_only = true;
        assert!(!encoded_surface_attachment(&draw));
    }
    fn encoded_incomplete_draw() -> PrimitiveDraw {
        let mut draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        draw.surface_texture_unavailable =
            Some(crate::surface_texture::SurfaceTextureUnavailable {
                resource_id: b"Martensite-encoded-incomplete-pipeline".to_vec(),
                fixed_environment: true,
                modulate: true,
                vertex_override: false,
                back_vertex: false,
            });
        draw
    }
    #[test]
    fn plain_point_variants_preserve_clipping_clamp_and_depth_policy() {
        for index in 36..40 {
            let kind = PipelineVariant::from_index(index, true);
            assert!(kind.point && !kind.point_blend);
            assert!(!kind.line && !kind.line_blend && !kind.surface_blend);
            assert_eq!(kind.clamp, index & 1 != 0);
            assert_eq!(kind.clipped, index & 2 != 0);
            let draw = plain_variant_draw(kind);
            assert_eq!(variant_index(&draw), index);
        }
    }
    fn plain_variant_draw(kind: PipelineVariant) -> PrimitiveDraw {
        let style = crate::PointStyle::new(1., [1, 1], vec![[1.; 4]])
            .unwrap()
            .with_alpha_blending(false);
        let mut draw =
            PrimitiveDraw::points(vec![], vec![], crate::identity_clip(), style).unwrap();
        draw.depth_clamp = kind.clamp;
        if kind.clipped {
            draw.clipping =
                Some(crate::RenderClipping::new(vec![], crate::identity_clip()).unwrap());
        }
        draw
    }
    #[test]
    fn stencil_variants_round_trip_clamp_and_clipping_flags() {
        for index in 0..28 {
            let kind = PipelineVariant::from_index(index, true);
            assert_eq!(
                kind.depth_format(Default::default()),
                vk::Format::D32_SFLOAT_S8_UINT
            );
            if index < 20 {
                assert_eq!(kind.capping, RenderCappingPass::None);
                continue;
            }
            let mut draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
            draw.capping_pass = kind.capping;
            draw.depth_clamp = kind.clamp;
            draw.clipping = kind.clipped.then(|| {
                crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip())
                    .unwrap()
            });
            assert_eq!(variant_index(&draw), index);
            assert!(!kind.line && !kind.point && !kind.text);
        }
        assert_eq!(
            PipelineVariant::from_index(0, false).depth_format(Default::default()),
            vk::Format::D32_SFLOAT
        );
    }
}

#[cfg(test)]
mod blend_tests {
    use super::*;
    #[test]
    fn blended_variants_keep_clipping_and_clamp_independent() {
        for index in 28..32 {
            let kind = PipelineVariant::from_index(index, false);
            assert!(kind.surface_blend);
            assert_eq!(kind.clamp, index & 1 != 0);
            assert_eq!(kind.clipped, index & 2 != 0);
            assert_eq!(kind.capping, RenderCappingPass::None);
            assert!(!kind.line && !kind.point && !kind.text && !kind.lighting);
        }
    }
}

#[cfg(test)]
mod forced_line_tests {
    use super::*;
    #[test]
    fn forced_line_variants_keep_topology_clamp_clipping_and_order_policy() {
        for index in 32..36 {
            let kind = PipelineVariant::from_index(index, true);
            assert!(kind.line && kind.line_blend);
            assert!(!kind.surface_blend && !kind.point && !kind.text);
            assert_eq!(kind.clamp, index & 1 != 0);
            assert_eq!(kind.clipped, index & 2 != 0);
            assert_eq!(kind.capping, RenderCappingPass::None);
            let draw = forced_draw(kind);
            assert_eq!(variant_index(&draw), index);
        }
    }
    fn forced_draw(kind: PipelineVariant) -> PrimitiveDraw {
        let style = crate::LineStyle::new(1., 0xffff, 1)
            .unwrap()
            .with_alpha_blending(true);
        let mut draw =
            PrimitiveDraw::segments(vec![], vec![], crate::identity_clip(), style).unwrap();
        draw.depth_clamp = kind.clamp;
        draw.clipping = kind.clipped.then(|| {
            crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip())
                .unwrap()
        });
        draw
    }
}

#[cfg(test)]
mod subtitle_pipeline_tests {
    use super::*;
    #[test]
    fn ordered_subtitle_alpha_uses_existing_blended_untextured_raster_variant() {
        let mut draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        draw.text_background_blend = true;
        assert!(!draw.surface_blend);
        assert_eq!(variant_index(&draw), 28);
        let kind = PipelineVariant::from_index(variant_index(&draw), false);
        assert!(kind.surface_blend && !kind.text && !kind.line);
        draw.text_background_blend = false;
        assert_eq!(variant_index(&draw), 0);
    }
}

#[cfg(test)]
mod xor_tests {
    use super::*;
    #[test]
    fn xor_cache_indices_keep_all_existing_variant_flags_and_texture_reservations() {
        for index in 40..44 {
            let kind = PipelineVariant::from_index(index, true);
            assert!(kind.text && kind.text_logic_xor && kind.stencil);
            assert!(!kind.text_blend && !kind.surface_blend && !kind.point);
            let mut draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
            draw.text_logic_xor = true;
            draw.depth_clamp = kind.clamp;
            draw.clipping = kind
                .clipped
                .then(|| crate::RenderClipping::new(vec![], crate::identity_clip()).unwrap());
            assert_eq!(variant_index(&draw), index);
            assert!(variant_index(&draw) < 64);
        }
    }
}

#[cfg(test)]
mod projection_key_tests {
    use super::*;
    #[test]
    fn identical_physical_triangle_geometry_keeps_distinct_semantic_projection_keys() {
        let draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        let ordinary = projection_variant_key(&draw, false, false, false, false, None);
        let factorized = projection_variant_key(&draw, false, false, false, true, None);
        assert_ne!(ordinary, factorized);
        assert_eq!(ordinary.0, factorized.0);
        assert_ne!(
            projection_variant_key(&draw, true, false, false, false, None),
            ordinary
        );
    }
}

#[cfg(test)]
mod combined_cache_tests {
    use super::*;
    #[test]
    fn every_world_mode_projection_and_window_word_has_a_distinct_cache_key() {
        let draw = PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        let mut windows = vec![None, Some([0; 10])];
        for index in 0..10 {
            let mut words = [0; 10];
            words[index] = 1;
            windows.push(Some(words));
        }
        let mut keys = std::collections::BTreeSet::new();
        for mode in 0..8 {
            for factorized in [false, true] {
                for window in &windows {
                    assert!(keys.insert(projection_variant_key(
                        &draw,
                        mode & 1 != 0,
                        mode & 2 != 0,
                        mode & 4 != 0,
                        factorized,
                        *window
                    )));
                }
            }
        }
        assert_eq!(keys.len(), 192);
    }
}
