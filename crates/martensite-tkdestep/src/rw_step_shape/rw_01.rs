#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_enum_text! {
    RwStepShapeRwBooleanOperator, StepShapeBooleanOperator, "StepShapeBooleanOperator", {
        BoDifference => ".DIFFERENCE.",
        BoIntersection => ".INTERSECTION.",
        BoUnion => ".UNION.",
    }
}

step_rw_class!(
    RwStepShapeRwAdvancedBrepShapeRepresentation,
    "RwStepShapeRwAdvancedBrepShapeRepresentation"
);
impl RwStepShapeRwAdvancedBrepShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeAdvancedBrepShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "advanced_brep_shape_representation")? {
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
        ent: &Handle<StepShapeAdvancedBrepShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeAdvancedBrepShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwAdvancedFace, "RwStepShapeRwAdvancedFace");
impl RwStepShapeRwAdvancedFace {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeAdvancedFace>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "advanced_face")? {
            return Ok(());
        }
        rw_block_0109!(a_bounds, a_name, ach, anent2, data, nsub2, num);
        if data.read_sub_list(num, 2, "bounds", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            if nb2 != 0 {
                a_bounds = step_new_array1::<Handle<StepShapeFaceBound>>(1, nb2)?;
                for i2 in 1..=nb2 {
                    if data.read_entity(
                        nsub2,
                        i2,
                        "face_bound",
                        ach,
                        StepShapeFaceBound::static_type(),
                        &mut anent2,
                    )? {
                        step_array1_set(&a_bounds, i2, anent2.clone(), "ReadStep")?;
                    }
                }
            }
        }
        let mut a_face_geometry: Handle<StepGeomSurface> = Default::default();
        data.read_entity(
            num,
            3,
            "face_geometry",
            ach,
            StepGeomSurface::static_type(),
            &mut a_face_geometry,
        )?;
        let mut a_same_sense: bool = true;
        data.read_boolean(num, 4, "same_sense", ach, &mut a_same_sense)?;
        ent.init(&a_name, &a_bounds, &a_face_geometry, a_same_sense);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeAdvancedFace>,
    ) -> Result<(), Failure> {
        rw_block_0094!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeAdvancedFace>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0188!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwAngularLocation, "RwStepShapeRwAngularLocation");
impl RwStepShapeRwAngularLocation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeAngularLocation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "angular_location")? {
            return Ok(());
        }
        rw_block_0043!(
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_name,
            a_shape_aspect_relationship_related_shape_aspect,
            a_shape_aspect_relationship_relating_shape_aspect,
            ach,
            data,
            has_shape_aspect_relationship_description,
            num
        );
        let mut a_angle_selection: StepShapeAngleRelator = StepShapeAngleRelator::Small;
        if data.param_type(num, 5)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 5)?;
            if text == b".EQUAL." {
                a_angle_selection = StepShapeAngleRelator::Equal;
            } else {
                if text == b".LARGE." {
                    a_angle_selection = StepShapeAngleRelator::Large;
                } else {
                    if text == b".SMALL." {
                        a_angle_selection = StepShapeAngleRelator::Small;
                    } else {
                        ach.add_fail_text(
                            "Parameter #5 (angle_selection) has not allowed value",
                            "",
                        );
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #5 (angle_selection) is not enumeration", "");
        }
        ent.init(
            &a_shape_aspect_relationship_name,
            has_shape_aspect_relationship_description,
            &a_shape_aspect_relationship_description,
            &a_shape_aspect_relationship_relating_shape_aspect,
            &a_shape_aspect_relationship_related_shape_aspect,
            a_angle_selection,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeAngularLocation>,
    ) -> Result<(), Failure> {
        rw_block_0122!(ent, sw);
        rw_block_0215!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeAngularLocation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.relating_shape_aspect().cast());
        iter.add_item(&ent.parent.parent.related_shape_aspect().cast());
        Ok(())
    }
}
