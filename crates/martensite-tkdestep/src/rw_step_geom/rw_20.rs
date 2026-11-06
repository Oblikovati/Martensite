#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwSurfaceCurveAndBoundedCurve,
    "RwStepGeomRwSurfaceCurveAndBoundedCurve"
);
impl RwStepGeomRwSurfaceCurveAndBoundedCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfaceCurveAndBoundedCurve>,
    ) -> Result<(), Failure> {
        let mut num1: i32 = num;
        num1 = data.next_for_complex(num1);
        num1 = data.next_for_complex(num1);
        num1 = data.next_for_complex(num1);
        if !data.check_nb_params(num1, 1, ach, "representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num1, 1, "name", ach, &mut a_name)?;
        num1 = data.next_for_complex(num1);
        if !data.check_nb_params(num1, 3, ach, "surface_curve")? {
            return Ok(());
        }
        let mut a_curve3d: Handle<StepGeomCurve> = Default::default();
        data.read_entity(
            num1,
            1,
            "curve_3d",
            ach,
            StepGeomCurve::static_type(),
            &mut a_curve3d,
        )?;
        let mut a_associated_geometry: Handle<HArray1<StepGeomPcurveOrSurface>> =
            Default::default();
        let mut a_associated_geometry_item: StepGeomPcurveOrSurface = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num1, 2, "associated_geometry", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_associated_geometry = step_new_array1::<StepGeomPcurveOrSurface>(1, nb3)?;
            for i3 in 1..=nb3 {
                if data.read_entity_select(
                    nsub3,
                    i3,
                    "associated_geometry",
                    ach,
                    &mut a_associated_geometry_item,
                )? {
                    step_array1_set(
                        &a_associated_geometry,
                        i3,
                        a_associated_geometry_item.clone(),
                        "ReadStep",
                    )?;
                }
            }
        }
        let mut a_master_representation: StepGeomPreferredSurfaceCurveRepresentation =
            StepGeomPreferredSurfaceCurveRepresentation::PscrCurve3d;
        if data.param_type(num1, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num1, 3)?;
            if !RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_enum(
                &text,
                &mut a_master_representation,
            ) {
                ach.add_fail_text(
                    "Enumeration preferred_surface_curve_representation has not an allowed value",
                    "",
                );
            }
        } else {
            ach.add_fail_text(
                "Parameter #3 (master_representation) is not an enumeration",
                "",
            );
        }
        ent.init(
            &a_name,
            &a_curve3d,
            &a_associated_geometry,
            a_master_representation,
        );
        ent.state.write().my_bounded_curve = Handle::new(StepGeomBoundedCurve::new());
        step_live(&ent.bounded_curve(), "ReadStep")?.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfaceCurveAndBoundedCurve>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "BOUNDED_CURVE")?;
        step_start_entity(sw, "CURVE")?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        step_start_entity(sw, "SURFACE_CURVE")?;
        rw_block_0101!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfaceCurveAndBoundedCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let tool: RwStepGeomRwSurfaceCurve = Default::default();
        tool.share(&ent.cast(), iter)?;
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwSurfaceOfLinearExtrusion,
    "RwStepGeomRwSurfaceOfLinearExtrusion"
);
impl RwStepGeomRwSurfaceOfLinearExtrusion {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfaceOfLinearExtrusion>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_of_linear_extrusion")? {
            return Ok(());
        }
        rw_block_0134!(a_name, a_swept_curve, ach, data, num);
        let mut a_extrusion_axis: Handle<StepGeomVector> = Default::default();
        data.read_entity(
            num,
            3,
            "extrusion_axis",
            ach,
            StepGeomVector::static_type(),
            &mut a_extrusion_axis,
        )?;
        ent.init(&a_name, &a_swept_curve, &a_extrusion_axis);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfaceOfLinearExtrusion>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_curve())?;
        sw.send_entity(&ent.extrusion_axis())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfaceOfLinearExtrusion>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_curve().cast());
        iter.get_one_item(&ent.extrusion_axis().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwSurfaceOfRevolution,
    "RwStepGeomRwSurfaceOfRevolution"
);
impl RwStepGeomRwSurfaceOfRevolution {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfaceOfRevolution>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_of_revolution")? {
            return Ok(());
        }
        rw_block_0134!(a_name, a_swept_curve, ach, data, num);
        let mut a_axis_position: Handle<StepGeomAxis1Placement> = Default::default();
        data.read_entity(
            num,
            3,
            "axis_position",
            ach,
            StepGeomAxis1Placement::static_type(),
            &mut a_axis_position,
        )?;
        ent.init(&a_name, &a_swept_curve, &a_axis_position);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfaceOfRevolution>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_curve())?;
        sw.send_entity(&ent.axis_position())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfaceOfRevolution>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_curve().cast());
        iter.get_one_item(&ent.axis_position().cast());
        Ok(())
    }
}
