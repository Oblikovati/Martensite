//! The element descriptors whose purposes are a list of sequences of select members. The volume
//! descriptor has flat purposes and is generated.
use super::*;
use crate::step_data::{StepDataSelectMember, StepDataStepReaderData, StepDataStepWriter};
use crate::step_element::*;
use crate::step_entity::*;
use crate::step_rw::*;
use martensite_tkernel::standard::Transient;

/// Reads the purposes of a descriptor: a list of lists of members of class `M`.
fn read_purposes<M>(
    data: &StepDataStepReaderData,
    num: i32,
    nump: i32,
    member_message: &str,
    ach: &Handle<InterfaceCheck>,
    where_called: &str,
) -> Result<Handle<HArray1<Handle<HSequence<Handle<M>>>>>, Failure>
where
    M: StepDataSelectMember + RuntimeTyped + Transient + Default,
    Handle<M>: Clone,
{
    let mut sub = 0;
    if !data.read_sub_list(num, nump, "purpose", ach, &mut sub, false, 0, 0)? {
        return Ok(Handle::default());
    }
    let count = data.nb_params(sub)?;
    let purposes = step_new_array1::<Handle<HSequence<Handle<M>>>>(1, count)?;
    for rank in 1..=count {
        let members = HSequence::<Handle<M>>::default();
        let mut inner = 0;
        if data.read_sub_list(sub, rank, "sub-part(purpose)", ach, &mut inner, false, 0, 0)? {
            let size = data.nb_params(data.param_number(sub, rank)?)?;
            for position in 1..=size {
                let mut member = Handle::new(M::default());
                data.read_member_typed(inner, position, member_message, ach, &mut member)?;
                members.append(member);
            }
        }
        step_array1_set(&purposes, rank, Handle::new(members), where_called)?;
    }
    Ok(purposes)
}

/// Writes the purposes of a descriptor as a list of lists.
fn write_purposes<M>(
    sw: &mut StepDataStepWriter,
    purposes: &Handle<HArray1<Handle<HSequence<Handle<M>>>>>,
    where_called: &str,
) -> Result<(), Failure>
where
    M: StepDataSelectMember + RuntimeTyped + Transient,
    Handle<M>: Clone,
{
    sw.open_sub()?;
    let live = step_live(purposes, where_called)?;
    for rank in 1..=live.array1().length() {
        sw.new_line(false)?;
        sw.open_sub()?;
        let members = step_array1_item(purposes, rank, where_called)?;
        let members = step_live(&members, where_called)?;
        for member in members.sequence(|items| items.iter().cloned().collect::<Vec<Handle<M>>>()) {
            sw.send_entity(&member)?;
        }
        sw.close_sub()?;
    }
    sw.close_sub()
}

/// Reads the common head of a descriptor record: the topology order and the description.
fn read_order_and_description(
    data: &StepDataStepReaderData,
    num: i32,
    ach: &Handle<InterfaceCheck>,
) -> Result<(StepElementElementOrder, Handle<HAsciiString>), Failure> {
    let mut order = StepElementElementOrder::Linear;
    if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
        match data.param_c_value(num, 1)?.as_slice() {
            b".LINEAR." => order = StepElementElementOrder::Linear,
            b".QUADRATIC." => order = StepElementElementOrder::Quadratic,
            b".CUBIC." => order = StepElementElementOrder::Cubic,
            _ => ach.add_fail_text(
                "Parameter #1 (element_descriptor.topology_order) has not allowed value",
                "",
            ),
        }
    } else {
        ach.add_fail_text(
            "Parameter #1 (element_descriptor.topology_order) is not enumeration",
            "",
        );
    }
    let mut description = Handle::<HAsciiString>::default();
    data.read_string(
        num,
        2,
        "element_descriptor.description",
        ach,
        &mut description,
    )?;
    Ok((order, description))
}

/// Writes the common head of a descriptor record.
fn write_order_and_description(
    sw: &mut StepDataStepWriter,
    descriptor: &StepElementElementDescriptor,
) -> Result<(), Failure> {
    sw.send_enum(match descriptor.topology_order() {
        StepElementElementOrder::Linear => b".LINEAR.",
        StepElementElementOrder::Quadratic => b".QUADRATIC.",
        StepElementElementOrder::Cubic => b".CUBIC.",
    })?;
    sw.send_entity(&descriptor.description())
}

/// Reads the shape of a surface descriptor.
fn read_element_shape(
    data: &StepDataStepReaderData,
    num: i32,
    ach: &Handle<InterfaceCheck>,
) -> Result<StepElementElement2dShape, Failure> {
    if data.param_type(num, 4)? != InterfaceParamType::ParamEnum {
        ach.add_fail_text("Parameter #4 (shape) is not enumeration", "");
        return Ok(StepElementElement2dShape::Quadrilateral);
    }
    Ok(match data.param_c_value(num, 4)?.as_slice() {
        b".TRIANGLE." => StepElementElement2dShape::Triangle,
        b".QUADRILATERAL." => StepElementElement2dShape::Quadrilateral,
        _ => {
            ach.add_fail_text("Parameter #4 (shape) has not allowed value", "");
            StepElementElement2dShape::Quadrilateral
        }
    })
}

impl RwStepElementRwCurve3dElementDescriptor {
    /// `read_step`.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementCurve3dElementDescriptor>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepElementRwCurve3dElementDescriptor::ReadStep";
        if !data.check_nb_params(num, 3, ach, "curve3d_element_descriptor")? {
            return Ok(());
        }
        let (order, description) = read_order_and_description(data, num, ach)?;
        let purposes = read_purposes::<StepElementCurveElementPurposeMember>(
            data,
            num,
            3,
            "curve_element_purpose",
            ach,
            WHERE,
        )?;
        step_live(ent, WHERE)?.init(order, &description, &purposes);
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementCurve3dElementDescriptor>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepElementRwCurve3dElementDescriptor::WriteStep";
        let entity = step_live(ent, WHERE)?;
        write_order_and_description(sw, entity)?;
        write_purposes(sw, &entity.purpose(), WHERE)
    }
}

impl RwStepElementRwSurface3dElementDescriptor {
    /// `read_step`.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementSurface3dElementDescriptor>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepElementRwSurface3dElementDescriptor::ReadStep";
        if !data.check_nb_params(num, 4, ach, "surface3d_element_descriptor")? {
            return Ok(());
        }
        let (order, description) = read_order_and_description(data, num, ach)?;
        let purposes = read_purposes::<StepElementSurfaceElementPurposeMember>(
            data,
            num,
            3,
            "surface_element_purpose",
            ach,
            WHERE,
        )?;
        let shape = read_element_shape(data, num, ach)?;
        step_live(ent, WHERE)?.init(order, &description, &purposes, shape);
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementSurface3dElementDescriptor>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepElementRwSurface3dElementDescriptor::WriteStep";
        let entity = step_live(ent, WHERE)?;
        write_order_and_description(sw, entity)?;
        write_purposes(sw, &entity.purpose(), WHERE)?;
        sw.send_enum(match entity.shape() {
            StepElementElement2dShape::Quadrilateral => b".QUADRILATERAL.",
            StepElementElement2dShape::Triangle => b".TRIANGLE.",
        })
    }
}
