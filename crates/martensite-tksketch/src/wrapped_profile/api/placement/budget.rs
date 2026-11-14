//! Static dispatch lets both reference contracts use the same physical publication checks.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedFrameTransport, CertifiedRotatedFrameSpatialBudget, CertifiedRotatedFrameTransport,
};
pub(in super::super) trait PlacementFrame {
    fn stretch_bounds(&self) -> [f64; 2];
    fn determinant_bounds(&self) -> [f64; 2];
}
pub(in super::super) trait PlacementBudget {
    type Frame: PlacementFrame;
    fn frame(&self) -> &Self::Frame;
    fn input(&self) -> FrameSpatialInput;
    fn placement_error_bound(&self) -> f64;
    fn evaluation_error_bound(&self) -> f64;
}
impl PlacementFrame for CertifiedFrameTransport {
    #[inline(always)]
    fn stretch_bounds(&self) -> [f64; 2] {
        self.stretch_bounds()
    }
    #[inline(always)]
    fn determinant_bounds(&self) -> [f64; 2] {
        self.determinant_bounds()
    }
}
impl PlacementFrame for CertifiedRotatedFrameTransport {
    #[inline(always)]
    fn stretch_bounds(&self) -> [f64; 2] {
        self.stretch_bounds()
    }
    #[inline(always)]
    fn determinant_bounds(&self) -> [f64; 2] {
        self.determinant_bounds()
    }
}
impl PlacementBudget for CertifiedFrameSpatialBudget {
    type Frame = CertifiedFrameTransport;
    #[inline(always)]
    fn frame(&self) -> &Self::Frame {
        self.frame()
    }
    #[inline(always)]
    fn input(&self) -> FrameSpatialInput {
        self.input()
    }
    #[inline(always)]
    fn placement_error_bound(&self) -> f64 {
        self.placement_error_bound()
    }
    #[inline(always)]
    fn evaluation_error_bound(&self) -> f64 {
        self.evaluation_error_bound()
    }
}
impl PlacementBudget for CertifiedRotatedFrameSpatialBudget {
    type Frame = CertifiedRotatedFrameTransport;
    #[inline(always)]
    fn frame(&self) -> &Self::Frame {
        self.frame()
    }
    #[inline(always)]
    fn input(&self) -> FrameSpatialInput {
        self.input()
    }
    #[inline(always)]
    fn placement_error_bound(&self) -> f64 {
        self.placement_error_bound()
    }
    #[inline(always)]
    fn evaluation_error_bound(&self) -> f64 {
        self.evaluation_error_bound()
    }
}
