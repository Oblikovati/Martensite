#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwGeometricSet, "RwStepShapeRwGeometricSet");
impl RwStepShapeRwGeometricSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeGeometricSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "geometric_set")? {
            return Ok(());
        }
        rw_block_0066!(
            a_elements,
            a_elements_item,
            a_name,
            ach,
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
        ent: &Handle<StepShapeGeometricSet>,
    ) -> Result<(), Failure> {
        rw_block_0159!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeGeometricSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_elements();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.elements_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation,
    "RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation"
);
impl RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeGeometricallyBoundedSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "geometrically_bounded_surface_shape_representation",
        )? {
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
        ent: &Handle<StepShapeGeometricallyBoundedSurfaceShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeGeometricallyBoundedSurfaceShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation,
    "RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation"
);
impl RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeGeometricallyBoundedWireframeShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "geometrically_bounded_wireframe_shape_representation",
        )? {
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
        ent: &Handle<StepShapeGeometricallyBoundedWireframeShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeGeometricallyBoundedWireframeShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwHalfSpaceSolid, "RwStepShapeRwHalfSpaceSolid");
impl RwStepShapeRwHalfSpaceSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeHalfSpaceSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "half_space_solid")? {
            return Ok(());
        }
        rw_block_0086!(a_agreement_flag, a_base_surface, a_name, ach, data, num);
        ent.init(&a_name, &a_base_surface, a_agreement_flag);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeHalfSpaceSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.base_surface())?;
        sw.send_boolean(ent.agreement_flag())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeHalfSpaceSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.base_surface().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwLimitsAndFits, "RwStepShapeRwLimitsAndFits");
impl RwStepShapeRwLimitsAndFits {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeLimitsAndFits>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "limits_and_fits")? {
            return Ok(());
        }
        let mut fv: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "form_variance", ach, &mut fv)?;
        let mut zv: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "zone_variance", ach, &mut zv)?;
        let mut gr: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "grade", ach, &mut gr)?;
        let mut sou: Handle<HAsciiString> = Default::default();
        data.read_string(num, 4, "source", ach, &mut sou)?;
        ent.init(&fv, &zv, &gr, &sou);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeLimitsAndFits>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.form_variance())?;
        sw.send_entity(&ent.zone_variance())?;
        sw.send_entity(&ent.grade())?;
        sw.send_entity(&ent.source())?;
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwLoop, "RwStepShapeRwLoop");
impl RwStepShapeRwLoop {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeLoop>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "loop")? {
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
        ent: &Handle<StepShapeLoop>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
