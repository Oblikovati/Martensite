#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwPlusMinusTolerance,
    "RwStepShapeRwPlusMinusTolerance"
);
impl RwStepShapeRwPlusMinusTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapePlusMinusTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "plus_minus_tolerance")? {
            return Ok(());
        }
        let mut a_range: StepShapeToleranceMethodDefinition = Default::default();
        data.read_entity_select(num, 1, "range", ach, &mut a_range)?;
        let mut a_td: StepShapeDimensionalCharacteristic = Default::default();
        data.read_entity_select(num, 2, "2dtolerance_dimensionrange", ach, &mut a_td)?;
        ent.init(&a_range, &a_td);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapePlusMinusTolerance>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.range().value())?;
        sw.send_entity(&ent.toleranced_dimension().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapePlusMinusTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.range().value());
        iter.add_item(&ent.toleranced_dimension().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwPointRepresentation,
    "RwStepShapeRwPointRepresentation"
);
impl RwStepShapeRwPointRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapePointRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "point_representation")? {
            return Ok(());
        }
        rw_block_0052!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapePointRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0099!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapePointRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0207!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwPolyLoop, "RwStepShapeRwPolyLoop");
impl RwStepShapeRwPolyLoop {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapePolyLoop>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "poly_loop")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_polygon: Handle<HArray1<Handle<StepGeomCartesianPoint>>> = Default::default();
        let mut anent2: Handle<StepGeomCartesianPoint> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "polygon", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_polygon = step_new_array1::<Handle<StepGeomCartesianPoint>>(1, nb2)?;
            for i2 in 1..=nb2 {
                if data.read_entity(
                    nsub2,
                    i2,
                    "cartesian_point",
                    ach,
                    StepGeomCartesianPoint::static_type(),
                    &mut anent2,
                )? {
                    step_array1_set(&a_polygon, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_polygon);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapePolyLoop>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_polygon() {
            sw.send_entity(&ent.polygon_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapePolyLoop>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_polygon();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.polygon_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwPrecisionQualifier,
    "RwStepShapeRwPrecisionQualifier"
);
impl RwStepShapeRwPrecisionQualifier {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapePrecisionQualifier>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "precision_qualifier")? {
            return Ok(());
        }
        let mut pv: i32 = Default::default();
        data.read_integer(num, 1, "precision_value", ach, &mut pv)?;
        ent.init(pv);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapePrecisionQualifier>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.precision_value())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwQualifiedRepresentationItem,
    "RwStepShapeRwQualifiedRepresentationItem"
);
impl RwStepShapeRwQualifiedRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeQualifiedRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "qualified_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut quals: Handle<HArray1<StepShapeValueQualifier>> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "qualifiers", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            quals = step_new_array1::<StepShapeValueQualifier>(1, nb2)?;
            for i2 in 1..=nb2 {
                let mut vq: StepShapeValueQualifier = Default::default();
                if data.read_entity_select(nsub2, i2, "qualifier", ach, &mut vq)? {
                    step_array1_set(&quals, i2, vq.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &quals);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeQualifiedRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_0118!(ent, i, nbq, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeQualifiedRepresentationItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0197!(ent, i, iter, nbq);
        Ok(())
    }
}
