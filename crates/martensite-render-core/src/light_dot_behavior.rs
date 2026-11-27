//! Versioned NaN contribution policy is independent of the requested surface normal model.
/// Choose programmable Phong light-dot handling; `RenderLightDotBehavior::NumericMaxZeroV2`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderLightDotBehavior {
    /// Retain driver-dependent floating maximum semantics, including NaN propagation.
    #[default]
    DriverNativeV1,
    /// A NaN dot product contributes zero diffuse/specular energy; finite terms are unchanged.
    /// Only built-in programmable Phong stages apply this policy; custom/PBR/fixed remain untouched.
    NumericMaxZeroV2,
}
impl RenderLightDotBehavior {
    /// Retain the explicit behavior in replay records; `behavior.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::DriverNativeV1 => 1,
            Self::NumericMaxZeroV2 => 2,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_is_default_and_numeric_policy_has_a_distinct_version() {
        let _: &(dyn Send + Sync) = &RenderLightDotBehavior::default();
        assert_eq!(
            RenderLightDotBehavior::default(),
            RenderLightDotBehavior::DriverNativeV1
        );
        assert_eq!(RenderLightDotBehavior::default().behavior_version(), 1);
        assert_eq!(
            RenderLightDotBehavior::NumericMaxZeroV2.behavior_version(),
            2
        );
    }
}
