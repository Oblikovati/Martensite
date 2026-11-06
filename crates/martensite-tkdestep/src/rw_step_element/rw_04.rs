#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepElementRwSurfaceSection,
    "RwStepElementRwSurfaceSection"
);
impl RwStepElementRwSurfaceSection {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementSurfaceSection>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_section")? {
            return Ok(());
        }
        let mut a_offset: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 1, "offset", ach, &mut a_offset)?;
        let mut a_non_structural_mass: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(
            num,
            2,
            "non_structural_mass",
            ach,
            &mut a_non_structural_mass,
        )?;
        let mut a_non_structural_mass_offset: StepElementMeasureOrUnspecifiedValue =
            Default::default();
        data.read_entity_select(
            num,
            3,
            "non_structural_mass_offset",
            ach,
            &mut a_non_structural_mass_offset,
        )?;
        ent.init(
            &a_offset,
            &a_non_structural_mass,
            &a_non_structural_mass_offset,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementSurfaceSection>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.offset().value())?;
        sw.send_entity(&ent.non_structural_mass().value())?;
        sw.send_entity(&ent.non_structural_mass_offset().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementSurfaceSection>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwSurfaceSectionField,
    "RwStepElementRwSurfaceSectionField"
);
impl RwStepElementRwSurfaceSectionField {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        _unused_3: &Handle<StepElementSurfaceSectionField>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 0, ach, "surface_section_field")? {
            return Ok(());
        }
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        _unused_0: &mut StepDataStepWriter,
        _unused_1: &Handle<StepElementSurfaceSectionField>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementSurfaceSectionField>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwSurfaceSectionFieldConstant,
    "RwStepElementRwSurfaceSectionFieldConstant"
);
impl RwStepElementRwSurfaceSectionFieldConstant {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementSurfaceSectionFieldConstant>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "surface_section_field_constant")? {
            return Ok(());
        }
        let mut a_definition: Handle<StepElementSurfaceSection> = Default::default();
        data.read_entity(
            num,
            1,
            "definition",
            ach,
            StepElementSurfaceSection::static_type(),
            &mut a_definition,
        )?;
        ent.init(&a_definition);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementSurfaceSectionFieldConstant>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.definition())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementSurfaceSectionFieldConstant>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.definition().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwSurfaceSectionFieldVarying,
    "RwStepElementRwSurfaceSectionFieldVarying"
);
impl RwStepElementRwSurfaceSectionFieldVarying {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementSurfaceSectionFieldVarying>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "surface_section_field_varying")? {
            return Ok(());
        }
        let mut a_definitions: Handle<HArray1<Handle<StepElementSurfaceSection>>> =
            Default::default();
        let mut sub1: i32 = 0;
        if data.read_sub_list(num, 1, "definitions", ach, &mut sub1, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub1)?;
            a_definitions = step_new_array1::<Handle<StepElementSurfaceSection>>(1, nb0)?;
            let num2: i32 = sub1;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepElementSurfaceSection> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "surface_section",
                    ach,
                    StepElementSurfaceSection::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_definitions, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_additional_node_values: bool = Default::default();
        data.read_boolean(
            num,
            2,
            "additional_node_values",
            ach,
            &mut a_additional_node_values,
        )?;
        ent.init(&a_definitions, a_additional_node_values);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementSurfaceSectionFieldVarying>,
    ) -> Result<(), Failure> {
        sw.open_sub()?;
        for i0 in 1..=step_live(&ent.definitions(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: Handle<StepElementSurfaceSection> =
                step_array1_item(&ent.definitions(), i0, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        sw.send_boolean(ent.additional_node_values())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementSurfaceSectionFieldVarying>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.definitions(), "Share")?.array1().length() {
            let var0: Handle<StepElementSurfaceSection> =
                step_array1_item(&ent.definitions(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwUniformSurfaceSection,
    "RwStepElementRwUniformSurfaceSection"
);
impl RwStepElementRwUniformSurfaceSection {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementUniformSurfaceSection>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "uniform_surface_section")? {
            return Ok(());
        }
        let mut a_surface_section_offset: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(
            num,
            1,
            "surface_section.offset",
            ach,
            &mut a_surface_section_offset,
        )?;
        let mut a_surface_section_non_structural_mass: StepElementMeasureOrUnspecifiedValue =
            Default::default();
        data.read_entity_select(
            num,
            2,
            "surface_section.non_structural_mass",
            ach,
            &mut a_surface_section_non_structural_mass,
        )?;
        let mut a_surface_section_non_structural_mass_offset: StepElementMeasureOrUnspecifiedValue =
            Default::default();
        data.read_entity_select(
            num,
            3,
            "surface_section.non_structural_mass_offset",
            ach,
            &mut a_surface_section_non_structural_mass_offset,
        )?;
        let mut a_thickness: f64 = Default::default();
        data.read_real(num, 4, "thickness", ach, &mut a_thickness)?;
        let mut a_bending_thickness: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 5, "bending_thickness", ach, &mut a_bending_thickness)?;
        let mut a_shear_thickness: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 6, "shear_thickness", ach, &mut a_shear_thickness)?;
        ent.init(
            &a_surface_section_offset,
            &a_surface_section_non_structural_mass,
            &a_surface_section_non_structural_mass_offset,
            a_thickness,
            &a_bending_thickness,
            &a_shear_thickness,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementUniformSurfaceSection>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.offset().value())?;
        sw.send_entity(&ent.parent.non_structural_mass().value())?;
        sw.send_entity(&ent.parent.non_structural_mass_offset().value())?;
        sw.send_real(ent.thickness())?;
        sw.send_entity(&ent.bending_thickness().value())?;
        sw.send_entity(&ent.shear_thickness().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementUniformSurfaceSection>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
