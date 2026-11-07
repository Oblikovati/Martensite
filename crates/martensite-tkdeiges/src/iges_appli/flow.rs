//! IgesAppliFlow.
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
use crate::iges_basic::entity_shape::iges_array1_length;
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines Flow, Type `402` Form `18`
/// Represents a single signal or a single fluid flow path
/// starting from a starting Connect Point Entity and
/// including additional intermediate connect points.
IgesAppliFlow, IgesAppliFlowState, "IgesAppliFlow", parent IgesDataIgesEntity, {
the_nb_context_flags: i32,
the_type_of_flow: i32,
the_function_flag: i32,
the_flow_associativities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
the_joins: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_flow_names: Handle<HArray1<Handle<HAsciiString>>>,
the_text_display_templates: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>>,
the_cont_flow_associativities: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesAppliFlow {
    iges_methods! {
    get nb_context_flags = the_nb_context_flags: i32;
    get type_of_flow = the_type_of_flow: i32;
    get function_flag = the_function_flag: i32;
    }
    /// This method is used to set the fields of the class Flow
    /// - nbContextFlags    : Count of Context Flags, always = 2
    /// - aFlowType         : Type of Flow, default = 0
    /// - aFuncFlag         : Function Flag, default = 0
    /// - allFlowAssocs     : Flow Associativity Entities
    /// - allConnectPoints  : Connect Point Entities
    /// - allJoins          : Join Entities
    /// - allFlowNames      : Flow Names
    /// - allTextDisps      : Text Display Template Entities
    /// - allContFlowAssocs : Continuation Flow Associativity Entities
    pub fn init(
        &self,
        nb_context_flags: i32,
        a_flow_type: i32,
        a_func_flag: i32,
        all_flow_assocs: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        all_connect_points: &Handle<HArray1<Handle<IgesDrawConnectPoint>>>,
        all_joins: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        all_flow_names: &Handle<HArray1<Handle<HAsciiString>>>,
        all_text_disps: &Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>>,
        all_cont_flow_assocs: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) {
        {
            let mut state = self.state.write();
            state.the_nb_context_flags = nb_context_flags;
            state.the_type_of_flow = a_flow_type;
            state.the_function_flag = a_func_flag;
            state.the_flow_associativities = all_flow_assocs.clone();
            state.the_connect_points = all_connect_points.clone();
            state.the_joins = all_joins.clone();
            state.the_flow_names = all_flow_names.clone();
            state.the_text_display_templates = all_text_disps.clone();
            state.the_cont_flow_associativities = all_cont_flow_assocs.clone();
        }
        self.init_type_and_form(402, 18);
    }
    /// forces NbContextFalgs to 2, returns True if changed
    pub fn own_correct(&self) -> bool {
        if (self.state.read().the_nb_context_flags) == 2 {
            return false;
        }
        self.state.write().the_nb_context_flags = 2;
        return true;
    }
    /// returns number of Flow Associativity Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_flow_associativities(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_flow_associativities.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_flow_associativities,
                "IgesAppliFlow::nb_flow_associativities",
            )?
        })
    }
    /// returns number of Connect Point Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_connect_points(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_connect_points.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_connect_points,
                "IgesAppliFlow::nb_connect_points",
            )?
        })
    }
    /// returns number of Join Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_joins(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_joins.is_null() {
            0
        } else {
            iges_array1_length(&self.state.read().the_joins, "IgesAppliFlow::nb_joins")?
        })
    }
    /// returns number of Flow Names
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_flow_names(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_flow_names.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_flow_names,
                "IgesAppliFlow::nb_flow_names",
            )?
        })
    }
    /// returns number of Text Display Template Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_text_display_templates(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_text_display_templates.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_text_display_templates,
                "IgesAppliFlow::nb_text_display_templates",
            )?
        })
    }
    /// returns number of Continuation Flow Associativity Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_cont_flow_associativities(&self) -> Result<i32, Failure> {
        Ok(
            if self.state.read().the_cont_flow_associativities.is_null() {
                0
            } else {
                iges_array1_length(
                    &self.state.read().the_cont_flow_associativities,
                    "IgesAppliFlow::nb_cont_flow_associativities",
                )?
            },
        )
    }
    /// returns Flow Associativity Entity
    /// raises exception if Index at most 0 or Index &gt; NbFlowAssociativities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn flow_associativity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_flow_associativities,
            index,
            "IgesAppliFlow::flow_associativity",
        )
    }
    /// returns Connect Point Entity
    /// raises exception if Index at most 0 or Index &gt; NbConnectPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn connect_point(&self, index: i32) -> Result<Handle<IgesDrawConnectPoint>, Failure> {
        iges_array1_item(
            &self.state.read().the_connect_points,
            index,
            "IgesAppliFlow::connect_point",
        )
    }
    /// returns Join Entity
    /// raises exception if Index at most 0 or Index &gt; NbJoins()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn join(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(&self.state.read().the_joins, index, "IgesAppliFlow::join")
    }
    /// returns Flow Name
    /// raises exception if Index at most 0 or Index &gt; NbFlowNames()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn flow_name(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_flow_names,
            index,
            "IgesAppliFlow::flow_name",
        )
    }
    /// returns Text Display Template Entity
    /// raises exception if Index at most 0 or Index &gt; NbTextDisplayTemplates()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn text_display_template(
        &self,
        index: i32,
    ) -> Result<Handle<IgesGraphTextDisplayTemplate>, Failure> {
        iges_array1_item(
            &self.state.read().the_text_display_templates,
            index,
            "IgesAppliFlow::text_display_template",
        )
    }
    /// returns Continuation Flow Associativity Entity
    /// raises exception if Index at most 0 or Index &gt; NbContFlowAssociativities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn cont_flow_associativity(
        &self,
        index: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_cont_flow_associativities,
            index,
            "IgesAppliFlow::cont_flow_associativity",
        )
    }
}
