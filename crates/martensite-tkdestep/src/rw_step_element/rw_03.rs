#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepElementRwElementDescriptor,
    "RwStepElementRwElementDescriptor"
);
impl RwStepElementRwElementDescriptor {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementElementDescriptor>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "element_descriptor")? {
            return Ok(());
        }
        let mut a_topology_order: StepElementElementOrder = StepElementElementOrder::Linear;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".LINEAR." {
                a_topology_order = StepElementElementOrder::Linear;
            } else {
                if text == b".QUADRATIC." {
                    a_topology_order = StepElementElementOrder::Quadratic;
                } else {
                    if text == b".CUBIC." {
                        a_topology_order = StepElementElementOrder::Cubic;
                    } else {
                        ach.add_fail_text(
                            "Parameter #1 (topology_order) has not allowed value",
                            "",
                        );
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #1 (topology_order) is not enumeration", "");
        }
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "description", ach, &mut a_description)?;
        ent.init(a_topology_order, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementElementDescriptor>,
    ) -> Result<(), Failure> {
        if ent.topology_order() == StepElementElementOrder::Linear {
            sw.send_enum(b".LINEAR.")?;
        } else {
            if ent.topology_order() == StepElementElementOrder::Quadratic {
                sw.send_enum(b".QUADRATIC.")?;
            } else {
                if ent.topology_order() == StepElementElementOrder::Cubic {
                    sw.send_enum(b".CUBIC.")?;
                }
            }
        }
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementElementDescriptor>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwElementMaterial,
    "RwStepElementRwElementMaterial"
);
impl RwStepElementRwElementMaterial {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementElementMaterial>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "element_material")? {
            return Ok(());
        }
        let mut a_material_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "material_id", ach, &mut a_material_id)?;
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "description", ach, &mut a_description)?;
        let mut a_properties: Handle<HArray1<Handle<StepReprMaterialPropertyRepresentation>>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "properties", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_properties =
                step_new_array1::<Handle<StepReprMaterialPropertyRepresentation>>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepReprMaterialPropertyRepresentation> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "material_property_representation",
                    ach,
                    StepReprMaterialPropertyRepresentation::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_properties, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_material_id, &a_description, &a_properties);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementElementMaterial>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.material_id())?;
        sw.send_entity(&ent.description())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.properties(), "WriteStep")?.array1().length() {
            let var0: Handle<StepReprMaterialPropertyRepresentation> =
                step_array1_item(&ent.properties(), i2, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementElementMaterial>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.properties(), "Share")?.array1().length() {
            let var0: Handle<StepReprMaterialPropertyRepresentation> =
                step_array1_item(&ent.properties(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwSurface3dElementDescriptor,
    "RwStepElementRwSurface3dElementDescriptor"
);
impl RwStepElementRwSurface3dElementDescriptor {
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementSurface3dElementDescriptor>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwSurfaceElementProperty,
    "RwStepElementRwSurfaceElementProperty"
);
impl RwStepElementRwSurfaceElementProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementSurfaceElementProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_element_property")? {
            return Ok(());
        }
        rw_block_b_0148!(a_description, a_property_id, ach, data, num);
        let mut a_section: Handle<StepElementSurfaceSectionField> = Default::default();
        data.read_entity(
            num,
            3,
            "section",
            ach,
            StepElementSurfaceSectionField::static_type(),
            &mut a_section,
        )?;
        ent.init(&a_property_id, &a_description, &a_section);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementSurfaceElementProperty>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.property_id())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.section())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementSurfaceElementProperty>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.section().cast());
        Ok(())
    }
}
