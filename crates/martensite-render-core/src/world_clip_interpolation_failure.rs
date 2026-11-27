//! Typed refusal for unsupported fragment world-clip interpolation compositions.
use crate::{
    RenderPhongInterpolationBehavior, RenderTriangleCoverageBehavior, RenderWorldClippingBehavior,
};
use std::fmt;

/// A selected clipping interpolation policy cannot preserve the requested raster contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderWorldClipInterpolationFailure {
    /// The clipping, coverage, or Phong policy combination is unsupported.
    UnsupportedComposition {
        /// Active world-clipping implementation.
        world_clipping: RenderWorldClippingBehavior,
        /// Active triangle-coverage contract.
        triangle_coverage: RenderTriangleCoverageBehavior,
        /// Active smooth-attribute interpolation contract.
        phong_interpolation: RenderPhongInterpolationBehavior,
    },
    /// The clipped draw does not match the registered opaque Phong triangle input.
    UnsupportedDraw {
        /// Actual topology or draw state that prevented selection.
        actual: String,
    },
}

impl fmt::Display for RenderWorldClipInterpolationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "world-clip interpolation rejected {self:?}; expected clipped opaque Phong triangles on the single-sample projected Q12 grid"
        )
    }
}

impl std::error::Error for RenderWorldClipInterpolationFailure {}
