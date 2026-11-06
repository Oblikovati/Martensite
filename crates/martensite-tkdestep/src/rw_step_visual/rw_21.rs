#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTessellatedPointSet,
    "RwStepVisualRwTessellatedPointSet"
);
impl RwStepVisualRwTessellatedPointSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedPointSet>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_check, "tessellated_point_set")? {
            return Ok(());
        }
        rw_block_b_0138!(
            a_coordinates,
            a_representation_item_name,
            the_check,
            the_data,
            the_num
        );
        let mut a_point_list: Handle<HArray1<i32>> = Default::default();
        let mut sub3: i32 = 0;
        if the_data.read_sub_list(the_num, 3, "point_list", the_check, &mut sub3, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub3)?;
            a_point_list = step_new_array1::<i32>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: i32 = Default::default();
                the_data.read_integer(num2, i0, "integer", the_check, &mut an_it0)?;
                step_array1_set(&a_point_list, i0, an_it0, "ReadStep")?;
            }
        }
        the_ent.init(&a_representation_item_name, &a_coordinates, &a_point_list);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedPointSet>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.coordinates())?;
        the_sw.open_sub()?;
        for i2 in 1..=step_live(&the_ent.point_list(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: i32 = step_array1_item(&the_ent.point_list(), i2, "WriteStep")?;
            the_sw.send_integer(var0)?;
        }
        the_sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedPointSet>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.coordinates().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedShapeRepresentation,
    "RwStepVisualRwTessellatedShapeRepresentation"
);
impl RwStepVisualRwTessellatedShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_check, "tessellated_shape_representation")? {
            return Ok(());
        }
        rw_block_b_0064!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_check,
            the_data,
            the_num
        );
        the_ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0105!(i1, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentation>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if !the_ent.parent.parent.items().is_null() {
            for i1 in 1..=step_live(&the_ent.parent.parent.items(), "Share")?
                .array1()
                .length()
            {
                let var0: Handle<StepReprRepresentationItem> =
                    step_array1_item(&the_ent.parent.parent.items(), i1, "Share")?;
                the_iter.add_item(&var0.cast());
            }
        }
        the_iter.add_item(&the_ent.parent.parent.context_of_items().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwTessellatedShapeRepresentationWithAccuracyParameters,
    "RwStepVisualRwTessellatedShapeRepresentationWithAccuracyParameters"
);
impl RwStepVisualRwTessellatedShapeRepresentationWithAccuracyParameters {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentationWithAccuracyParameters>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            4,
            the_check,
            "tessellated_shape_representation_with_accuracy_parameters",
        )? {
            return Ok(());
        }
        rw_block_b_0064!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_check,
            the_data,
            the_num
        );
        let mut a_tessellation_accuracy_parameters: Handle<HArray1<f64>> = Default::default();
        let mut sub4: i32 = 0;
        if the_data.read_sub_list(
            the_num,
            4,
            "tessellation_accuracy_parameters",
            the_check,
            &mut sub4,
            false,
            0,
            0,
        )? {
            let nb0: i32 = the_data.nb_params(sub4)?;
            a_tessellation_accuracy_parameters = step_new_array1::<f64>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: f64 = Default::default();
                the_data.read_real(
                    num2,
                    i0,
                    "tessellation_accuracy_parameter_item",
                    the_check,
                    &mut an_it0,
                )?;
                step_array1_set(&a_tessellation_accuracy_parameters, i0, an_it0, "ReadStep")?;
            }
        }
        the_ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
            &a_tessellation_accuracy_parameters,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentationWithAccuracyParameters>,
    ) -> Result<(), Failure> {
        rw_block_b_0105!(i1, the_ent, the_sw, var0);
        the_sw.open_sub()?;
        for i3 in 1..=step_live(&the_ent.tessellation_accuracy_parameters(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: f64 =
                step_array1_item(&the_ent.tessellation_accuracy_parameters(), i3, "WriteStep")?;
            the_sw.send_real(var0)?;
        }
        the_sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedShapeRepresentationWithAccuracyParameters>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&the_ent.parent.parent.parent.items(), "Share")?
            .array1()
            .length()
        {
            let var0: Handle<StepReprRepresentationItem> =
                step_array1_item(&the_ent.parent.parent.parent.items(), i1, "Share")?;
            the_iter.add_item(&var0.cast());
        }
        the_iter.add_item(&the_ent.parent.parent.parent.context_of_items().cast());
        Ok(())
    }
}
