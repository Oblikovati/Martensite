#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwParametricCurve3dElementCoordinateSystem,
    "RwStepFEARwParametricCurve3dElementCoordinateSystem"
);
impl RwStepFEARwParametricCurve3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "parametric_curve3d_element_coordinate_system")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_direction: Handle<StepFEAParametricCurve3dElementCoordinateDirection> =
            Default::default();
        data.read_entity(
            num,
            2,
            "direction",
            ach,
            StepFEAParametricCurve3dElementCoordinateDirection::static_type(),
            &mut a_direction,
        )?;
        ent.init(&a_representation_item_name, &a_direction);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.direction())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.direction().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwParametricSurface3dElementCoordinateSystem,
    "RwStepFEARwParametricSurface3dElementCoordinateSystem"
);
impl RwStepFEARwParametricSurface3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAParametricSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "parametric_surface3d_element_coordinate_system",
        )? {
            return Ok(());
        }
        rw_block_b_0077!(
            a_angle,
            a_axis,
            a_representation_item_name,
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
        ent: &Handle<StepFEAParametricSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_integer(ent.axis())?;
        sw.send_real(ent.angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAParametricSurface3dElementCoordinateSystem>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwSurface3dElementRepresentation,
    "RwStepFEARwSurface3dElementRepresentation"
);
impl RwStepFEARwSurface3dElementRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEASurface3dElementRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "surface3d_element_representation")? {
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
        let mut a_element_descriptor: Handle<StepElementSurface3dElementDescriptor> =
            Default::default();
        data.read_entity(
            num,
            6,
            "element_descriptor",
            ach,
            StepElementSurface3dElementDescriptor::static_type(),
            &mut a_element_descriptor,
        )?;
        let mut a_property: Handle<StepElementSurfaceElementProperty> = Default::default();
        data.read_entity(
            num,
            7,
            "property",
            ach,
            StepElementSurfaceElementProperty::static_type(),
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
        ent: &Handle<StepFEASurface3dElementRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0028!(ent, i1, i3, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEASurface3dElementRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0067!(ent, i1, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwVolume3dElementRepresentation,
    "RwStepFEARwVolume3dElementRepresentation"
);
impl RwStepFEARwVolume3dElementRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAVolume3dElementRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "volume3d_element_representation")? {
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
        let mut a_element_descriptor: Handle<StepElementVolume3dElementDescriptor> =
            Default::default();
        data.read_entity(
            num,
            6,
            "element_descriptor",
            ach,
            StepElementVolume3dElementDescriptor::static_type(),
            &mut a_element_descriptor,
        )?;
        let mut a_material: Handle<StepElementElementMaterial> = Default::default();
        data.read_entity(
            num,
            7,
            "material",
            ach,
            StepElementElementMaterial::static_type(),
            &mut a_material,
        )?;
        ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
            &a_element_representation_node_list,
            &a_model_ref,
            &a_element_descriptor,
            &a_material,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAVolume3dElementRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0041!(ent, i1, i3, sw, var0);
        sw.send_entity(&ent.material())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAVolume3dElementRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0100!(ent, i1, i3, iter, var0);
        iter.add_item(&ent.material().cast());
        Ok(())
    }
}
