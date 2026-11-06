//! IgesDrawNetworkSubfigure.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_draw::{IgesDrawConnectPoint, IgesDrawNetworkSubfigureDef};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines IGES Network Subfigure Instance Entity,
///
/// Used to specify each instance of Network Subfigure
/// Definition Entity (Type 320, Form 0).
IgesDrawNetworkSubfigure, IgesDrawNetworkSubfigureState, "IgesDrawNetworkSubfigure", parent IgesDataIgesEntity, {
the_subfigure_definition: Handle<IgesDrawNetworkSubfigureDef>,
the_translation: Xyz,
the_scale_factor: Xyz,
the_type_flag: i32,
the_designator: Handle<HAsciiString>,
the_designator_template: Handle<IgesGraphTextDisplayTemplate>,
the_connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
}
}

impl IgesDrawNetworkSubfigure {
    iges_methods! {
    get subfigure_definition = the_subfigure_definition: Handle<IgesDrawNetworkSubfigureDef>;
    get translation = the_translation: Xyz;
    get scale_factors = the_scale_factor: Xyz;
    get type_flag = the_type_flag: i32;
    get reference_designator = the_designator: Handle<HAsciiString>;
    get designator_template = the_designator_template: Handle<IgesGraphTextDisplayTemplate>;
    }
    /// This method is used to set the fields of the class
    /// NetworkSubfigure
    /// - aDefinition      : Network Subfigure Definition Entity
    /// - aTranslation     : Translation data relative to the model
    ///   space or the definition space
    /// - aScaleFactor     : Scale factors in the definition space
    /// - aTypeFlag        : Type flag
    /// - aDesignator      : Primary reference designator
    /// - aTemplate        : Primary reference designator Text
    ///   display Template Entity
    /// - allConnectPoints : Associated Connect Point Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_definition: &Handle<IgesDrawNetworkSubfigureDef>,
        a_translation: Xyz,
        a_scale_factor: Xyz,
        a_type_flag: i32,
        a_designator: &Handle<HAsciiString>,
        a_template: &Handle<IgesGraphTextDisplayTemplate>,
        all_connect_points: &Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
    ) -> Result<(), Failure> {
        if !all_connect_points.is_null() {
            if iges_array1_lower(all_connect_points, "IgesDrawNetworkSubfigure::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawNetworkSubfigure : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_subfigure_definition = a_definition.clone();
            state.the_translation = a_translation;
            state.the_scale_factor = a_scale_factor;
            state.the_type_flag = a_type_flag;
            state.the_designator = a_designator.clone();
            state.the_designator_template = a_template.clone();
            state.the_connect_points = all_connect_points.clone();
        }
        self.init_type_and_form(420, 0);
        Ok(())
    }
    /// returns the Transformed Translation Data relative to either model
    /// space or to the definition space of a referring entity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_translation(&self) -> Result<Xyz, Failure> {
        let mut temp_xyz: Xyz = self.state.read().the_translation;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(temp_xyz)
    }
    /// returns True if Text Display Template Entity is specified,
    /// else False
    pub fn has_designator_template(&self) -> bool {
        !(self.state.read().the_designator_template.is_null())
    }
    /// returns the number of associated Connect Point Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_connect_points(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_connect_points.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_connect_points,
                "IgesDrawNetworkSubfigure::nb_connect_points",
            )?
        })
    }
    /// returns the Index'th  associated Connect point Entity
    /// raises exception if Index at most 0 or Index &gt; NbConnectPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn connect_point(&self, index: i32) -> Result<Handle<IgesDrawConnectPoint>, Failure> {
        iges_array1_item(
            &self.state.read().the_connect_points,
            index,
            "IgesDrawNetworkSubfigure::connect_point",
        )
    }
}
