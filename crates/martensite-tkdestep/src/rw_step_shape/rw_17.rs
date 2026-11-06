#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwSeamEdge, "RwStepShapeRwSeamEdge");
impl RwStepShapeRwSeamEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSeamEdge>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "seam_edge")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let a_edge_edge_start: Handle<StepShapeVertex> = Default::default();
        data.check_derived(num, 2, "edge.edge_start", ach, false)?;
        let a_edge_edge_end: Handle<StepShapeVertex> = Default::default();
        data.check_derived(num, 3, "edge.edge_end", ach, false)?;
        let mut a_oriented_edge_edge_element: Handle<StepShapeEdge> = Default::default();
        data.read_entity(
            num,
            4,
            "oriented_edge.edge_element",
            ach,
            StepShapeEdge::static_type(),
            &mut a_oriented_edge_edge_element,
        )?;
        let mut a_oriented_edge_orientation: bool = Default::default();
        data.read_boolean(
            num,
            5,
            "oriented_edge.orientation",
            ach,
            &mut a_oriented_edge_orientation,
        )?;
        let mut a_pcurve_reference: Handle<StepGeomPcurve> = Default::default();
        data.read_entity(
            num,
            6,
            "pcurve_reference",
            ach,
            StepGeomPcurve::static_type(),
            &mut a_pcurve_reference,
        )?;
        ent.init(
            &a_representation_item_name,
            &a_oriented_edge_edge_element,
            a_oriented_edge_orientation,
            &a_pcurve_reference,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSeamEdge>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.parent.name())?;
        sw.send_entity(&ent.parent.parent.edge_start())?;
        sw.send_entity(&ent.parent.parent.edge_end())?;
        sw.send_entity(&ent.parent.edge_element())?;
        sw.send_boolean(ent.parent.orientation())?;
        sw.send_entity(&ent.pcurve_reference())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSeamEdge>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.edge_start().cast());
        iter.add_item(&ent.parent.parent.edge_end().cast());
        iter.add_item(&ent.parent.edge_element().cast());
        iter.add_item(&ent.pcurve_reference().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwShapeDefinitionRepresentation,
    "RwStepShapeRwShapeDefinitionRepresentation"
);
impl RwStepShapeRwShapeDefinitionRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeShapeDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "shape_definition_representation")? {
            return Ok(());
        }
        rw_block_0142!(
            a_property_definition_representation_definition,
            a_property_definition_representation_used_representation,
            ach,
            data,
            num
        );
        ent.init(
            &a_property_definition_representation_definition,
            &a_property_definition_representation_used_representation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeShapeDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.definition().value())?;
        sw.send_entity(&ent.parent.used_representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeShapeDefinitionRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.definition().value());
        iter.add_item(&ent.parent.used_representation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwShapeDimensionRepresentation,
    "RwStepShapeRwShapeDimensionRepresentation"
);
impl RwStepShapeRwShapeDimensionRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeShapeDimensionRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "shape_dimension_representation")? {
            return Ok(());
        }
        rw_block_0185!(
            a_representation_items,
            a_representation_name,
            ach,
            data,
            num
        );
        let mut an_items: Handle<HArray1<StepShapeShapeDimensionRepresentationItem>> =
            Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "representation.items", ach, &mut sub2, false, 0, 0)? {
            let num2: i32 = sub2;
            let nb0: i32 = data.nb_params(num2)?;
            let mut an_it0: Handle<StepReprRepresentationItem> = Default::default();
            let mut an_it0_ap242: StepShapeShapeDimensionRepresentationItem = Default::default();
            if data.read_entity(
                num2,
                1,
                "representation.items",
                ach,
                StepReprRepresentationItem::static_type(),
                &mut an_it0,
            )? {
                a_representation_items =
                    step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb0)?;
                for i0 in 1..=nb0 {
                    data.read_entity(
                        num2,
                        i0,
                        "representation.items",
                        ach,
                        StepReprRepresentationItem::static_type(),
                        &mut an_it0,
                    )?;
                    step_array1_set(&a_representation_items, i0, an_it0.clone(), "ReadStep")?;
                }
            } else {
                an_items = step_new_array1::<StepShapeShapeDimensionRepresentationItem>(1, nb0)?;
                for i0 in 1..=nb0 {
                    data.read_entity_select(
                        num2,
                        i0,
                        "representation.items",
                        ach,
                        &mut an_it0_ap242,
                    )?;
                    step_array1_set(&an_items, i0, an_it0_ap242.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_representation_context_of_items: Handle<StepReprRepresentationContext> =
            Default::default();
        data.read_entity(
            num,
            3,
            "representation.context_of_items",
            ach,
            StepReprRepresentationContext::static_type(),
            &mut a_representation_context_of_items,
        )?;
        if an_items.is_null() {
            ent.init(
                &a_representation_name,
                &a_representation_items,
                &a_representation_context_of_items,
            );
        } else {
            ent.init_fields(
                &a_representation_name,
                &an_items,
                &a_representation_context_of_items,
            );
        }
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeShapeDimensionRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.open_sub()?;
        if ent.items_ap242().is_null() {
            for i1 in 1..=ent.parent.parent.nb_items() {
                let var0: Handle<StepReprRepresentationItem> =
                    step_array1_item(&ent.parent.parent.items(), i1, "WriteStep")?;
                sw.send_entity(&var0)?;
            }
        } else {
            for i1 in 1..=step_live(&ent.items_ap242(), "WriteStep")?
                .array1()
                .length()
            {
                let var0: StepShapeShapeDimensionRepresentationItem =
                    step_array1_item(&ent.items_ap242(), i1, "WriteStep")?;
                sw.send_entity(&var0.value())?;
            }
        }
        sw.close_sub()?;
        sw.send_entity(&ent.parent.parent.context_of_items())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeShapeDimensionRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if ent.items_ap242().is_null() {
            for i1 in 1..=ent.parent.parent.nb_items() {
                let var0: Handle<StepReprRepresentationItem> =
                    step_array1_item(&ent.parent.parent.items(), i1, "Share")?;
                iter.add_item(&var0.cast());
            }
        } else {
            for i1 in 1..=step_live(&ent.items_ap242(), "Share")?.array1().length() {
                let var0: StepShapeShapeDimensionRepresentationItem =
                    step_array1_item(&ent.items_ap242(), i1, "Share")?;
                iter.add_item(&var0.value());
            }
        }
        iter.add_item(&ent.parent.parent.context_of_items().cast());
        Ok(())
    }
}
