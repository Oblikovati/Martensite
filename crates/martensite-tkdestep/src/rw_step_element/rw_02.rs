#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepElementRwCurveElementSectionDerivedDefinitions,
    "RwStepElementRwCurveElementSectionDerivedDefinitions"
);
impl RwStepElementRwCurveElementSectionDerivedDefinitions {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementCurveElementSectionDerivedDefinitions>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 12, ach, "curve_element_section_derived_definitions")? {
            return Ok(());
        }
        let mut a_curve_element_section_definition_description: Handle<HAsciiString> =
            Default::default();
        data.read_string(
            num,
            1,
            "curve_element_section_definition.description",
            ach,
            &mut a_curve_element_section_definition_description,
        )?;
        let mut a_curve_element_section_definition_section_angle: f64 = Default::default();
        data.read_real(
            num,
            2,
            "curve_element_section_definition.section_angle",
            ach,
            &mut a_curve_element_section_definition_section_angle,
        )?;
        let mut a_cross_sectional_area: f64 = Default::default();
        data.read_real(
            num,
            3,
            "cross_sectional_area",
            ach,
            &mut a_cross_sectional_area,
        )?;
        let mut a_shear_area: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>> =
            Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "shear_area", ach, &mut sub4, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub4)?;
            a_shear_area = step_new_array1::<StepElementMeasureOrUnspecifiedValue>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: StepElementMeasureOrUnspecifiedValue = Default::default();
                data.read_entity_select(
                    num2,
                    i0,
                    "measure_or_unspecified_value",
                    ach,
                    &mut an_it0,
                )?;
                step_array1_set(&a_shear_area, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_second_moment_of_area: Handle<HArray1<f64>> = Default::default();
        let mut sub5: i32 = 0;
        if data.read_sub_list(num, 5, "second_moment_of_area", ach, &mut sub5, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub5)?;
            a_second_moment_of_area = step_new_array1::<f64>(1, nb0)?;
            let num2: i32 = sub5;
            for i0 in 1..=nb0 {
                let mut an_it0: f64 = Default::default();
                data.read_real(num2, i0, "real", ach, &mut an_it0)?;
                step_array1_set(&a_second_moment_of_area, i0, an_it0, "ReadStep")?;
            }
        }
        let mut a_torsional_constant: f64 = Default::default();
        data.read_real(num, 6, "torsional_constant", ach, &mut a_torsional_constant)?;
        let mut a_warping_constant: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 7, "warping_constant", ach, &mut a_warping_constant)?;
        let mut a_location_of_centroid: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>> =
            Default::default();
        let mut sub8: i32 = 0;
        if data.read_sub_list(num, 8, "location_of_centroid", ach, &mut sub8, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub8)?;
            a_location_of_centroid =
                step_new_array1::<StepElementMeasureOrUnspecifiedValue>(1, nb0)?;
            let num2: i32 = sub8;
            for i0 in 1..=nb0 {
                let mut an_it0: StepElementMeasureOrUnspecifiedValue = Default::default();
                data.read_entity_select(
                    num2,
                    i0,
                    "measure_or_unspecified_value",
                    ach,
                    &mut an_it0,
                )?;
                step_array1_set(&a_location_of_centroid, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_location_of_shear_centre: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>> =
            Default::default();
        let mut sub9: i32 = 0;
        if data.read_sub_list(
            num,
            9,
            "location_of_shear_centre",
            ach,
            &mut sub9,
            false,
            0,
            0,
        )? {
            let nb0: i32 = data.nb_params(sub9)?;
            a_location_of_shear_centre =
                step_new_array1::<StepElementMeasureOrUnspecifiedValue>(1, nb0)?;
            let num2: i32 = sub9;
            for i0 in 1..=nb0 {
                let mut an_it0: StepElementMeasureOrUnspecifiedValue = Default::default();
                data.read_entity_select(
                    num2,
                    i0,
                    "measure_or_unspecified_value",
                    ach,
                    &mut an_it0,
                )?;
                step_array1_set(&a_location_of_shear_centre, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_location_of_non_structural_mass: Handle<
            HArray1<StepElementMeasureOrUnspecifiedValue>,
        > = Default::default();
        let mut sub10: i32 = 0;
        if data.read_sub_list(
            num,
            10,
            "location_of_non_structural_mass",
            ach,
            &mut sub10,
            false,
            0,
            0,
        )? {
            let nb0: i32 = data.nb_params(sub10)?;
            a_location_of_non_structural_mass =
                step_new_array1::<StepElementMeasureOrUnspecifiedValue>(1, nb0)?;
            let num2: i32 = sub10;
            for i0 in 1..=nb0 {
                let mut an_it0: StepElementMeasureOrUnspecifiedValue = Default::default();
                data.read_entity_select(
                    num2,
                    i0,
                    "measure_or_unspecified_value",
                    ach,
                    &mut an_it0,
                )?;
                step_array1_set(
                    &a_location_of_non_structural_mass,
                    i0,
                    an_it0.clone(),
                    "ReadStep",
                )?;
            }
        }
        let mut a_non_structural_mass: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(
            num,
            11,
            "non_structural_mass",
            ach,
            &mut a_non_structural_mass,
        )?;
        let mut a_polar_moment: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 12, "polar_moment", ach, &mut a_polar_moment)?;
        ent.init(
            &a_curve_element_section_definition_description,
            a_curve_element_section_definition_section_angle,
            a_cross_sectional_area,
            &a_shear_area,
            &a_second_moment_of_area,
            a_torsional_constant,
            &a_warping_constant,
            &a_location_of_centroid,
            &a_location_of_shear_centre,
            &a_location_of_non_structural_mass,
            &a_non_structural_mass,
            &a_polar_moment,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementCurveElementSectionDerivedDefinitions>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.description())?;
        sw.send_real(ent.parent.section_angle())?;
        sw.send_real(ent.cross_sectional_area())?;
        sw.open_sub()?;
        for i3 in 1..=step_live(&ent.shear_area(), "WriteStep")?.array1().length() {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.shear_area(), i3, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i4 in 1..=step_live(&ent.second_moment_of_area(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: f64 = step_array1_item(&ent.second_moment_of_area(), i4, "WriteStep")?;
            sw.send_real(var0)?;
        }
        sw.close_sub()?;
        sw.send_real(ent.torsional_constant())?;
        sw.send_entity(&ent.warping_constant().value())?;
        sw.open_sub()?;
        for i7 in 1..=step_live(&ent.location_of_centroid(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_centroid(), i7, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i8 in 1..=step_live(&ent.location_of_shear_centre(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_shear_centre(), i8, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i9 in 1..=step_live(&ent.location_of_non_structural_mass(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_non_structural_mass(), i9, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.non_structural_mass().value())?;
        sw.send_entity(&ent.polar_moment().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementCurveElementSectionDerivedDefinitions>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.shear_area(), "Share")?.array1().length() {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.shear_area(), i1, "Share")?;
            iter.add_item(&var0.value());
        }
        iter.add_item(&ent.warping_constant().value());
        for i3 in 1..=step_live(&ent.location_of_centroid(), "Share")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_centroid(), i3, "Share")?;
            iter.add_item(&var0.value());
        }
        for i4 in 1..=step_live(&ent.location_of_shear_centre(), "Share")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_shear_centre(), i4, "Share")?;
            iter.add_item(&var0.value());
        }
        for i5 in 1..=step_live(&ent.location_of_non_structural_mass(), "Share")?
            .array1()
            .length()
        {
            let var0: StepElementMeasureOrUnspecifiedValue =
                step_array1_item(&ent.location_of_non_structural_mass(), i5, "Share")?;
            iter.add_item(&var0.value());
        }
        iter.add_item(&ent.non_structural_mass().value());
        iter.add_item(&ent.polar_moment().value());
        Ok(())
    }
}
