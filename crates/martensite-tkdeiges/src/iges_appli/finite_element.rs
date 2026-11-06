//! IgesAppliFiniteElement.
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
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines FiniteElement, Type `136` Form `0`
/// Used to define a finite element with the help of an
/// element topology.
IgesAppliFiniteElement, IgesAppliFiniteElementState, "IgesAppliFiniteElement", parent IgesDataIgesEntity, {
the_topology: i32,
the_nodes: Handle<HArray1<Handle<IgesAppliNode>>>,
the_name: Handle<HAsciiString>,
}
}

impl IgesAppliFiniteElement {
    iges_methods! {
    get topology = the_topology: i32;
    get name = the_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// FiniteElement
    /// - aType    : Indicates the topology type
    /// - allNodes : List of Nodes defining the element
    /// - aName    : Element type name
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_type: i32,
        all_nodes: &Handle<HArray1<Handle<IgesAppliNode>>>,
        a_name: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_nodes, "IgesAppliFiniteElement::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliFiniteElement : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_topology = a_type;
            state.the_nodes = all_nodes.clone();
            state.the_name = a_name.clone();
        }
        self.init_type_and_form(136, 0);
        Ok(())
    }
    /// returns the number of nodes defining the element
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_nodes(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_nodes,
            "IgesAppliFiniteElement::nb_nodes",
        )
    }
    /// returns Node defining element entity
    /// raises exception if Index at most 0 or Index &gt; NbNodes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn node(&self, index: i32) -> Result<Handle<IgesAppliNode>, Failure> {
        iges_array1_item(
            &self.state.read().the_nodes,
            index,
            "IgesAppliFiniteElement::node",
        )
    }
}
