//! Keep stock fragment keys for version registries while loading matching diagnostic interfaces.
use super::*;
impl DrawPipeline {
    pub(super) fn capture_shader_source(
        &mut self,
        stock: &'static [u8],
    ) -> Result<&'static [u8], RenderError> {
        let stock = self.interpolation_shader_source(stock)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        {
            self.capture_enabled_source(stock)
        }
        #[cfg(not(feature = "vertex-output-diagnostics"))]
        {
            Ok(stock)
        }
    }
    fn interpolation_shader_source(
        &mut self,
        stock: &'static [u8],
    ) -> Result<&'static [u8], RenderError> {
        if self.runtime.phong_interpolation == crate::RenderPhongInterpolationBehavior::HardwareV1 {
            return Ok(stock);
        }
        if self.shaders.is_empty() {
            if !super::super::phong_interpolation_shader::triangle_kind(self.kind) {
                return Ok(stock);
            }
            if super::super::fixed_grid_record::logical_facing_active(
                self.runtime.triangle_coverage,
                self.kind.triangle_grid_coverage,
            ) {
                let selected = if self.kind.clipped {
                    super::super::v4_geometry_transport::clipped_vertex(stock)
                } else {
                    super::super::v4_geometry_transport::vertex(stock)
                }
                .ok_or_else(|| {
                    crate::RenderTriangleCoverageFailure::UnsupportedInterface {
                        actual: format!("V4 registered vertex module of {} bytes", stock.len()),
                    }
                })?;
                self.phong_interpolation_shader = true;
                self.triangle_grid_coverage_shader = self.kind.triangle_grid_coverage;
                self.world_clip_grid12_shader = self.kind.world_clip_grid12;
                return Ok(selected);
            }
            let selected = super::super::phong_interpolation_shader::vertex(stock);
            self.phong_interpolation_shader = selected.is_some();
            self.triangle_grid_coverage_shader =
                self.kind.triangle_grid_coverage && selected.is_some();
            self.world_clip_grid12_shader = self.kind.world_clip_grid12 && selected.is_some();
            return Ok(selected.unwrap_or(stock));
        }
        if !self.phong_interpolation_shader {
            return Ok(stock);
        }
        if self.world_clip_grid12_shader {
            return self.world_clip_grid12_fragment(stock);
        }
        if self.triangle_grid_coverage_shader {
            return self.triangle_grid_fragment(stock);
        }
        super::super::phong_interpolation_shader::fragment_for(
            self.runtime.phong_interpolation,
            stock,
        )
    }
    fn triangle_grid_fragment(&self, stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
        if super::super::fixed_grid_record::logical_facing_active(
            self.runtime.triangle_coverage,
            self.triangle_grid_coverage_shader,
        ) {
            return triangle_grid_v4_fragment_for(self.runtime.phong_interpolation, stock);
        }
        triangle_grid_fragment_for(self.runtime.phong_interpolation, stock)
    }
    fn world_clip_grid12_fragment(
        &self,
        stock: &'static [u8],
    ) -> Result<&'static [u8], RenderError> {
        world_clip_fragment_for(
            self.runtime.world_clip_interpolation,
            self.runtime.world_clipping,
            self.runtime.triangle_coverage,
            self.runtime.phong_interpolation,
            self.triangle_grid_coverage_shader
                && super::super::fixed_grid_record::logical_facing_active(
                    self.runtime.triangle_coverage,
                    self.triangle_grid_coverage_shader,
                ),
            stock,
        )
    }
    #[cfg(feature = "vertex-output-diagnostics")]
    fn capture_enabled_source(
        &mut self,
        stock: &'static [u8],
    ) -> Result<&'static [u8], RenderError> {
        if self.shaders.is_empty() {
            return Ok(self.capture_vertex_source(stock));
        }
        if !self.vertex_output_shader {
            return Ok(stock);
        }
        capture_fragment_source(stock, self.runtime.vertex_output.as_ref().unwrap().format())
    }
    #[cfg(feature = "vertex-output-diagnostics")]
    fn capture_vertex_source(&mut self, stock: &'static [u8]) -> &'static [u8] {
        let observed = self.runtime.vertex_output.as_ref().and_then(|request| {
            super::super::vertex_output::selected_for_format(stock, request.format())
        });
        self.vertex_output_shader = self.runtime.vertex_output.is_some() && observed.is_some();
        if self.vertex_output_shader {
            return observed.unwrap_or(stock);
        }
        stock
    }
}

