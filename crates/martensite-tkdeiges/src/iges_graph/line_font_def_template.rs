//! IgesGraphLineFontDefTemplate.
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
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_data::IgesDataLineFontEntity;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::Handle;

iges_entity! {
/// defines IGESLineFontDefTemplate, Type `304` Form `1`
///
/// Line Font can be defined as a repetition of Template figure
/// that is displayed at regularly spaced locations along a
/// planer anchoring curve. The anchoring curve itself has
/// no visual purpose.
IgesGraphLineFontDefTemplate, IgesGraphLineFontDefTemplateState, "IgesGraphLineFontDefTemplate", kind dyn IgesDataLineFontEntity, {
the_orientation: i32,
the_template_entity: Handle<IgesBasicSubfigureDef>,
the_distance: f64,
the_scale: f64,
}
}

impl IgesDataLineFontEntity for IgesGraphLineFontDefTemplate {}

impl IgesGraphLineFontDefTemplate {
    iges_methods! {
    get orientation = the_orientation: i32;
    get template_entity = the_template_entity: Handle<IgesBasicSubfigureDef>;
    get distance = the_distance: f64;
    get scale = the_scale: f64;
    }
    /// This method is used to set the fields of the class
    /// LineFontDefTemplate
    /// - anOrientation : Orientation of Template figure on
    ///   anchoring curve
    /// - aTemplate     : SubfigureDef entity used as Template figure
    /// - aDistance     : Distance between the neighbouring Template
    ///   figures
    /// - aScale        : Scale factor applied to the Template figure
    pub fn init(
        &self,
        an_orientation: i32,
        a_template: &Handle<IgesBasicSubfigureDef>,
        a_distance: f64,
        a_scale: f64,
    ) {
        {
            let mut state = self.state.write();
            state.the_orientation = an_orientation;
            state.the_template_entity = a_template.clone();
            state.the_distance = a_distance;
            state.the_scale = a_scale;
        }
        self.init_type_and_form(304, 1);
    }
}
