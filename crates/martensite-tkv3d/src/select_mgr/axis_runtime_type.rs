//! Concrete selectors inherit their native descriptor and register their actual Rust interfaces.
use super::{
    SelectMgrAxisIntersector, SelectMgrBaseFrustum, SelectMgrBaseIntersector,
    SelectMgrRectangularFrustum, SelectMgrTriangularFrustumSet,
};
use crate::inherited_native_type::inherited_native_type;

inherited_native_type!(
    SelectMgrAxisIntersector,
    dyn SelectMgrBaseIntersector,
    "SelectMgrAxisIntersector Rust casters",
    inherited,
    [dyn SelectMgrBaseIntersector]
);
inherited_native_type!(
    SelectMgrRectangularFrustum,
    dyn SelectMgrBaseFrustum,
    "super::SelectMgrRectangularFrustum Rust casters",
    inherited,
    [dyn SelectMgrBaseFrustum, dyn SelectMgrBaseIntersector]
);
inherited_native_type!(
    SelectMgrTriangularFrustumSet,
    dyn SelectMgrBaseFrustum,
    "super::SelectMgrTriangularFrustumSet Rust casters",
    inherited,
    [dyn SelectMgrBaseFrustum, dyn SelectMgrBaseIntersector]
);
