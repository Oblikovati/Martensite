//! Explicit depth-bias units preserve replay independently of attachment storage.
/// Select the bias representation; `RenderDepthBiasBehavior::Native24UnitsV2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderDepthBiasBehavior {
    /// Existing format-dependent hardware units, including floating-depth exponents.
    #[default]
    HardwareFormatV1,
    /// Named Native24 equivalence profile: one unit is exactly 2^-23 window-depth units.
    /// This preserves the reviewed Mesa 26.1.6 D24 contract, not every OpenGL driver.
    Native24UnitsV2,
}
impl RenderDepthBiasBehavior {
    /// Record the selected behavior; `RenderDepthBiasBehavior::Native24UnitsV2.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareFormatV1 => 1,
            Self::Native24UnitsV2 => 2,
        }
    }
}
/// Typed refusal of an explicitly selected fixed-unit operation.
/// Example: `RenderDepthBiasFailure::MissingCommand` never selects V1 instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDepthBiasFailure {
    /// V4 shader depth cannot infer HardwareFormatV1 units from the selected depth format.
    UnresolvedHardwareUnits {
        /// The active units value as IEEE Float32 bits.
        units_bits: u32,
    },
    /// The V4 viewport grid cannot bound this finite slope factor inside f32 arithmetic.
    UnboundedSlopeFactor {
        /// The active factor as IEEE Float32 bits.
        factor_bits: u32,
    },
    /// Exact FLOAT representation requires all three features and the extension.
    MissingExactFloatControl {
        /// Whether the extension requirement holds.
        extension: bool,
        /// Whether the control requirement holds.
        control: bool,
        /// Whether the float requirement holds.
        float: bool,
        /// Whether the exact requirement holds.
        exact: bool,
    },
    /// The enabled device did not expose vkCmdSetDepthBias2EXT.
    MissingCommand,
}
impl std::fmt::Display for RenderDepthBiasFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnresolvedHardwareUnits { units_bits } => write!(
                f,
                "received V4 polygon-offset unit bits {units_bits:#010x}; expected zero units or Native24UnitsV2"
            ),
            Self::UnboundedSlopeFactor { factor_bits } => write!(
                f,
                "received V4 polygon-offset factor bits {factor_bits:#010x}; expected absolute factor at most 1.0e25"
            ),
            _ => write!(
                f,
                "received Native24UnitsV2 depth bias capability {self:?}; expected VK_EXT_depth_bias_control with depthBiasControl, floatRepresentation, depthBiasExact and vkCmdSetDepthBias2EXT"
            ),
        }
    }
}
impl std::error::Error for RenderDepthBiasFailure {}
const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderDepthBiasBehavior>();
    send_sync::<RenderDepthBiasFailure>();
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_versions_keep_format_dependent_default() {
        assert_eq!(
            RenderDepthBiasBehavior::default(),
            RenderDepthBiasBehavior::HardwareFormatV1
        );
        assert_eq!(RenderDepthBiasBehavior::default().behavior_version(), 1);
        assert_eq!(
            RenderDepthBiasBehavior::Native24UnitsV2.behavior_version(),
            2
        );
    }
    #[test]
    fn refusal_is_typed_and_describes_the_operation() {
        let e: crate::RenderError = RenderDepthBiasFailure::MissingCommand.into();
        assert_eq!(
            e.depth_bias_failure(),
            Some(&RenderDepthBiasFailure::MissingCommand)
        );
        assert!(e.to_string().contains("vkCmdSetDepthBias2EXT"));
        assert_eq!(
            std::error::Error::source(&e).unwrap().to_string(),
            RenderDepthBiasFailure::MissingCommand.to_string()
        );
        assert!(
            crate::RenderError::rejected(0, "positive")
                .depth_bias_failure()
                .is_none()
        );
    }

    #[test]
    fn unresolved_v4_units_refusal_preserves_the_offending_float_bits() {
        let failure = RenderDepthBiasFailure::UnresolvedHardwareUnits {
            units_bits: 2.0f32.to_bits(),
        };
        let error: crate::RenderError = failure.into();
        assert_eq!(error.depth_bias_failure(), Some(&failure));
        assert!(error.to_string().contains("0x40000000"));
        assert!(error.to_string().contains("Native24UnitsV2"));
    }
}
