#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwValueFormatTypeQualifier,
    "RwStepShapeRwValueFormatTypeQualifier"
);
impl RwStepShapeRwValueFormatTypeQualifier {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeValueFormatTypeQualifier>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "value_format_type_qualifier")? {
            return Ok(());
        }
        let mut a_format_type: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "format_type", ach, &mut a_format_type)?;
        ent.init(&a_format_type);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeValueFormatTypeQualifier>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.format_type())?;
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwVertex, "RwStepShapeRwVertex");
impl RwStepShapeRwVertex {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeVertex>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "vertex")? {
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
        ent: &Handle<StepShapeVertex>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwVertexLoop, "RwStepShapeRwVertexLoop");
impl RwStepShapeRwVertexLoop {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeVertexLoop>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "vertex_loop")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_loop_vertex: Handle<StepShapeVertex> = Default::default();
        data.read_entity(
            num,
            2,
            "loop_vertex",
            ach,
            StepShapeVertex::static_type(),
            &mut a_loop_vertex,
        )?;
        ent.init(&a_name, &a_loop_vertex);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeVertexLoop>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.loop_vertex())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeVertexLoop>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.loop_vertex().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwVertexPoint, "RwStepShapeRwVertexPoint");
impl RwStepShapeRwVertexPoint {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeVertexPoint>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "vertex_point")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_vertex_geometry: Handle<StepGeomPoint> = Default::default();
        data.read_entity(
            num,
            2,
            "vertex_geometry",
            ach,
            StepGeomPoint::static_type(),
            &mut a_vertex_geometry,
        )?;
        ent.init(&a_name, &a_vertex_geometry);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeVertexPoint>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.vertex_geometry())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeVertexPoint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.vertex_geometry().cast());
        Ok(())
    }
}
