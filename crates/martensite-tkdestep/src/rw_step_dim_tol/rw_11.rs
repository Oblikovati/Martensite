#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwSurfaceProfileTolerance,
    "RwStepDimTolRwSurfaceProfileTolerance"
);
impl RwStepDimTolRwSurfaceProfileTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolSurfaceProfileTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "surface_profile_tolerance")? {
            return Ok(());
        }
        rw_block_b_0052!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
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
        ent: &Handle<StepDimTolSurfaceProfileTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolSurfaceProfileTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwSymmetryTolerance,
    "RwStepDimTolRwSymmetryTolerance"
);
impl RwStepDimTolRwSymmetryTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolSymmetryTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "symmetry_tolerance")? {
            return Ok(());
        }
        rw_block_b_0027!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
            a_geometric_tolerance_with_datum_reference_datum_system,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub5
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolSymmetryTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolSymmetryTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwToleranceZone, "RwStepDimTolRwToleranceZone");
impl RwStepDimTolRwToleranceZone {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolToleranceZone>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "tolerance_zone")? {
            return Ok(());
        }
        rw_block_b_0058!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        let mut an_items: Handle<HArray1<StepDimTolToleranceZoneTarget>> = Default::default();
        let mut an_ent: StepDimTolToleranceZoneTarget = Default::default();
        let mut nb_sub: i32 = Default::default();
        if data.read_sub_list(num, 5, "defining_tolerance", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            an_items = step_new_array1::<StepDimTolToleranceZoneTarget>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "tolerance_zone_target", ach, &mut an_ent)? {
                    step_array1_set(&an_items, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_form: Handle<StepDimTolToleranceZoneForm> = Default::default();
        data.read_entity(
            num,
            6,
            "form",
            ach,
            StepDimTolToleranceZoneForm::static_type(),
            &mut a_form,
        )?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &an_items,
            &a_form,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolToleranceZone>,
    ) -> Result<(), Failure> {
        rw_block_b_0165!(ent, sw);
        sw.open_sub()?;
        for i in 1..=ent.nb_defining_tolerances() {
            sw.send_entity(&ent.defining_tolerance_value(i)?.value())?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.form())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolToleranceZone>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_defining_tolerances();
        for i in 1..=nb {
            iter.add_item(&ent.defining_tolerance_value(i)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwToleranceZoneDefinition,
    "RwStepDimTolRwToleranceZoneDefinition"
);
impl RwStepDimTolRwToleranceZoneDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolToleranceZoneDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "tolerance_zone_definition")? {
            return Ok(());
        }
        let mut a_tolerance_zone: Handle<StepDimTolToleranceZone> = Default::default();
        data.read_entity(
            num,
            1,
            "zone",
            ach,
            StepDimTolToleranceZone::static_type(),
            &mut a_tolerance_zone,
        )?;
        rw_block_b_0190!(an_ent, an_items, nb_sub);
        if data.read_sub_list(num, 2, "boundaries", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            an_items = step_new_array1::<Handle<StepReprShapeAspect>>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity(
                    nb_sub,
                    i,
                    "shape_aspect",
                    ach,
                    StepReprShapeAspect::static_type(),
                    &mut an_ent,
                )? {
                    step_array1_set(&an_items, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_tolerance_zone, &an_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolToleranceZoneDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0164!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolToleranceZoneDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0151!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwToleranceZoneForm,
    "RwStepDimTolRwToleranceZoneForm"
);
impl RwStepDimTolRwToleranceZoneForm {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolToleranceZoneForm>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "tolerance_zone_form")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolToleranceZoneForm>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
