#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwAnnotationFillAreaOccurrence,
    "RwStepVisualRwAnnotationFillAreaOccurrence"
);
impl RwStepVisualRwAnnotationFillAreaOccurrence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationFillAreaOccurrence>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "annotation_fill_area_occurrence")? {
            return Ok(());
        }
        rw_block_b_0182!(a_name, a_styles, ach, data, num);
        let mut anent: Handle<StepVisualPresentationStyleAssignment> = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 2, "styles", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            a_styles = step_new_array1::<Handle<StepVisualPresentationStyleAssignment>>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity(
                    nsub,
                    i,
                    "presentation_style_assignment",
                    ach,
                    StepVisualPresentationStyleAssignment::static_type(),
                    &mut anent,
                )? {
                    step_array1_set(&a_styles, i, anent.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_item: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            3,
            "item",
            ach,
            StandardTransient::static_type(),
            &mut a_item,
        )?;
        let mut a_fill_style_target: Handle<StepGeomGeometricRepresentationItem> =
            Default::default();
        data.read_entity(
            num,
            4,
            "item",
            ach,
            StepGeomGeometricRepresentationItem::static_type(),
            &mut a_fill_style_target,
        )?;
        ent.init(&a_name, &a_styles, &a_item, &a_fill_style_target);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationFillAreaOccurrence>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_b_0163!(ent, i, sw);
        sw.send_entity(&ent.fill_style_target())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationFillAreaOccurrence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_styles();
        for i in 1..=nb_elem1 {
            iter.get_one_item(&ent.styles_value(i)?.cast());
        }
        iter.get_one_item(&ent.item().cast());
        iter.get_one_item(&ent.fill_style_target().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwAnnotationOccurrence,
    "RwStepVisualRwAnnotationOccurrence"
);
impl RwStepVisualRwAnnotationOccurrence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationOccurrence>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "annotation_occurrence")? {
            return Ok(());
        }
        rw_block_b_0043!(
            a_item, a_name, a_styles, ach, anent2, data, ent, i2, nb2, nsub2, num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_b_0119!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationOccurrence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0186!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwAnnotationPlane,
    "RwStepVisualRwAnnotationPlane"
);
impl RwStepVisualRwAnnotationPlane {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationPlane>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "annotation_plane")? {
            return Ok(());
        }
        rw_block_b_0055!(
            a_item, a_name, a_styles, ach, anent2, data, i2, nb2, nsub2, num
        );
        let mut an_elements: Handle<HArray1<StepVisualAnnotationPlaneElement>> = Default::default();
        let mut an_ent: StepVisualAnnotationPlaneElement = Default::default();
        let mut nb_sub: i32 = Default::default();
        if data.read_sub_list(num, 4, "elements", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            an_elements = step_new_array1::<StepVisualAnnotationPlaneElement>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity_select(nb_sub, i, "content", ach, &mut an_ent)? {
                    step_array1_set(&an_elements, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_styles, &a_item, &an_elements);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationPlane>,
    ) -> Result<(), Failure> {
        rw_block_b_0119!(ent, i2, sw);
        rw_block_b_0196!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationPlane>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0186!(ent, is1, iter, nb_elem1);
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_elements();
        for i in 1..=nb {
            iter.add_item(&ent.elements_value(i)?.value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwAreaInSet, "RwStepVisualRwAreaInSet");
impl RwStepVisualRwAreaInSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAreaInSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "area_in_set")? {
            return Ok(());
        }
        let mut a_area: Handle<StepVisualPresentationArea> = Default::default();
        data.read_entity(
            num,
            1,
            "area",
            ach,
            StepVisualPresentationArea::static_type(),
            &mut a_area,
        )?;
        let mut a_in_set: Handle<StepVisualPresentationSet> = Default::default();
        data.read_entity(
            num,
            2,
            "in_set",
            ach,
            StepVisualPresentationSet::static_type(),
            &mut a_in_set,
        )?;
        ent.init(&a_area, &a_in_set);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAreaInSet>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.area())?;
        sw.send_entity(&ent.in_set())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAreaInSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.area().cast());
        iter.get_one_item(&ent.in_set().cast());
        Ok(())
    }
}
