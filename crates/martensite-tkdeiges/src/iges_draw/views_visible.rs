//! IgesDrawViewsVisible.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, IgesDataViewKindEntity, iges_array1_item};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// Defines IGESViewsVisible, Type `402`, Form `3`
///
/// If an entity is to be displayed in more than one views,
/// this class instance is used, which contains the Visible
/// views and the associated entity Displays.
IgesDrawViewsVisible, IgesDrawViewsVisibleState, "IgesDrawViewsVisible", kind dyn IgesDataViewKindEntity, {
the_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_display_entity: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDataViewKindEntity for IgesDrawViewsVisible {
    fn is_single(&self) -> bool {
        false
    }
    fn nb_views(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_view_entities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_view_entities,
                "IgesDrawViewsVisible::nb_views",
            )?
        })
    }
    fn view_item(&self, index: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_view_entities,
            index,
            "IgesDrawViewsVisible::view_item",
        )
    }
}

impl IgesDrawViewsVisible {
    /// This method is used to set the fields of the class
    /// ViewsVisible
    /// - allViewEntities  : All View kind entities
    /// - allDisplayEntity : All entities whose display is specified
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_view_entities: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_display_entity: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_display_entity.is_null() {
            if iges_array1_lower(all_display_entity, "IgesDrawViewsVisible::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisible : Init",
                ));
            }
        }
        if !all_view_entities.is_null() {
            if iges_array1_lower(all_view_entities, "IgesDrawViewsVisible::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisible : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_view_entities = all_view_entities.clone();
            state.the_display_entity = all_display_entity.clone();
        }
        self.init_type_and_form(402, 3);
        Ok(())
    }
    /// Changes only the list of Displayed Entities (Null allowed)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init_implied(
        &self,
        all_display_entity: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_display_entity.is_null() {
            if iges_array1_lower(all_display_entity, "IgesDrawViewsVisible::init_implied")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisible : InitImplied",
                ));
            }
        }
        self.state.write().the_display_entity = all_display_entity.clone();
        Ok(())
    }
    /// returns the number of entities displayed in the Views or zero if
    /// no Entities specified in these Views
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_displayed_entities(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_display_entity.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_display_entity,
                "IgesDrawViewsVisible::nb_displayed_entities",
            )?
        })
    }
    /// returns the Index'th entity whose display is being specified by
    /// this associativity instance
    /// raises exception if Index at most 0 or Index &gt; NbEntityDisplayed()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn displayed_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_display_entity,
            index,
            "IgesDrawViewsVisible::displayed_entity",
        )
    }
}
