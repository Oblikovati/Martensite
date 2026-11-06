#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation,
    "RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation"
);
impl RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "mechanical_design_geometric_presentation_representation",
        )? {
            return Ok(());
        }
        rw_block_b_0109!(a_items, a_name, ach, anent2, data, nsub2, num);
        if data.read_sub_list(num, 2, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            if nb2 > 0 {
                a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb2)?;
            }
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "representation_item",
                    ach,
                    StepReprRepresentationItem::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_items, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_b_0183!(a_context_of_items, a_items, a_name, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0120!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwOverRidingStyledItem,
    "RwStepVisualRwOverRidingStyledItem"
);
impl RwStepVisualRwOverRidingStyledItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualOverRidingStyledItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "over_riding_styled_item")? {
            return Ok(());
        }
        rw_block_b_0039!(
            a_item,
            a_name,
            a_over_ridden_style,
            a_styles,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        ent.init(&a_name, &a_styles, &a_item, &a_over_ridden_style);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualOverRidingStyledItem>,
    ) -> Result<(), Failure> {
        rw_block_b_0098!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualOverRidingStyledItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0146!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwPlanarBox, "RwStepVisualRwPlanarBox");
impl RwStepVisualRwPlanarBox {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPlanarBox>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "planar_box")? {
            return Ok(());
        }
        rw_block_b_0097!(a_name, a_size_in_x, a_size_in_y, ach, data, num);
        let mut a_placement: StepGeomAxis2Placement = Default::default();
        data.read_entity_select(num, 4, "placement", ach, &mut a_placement)?;
        ent.init(&a_name, a_size_in_x, a_size_in_y, &a_placement);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPlanarBox>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_real(ent.size_in_x())?;
        sw.send_real(ent.size_in_y())?;
        sw.send_entity(&ent.placement().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPlanarBox>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.placement().value());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwPlanarExtent, "RwStepVisualRwPlanarExtent");
impl RwStepVisualRwPlanarExtent {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPlanarExtent>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "planar_extent")? {
            return Ok(());
        }
        rw_block_b_0097!(a_name, a_size_in_x, a_size_in_y, ach, data, num);
        ent.init(&a_name, a_size_in_x, a_size_in_y);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPlanarExtent>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_real(ent.size_in_x())?;
        sw.send_real(ent.size_in_y())?;
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwPointStyle, "RwStepVisualRwPointStyle");
impl RwStepVisualRwPointStyle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualPointStyle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "point_style")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_marker: StepVisualMarkerSelect = Default::default();
        data.read_entity_select(num, 2, "marker", ach, &mut a_marker)?;
        let mut a_marker_size: StepBasicSizeSelect = Default::default();
        data.read_entity_select(num, 3, "marker_size", ach, &mut a_marker_size)?;
        let mut a_marker_colour: Handle<StepVisualColour> = Default::default();
        data.read_entity(
            num,
            4,
            "marker_colour",
            ach,
            StepVisualColour::static_type(),
            &mut a_marker_colour,
        )?;
        ent.init(&a_name, &a_marker, &a_marker_size, &a_marker_colour);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualPointStyle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.marker().value())?;
        sw.send_entity(&ent.marker_size().value())?;
        sw.send_entity(&ent.marker_colour())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualPointStyle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if ent.marker().case_number() > 0 {
            iter.get_one_item(&ent.marker().value());
        }
        iter.get_one_item(&ent.marker_colour().cast());
        Ok(())
    }
}
