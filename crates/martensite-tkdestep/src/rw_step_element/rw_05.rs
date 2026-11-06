#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepElementRwVolume3dElementDescriptor,
    "RwStepElementRwVolume3dElementDescriptor"
);
impl RwStepElementRwVolume3dElementDescriptor {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementVolume3dElementDescriptor>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "volume3d_element_descriptor")? {
            return Ok(());
        }
        let mut a_element_descriptor_topology_order: StepElementElementOrder =
            StepElementElementOrder::Linear;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".LINEAR." {
                a_element_descriptor_topology_order = StepElementElementOrder::Linear;
            } else {
                if text == b".QUADRATIC." {
                    a_element_descriptor_topology_order = StepElementElementOrder::Quadratic;
                } else {
                    if text == b".CUBIC." {
                        a_element_descriptor_topology_order = StepElementElementOrder::Cubic;
                    } else {
                        ach.add_fail_text("Parameter #1 (element_descriptor.topology_order) has not allowed value", "");
                    }
                }
            }
        } else {
            ach.add_fail_text(
                "Parameter #1 (element_descriptor.topology_order) is not enumeration",
                "",
            );
        }
        let mut a_element_descriptor_description: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            2,
            "element_descriptor.description",
            ach,
            &mut a_element_descriptor_description,
        )?;
        let mut a_purpose: Handle<HArray1<Handle<StepElementVolumeElementPurposeMember>>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "purpose", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_purpose = step_new_array1::<Handle<StepElementVolumeElementPurposeMember>>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut a_member: Handle<StepElementVolumeElementPurposeMember> =
                    Handle::new(StepElementVolumeElementPurposeMember::new());
                data.read_member_typed(num2, i0, "volume_element_purpose", ach, &mut a_member)?;
                step_array1_set(&a_purpose, i0, a_member.clone(), "ReadStep")?;
            }
        }
        let mut a_shape: StepElementVolume3dElementShape =
            StepElementVolume3dElementShape::Hexahedron;
        if data.param_type(num, 4)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 4)?;
            if text == b".HEXAHEDRON." {
                a_shape = StepElementVolume3dElementShape::Hexahedron;
            } else {
                if text == b".WEDGE." {
                    a_shape = StepElementVolume3dElementShape::Wedge;
                } else {
                    if text == b".TETRAHEDRON." {
                        a_shape = StepElementVolume3dElementShape::Tetrahedron;
                    } else {
                        if text == b".PYRAMID." {
                            a_shape = StepElementVolume3dElementShape::Pyramid;
                        } else {
                            ach.add_fail_text("Parameter #4 (shape) has not allowed value", "");
                        }
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #4 (shape) is not enumeration", "");
        }
        ent.init(
            a_element_descriptor_topology_order,
            &a_element_descriptor_description,
            &a_purpose,
            a_shape,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementVolume3dElementDescriptor>,
    ) -> Result<(), Failure> {
        if ent.parent.topology_order() == StepElementElementOrder::Linear {
            sw.send_enum(b".LINEAR.")?;
        } else {
            if ent.parent.topology_order() == StepElementElementOrder::Quadratic {
                sw.send_enum(b".QUADRATIC.")?;
            } else {
                if ent.parent.topology_order() == StepElementElementOrder::Cubic {
                    sw.send_enum(b".CUBIC.")?;
                }
            }
        }
        sw.send_entity(&ent.parent.description())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.purpose(), "WriteStep")?.array1().length() {
            let var0: Handle<StepElementVolumeElementPurposeMember> =
                step_array1_item(&ent.purpose(), i2, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        if ent.shape() == StepElementVolume3dElementShape::Hexahedron {
            sw.send_enum(b".HEXAHEDRON.")?;
        } else {
            if ent.shape() == StepElementVolume3dElementShape::Wedge {
                sw.send_enum(b".WEDGE.")?;
            } else {
                if ent.shape() == StepElementVolume3dElementShape::Tetrahedron {
                    sw.send_enum(b".TETRAHEDRON.")?;
                } else {
                    if ent.shape() == StepElementVolume3dElementShape::Pyramid {
                        sw.send_enum(b".PYRAMID.")?;
                    }
                }
            }
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementVolume3dElementDescriptor>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
