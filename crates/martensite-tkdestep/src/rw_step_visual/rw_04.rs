#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwCameraModelD3MultiClipping,
    "RwStepVisualRwCameraModelD3MultiClipping"
);
impl RwStepVisualRwCameraModelD3MultiClipping {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModelD3MultiClipping>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "camera_model_d3_multi_clipping")? {
            return Ok(());
        }
        rw_block_b_0087!(
            a_name,
            a_perspective_of_volume,
            a_view_reference_system,
            ach,
            data,
            num
        );
        rw_block_b_0197!(a_shape_clipping, an_ent, nb_sub);
        if data.read_sub_list(num, 4, "shape_clipping", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            a_shape_clipping = step_new_array1::<
                StepVisualCameraModelD3MultiClippingInterectionSelect,
            >(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "shape_clipping", ach, &mut an_ent)? {
                    step_array1_set(&a_shape_clipping, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(
            &a_name,
            &a_view_reference_system,
            &a_perspective_of_volume,
            &a_shape_clipping,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraModelD3MultiClipping>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.view_reference_system())?;
        sw.send_entity(&ent.perspective_of_volume())?;
        rw_block_b_0180!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraModelD3MultiClipping>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.view_reference_system().cast());
        iter.get_one_item(&ent.perspective_of_volume().cast());
        rw_block_b_0178!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwCameraModelD3MultiClippingIntersection,
    "RwStepVisualRwCameraModelD3MultiClippingIntersection"
);
impl RwStepVisualRwCameraModelD3MultiClippingIntersection {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModelD3MultiClippingIntersection>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "camera_model_d3_multi_clipping_intersection")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        rw_block_b_0197!(a_shape_clipping, an_ent, nb_sub);
        if data.read_sub_list(num, 2, "shape_clipping", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            a_shape_clipping = step_new_array1::<
                StepVisualCameraModelD3MultiClippingInterectionSelect,
            >(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "shape_clipping", ach, &mut an_ent)? {
                    step_array1_set(&a_shape_clipping, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_shape_clipping);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraModelD3MultiClippingIntersection>,
    ) -> Result<(), Failure> {
        rw_block_b_0137!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraModelD3MultiClippingIntersection>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0178!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwCameraModelD3MultiClippingUnion,
    "RwStepVisualRwCameraModelD3MultiClippingUnion"
);
impl RwStepVisualRwCameraModelD3MultiClippingUnion {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraModelD3MultiClippingUnion>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "camera_model_d3_multi_clipping_union")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingUnionSelect>> =
            Default::default();
        let mut an_ent: StepVisualCameraModelD3MultiClippingUnionSelect = Default::default();
        let mut nb_sub: i32 = Default::default();
        if data.read_sub_list(num, 2, "shape_clipping", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            a_shape_clipping =
                step_new_array1::<StepVisualCameraModelD3MultiClippingUnionSelect>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "shape_clipping", ach, &mut an_ent)? {
                    step_array1_set(&a_shape_clipping, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_shape_clipping);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraModelD3MultiClippingUnion>,
    ) -> Result<(), Failure> {
        rw_block_b_0137!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraModelD3MultiClippingUnion>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0178!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwCameraUsage, "RwStepVisualRwCameraUsage");
impl RwStepVisualRwCameraUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCameraUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "camera_usage")? {
            return Ok(());
        }
        let mut a_mapping_origin: Handle<StepReprRepresentationItem> = Default::default();
        data.read_entity(
            num,
            1,
            "mapping_origin",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut a_mapping_origin,
        )?;
        let mut a_mapped_representation: Handle<StepReprRepresentation> = Default::default();
        data.read_entity(
            num,
            2,
            "mapped_representation",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_mapped_representation,
        )?;
        ent.init(&a_mapping_origin, &a_mapped_representation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCameraUsage>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.mapping_origin())?;
        sw.send_entity(&ent.mapped_representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCameraUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.mapping_origin().cast());
        iter.get_one_item(&ent.mapped_representation().cast());
        Ok(())
    }
}
