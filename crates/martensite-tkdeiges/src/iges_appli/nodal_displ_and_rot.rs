//! IgesAppliNodalDisplAndRot.
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
use crate::iges_basic::IgesBasicHArray1OfHArray1OfXyz;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::iges_dimen::IgesDimenGeneralNote;
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines NodalDisplAndRot, Type `138` Form `0`
/// Used to communicate finite element post processing
/// data.
IgesAppliNodalDisplAndRot, IgesAppliNodalDisplAndRotState, "IgesAppliNodalDisplAndRot", parent IgesDataIgesEntity, {
the_notes: Handle<HArray1<Handle<IgesDimenGeneralNote>>>,
the_node_identifiers: Handle<HArray1<i32>>,
the_nodes: Handle<HArray1<Handle<IgesAppliNode>>>,
the_trans_param: Handle<IgesBasicHArray1OfHArray1OfXyz>,
the_rot_param: Handle<IgesBasicHArray1OfHArray1OfXyz>,
}
}

impl IgesAppliNodalDisplAndRot {
    /// This method is used to set the fields of the class
    /// NodalDisplAndRot
    /// - allNotes       : Used to store the general note describing
    ///   the analysis cases
    /// - allIdentifiers : Used to store the node number
    ///   identifier for the nodes
    /// - allNodes       : Used to store the nodes
    /// - allRotParams   : Used to store the rotation for the nodes
    /// - allTransParams : Used to store the incremental
    ///   displacements for the nodes
    ///   raises exception if Lengths of allIdentifiers, allNodes,
    ///   allRotParams, and allTransParams are not same
    ///   or if length of allNotes and size of each element of allRotParams
    ///   and allTransParam are not same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_notes: &Handle<HArray1<Handle<IgesDimenGeneralNote>>>,
        all_identifiers: &Handle<HArray1<i32>>,
        all_nodes: &Handle<HArray1<Handle<IgesAppliNode>>>,
        all_rot_params: &Handle<IgesBasicHArray1OfHArray1OfXyz>,
        all_trans_params: &Handle<IgesBasicHArray1OfHArray1OfXyz>,
    ) -> Result<(), Failure> {
        if (((iges_array1_lower(all_nodes, "IgesAppliNodalDisplAndRot::init")? != 1)
            || ((iges_array1_lower(all_identifiers, "IgesAppliNodalDisplAndRot::init")? != 1)
                || (iges_array1_length(all_identifiers, "IgesAppliNodalDisplAndRot::init")?
                    != iges_array1_length(all_nodes, "IgesAppliNodalDisplAndRot::init")?)))
            || (((iges_live(all_trans_params, "IgesAppliNodalDisplAndRot::init")?.lower()) != 1)
                || ((iges_live(all_trans_params, "IgesAppliNodalDisplAndRot::init")?.length())
                    != iges_array1_length(all_nodes, "IgesAppliNodalDisplAndRot::init")?)))
            || (((iges_live(all_rot_params, "IgesAppliNodalDisplAndRot::init")?.lower()) != 1)
                || ((iges_live(all_rot_params, "IgesAppliNodalDisplAndRot::init")?.length())
                    != iges_array1_length(all_nodes, "IgesAppliNodalDisplAndRot::init")?))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliNodalDisplAndRot : Init(Lengths of arrays inconsistent)",
            ));
        }
        for i in 1..=iges_array1_length(all_nodes, "IgesAppliNodalDisplAndRot::init")? {
            let mut temp1: Handle<HArray1<Xyz>> =
                iges_live(all_trans_params, "IgesAppliNodalDisplAndRot::init")?.value(i)?;
            let mut temp2: Handle<HArray1<Xyz>> =
                iges_live(all_rot_params, "IgesAppliNodalDisplAndRot::init")?.value(i)?;
            if ((iges_array1_lower(&temp1, "IgesAppliNodalDisplAndRot::init")? != 1)
                || (iges_array1_length(&temp1, "IgesAppliNodalDisplAndRot::init")?
                    != iges_array1_length(all_notes, "IgesAppliNodalDisplAndRot::init")?))
                || ((iges_array1_lower(&temp2, "IgesAppliNodalDisplAndRot::init")? != 1)
                    || (iges_array1_length(&temp2, "IgesAppliNodalDisplAndRot::init")?
                        != iges_array1_length(all_notes, "IgesAppliNodalDisplAndRot::init")?))
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesAppliNodalDisplAndRot: Init(No. of Param per Node != Nbcases)",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_notes = all_notes.clone();
            state.the_nodes = all_nodes.clone();
            state.the_node_identifiers = all_identifiers.clone();
            state.the_trans_param = all_trans_params.clone();
            state.the_rot_param = all_rot_params.clone();
        }
        self.init_type_and_form(138, 0);
        Ok(())
    }
    /// returns the number of analysis cases
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_cases(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_notes,
            "IgesAppliNodalDisplAndRot::nb_cases",
        )
    }
    /// returns the number of nodes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_nodes(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_nodes,
            "IgesAppliNodalDisplAndRot::nb_nodes",
        )
    }
    /// returns the General Note that describes the Index analysis case
    /// raises exception if Index at most 0 or Index &gt; NbCases
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn note(&self, index: i32) -> Result<Handle<IgesDimenGeneralNote>, Failure> {
        iges_array1_item(
            &self.state.read().the_notes,
            index,
            "IgesAppliNodalDisplAndRot::note",
        )
    }
    /// returns the node identifier as specified by the Index
    /// raises exception if Index at most 0 or Index &gt; NbNodes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn node_identifier(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_node_identifiers,
            index,
            "IgesAppliNodalDisplAndRot::node_identifier",
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
            "IgesAppliNodalDisplAndRot::node",
        )
    }
    /// returns the Translational Parameters for the particular Index
    /// Exception raised if NodeNum at most 0 or NodeNum &gt; NbNodes()
    /// or CaseNum at most 0 or CaseNum &gt; NbCases()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn translation_parameter(&self, node_num: i32, case_num: i32) -> Result<Xyz, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_trans_param,
                "IgesAppliNodalDisplAndRot::translation_parameter",
            )?
            .value(node_num)?,
            case_num,
            "IgesAppliNodalDisplAndRot::translation_parameter",
        )
    }
    /// returns the Rotational Parameters for Index
    /// Exception raised if NodeNum at most 0 or NodeNum &gt; NbNodes()
    /// or CaseNum at most 0 or CaseNum &gt; NbCases()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn rotational_parameter(&self, node_num: i32, case_num: i32) -> Result<Xyz, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_rot_param,
                "IgesAppliNodalDisplAndRot::rotational_parameter",
            )?
            .value(node_num)?,
            case_num,
            "IgesAppliNodalDisplAndRot::rotational_parameter",
        )
    }
}
