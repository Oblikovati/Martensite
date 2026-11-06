#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwDimensionalLocationWithPath,
    "RwStepShapeRwDimensionalLocationWithPath"
);
impl RwStepShapeRwDimensionalLocationWithPath {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDimensionalLocationWithPath>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "dimensional_location_with_path")? {
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
        let mut a_path: Handle<StepReprShapeAspect> = Default::default();
        data.read_entity(
            num,
            5,
            "path",
            ach,
            StepReprShapeAspect::static_type(),
            &mut a_path,
        )?;
        ent.init(
            &a_shape_aspect_relationship_name,
            has_shape_aspect_relationship_description,
            &a_shape_aspect_relationship_description,
            &a_shape_aspect_relationship_relating_shape_aspect,
            &a_shape_aspect_relationship_related_shape_aspect,
            &a_path,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDimensionalLocationWithPath>,
    ) -> Result<(), Failure> {
        rw_block_0122!(ent, sw);
        sw.send_entity(&ent.path())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDimensionalLocationWithPath>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.relating_shape_aspect().cast());
        iter.add_item(&ent.parent.parent.related_shape_aspect().cast());
        iter.add_item(&ent.path().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwDimensionalSize, "RwStepShapeRwDimensionalSize");
impl RwStepShapeRwDimensionalSize {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDimensionalSize>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "dimensional_size")? {
            return Ok(());
        }
        let mut a_applies_to: Handle<StepReprShapeAspect> = Default::default();
        data.read_entity(
            num,
            1,
            "applies_to",
            ach,
            StepReprShapeAspect::static_type(),
            &mut a_applies_to,
        )?;
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "name", ach, &mut a_name)?;
        ent.init(&a_applies_to, &a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDimensionalSize>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.applies_to())?;
        sw.send_entity(&ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDimensionalSize>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.applies_to().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwDimensionalSizeWithPath,
    "RwStepShapeRwDimensionalSizeWithPath"
);
impl RwStepShapeRwDimensionalSizeWithPath {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeDimensionalSizeWithPath>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "dimensional_size_with_path")? {
            return Ok(());
        }
        rw_block_0135!(
            a_dimensional_size_applies_to,
            a_dimensional_size_name,
            ach,
            data,
            num
        );
        let mut a_path: Handle<StepReprShapeAspect> = Default::default();
        data.read_entity(
            num,
            3,
            "path",
            ach,
            StepReprShapeAspect::static_type(),
            &mut a_path,
        )?;
        ent.init(
            &a_dimensional_size_applies_to,
            &a_dimensional_size_name,
            &a_path,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeDimensionalSizeWithPath>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.applies_to())?;
        sw.send_entity(&ent.parent.name())?;
        sw.send_entity(&ent.path())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeDimensionalSizeWithPath>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.applies_to().cast());
        iter.add_item(&ent.path().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwEdge, "RwStepShapeRwEdge");
impl RwStepShapeRwEdge {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeEdge>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "edge")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_edge_start: Handle<StepShapeVertex> = Default::default();
        data.read_entity(
            num,
            2,
            "edge_start",
            ach,
            StepShapeVertex::static_type(),
            &mut a_edge_start,
        )?;
        let mut a_edge_end: Handle<StepShapeVertex> = Default::default();
        data.read_entity(
            num,
            3,
            "edge_end",
            ach,
            StepShapeVertex::static_type(),
            &mut a_edge_end,
        )?;
        ent.init(&a_name, &a_edge_start, &a_edge_end);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeEdge>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.edge_start())?;
        sw.send_entity(&ent.edge_end())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeEdge>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.edge_start().cast());
        iter.get_one_item(&ent.edge_end().cast());
        Ok(())
    }
}
