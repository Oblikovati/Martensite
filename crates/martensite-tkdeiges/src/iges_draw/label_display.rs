//! IgesDrawLabelDisplay.
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
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataLabelDisplayEntity, IgesDataViewKindEntity, iges_array1_item,
    iges_live,
};
use crate::iges_dimen::IgesDimenLeaderArrow;
use crate::iges_draw::{IgesDrawPerspectiveView, IgesDrawView};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGESLabelDisplay, Type `402` Form `5`
///
/// Permits one or more displays for the
/// entity labels of an entity
IgesDrawLabelDisplay, IgesDrawLabelDisplayState, "IgesDrawLabelDisplay", kind dyn IgesDataLabelDisplayEntity, {
the_views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_text_locations: Handle<HArray1<Xyz>>,
the_leader_entities: Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
the_label_levels: Handle<HArray1<i32>>,
the_displayed_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDataLabelDisplayEntity for IgesDrawLabelDisplay {}

impl IgesDrawLabelDisplay {
    /// This method is used to set the fields of the class
    /// LabelDisplay
    /// - allViews             : Pointers to View Entities
    /// - allTextLocations     : Coordinates of text locations in the views
    /// - allLeaderEntities    : Pointers to Leader Entities in the views
    /// - allLabelLevels       : Entity label level numbers in the views
    /// - allDisplayedEntities : Pointers to the entities being displayed
    ///   raises exception if Lengths of allViews, allTextLocations,
    ///   allLeaderEntities, allLabelLevels and allDisplayedEntities are
    ///   not same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_views: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_text_locations: &Handle<HArray1<Xyz>>,
        all_leader_entities: &Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
        all_label_levels: &Handle<HArray1<i32>>,
        all_displayed_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        let mut ln: i32 = iges_array1_length(all_views, "IgesDrawLabelDisplay::init")?;
        if ((((iges_array1_lower(all_views, "IgesDrawLabelDisplay::init")? != 1)
            || ((iges_array1_lower(all_text_locations, "IgesDrawLabelDisplay::init")? != 1)
                || (iges_array1_length(all_text_locations, "IgesDrawLabelDisplay::init")? != ln)))
            || ((iges_array1_lower(all_leader_entities, "IgesDrawLabelDisplay::init")? != 1)
                || (iges_array1_length(all_leader_entities, "IgesDrawLabelDisplay::init")? != ln)))
            || ((iges_array1_lower(all_label_levels, "IgesDrawLabelDisplay::init")? != 1)
                || (iges_array1_length(all_label_levels, "IgesDrawLabelDisplay::init")? != ln)))
            || ((iges_array1_lower(all_displayed_entities, "IgesDrawLabelDisplay::init")? != 1)
                || (iges_array1_length(all_displayed_entities, "IgesDrawLabelDisplay::init")?
                    != ln))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDrawLabelDisplay : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_views = all_views.clone();
            state.the_text_locations = all_text_locations.clone();
            state.the_leader_entities = all_leader_entities.clone();
            state.the_label_levels = all_label_levels.clone();
            state.the_displayed_entities = all_displayed_entities.clone();
        }
        self.init_type_and_form(402, 5);
        Ok(())
    }
    /// returns the number of label placements in `me`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_labels(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_views,
            "IgesDrawLabelDisplay::nb_labels",
        )
    }
    /// returns the View entity indicated by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_item(
        &self,
        view_index: i32,
    ) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_views,
            view_index,
            "IgesDrawLabelDisplay::view_item",
        )
    }
    /// returns the 3d-Point coordinates of the text location, in the
    /// view indicated by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn text_location(&self, view_index: i32) -> Result<Point3, Failure> {
        Ok(Point3::from(iges_array1_item(
            &self.state.read().the_text_locations,
            view_index,
            "IgesDrawLabelDisplay::text_location",
        )?))
    }
    /// returns the Leader entity in the view indicated by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn leader_entity(&self, view_index: i32) -> Result<Handle<IgesDimenLeaderArrow>, Failure> {
        iges_array1_item(
            &self.state.read().the_leader_entities,
            view_index,
            "IgesDrawLabelDisplay::leader_entity",
        )
    }
    /// returns the Entity label level number in the view indicated
    /// by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn label_level(&self, view_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_label_levels,
            view_index,
            "IgesDrawLabelDisplay::label_level",
        )
    }
    /// returns the entity indicated by EntityIndex
    /// raises an exception if EntityIndex at most 0 or EntityIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn displayed_entity(
        &self,
        entity_index: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_displayed_entities,
            entity_index,
            "IgesDrawLabelDisplay::displayed_entity",
        )
    }
    /// returns the transformed 3d-Point coordinates of the text
    /// location, in the view indicated by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbLabels().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_text_location(&self, view_index: i32) -> Result<Point3, Failure> {
        let mut ret_xyz: Xyz = Default::default();
        let mut temp_xyz: Xyz = iges_array1_item(
            &self.state.read().the_text_locations,
            view_index,
            "IgesDrawLabelDisplay::transformed_text_location",
        )?;
        let mut temp_view: Handle<dyn IgesDataViewKindEntity> = iges_array1_item(
            &self.state.read().the_views,
            view_index,
            "IgesDrawLabelDisplay::transformed_text_location",
        )?;
        if temp_view.is_kind(<IgesDrawView>::static_type()) {
            let mut this_view: Handle<IgesDrawView> = temp_view.cast();
            ret_xyz = iges_live(
                &this_view,
                "IgesDrawLabelDisplay::transformed_text_location",
            )?
            .model_to_view(temp_xyz)?;
        } else {
            if temp_view.is_kind(<IgesDrawPerspectiveView>::static_type()) {
                let mut this_view: Handle<IgesDrawPerspectiveView> = temp_view.cast();
                ret_xyz = iges_live(
                    &this_view,
                    "IgesDrawLabelDisplay::transformed_text_location",
                )?
                .model_to_view(temp_xyz)?;
            }
        }
        Ok(Point3::from(ret_xyz))
    }
}
