#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwAngularityTolerance,
    "RwStepDimTolRwAngularityTolerance"
);
impl RwStepDimTolRwAngularityTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolAngularityTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "angularity_tolerance")? {
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
        ent: &Handle<StepDimTolAngularityTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolAngularityTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwCircularRunoutTolerance,
    "RwStepDimTolRwCircularRunoutTolerance"
);
impl RwStepDimTolRwCircularRunoutTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolCircularRunoutTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "circular_runout_tolerance")? {
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
        ent: &Handle<StepDimTolCircularRunoutTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolCircularRunoutTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwCoaxialityTolerance,
    "RwStepDimTolRwCoaxialityTolerance"
);
impl RwStepDimTolRwCoaxialityTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolCoaxialityTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "coaxiality_tolerance")? {
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
        ent: &Handle<StepDimTolCoaxialityTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolCoaxialityTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwCommonDatum, "RwStepDimTolRwCommonDatum");
impl RwStepDimTolRwCommonDatum {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolCommonDatum>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 9, ach, "common_datum")? {
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
        let mut a_datum_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 5, "shape_aspect.name", ach, &mut a_datum_name)?;
        let mut a_datum_description: Handle<HAsciiString> = Default::default();
        if data.is_param_defined(num, 6)? {
            data.read_string(
                num,
                6,
                "shape_aspect.description",
                ach,
                &mut a_datum_description,
            )?;
        }
        let mut a_datum_of_shape: Handle<StepReprProductDefinitionShape> = Default::default();
        data.read_entity(
            num,
            7,
            "shape_aspect.of_shape",
            ach,
            StepReprProductDefinitionShape::static_type(),
            &mut a_datum_of_shape,
        )?;
        let mut a_datum_product_definitional: StepDataLogical = Default::default();
        data.read_logical(
            num,
            8,
            "shape_aspect.product_definitional",
            ach,
            &mut a_datum_product_definitional,
        )?;
        let mut a_datum_identification: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            9,
            "datum.identification",
            ach,
            &mut a_datum_identification,
        )?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &a_datum_name,
            &a_datum_description,
            &a_datum_of_shape,
            a_datum_product_definitional,
            &a_datum_identification,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolCommonDatum>,
    ) -> Result<(), Failure> {
        rw_block_b_0150!(ent, sw);
        sw.send_entity(&step_live(&ent.datum(), "WriteStep")?.parent.name())?;
        sw.send_entity(&step_live(&ent.datum(), "WriteStep")?.parent.description())?;
        sw.send_entity(&step_live(&ent.datum(), "WriteStep")?.parent.of_shape())?;
        sw.send_logical(
            step_live(&ent.datum(), "WriteStep")?
                .parent
                .product_definitional(),
        )?;
        sw.send_entity(&step_live(&ent.datum(), "WriteStep")?.identification())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolCommonDatum>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.of_shape().cast());
        iter.add_item(&step_live(&ent.datum(), "Share")?.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwConcentricityTolerance,
    "RwStepDimTolRwConcentricityTolerance"
);
impl RwStepDimTolRwConcentricityTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolConcentricityTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "concentricity_tolerance")? {
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
        ent: &Handle<StepDimTolConcentricityTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolConcentricityTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwCylindricityTolerance,
    "RwStepDimTolRwCylindricityTolerance"
);
impl RwStepDimTolRwCylindricityTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolCylindricityTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "cylindricity_tolerance")? {
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
        ent: &Handle<StepDimTolCylindricityTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolCylindricityTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}