fn triangle_grid_v4_fragment_for(
    phong: crate::RenderPhongInterpolationBehavior,
    stock: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    match phong {
        crate::RenderPhongInterpolationBehavior::UnsnappedV3 => {
            super::super::triangle_grid_coverage_shader::fragment_v4_v3(stock)
        }
        crate::RenderPhongInterpolationBehavior::UnsnappedV2 => {
            let v2 = super::super::phong_interpolation_shader::fragment(stock)?;
            super::super::triangle_grid_coverage_shader::fragment_v4(v2)
        }
        actual => Err(crate::RenderTriangleCoverageFailure::UnsupportedInterface {
            actual: format!("Phong interpolation {actual:?}"),
        }
        .into()),
    }
}

fn world_clip_fragment_for(
    world_clip: crate::RenderWorldClipInterpolationBehavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    phong: crate::RenderPhongInterpolationBehavior,
    native_grid_v4: bool,
    stock: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    match (world_clip, phong, native_grid_v4) {
        (
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V1,
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            _,
        ) => {
            let v2 = super::super::phong_interpolation_shader::fragment(stock)?;
            super::super::world_clip_grid12_shader::fragment(v2)
        }
        (
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            true,
        ) => super::super::world_clip_grid12_shader::fragment_v4_compact(stock),
        (
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            false,
        ) => super::super::world_clip_grid12_shader::fragment_v3(stock),
        _ => Err(
            crate::RenderWorldClipInterpolationFailure::UnsupportedComposition {
                world_clipping,
                triangle_coverage,
                phong_interpolation: phong,
            }
            .into(),
        ),
    }
}

fn triangle_grid_fragment_for(
    behavior: crate::RenderPhongInterpolationBehavior,
    stock: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    match behavior {
        crate::RenderPhongInterpolationBehavior::UnsnappedV2 => {
            let v2 = super::super::phong_interpolation_shader::fragment(stock)?;
            super::super::triangle_grid_coverage_shader::fragment(v2)
        }
        crate::RenderPhongInterpolationBehavior::UnsnappedV3 => {
            super::super::triangle_grid_coverage_shader::fragment_v3(stock)
        }
        actual => Err(crate::RenderTriangleCoverageFailure::UnsupportedInterface {
            actual: format!("Phong interpolation {actual:?}"),
        }
        .into()),
    }
}

#[cfg(test)]
mod triangle_grid_routing_tests {
    use super::*;

    #[test]
    fn unclipped_v3_uses_only_its_registered_grid_module() {
        let stock = include_bytes!("../../shaders/lighting.frag.spv");
        let selected =
            triangle_grid_fragment_for(crate::RenderPhongInterpolationBehavior::UnsnappedV3, stock)
                .unwrap();
        assert_eq!(
            selected,
            include_bytes!("../../shaders/triangle-grid-v3.unsnapped-v3.lighting.frag.spv")
        );
        assert_ne!(selected, stock);
    }

    #[test]
    fn unclipped_v1_does_not_fall_back_to_a_grid_module() {
        let stock = include_bytes!("../../shaders/lighting.frag.spv");
        let error =
            triangle_grid_fragment_for(crate::RenderPhongInterpolationBehavior::HardwareV1, stock)
                .unwrap_err();
        assert!(error.triangle_coverage_failure().is_some());
    }

    #[test]
    fn v4_without_active_clip_keeps_v2_route_and_has_an_exact_v3_route() {
        let stock = include_bytes!("../../shaders/lighting.frag.spv");
        let v2 = triangle_grid_v4_fragment_for(
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            stock,
        )
        .unwrap();
        assert_eq!(
            v2,
            include_bytes!("../../shaders/triangle-grid-v4.unsnapped-v2.lighting.frag.spv")
        );
        let v3 = triangle_grid_v4_fragment_for(
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            stock,
        )
        .unwrap();
        assert_eq!(
            v3,
            include_bytes!("../../shaders/triangle-grid-v4.unsnapped-v3.lighting.frag.spv")
        );
        assert_ne!(v2, v3);
    }

