//! Anonymous native command operands must not acquire a named-registry owner.
use super::*;
use martensite_tkv3d::prs_mgr::PrsMgrDisplayStatus;

pub(super) fn anonymous_box(
    context: &AisInteractiveContext,
) -> Result<(Shape, Handle<dyn AisInteractiveObject>), Failure> {
    let shape = crate::primitive_shapes::box_sized([10.; 3])?;
    let object = AisShape::new(&shape).cast();
    context.display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)?;
    Ok((shape, object))
}
