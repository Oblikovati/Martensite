//! IgesDrawViewsVisibleWithAttr.
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
    IgesDataIgesEntity, IgesDataLineFontEntity, IgesDataViewKindEntity, iges_array1_item,
};
use crate::iges_entity;
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines IGESViewsVisibleWithAttr, Type `402`, Form `4`
///
/// This class is extension of Class ViewsVisible. It is used
/// for those entities that are visible in multiple views, but
/// must have a different line font, color number, or
/// line weight in each view.
IgesDrawViewsVisibleWithAttr, IgesDrawViewsVisibleWithAttrState, "IgesDrawViewsVisibleWithAttr", kind dyn IgesDataViewKindEntity, {
the_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_line_fonts: Handle<HArray1<i32>>,
the_line_definitions: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>>,
the_color_values: Handle<HArray1<i32>>,
the_color_definitions: Handle<HArray1<Handle<IgesGraphColor>>>,
the_line_weights: Handle<HArray1<i32>>,
the_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDataViewKindEntity for IgesDrawViewsVisibleWithAttr {
    fn is_single(&self) -> bool {
        false
    }
    fn nb_views(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_view_entities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_view_entities,
                "IgesDrawViewsVisibleWithAttr::nb_views",
            )?
        })
    }
    fn view_item(&self, index: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_view_entities,
            index,
            "IgesDrawViewsVisibleWithAttr::view_item",
        )
    }
}

impl IgesDrawViewsVisibleWithAttr {
    /// This method is used to set fields of the class
    /// ViewsVisibleWithAttr
    /// - allViewEntities     : All View kind entities
    /// - allLineFonts        : All Line Font values or zero(0)
    /// - allLineDefinitions  : Line Font Definition
    ///   (if Line Font value = 0)
    /// - allColorValues      : All Color values
    /// - allColorDefinitions : All Color Definition Entities
    /// - allLineWeights      : All Line Weight values
    /// - allDisplayEntities  : Entities which are member of
    ///   this associativity
    ///   raises exception if Lengths of allViewEntities, allLineFonts,
    ///   allColorValues,allColorDefinitions, allLineWeights are not same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_view_entities: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_line_fonts: &Handle<HArray1<i32>>,
        all_line_definitions: &Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>>,
        all_color_values: &Handle<HArray1<i32>>,
        all_color_definitions: &Handle<HArray1<Handle<IgesGraphColor>>>,
        all_line_weights: &Handle<HArray1<i32>>,
        all_display_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_view_entities.is_null() {
            let mut exp: i32 =
                iges_array1_length(all_view_entities, "IgesDrawViewsVisibleWithAttr::init")?;
            if (((((iges_array1_lower(all_view_entities, "IgesDrawViewsVisibleWithAttr::init")?
                != 1)
                || ((iges_array1_lower(all_line_fonts, "IgesDrawViewsVisibleWithAttr::init")?
                    != 1)
                    || (iges_array1_length(
                        all_line_fonts,
                        "IgesDrawViewsVisibleWithAttr::init",
                    )? != exp)))
                || ((iges_array1_lower(all_color_values, "IgesDrawViewsVisibleWithAttr::init")?
                    != 1)
                    || (iges_array1_length(
                        all_color_values,
                        "IgesDrawViewsVisibleWithAttr::init",
                    )? != exp)))
                || ((iges_array1_lower(all_line_weights, "IgesDrawViewsVisibleWithAttr::init")?
                    != 1)
                    || (iges_array1_length(
                        all_line_weights,
                        "IgesDrawViewsVisibleWithAttr::init",
                    )? != exp)))
                || ((iges_array1_lower(
                    all_line_definitions,
                    "IgesDrawViewsVisibleWithAttr::init",
                )? != 1)
                    || (iges_array1_length(
                        all_line_definitions,
                        "IgesDrawViewsVisibleWithAttr::init",
                    )? != exp)))
                || ((iges_array1_lower(
                    all_color_definitions,
                    "IgesDrawViewsVisibleWithAttr::init",
                )? != 1)
                    || (iges_array1_length(
                        all_color_definitions,
                        "IgesDrawViewsVisibleWithAttr::init",
                    )? != exp))
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisibleWithAttr : Init",
                ));
            }
        }
        if !all_display_entities.is_null() {
            if iges_array1_lower(all_display_entities, "IgesDrawViewsVisibleWithAttr::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisibleWithAttr : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_view_entities = all_view_entities.clone();
            state.the_line_fonts = all_line_fonts.clone();
            state.the_line_definitions = all_line_definitions.clone();
            state.the_color_values = all_color_values.clone();
            state.the_color_definitions = all_color_definitions.clone();
            state.the_line_weights = all_line_weights.clone();
            state.the_display_entities = all_display_entities.clone();
        }
        self.init_type_and_form(402, 4);
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
            if iges_array1_lower(
                all_display_entity,
                "IgesDrawViewsVisibleWithAttr::init_implied",
            )? != 1
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawViewsVisibleWithAttr : InitImplied",
                ));
            }
        }
        self.state.write().the_display_entities = all_display_entity.clone();
        Ok(())
    }
    /// returns the number of entities which have this particular set of
    /// display characteristic, or zero if no Entities specified
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_displayed_entities(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_display_entities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_display_entities,
                "IgesDrawViewsVisibleWithAttr::nb_displayed_entities",
            )?
        })
    }
    /// returns the Index'th Line font value or zero
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn line_font_value(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_line_fonts,
            index,
            "IgesDrawViewsVisibleWithAttr::line_font_value",
        )
    }
    /// returns True if the Index'th Line Font Definition is specified
    /// else returns False
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_font_definition(&self, index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_line_definitions,
            index,
            "IgesDrawViewsVisibleWithAttr::is_font_definition",
        )?
        .is_null()))
    }
    /// returns the Index'th Line Font Definition Entity or NULL(0)
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn font_definition(
        &self,
        index: i32,
    ) -> Result<Handle<dyn IgesDataLineFontEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_line_definitions,
            index,
            "IgesDrawViewsVisibleWithAttr::font_definition",
        )
    }
    /// returns the Index'th Color number value
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn color_value(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_color_values,
            index,
            "IgesDrawViewsVisibleWithAttr::color_value",
        )
    }
    /// returns True if Index'th Color Definition is specified
    /// else returns False
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_color_definition(&self, index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_color_definitions,
            index,
            "IgesDrawViewsVisibleWithAttr::is_color_definition",
        )?
        .is_null()))
    }
    /// returns the Index'th Color Definition Entity
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn color_definition(&self, index: i32) -> Result<Handle<IgesGraphColor>, Failure> {
        iges_array1_item(
            &self.state.read().the_color_definitions,
            index,
            "IgesDrawViewsVisibleWithAttr::color_definition",
        )
    }
    /// returns the Index'th Color Line Weight
    /// raises exception if Index at most 0 or Index &gt; NbViews()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn line_weight_item(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_line_weights,
            index,
            "IgesDrawViewsVisibleWithAttr::line_weight_item",
        )
    }
    /// returns Index'th Display entity with this particular characteristics
    /// raises exception if Index at most 0 or Index &gt; NbEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn displayed_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_display_entities,
            index,
            "IgesDrawViewsVisibleWithAttr::displayed_entity",
        )
    }
}