    #[test]
    fn empty_clip_state_retains_registered_clipped_v3_grid_module() {
        let stock = include_bytes!("../../shaders/lighting.clipped.frag.spv");
        let selected =
            triangle_grid_fragment_for(crate::RenderPhongInterpolationBehavior::UnsnappedV3, stock)
                .unwrap();
        assert_eq!(
            selected,
            include_bytes!("../../shaders/triangle-grid-v3.unsnapped-v3.lighting.clipped.frag.spv")
        );
        assert_ne!(
            selected,
            include_bytes!("../../shaders/triangle-grid-v3.unsnapped-v3.lighting.frag.spv")
        );
    }

    #[test]
    fn world_clip_v4_route_selects_combined_v4_clip_fragment() {
        let stock = include_bytes!("../../shaders/lighting.clipped.frag.spv");
        let selected = world_clip_fragment_for(
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            crate::RenderWorldClippingBehavior::FragmentV1,
            crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
                origin: crate::RenderTriangleGridOrigin::TopDown,
            },
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            true,
            stock,
        )
        .unwrap();
        assert_eq!(
            selected,
            include_bytes!(
                "../../shaders/world-clip-grid12-v2.triangle-grid-v4-compact.unsnapped-v3.lighting.clipped.frag.spv"
            )
        );
        assert_ne!(selected, stock);

        let legacy_v3 = world_clip_fragment_for(
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            crate::RenderWorldClippingBehavior::FragmentV1,
            crate::RenderTriangleCoverageBehavior::FixedGrid12V3 {
                origin: crate::RenderTriangleGridOrigin::TopDown,
            },
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            false,
            stock,
        )
        .unwrap();
        assert_eq!(
            legacy_v3,
            include_bytes!(
                "../../shaders/world-clip-grid12-v2.unsnapped-v3.lighting.clipped.frag.spv"
            )
        );
        assert_ne!(selected, legacy_v3);
    }

    #[test]
    fn world_clip_v4_route_does_not_fall_back_to_v3_or_legacy_modules() {
        let stock = include_bytes!("../../shaders/lighting.clipped.frag.spv");
        for (coverage, phong) in [
            (
                crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
                    origin: crate::RenderTriangleGridOrigin::TopDown,
                },
                crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            ),
            (
                crate::RenderTriangleCoverageBehavior::FixedGrid12V3 {
                    origin: crate::RenderTriangleGridOrigin::TopDown,
                },
                crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            ),
        ] {
            assert!(
                world_clip_fragment_for(
                    crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
                    crate::RenderWorldClippingBehavior::FragmentV1,
                    coverage,
                    phong,
                    true,
                    stock,
                )
                .is_err()
            );
        }
    }
}
#[cfg(feature = "vertex-output-diagnostics")]
fn capture_fragment_source(
    stock: &'static [u8],
    format: crate::VertexOutputFormat,
) -> Result<&'static [u8], RenderError> {
    super::super::vertex_output::fragment_for_format(stock, format).ok_or_else(|| {
        super::super::vertex_output::unsupported(
            "matching diagnostic fragment interface",
            (format, stock.len()),
        )
    })
}
#[cfg(all(test, feature = "vertex-output-diagnostics"))]
mod tests {
    use super::*;

    #[test]
    fn text_capture_pipeline_loads_the_matching_diagnostic_fragment() {
        let stock = include_bytes!("../../shaders/text.frag.spv");
        let selected =
            capture_fragment_source(stock, crate::VertexOutputFormat::TextClipUvId28).unwrap();
        assert_eq!(
            selected,
            include_bytes!("../../shaders/diagnostic.vertex-output.text.frag.spv")
        );
        assert_ne!(selected, stock);
    }
}
