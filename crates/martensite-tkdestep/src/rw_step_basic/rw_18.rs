#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwPersonAndOrganization,
    "RwStepBasicRwPersonAndOrganization"
);
impl RwStepBasicRwPersonAndOrganization {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPersonAndOrganization>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "person_and_organization")? {
            return Ok(());
        }
        let mut a_the_person: Handle<StepBasicPerson> = Default::default();
        data.read_entity(
            num,
            1,
            "the_person",
            ach,
            StepBasicPerson::static_type(),
            &mut a_the_person,
        )?;
        let mut a_the_organization: Handle<StepBasicOrganization> = Default::default();
        data.read_entity(
            num,
            2,
            "the_organization",
            ach,
            StepBasicOrganization::static_type(),
            &mut a_the_organization,
        )?;
        ent.init(&a_the_person, &a_the_organization);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicPersonAndOrganization>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.the_person())?;
        sw.send_entity(&ent.the_organization())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicPersonAndOrganization>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.the_person().cast());
        iter.get_one_item(&ent.the_organization().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwPersonAndOrganizationRole,
    "RwStepBasicRwPersonAndOrganizationRole"
);
impl RwStepBasicRwPersonAndOrganizationRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPersonAndOrganizationRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "person_and_organization_role")? {
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
        ent: &Handle<StepBasicPersonAndOrganizationRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwPersonalAddress, "RwStepBasicRwPersonalAddress");
impl RwStepBasicRwPersonalAddress {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPersonalAddress>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 14, ach, "personal_address")? {
            return Ok(());
        }
        rw_block_0000!(
            a_country,
            a_electronic_mail_address,
            a_facsimile_number,
            a_internal_location,
            a_postal_box,
            a_postal_code,
            a_region,
            a_street,
            a_street_number,
            a_telephone_number,
            a_telex_number,
            a_town,
            ach,
            data,
            has_acountry,
            has_aelectronic_mail_address,
            has_afacsimile_number,
            has_ainternal_location,
            has_apostal_box,
            has_apostal_code,
            has_aregion,
            has_astreet,
            has_astreet_number,
            has_atelephone_number,
            has_atelex_number,
            has_atown,
            num
        );
        let mut a_people: Handle<HArray1<Handle<StepBasicPerson>>> = Default::default();
        let mut anent13: Handle<StepBasicPerson> = Default::default();
        let mut nsub13: i32 = Default::default();
        if data.read_sub_list(num, 13, "people", ach, &mut nsub13, false, 0, 0)? {
            let nb13: i32 = data.nb_params(nsub13)?;
            a_people = step_new_array1::<Handle<StepBasicPerson>>(1, nb13)?;
            for i13 in 1..=nb13 {
                if data.read_entity(
                    nsub13,
                    i13,
                    "person",
                    ach,
                    StepBasicPerson::static_type(),
                    &mut anent13,
                )? {
                    step_array1_set(&a_people, i13, anent13.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0213!(a_description, ach, data, num);
        ent.init(
            has_ainternal_location,
            &a_internal_location,
            has_astreet_number,
            &a_street_number,
            has_astreet,
            &a_street,
            has_apostal_box,
            &a_postal_box,
            has_atown,
            &a_town,
            has_aregion,
            &a_region,
            has_apostal_code,
            &a_postal_code,
            has_acountry,
            &a_country,
            has_afacsimile_number,
            &a_facsimile_number,
            has_atelephone_number,
            &a_telephone_number,
            has_aelectronic_mail_address,
            &a_electronic_mail_address,
            has_atelex_number,
            &a_telex_number,
            &a_people,
            &a_description,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicPersonalAddress>,
    ) -> Result<(), Failure> {
        rw_block_0003!(
            ent,
            has_acountry,
            has_aelectronic_mail_address,
            has_afacsimile_number,
            has_ainternal_location,
            has_apostal_box,
            has_apostal_code,
            has_aregion,
            has_astreet,
            has_astreet_number,
            has_atelephone_number,
            has_atelex_number,
            has_atown,
            sw
        );
        for i13 in 1..=ent.nb_people() {
            sw.send_entity(&ent.people_value(i13)?)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicPersonalAddress>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_people();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.people_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwPlaneAngleMeasureWithUnit,
    "RwStepBasicRwPlaneAngleMeasureWithUnit"
);
impl RwStepBasicRwPlaneAngleMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPlaneAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "plane_angle_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0111!(a_unit_component, ach, data, ent, mvc, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicPlaneAngleMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicPlaneAngleMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwPlaneAngleUnit, "RwStepBasicRwPlaneAngleUnit");
impl RwStepBasicRwPlaneAngleUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "plane_angle_unit")? {
            return Ok(());
        }
        rw_block_0201!(a_dimensions, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicPlaneAngleUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimensions())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicPlaneAngleUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        Ok(())
    }
}
