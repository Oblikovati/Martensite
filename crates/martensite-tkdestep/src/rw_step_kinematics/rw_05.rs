#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwKinematicJoint,
    "RwStepKinematicsRwKinematicJoint"
);
impl RwStepKinematicsRwKinematicJoint {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicJoint>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "kinematic_joint")? {
            return Ok(());
        }
        rw_block_b_0088!(
            a_edge_edge_end,
            a_edge_edge_start,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        the_ent.init(
            &a_representation_item_name,
            &a_edge_edge_start,
            &a_edge_edge_end,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicJoint>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.edge_start())?;
        the_sw.send_entity(&the_ent.edge_end())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicJoint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.edge_start().cast());
        iter.add_item(&the_ent.parent.edge_end().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwKinematicLink,
    "RwStepKinematicsRwKinematicLink"
);
impl RwStepKinematicsRwKinematicLink {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicLink>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 1, the_arch, "kinematic_link")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_arch,
            &mut a_representation_item_name,
        )?;
        the_ent.init(&a_representation_item_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicLink>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepKinematicsKinematicLink>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwKinematicLinkRepresentationAssociation,
    "RwStepKinematicsRwKinematicLinkRepresentationAssociation"
);
impl RwStepKinematicsRwKinematicLinkRepresentationAssociation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicLinkRepresentationAssociation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            4,
            the_arch,
            "kinematic_link_representation_association",
        )? {
            return Ok(());
        }
        let mut a_representation_relationship_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_relationship.name",
            the_arch,
            &mut a_representation_relationship_name,
        )?;
        let mut a_representation_relationship_description: Handle<HAsciiString> =
            Default::default();
        if the_data.is_param_defined(the_num, 2)? {
            the_data.read_string(
                the_num,
                2,
                "representation_relationship.description",
                the_arch,
                &mut a_representation_relationship_description,
            )?;
        } else {
            a_representation_relationship_description = Default::default();
        }
        let mut a_representation_relationship_rep1: StepReprRepresentationOrRepresentationReference = Default::default();
        the_data.read_entity_select(
            the_num,
            3,
            "representation_relationship.rep1",
            the_arch,
            &mut a_representation_relationship_rep1,
        )?;
        let mut a_representation_relationship_rep2: StepReprRepresentationOrRepresentationReference = Default::default();
        the_data.read_entity_select(
            the_num,
            4,
            "representation_relationship.rep2",
            the_arch,
            &mut a_representation_relationship_rep2,
        )?;
        if a_representation_relationship_rep1.case_number() != 1
            || a_representation_relationship_rep1.case_number() != 1
        {
            return Ok(());
        }
        the_ent.init(
            &a_representation_relationship_name,
            &a_representation_relationship_description,
            &a_representation_relationship_rep1.representation(),
            &a_representation_relationship_rep2.representation(),
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicLinkRepresentationAssociation>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        if the_ent.has_description() {
            the_sw.send_entity(&the_ent.description())?;
        } else {
            the_sw.send_undef()?;
        }
        the_sw.send_entity(&the_ent.rep1())?;
        the_sw.send_entity(&the_ent.rep2())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicLinkRepresentationAssociation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.rep1().cast());
        iter.add_item(&the_ent.parent.rep2().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwKinematicPropertyMechanismRepresentation,
    "RwStepKinematicsRwKinematicPropertyMechanismRepresentation"
);
impl RwStepKinematicsRwKinematicPropertyMechanismRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicPropertyMechanismRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            3,
            the_arch,
            "kinematic_property_mechanism_representation",
        )? {
            return Ok(());
        }
        let mut a_property_definition_representation_definition: StepReprRepresentedDefinition =
            Default::default();
        the_data.read_entity_select(
            the_num,
            1,
            "property_definition_representation.definition",
            the_arch,
            &mut a_property_definition_representation_definition,
        )?;
        let mut a_property_definition_representation_used_representation: Handle<
            StepReprRepresentation,
        > = Default::default();
        the_data.read_entity(
            the_num,
            2,
            "property_definition_representation.used_representation",
            the_arch,
            StepReprRepresentation::static_type(),
            &mut a_property_definition_representation_used_representation,
        )?;
        let mut a_base: Handle<StepKinematicsKinematicLinkRepresentation> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "base",
            the_arch,
            StepKinematicsKinematicLinkRepresentation::static_type(),
            &mut a_base,
        )?;
        the_ent.init(
            &a_property_definition_representation_definition,
            &a_property_definition_representation_used_representation,
            &a_base,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicPropertyMechanismRepresentation>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.definition().value())?;
        the_sw.send_entity(&the_ent.used_representation())?;
        the_sw.send_entity(&the_ent.base())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicPropertyMechanismRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.parent.definition().value());
        iter.add_item(&the_ent.parent.parent.used_representation().cast());
        iter.add_item(&the_ent.base().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwKinematicTopologyDirectedStructure,
    "RwStepKinematicsRwKinematicTopologyDirectedStructure"
);
impl RwStepKinematicsRwKinematicTopologyDirectedStructure {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicTopologyDirectedStructure>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            4,
            the_arch,
            "kinematic_topology_directed_structure",
        )? {
            return Ok(());
        }
        rw_block_b_0035!(
            a_parent,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicTopologyDirectedStructure>,
    ) -> Result<(), Failure> {
        rw_block_b_0092!(i1, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicTopologyDirectedStructure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0171!(i1, iter, the_ent, var0);
        Ok(())
    }
}
