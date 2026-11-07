//! IgesAppliNodalResults.
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
use crate::iges_basic::entity_shape::{
    iges_array1_length, iges_array1_lower, iges_array2_lower_col, iges_array2_lower_row,
    iges_array2_row_length, iges_array2_upper_row,
};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_array2_item};
use crate::iges_dimen::IgesDimenGeneralNote;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines NodalResults, Type `146`
/// Used to store the Analysis Data results per FEM Node
IgesAppliNodalResults, IgesAppliNodalResultsState, "IgesAppliNodalResults", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_sub_case_num: i32,
the_time: f64,
the_node_identifiers: Handle<HArray1<i32>>,
the_nodes: Handle<HArray1<Handle<IgesAppliNode>>>,
the_data: Handle<HArray2<f64>>,
}
}

impl IgesAppliNodalResults {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get sub_case_number = the_sub_case_num: i32;
    get time = the_time: f64;
    }
    /// This method is used to set the fields of the class
    /// NodalResults
    /// - aNote              : General Note that describes the
    ///   analysis case
    /// - aNumber            : Analysis Subcase number
    /// - aTime              : Analysis time
    /// - allNodeIdentifiers : Node identifiers for the nodes
    /// - allNodes           : List of FEM Node Entities
    /// - allData            : Values of the Finite Element analysis
    ///   result data
    ///   raises exception if Lengths of allNodeIdentifiers, allNodes and
    ///   allData (Cols) are not same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_number: i32,
        a_time: f64,
        all_node_identifiers: &Handle<HArray1<i32>>,
        all_nodes: &Handle<HArray1<Handle<IgesAppliNode>>>,
        all_data: &Handle<HArray2<f64>>,
    ) -> Result<(), Failure> {
        if (((((iges_array1_lower(all_nodes, "IgesAppliNodalResults::init")? != 1)
            || (iges_array1_lower(all_node_identifiers, "IgesAppliNodalResults::init")? != 1))
            || (iges_array1_length(all_nodes, "IgesAppliNodalResults::init")?
                != iges_array1_length(all_node_identifiers, "IgesAppliNodalResults::init")?))
            || (iges_array2_lower_col(all_data, "IgesAppliNodalResults::init")? != 1))
            || (iges_array2_lower_row(all_data, "IgesAppliNodalResults::init")? != 1))
            || (iges_array1_length(all_nodes, "IgesAppliNodalResults::init")?
                != iges_array2_upper_row(all_data, "IgesAppliNodalResults::init")?)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliNodalResults : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_sub_case_num = a_number;
            state.the_time = a_time;
            state.the_node_identifiers = all_node_identifiers.clone();
            state.the_nodes = all_nodes.clone();
            state.the_data = all_data.clone();
        }
        self.init_type_and_form(146, self.form_number());
        Ok(())
    }
    /// Changes the FormNumber (which indicates Type of Result)
    /// Error if not in range \[0-34\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if (form < 0) || (form > 34) {
            return Err(exceptions::out_of_range(
                "IgesAppliNodalResults : SetFormNumber",
            ));
        }
        self.init_type_and_form(146, form);
        Ok(())
    }
    /// returns number of real values in array V for a FEM node
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_data(&self) -> Result<i32, Failure> {
        iges_array2_row_length(
            &self.state.read().the_data,
            "IgesAppliNodalResults::nb_data",
        )
    }
    /// returns number of FEM nodes for which data is to be read.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_nodes(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_nodes,
            "IgesAppliNodalResults::nb_nodes",
        )
    }
    /// returns FEM node number identifier for the (Index)th node
    /// raises exception if Index at most 0 or Index &gt; NbNodes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn node_identifier(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_node_identifiers,
            index,
            "IgesAppliNodalResults::node_identifier",
        )
    }
    /// returns the node as specified by the Index
    /// raises exception if Index at most 0 or Index &gt; NbNodes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn node(&self, index: i32) -> Result<Handle<IgesAppliNode>, Failure> {
        iges_array1_item(
            &self.state.read().the_nodes,
            index,
            "IgesAppliNodalResults::node",
        )
    }
    /// returns the finite element analysis result value
    /// raises exception if (NodeNum at most 0 or NodeNum &gt; NbNodes()) or
    /// if (DataNum at most0 or DataNum &gt; NbData())
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn data(&self, node_num: i32, data_num: i32) -> Result<f64, Failure> {
        iges_array2_item(
            &self.state.read().the_data,
            node_num,
            data_num,
            "IgesAppliNodalResults::data",
        )
    }
}
