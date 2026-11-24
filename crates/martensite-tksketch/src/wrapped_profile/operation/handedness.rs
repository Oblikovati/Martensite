//! The admitted support map fixes development orientation independently of caller depth names.
use martensite_tkg3d::top_abs::Orientation;
/// #540: cone Cartesian development reverses `(U,V)`; cylinder `(s,z)` preserves it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::surface_sketch::wrapped_profile) enum DevelopedMapHandedness {
    Preserving,
    Reversing,
}
impl DevelopedMapHandedness {
    pub(in crate::surface_sketch::wrapped_profile) fn reverses(self) -> bool {
        self == Self::Reversing
    }
    pub(in crate::surface_sketch::wrapped_profile) fn boundary_orientation(self) -> Orientation {
        match self {
            Self::Preserving => Orientation::Forward,
            Self::Reversing => Orientation::Reversed,
        }
    }
}
