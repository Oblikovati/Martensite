#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_enum_text! {
    RwStepVisualRwCentralOrParallel, StepVisualCentralOrParallel, "StepVisualCentralOrParallel", {
        CopCentral => ".CENTRAL.",
        CopParallel => ".PARALLEL.",
    }
}

step_enum_text! {
    RwStepVisualRwSurfaceSide, StepVisualSurfaceSide, "StepVisualSurfaceSide", {
        SsNegative => ".NEGATIVE.",
        SsPositive => ".POSITIVE.",
        SsBoth => ".BOTH.",
    }
}

step_enum_text! {
    RwStepVisualRwTextPath, StepVisualTextPath, "StepVisualTextPath", {
        TpUp => ".UP.",
        TpRight => ".RIGHT.",
        TpDown => ".DOWN.",
        TpLeft => ".LEFT.",
    }
}

step_rw_class!(
    RwStepVisualRwAnnotationCurveOccurrence,
    "RwStepVisualRwAnnotationCurveOccurrence"
);
impl RwStepVisualRwAnnotationCurveOccurrence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationCurveOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_b_0040!(
            a_item, a_name, a_styles, ach, anent2, data, ent, i2, nb2, nsub2, num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationCurveOccurrence>,
    ) -> Result<(), Failure> {
        rw_block_b_0119!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationCurveOccurrence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0186!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwAnnotationCurveOccurrenceAndGeomReprItem,
    "RwStepVisualRwAnnotationCurveOccurrenceAndGeomReprItem"
);
impl RwStepVisualRwAnnotationCurveOccurrenceAndGeomReprItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationCurveOccurrenceAndGeomReprItem>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("REPRESENTATION_ITEM", "RPRITM", num0, &mut num, ach);
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        data.named_for_complex_short("STYLED_ITEM", "STYITM", num0, &mut num, ach);
        let mut a_styles: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>> =
            Default::default();
        let mut an_ent: Handle<StepVisualPresentationStyleAssignment> = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 1, "styles", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            a_styles = step_new_array1::<Handle<StepVisualPresentationStyleAssignment>>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity(
                    nsub,
                    i,
                    "presentation_style_assignment",
                    ach,
                    StepVisualPresentationStyleAssignment::static_type(),
                    &mut an_ent,
                )? {
                    step_array1_set(&a_styles, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_item: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            2,
            "item",
            ach,
            StandardTransient::static_type(),
            &mut a_item,
        )?;
        ent.init(&a_name, &a_styles, &a_item);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationCurveOccurrenceAndGeomReprItem>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "ANNOTATION_CURVE_OCCURRENCE")?;
        step_start_entity(sw, "ANNOTATION_OCCURRENCE")?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        step_start_entity(sw, "STYLED_ITEM")?;
        rw_block_b_0163!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationCurveOccurrenceAndGeomReprItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem: i32 = ent.nb_styles();
        for i in 1..=nb_elem {
            iter.get_one_item(&ent.styles_value(i)?.cast());
        }
        iter.get_one_item(&ent.item().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwAnnotationFillArea,
    "RwStepVisualRwAnnotationFillArea"
);
impl RwStepVisualRwAnnotationFillArea {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualAnnotationFillArea>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "annotation_fill_area")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_elements: Handle<HArray1<StepShapeGeometricSetSelect>> = Default::default();
        let mut a_elements_item: StepShapeGeometricSetSelect = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 2, "boundaries", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            a_elements = step_new_array1::<StepShapeGeometricSetSelect>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity_select(nsub, i, "boundaries", ach, &mut a_elements_item)? {
                    step_array1_set(&a_elements, i, a_elements_item.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_name, &a_elements);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualAnnotationFillArea>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        rw_block_b_0196!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualAnnotationFillArea>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_bound: i32 = ent.nb_elements();
        for i in 1..=nb_bound {
            iter.get_one_item(&ent.elements_value(i)?.value());
        }
        Ok(())
    }
}
