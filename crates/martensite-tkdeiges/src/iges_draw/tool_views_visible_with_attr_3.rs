//! More of IgesDrawToolViewsVisibleWithAttr.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_data::{IgesDataIgesEntity, IgesDataViewKindEntity, iges_live};
use crate::iges_draw::{IgesDrawToolViewsVisibleWithAttr, IgesDrawViewsVisibleWithAttr};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};

impl IgesDrawToolViewsVisibleWithAttr {
    /// Sets automatic unambiguous Correction on a ViewsVisibleWithAttr
    /// (all displayed entities must refer to `ent` in directory part,
    /// else the list is cleared)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDrawViewsVisibleWithAttr>) -> Result<bool, Failure> {
        let mut res: bool = false;
        let mut nb: i32 = ent.nb_displayed_entities()?;
        let mut entcomp: Handle<dyn IgesDataViewKindEntity> = ent.cast();
        for i in 1..=nb {
            let mut displayed: Handle<IgesDataIgesEntity> = ent.displayed_entity(i)?;
            if entcomp.object_address()
                != iges_live(&displayed, "IgesDrawToolViewsVisibleWithAttr::own_correct")?
                    .view()
                    .object_address()
            {
                res = true;
            }
        }
        crate::iges_shared_blocks::iges_block_0051!(ent, nul_display_entities, res);
    }
}
