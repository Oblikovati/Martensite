#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwCurve3dElementProperty,
    "RwStepFEARwCurve3dElementProperty"
);
impl RwStepFEARwCurve3dElementProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurve3dElementProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "curve3d_element_property")? {
            return Ok(());
        }
        rw_block_b_0148!(a_description, a_property_id, ach, data, num);
        let mut a_interval_definitions: Handle<HArray1<Handle<StepFEACurveElementInterval>>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "interval_definitions", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_interval_definitions =
                step_new_array1::<Handle<StepFEACurveElementInterval>>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepFEACurveElementInterval> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "curve_element_interval",
                    ach,
                    StepFEACurveElementInterval::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_interval_definitions, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_end_offsets: Handle<HArray1<Handle<StepFEACurveElementEndOffset>>> =
            Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "end_offsets", ach, &mut sub4, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub4)?;
            a_end_offsets = step_new_array1::<Handle<StepFEACurveElementEndOffset>>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepFEACurveElementEndOffset> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "curve_element_end_offset",
                    ach,
                    StepFEACurveElementEndOffset::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_end_offsets, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_end_releases: Handle<HArray1<Handle<StepFEACurveElementEndRelease>>> =
            Default::default();
        let mut sub5: i32 = 0;
        if data.read_sub_list(num, 5, "end_releases", ach, &mut sub5, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub5)?;
            a_end_releases = step_new_array1::<Handle<StepFEACurveElementEndRelease>>(1, nb0)?;
            let num2: i32 = sub5;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepFEACurveElementEndRelease> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "curve_element_end_release",
                    ach,
                    StepFEACurveElementEndRelease::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_end_releases, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_property_id,
            &a_description,
            &a_interval_definitions,
            &a_end_offsets,
            &a_end_releases,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurve3dElementProperty>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.property_id())?;
        sw.send_entity(&ent.description())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.interval_definitions(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepFEACurveElementInterval> =
                step_array1_item(&ent.interval_definitions(), i2, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i3 in 1..=step_live(&ent.end_offsets(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepFEACurveElementEndOffset> =
                step_array1_item(&ent.end_offsets(), i3, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i4 in 1..=step_live(&ent.end_releases(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepFEACurveElementEndRelease> =
                step_array1_item(&ent.end_releases(), i4, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurve3dElementProperty>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.interval_definitions(), "Share")?
            .array1()
            .length()
        {
            let var0: Handle<StepFEACurveElementInterval> =
                step_array1_item(&ent.interval_definitions(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        for i2 in 1..=step_live(&ent.end_offsets(), "Share")?.array1().length() {
            let var0: Handle<StepFEACurveElementEndOffset> =
                step_array1_item(&ent.end_offsets(), i2, "Share")?;
            iter.add_item(&var0.cast());
        }
        for i3 in 1..=step_live(&ent.end_releases(), "Share")?.array1().length() {
            let var0: Handle<StepFEACurveElementEndRelease> =
                step_array1_item(&ent.end_releases(), i3, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwCurve3dElementRepresentation,
    "RwStepFEARwCurve3dElementRepresentation"
);
impl RwStepFEARwCurve3dElementRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurve3dElementRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "curve3d_element_representation")? {
            return Ok(());
        }
        rw_block_b_0025!(
            a_element_representation_node_list,
            a_model_ref,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num2,
            sub2,
            sub4
        );
        let mut a_element_descriptor: Handle<StepElementCurve3dElementDescriptor> =
            Default::default();
        data.read_entity(
            num,
            6,
            "element_descriptor",
            ach,
            StepElementCurve3dElementDescriptor::static_type(),
            &mut a_element_descriptor,
        )?;
        let mut a_property: Handle<StepFEACurve3dElementProperty> = Default::default();
        data.read_entity(
            num,
            7,
            "property",
            ach,
            StepFEACurve3dElementProperty::static_type(),
            &mut a_property,
        )?;
        rw_block_b_0179!(
            a_element_descriptor,
            a_element_representation_node_list,
            a_material,
            a_model_ref,
            a_property,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurve3dElementRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0028!(ent, i1, i3, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurve3dElementRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0067!(ent, i1, i3, iter, var0);
        Ok(())
    }
}
