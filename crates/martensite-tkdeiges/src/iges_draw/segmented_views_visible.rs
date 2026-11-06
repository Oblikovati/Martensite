//! IgesDrawSegmentedViewsVisible.
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
use crate::iges_data::{IgesDataLineFontEntity, IgesDataViewKindEntity, iges_array1_item};
use crate::iges_entity;
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines IGESSegmentedViewsVisible, Type `402` Form `19`
///
/// Permits the association of display parameters with the
/// segments of curves in a given view
IgesDrawSegmentedViewsVisible, IgesDrawSegmentedViewsVisibleState, "IgesDrawSegmentedViewsVisible", kind dyn IgesDataViewKindEntity, {
the_views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_breakpoint_parameters: Handle<HArray1<f64>>,
the_display_flags: Handle<HArray1<i32>>,
the_color_values: Handle<HArray1<i32>>,
the_color_definitions: Handle<HArray1<Handle<IgesGraphColor>>>,
the_line_font_values: Handle<HArray1<i32>>,
the_line_font_definitions: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>>,
the_line_weights: Handle<HArray1<i32>>,
}
}

impl IgesDataViewKindEntity for IgesDrawSegmentedViewsVisible {
    fn is_single(&self) -> bool {
        false
    }
    fn nb_views(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_views,
            "IgesDrawSegmentedViewsVisible::nb_views",
        )
    }
    fn view_item(&self, view_index: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_views,
            view_index,
            "IgesDrawSegmentedViewsVisible::view_item",
        )
    }
}

