#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwShapeRepresentation,
    "RwStepShapeRwShapeRepresentation"
);
impl RwStepShapeRwShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "shape_representation")? {
            return Ok(());
        }
        rw_block_0039!(
            a_context_of_items,
            a_items,
            a_name,
            ach,
            anent2,
            data,
            ent,
            i2,
            nb2,
            nsub2,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwShapeRepresentationWithParameters,
    "RwStepShapeRwShapeRepresentationWithParameters"
);
impl RwStepShapeRwShapeRepresentationWithParameters {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeShapeRepresentationWithParameters>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "shape_representation_with_parameters")? {
            return Ok(());
        }
        rw_block_0052!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeShapeRepresentationWithParameters>,
    ) -> Result<(), Failure> {
        rw_block_0099!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeShapeRepresentationWithParameters>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwShellBasedSurfaceModel,
    "RwStepShapeRwShellBasedSurfaceModel"
);
impl RwStepShapeRwShellBasedSurfaceModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeShellBasedSurfaceModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "shell_based_surface_model")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_sbsm_boundary: Handle<HArray1<StepShapeShell>> = Default::default();
        let mut a_sbsm_boundary_item: StepShapeShell = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "sbsm_boundary", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_sbsm_boundary = step_new_array1::<StepShapeShell>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity_select(
                    nsub2,
                    i2,
                    "sbsm_boundary",
                    ach,
                    &mut a_sbsm_boundary_item,
                )? {
                    step_array1_set(
                        &a_sbsm_boundary,
                        i2,
                        a_sbsm_boundary_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        }
        ent.init(&a_name, &a_sbsm_boundary);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeShellBasedSurfaceModel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_sbsm_boundary()? {
            sw.send_entity(&ent.sbsm_boundary_value(i2)?.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeShellBasedSurfaceModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_sbsm_boundary()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.sbsm_boundary_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwSolidModel, "RwStepShapeRwSolidModel");
impl RwStepShapeRwSolidModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSolidModel>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "solid_model")? {
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
        ent: &Handle<StepShapeSolidModel>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwSolidReplica, "RwStepShapeRwSolidReplica");
impl RwStepShapeRwSolidReplica {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSolidReplica>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "solid_replica")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_parent_solid: Handle<StepShapeSolidModel> = Default::default();
        data.read_entity(
            num,
            2,
            "parent_solid",
            ach,
            StepShapeSolidModel::static_type(),
            &mut a_parent_solid,
        )?;
        let mut a_transformation: Handle<StepGeomCartesianTransformationOperator3d> =
            Default::default();
        data.read_entity(
            num,
            3,
            "transformation",
            ach,
            StepGeomCartesianTransformationOperator3d::static_type(),
            &mut a_transformation,
        )?;
        ent.init(&a_name, &a_parent_solid, &a_transformation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSolidReplica>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.parent_solid())?;
        sw.send_entity(&ent.transformation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSolidReplica>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_solid().cast());
        iter.get_one_item(&ent.transformation().cast());
        Ok(())
    }
}
