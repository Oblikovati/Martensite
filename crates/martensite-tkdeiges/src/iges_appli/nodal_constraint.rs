//! IgesAppliNodalConstraint.
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
use crate::iges_appli::IgesAppliNode;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_defs::IgesDefsTabularData;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines NodalConstraint, Type `418` Form `0`
/// Relates loads and/or constraints to specific nodes in
/// the Finite Element Model by creating a relation between
/// Node entities and Tabular Data Property that contains
/// the load or constraint data
IgesAppliNodalConstraint, IgesAppliNodalConstraintState, "IgesAppliNodalConstraint", parent IgesDataIgesEntity, {
the_type: i32,
the_node: Handle<IgesAppliNode>,
the_tabular_data_props: Handle<HArray1<Handle<IgesDefsTabularData>>>,
}
}

impl IgesAppliNodalConstraint {
    iges_methods! {
    get type_ = the_type: i32;
    get node_entity = the_node: Handle<IgesAppliNode>;
    }
    /// This method is used to set the fields of the class
    /// NodalConstraint
    /// - aType      : Loads / Constraints
    /// - aNode      : the Node
    /// - allTabData : Tabular Data Property carrying the load
    ///   or constraint vector
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_type: i32,
        a_node: &Handle<IgesAppliNode>,
        all_tab_data: &Handle<HArray1<Handle<IgesDefsTabularData>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_tab_data, "IgesAppliNodalConstraint::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliNodalConstraint : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_type = a_type;
            state.the_node = a_node.clone();
            state.the_tabular_data_props = all_tab_data.clone();
        }
        self.init_type_and_form(418, 0);
        Ok(())
    }
    /// returns total number of cases
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_cases(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_tabular_data_props,
            "IgesAppliNodalConstraint::nb_cases",
        )
    }
    /// returns Tabular Data Property carrying load or constraint vector
    /// raises exception if Index at most 0 or Index &gt; NbCases
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn tabular_data(&self, index: i32) -> Result<Handle<IgesDefsTabularData>, Failure> {
        iges_array1_item(
            &self.state.read().the_tabular_data_props,
            index,
            "IgesAppliNodalConstraint::tabular_data",
        )
    }
}