impl IgesDrawSegmentedViewsVisible {
    /// This method is used to set the fields of the class
    /// SegmentedViewsVisible
    /// - allViews                : Pointers to View Entities
    /// - allBreakpointParameters : Parameters of breakpoints
    /// - allDisplayFlags         : Display flags
    /// - allColorValues          : Color Values
    /// - allColorDefinitions     : Color Definitions
    /// - allLineFontValues       : LineFont values
    /// - allLineFontDefinitions  : LineFont Definitions
    /// - allLineWeights          : Line weights
    ///   raises exception if Lengths of allViews, allBreakpointParameters,
    ///   allDisplayFlags, allColorValues, allColorDefinitions,
    ///   allLineFontValues, allLineFontDefinitions and allLineWeights
    ///   are not same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_views: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_breakpoint_parameters: &Handle<HArray1<f64>>,
        all_display_flags: &Handle<HArray1<i32>>,
        all_color_values: &Handle<HArray1<i32>>,
        all_color_definitions: &Handle<HArray1<Handle<IgesGraphColor>>>,
        all_line_font_values: &Handle<HArray1<i32>>,
        all_line_font_definitions: &Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>>,
        all_line_weights: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        let mut len: i32 = iges_array1_length(all_views, "IgesDrawSegmentedViewsVisible::init")?;
        if (((((((iges_array1_lower(all_views, "IgesDrawSegmentedViewsVisible::init")? != 1)
            || ((iges_array1_lower(
                all_breakpoint_parameters,
                "IgesDrawSegmentedViewsVisible::init",
            )? != 1)
                || (iges_array1_length(
                    all_breakpoint_parameters,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(all_display_flags, "IgesDrawSegmentedViewsVisible::init")?
                != 1)
                || (iges_array1_length(
                    all_display_flags,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(all_color_values, "IgesDrawSegmentedViewsVisible::init")?
                != 1)
                || (iges_array1_length(
                    all_color_values,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(
                all_color_definitions,
                "IgesDrawSegmentedViewsVisible::init",
            )? != 1)
                || (iges_array1_length(
                    all_color_definitions,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(all_line_font_values, "IgesDrawSegmentedViewsVisible::init")?
                != 1)
                || (iges_array1_length(
                    all_line_font_values,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(
                all_line_font_definitions,
                "IgesDrawSegmentedViewsVisible::init",
            )? != 1)
                || (iges_array1_length(
                    all_line_font_definitions,
                    "IgesDrawSegmentedViewsVisible::init",
                )? != len)))
            || ((iges_array1_lower(all_line_weights, "IgesDrawSegmentedViewsVisible::init")? != 1)
                || (iges_array1_length(all_line_weights, "IgesDrawSegmentedViewsVisible::init")?
                    != len))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDrawSegmentedViewsVisible : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_views = all_views.clone();
            state.the_breakpoint_parameters = all_breakpoint_parameters.clone();
            state.the_display_flags = all_display_flags.clone();
            state.the_color_values = all_color_values.clone();
            state.the_color_definitions = all_color_definitions.clone();
            state.the_line_font_values = all_line_font_values.clone();
            state.the_line_font_definitions = all_line_font_definitions.clone();
            state.the_line_weights = all_line_weights.clone();
        }
        self.init_type_and_form(402, 19);
        Ok(())
    }
    /// returns the number of view/segment blocks in `me`
    /// Similar to NbViews but has a more general significance
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_segment_blocks(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_views,
            "IgesDrawSegmentedViewsVisible::nb_segment_blocks",
        )
    }
    /// returns the parameter of the breakpoint indicated by
    /// BreakpointIndex
    /// raises an exception if BreakpointIndex at most 0 or
    /// BreakpointIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn breakpoint_parameter(&self, breakpoint_index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_breakpoint_parameters,
            breakpoint_index,
            "IgesDrawSegmentedViewsVisible::breakpoint_parameter",
        )
    }
    /// returns the Display flag indicated by FlagIndex
    /// raises an exception if FlagIndex at most 0 or
    /// FlagIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn display_flag(&self, flag_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_display_flags,
            flag_index,
            "IgesDrawSegmentedViewsVisible::display_flag",
        )
    }
    /// returns True if the ColorIndex'th value of the
    /// "theColorDefinitions" field of `me` is a pointer
    /// raises an exception if ColorIndex at most 0 or
    /// ColorIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_color_definition(&self, color_index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_color_definitions,
            color_index,
            "IgesDrawSegmentedViewsVisible::is_color_definition",
        )?
        .is_null()))
    }
    /// returns the Color value indicated by ColorIndex
    /// raises an exception if ColorIndex at most 0 or
    /// ColorIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn color_value(&self, color_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_color_values,
            color_index,
            "IgesDrawSegmentedViewsVisible::color_value",
        )
    }
    /// returns the Color definition entity indicated by ColorIndex
    /// raises an exception if ColorIndex at most 0 or
    /// ColorIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn color_definition(&self, color_index: i32) -> Result<Handle<IgesGraphColor>, Failure> {
        iges_array1_item(
            &self.state.read().the_color_definitions,
            color_index,
            "IgesDrawSegmentedViewsVisible::color_definition",
        )
    }
    /// returns True if the FontIndex'th value of the
    /// "theLineFontDefinitions" field of `me` is a pointer
    /// raises an exception if FontIndex at most 0 or
    /// FontIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_font_definition(&self, font_index: i32) -> Result<bool, Failure> {
        Ok(!(iges_array1_item(
            &self.state.read().the_line_font_definitions,
            font_index,
            "IgesDrawSegmentedViewsVisible::is_font_definition",
        )?
        .is_null()))
    }
    /// returns the LineFont value indicated by FontIndex
    /// raises an exception if FontIndex at most 0 or
    /// FontIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn line_font_value(&self, font_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_line_font_values,
            font_index,
            "IgesDrawSegmentedViewsVisible::line_font_value",
        )
    }
    /// returns the LineFont definition entity indicated by FontIndex
    /// raises an exception if FontIndex at most 0 or
    /// FontIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn line_font_definition(
        &self,
        font_index: i32,
    ) -> Result<Handle<dyn IgesDataLineFontEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_line_font_definitions,
            font_index,
            "IgesDrawSegmentedViewsVisible::line_font_definition",
        )
    }
    /// returns the LineWeight value indicated by WeightIndex
    /// raises an exception if WeightIndex at most 0 or
    /// WeightIndex &gt; NbSegmentBlocks().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn line_weight_item(&self, weight_index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_line_weights,
            weight_index,
            "IgesDrawSegmentedViewsVisible::line_weight_item",
        )
    }
}
