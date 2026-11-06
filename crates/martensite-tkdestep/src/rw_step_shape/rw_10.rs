#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwFacetedBrep, "RwStepShapeRwFacetedBrep");
impl RwStepShapeRwFacetedBrep {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFacetedBrep>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "faceted_brep")? {
            return Ok(());
        }
        rw_block_0136!(a_name, a_outer, ach, data, num);
        ent.init(&a_name, &a_outer);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFacetedBrep>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.outer())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFacetedBrep>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.outer().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwFacetedBrepAndBrepWithVoids,
    "RwStepShapeRwFacetedBrepAndBrepWithVoids"
);
impl RwStepShapeRwFacetedBrepAndBrepWithVoids {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFacetedBrepAndBrepWithVoids>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        if !data.check_nb_params(num, 1, ach, "brep_with_voids")? {
            return Ok(());
        }
        let mut a_voids: Handle<HArray1<Handle<StepShapeOrientedClosedShell>>> = Default::default();
        let mut anent: Handle<StepShapeOrientedClosedShell> = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "voids", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_voids = step_new_array1::<Handle<StepShapeOrientedClosedShell>>(1, nb1)?;
            for i1 in 1..=nb1 {
                if data.read_entity(
                    nsub1,
                    i1,
                    "oriented_closed_shell",
                    ach,
                    StepShapeOrientedClosedShell::static_type(),
                    &mut anent,
                )? {
                    step_array1_set(&a_voids, i1, anent.clone(), "ReadStep")?;
                }
            }
        }
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 0, ach, "faceted_brep")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 0, ach, "geometric_representation_item")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 1, ach, "manifold_solid_brep")? {
            return Ok(());
        }
        let mut a_outer: Handle<StepShapeClosedShell> = Default::default();
        data.read_entity(
            num,
            1,
            "outer",
            ach,
            StepShapeClosedShell::static_type(),
            &mut a_outer,
        )?;
        rw_block_0114!(a_name, ach, data, num);
        if !data.check_nb_params(num, 0, ach, "solid_model")? {
            return Ok(());
        }
        ent.init_fields(&a_name, &a_outer, &a_voids);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeFacetedBrepAndBrepWithVoids>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "BREP_WITH_VOIDS")?;
        sw.open_sub()?;
        for i1 in 1..=ent.nb_voids()? {
            sw.send_entity(&ent.voids_value(i1)?)?;
        }
        sw.close_sub()?;
        step_start_entity(sw, "FACETED_BREP")?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "MANIFOLD_SOLID_BREP")?;
        sw.send_entity(&ent.outer())?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        step_start_entity(sw, "SOLID_MODEL")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFacetedBrepAndBrepWithVoids>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.outer().cast());
        let nb_elem2: i32 = ent.nb_voids()?;
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.voids_value(is2)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwFacetedBrepShapeRepresentation,
    "RwStepShapeRwFacetedBrepShapeRepresentation"
);
impl RwStepShapeRwFacetedBrepShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeFacetedBrepShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "faceted_brep_shape_representation")? {
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
        ent: &Handle<StepShapeFacetedBrepShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeFacetedBrepShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwGeometricCurveSet,
    "RwStepShapeRwGeometricCurveSet"
);
impl RwStepShapeRwGeometricCurveSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeGeometricCurveSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "geometric_curve_set")? {
            return Ok(());
        }
        rw_block_0066!(
            a_elements,
            a_elements_item,
            a_name,
            ach,
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
        ent: &Handle<StepShapeGeometricCurveSet>,
    ) -> Result<(), Failure> {
        rw_block_0159!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeGeometricCurveSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_elements();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.elements_value(is1)?.value());
        }
        Ok(())
    }
}
