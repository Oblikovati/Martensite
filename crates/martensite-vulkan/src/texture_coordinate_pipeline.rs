//! Inject coordinate behavior independently from shader lighting or the Vulkan device.

/// Select the legacy texture-coordinate path; `RenderTextureCoordinatePipeline::default()`
/// preserves the ordinary programmable path, including its supplied homogeneous UVs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderTextureCoordinatePipeline {
    /// Built-in programmable shaders consume the UV attribute independently of GenMode.
    #[default]
    Programmable,
    /// Emulate fixed-function texture-coordinate generation and its retained binding state.
    /// This selects coordinate behavior only; it does not select fixed-function lighting.
    GeneratedCompatibility,
}

impl RenderTextureCoordinatePipeline {
    pub(crate) fn effective(self, fixed_function: bool, user_program: bool) -> Self {
        // SetTextureMatrix uses the homogeneous matrix only without an active GLSL program.
        if fixed_function && !user_program {
            return Self::GeneratedCompatibility;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn effective_coordinates_follow_fixed_capability_without_overriding_user_programs() {
        use RenderTextureCoordinatePipeline::{GeneratedCompatibility, Programmable};
        for (fixed, program, expected) in [
            (false, false, Programmable),
            (false, true, Programmable),
            (true, true, Programmable),
            (true, false, GeneratedCompatibility),
        ] {
            assert_eq!(Programmable.effective(fixed, program), expected);
            assert_eq!(
                GeneratedCompatibility.effective(fixed, program),
                GeneratedCompatibility
            );
        }
    }
    #[test]
    fn default_coordinate_pipeline_preserves_programmable_behavior() {
        assert_eq!(
            RenderTextureCoordinatePipeline::default(),
            RenderTextureCoordinatePipeline::Programmable
        );
        assert_ne!(
            RenderTextureCoordinatePipeline::GeneratedCompatibility,
            RenderTextureCoordinatePipeline::Programmable
        );
    }
}
