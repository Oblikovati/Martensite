//! Typed refusal prevents an unsupported depth route from becoming a replay fallback.
use std::fmt;

/// The exact numerical boundary whose proof was incomplete.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderWindowDepthBoundary {
    /// Source transformation or shader arithmetic could be nonfinite.
    ShaderArithmetic,
    /// The original primitive could be clipped or have nonpositive W.
    OriginalClip,
    /// The declared reference-grid plane could have zero area.
    ReferenceGrid,
    /// A window coordinate could overflow the declared fixed-point domain.
    WindowRange,
    /// Correcting Z could alter clipping instead of only depth interpolation.
    CorrectedClip,
}

/// An incomplete window-depth operation, with its offending entity or route.
/// Example: `RenderWindowDepthFailure::MissingGeometryShader` refuses selected V2.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderWindowDepthFailure {
    /// No enabled geometry stage can execute the selected operation.
    MissingGeometryShader,
    /// Exact empty-grid classification requires enabled signed 64-bit shader arithmetic.
    MissingShaderInt64,
    /// Binary64 reference arithmetic requires an explicitly enabled shader feature.
    MissingShaderFloat64,
    /// Geometry interface limits are below the registered two-module contract.
    GeometryLimits {
        /// The actual of this failure.
        actual: [u32; 4],
        /// The required of this failure.
        required: [u32; 4],
    },
    /// Source selection, actual pipeline and mandatory status descriptor did not agree.
    GpuBinding {
        /// Whether the required requirement holds.
        required: bool,
        /// Whether the actual pipeline requirement holds.
        actual_pipeline: bool,
        /// Whether the present requirement holds.
        present: bool,
    },
    /// The operation reused a status allocation or never recorded its generation.
    GpuGeneration {
        /// The expected of this failure.
        expected: usize,
        /// The observed of this failure.
        observed: usize,
    },
    /// An actual draw command did not match its source triangle membership.
    GpuDispatch {
        /// The expected indices of this failure.
        expected_indices: u32,
        /// The observed indices of this failure.
        observed_indices: u32,
    },
    /// A frame draw retains the exact typed completion failure beneath its ordinal.
    GpuDraw {
        /// The draw of this failure.
        draw: usize,
        /// The typed failure beneath this one.
        cause: Box<RenderWindowDepthFailure>,
    },
    /// A status readback did not retain the complete allocated byte range.
    GpuReadback {
        /// The expected bytes of this failure.
        expected_bytes: usize,
        /// The observed bytes of this failure.
        observed_bytes: usize,
    },
    /// GPU completion is missing, malformed, reused or inconsistent with actual dispatches.
    GpuCompletion {
        /// The expected triangles of this failure.
        expected_triangles: usize,
        /// The expected invocations of this failure.
        expected_invocations: u64,
        /// The observed header of this failure.
        observed_header: [u32; 4],
    },
    /// Aggregate completion cannot hide a missing or duplicated primitive invocation.
    GpuPrimitiveCount {
        /// The triangle of this failure.
        triangle: usize,
        /// The vertex indices of this failure.
        vertex_indices: [u32; 3],
        /// The expected of this failure.
        expected: u64,
        /// The observed of this failure.
        observed: u32,
    },
    /// The expected invocation count exceeded its bounded representation.
    GpuInvocationOverflow {
        /// The current of this failure.
        current: u64,
        /// The increment of this failure.
        increment: u64,
    },
    /// A submitted source primitive did not complete its actual shader arithmetic validation.
    GpuTriangle {
        /// The triangle of this failure.
        triangle: usize,
        /// The vertex indices of this failure.
        vertex_indices: [u32; 3],
        /// The flags of this failure.
        flags: u32,
    },
    /// CPU-written reference constants changed during the submitted operation.
    GpuConstantWord {
        /// The word of this failure.
        word: usize,
        /// The expected of this failure.
        expected: u32,
        /// The observed of this failure.
        observed: u32,
    },
    /// The selected draw or actual pipeline has an unsupported interface.
    UnsupportedInterface {
        /// The actual of this failure.
        actual: String,
    },
    /// The actual viewport is not the finite admitted viewport and depth range.
    InvalidViewport {
        /// The words of this failure.
        words: [u32; 6],
    },
    /// Original source indices identify the triangle whose proof was incomplete.
    Triangle {
        /// The triangle of this failure.
        triangle: usize,
        /// The vertex indices of this failure.
        vertex_indices: [u32; 3],
        /// The boundary of this failure.
        boundary: RenderWindowDepthBoundary,
    },
}

impl fmt::Display for RenderWindowDepthFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "window-depth reconstruction received {self:?}; expected admitted finite triangle in the selected clipping domain, explicit grid, viewport and matching pipeline"
        )
    }
}
impl std::error::Error for RenderWindowDepthFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GpuDraw { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refusal_retains_typed_cause_and_expected_contract() {
        let failure = RenderWindowDepthFailure::Triangle {
            triangle: 2,
            vertex_indices: [7, 8, 9],
            boundary: RenderWindowDepthBoundary::CorrectedClip,
        };
        let error: crate::RenderError = failure.clone().into();
        assert_eq!(error.window_depth_failure(), Some(&failure));
        assert_eq!(
            std::error::Error::source(&error).unwrap().to_string(),
            failure.to_string()
        );
        assert!(error.to_string().contains("[7, 8, 9]"));
        assert!(error.to_string().contains("expected admitted"));
    }
}
