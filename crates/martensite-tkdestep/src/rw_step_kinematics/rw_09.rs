#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwMechanismRepresentation,
    "RwStepKinematicsRwMechanismRepresentation"
);
impl RwStepKinematicsRwMechanismRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsMechanismRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "mechanism_representation")? {
            return Ok(());
        }
        rw_block_b_0065!(
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
            the_num
        );
        let mut a_represented_topology: StepKinematicsKinematicTopologyRepresentationSelect =
            Default::default();
        the_data.read_entity_select(
            the_num,
            4,
            "represented_topology",
            the_arch,
            &mut a_represented_topology,
        )?;
        the_ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
            &a_represented_topology,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsMechanismRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0105!(i1, the_ent, the_sw, var0);
        the_sw.send_entity(&the_ent.represented_topology().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsMechanismRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0204!(i1, iter, the_ent, var0);
        iter.add_item(&the_ent.represented_topology().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwMechanismStateRepresentation,
    "RwStepKinematicsRwMechanismStateRepresentation"
);
impl RwStepKinematicsRwMechanismStateRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsMechanismStateRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "mechanism_state_representation")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(the_num, 1, "name", the_arch, &mut a_name)?;
        let mut a_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
        let mut an_ent: Handle<StepReprRepresentationItem> = Default::default();
        let mut nsub: i32 = Default::default();
        if the_data.read_sub_list(the_num, 2, "items", the_arch, &mut nsub, false, 0, 0)? {
            let nb: i32 = the_data.nb_params(nsub)?;
            a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb)?;
            for i in 1..=nb {
                if the_data.read_entity(
                    nsub,
                    i,
                    "representation_item",
                    the_arch,
                    StepReprRepresentationItem::static_type(),
                    &mut an_ent,
                )? {
                    step_array1_set(&a_items, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "context_of_items",
            the_arch,
            StepReprRepresentationContext::static_type(),
            &mut a_context_of_items,
        )?;
        let mut a_represented_mechanism: Handle<StepKinematicsMechanismRepresentation> =
            Default::default();
        the_data.read_entity(
            the_num,
            4,
            "represented_mechanism",
            the_arch,
            StepKinematicsMechanismRepresentation::static_type(),
            &mut a_represented_mechanism,
        )?;
        the_ent.init(
            &a_name,
            &a_items,
            &a_context_of_items,
            &a_represented_mechanism,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsMechanismStateRepresentation>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.open_sub()?;
        for i in 1..=the_ent.nb_items() {
            the_sw.send_entity(&the_ent.items_value(i)?)?;
        }
        the_sw.close_sub()?;
        the_sw.send_entity(&the_ent.context_of_items())?;
        the_sw.send_entity(&the_ent.mechanism())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsMechanismStateRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem: i32 = the_ent.nb_items();
        for i in 1..=nb_elem {
            iter.get_one_item(&the_ent.items_value(i)?.cast());
        }
        iter.get_one_item(&the_ent.context_of_items().cast());
        iter.get_one_item(&the_ent.mechanism().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwOrientedJoint,
    "RwStepKinematicsRwOrientedJoint"
);
impl RwStepKinematicsRwOrientedJoint {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsOrientedJoint>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 5, the_arch, "oriented_joint")? {
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
        let mut a_oriented_edge_edge_element: Handle<StepShapeEdge> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "oriented_edge.edge_element",
            the_arch,
            StepShapeEdge::static_type(),
            &mut a_oriented_edge_edge_element,
        )?;
        let mut a_oriented_edge_orientation: bool = Default::default();
        the_data.read_boolean(
            the_num,
            5,
            "oriented_edge.orientation",
            the_arch,
            &mut a_oriented_edge_orientation,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_oriented_edge_edge_element,
            a_oriented_edge_orientation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsOrientedJoint>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.edge_start())?;
        the_sw.send_entity(&the_ent.edge_end())?;
        the_sw.send_entity(&the_ent.edge_element())?;
        the_sw.send_boolean(the_ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsOrientedJoint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.parent.edge_start().cast());
        iter.add_item(&the_ent.parent.parent.edge_end().cast());
        iter.add_item(&the_ent.parent.edge_element().cast());
        Ok(())
    }
}
