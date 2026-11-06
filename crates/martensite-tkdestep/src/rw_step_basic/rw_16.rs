#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwOrganizationRole,
    "RwStepBasicRwOrganizationRole"
);
impl RwStepBasicRwOrganizationRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicOrganizationRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "organization_role")? {
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
        ent: &Handle<StepBasicOrganizationRole>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwOrganizationalAddress,
    "RwStepBasicRwOrganizationalAddress"
);
impl RwStepBasicRwOrganizationalAddress {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicOrganizationalAddress>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 14, ach, "organizational_address")? {
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
        let mut a_organizations: Handle<HArray1<Handle<StepBasicOrganization>>> =
            Default::default();
        let mut anent13: Handle<StepBasicOrganization> = Default::default();
        let mut nsub13: i32 = Default::default();
        if data.read_sub_list(num, 13, "organizations", ach, &mut nsub13, false, 0, 0)? {
            let nb13: i32 = data.nb_params(nsub13)?;
            a_organizations = step_new_array1::<Handle<StepBasicOrganization>>(1, nb13)?;
            for i13 in 1..=nb13 {
                if data.read_entity(
                    nsub13,
                    i13,
                    "organization",
                    ach,
                    StepBasicOrganization::static_type(),
                    &mut anent13,
                )? {
                    step_array1_set(&a_organizations, i13, anent13.clone(), "ReadStep")?;
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
            &a_organizations,
            &a_description,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicOrganizationalAddress>,
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
        for i13 in 1..=ent.nb_organizations() {
            sw.send_entity(&ent.organizations_value(i13)?)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicOrganizationalAddress>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_organizations();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.organizations_value(is1)?.cast());
        }
        Ok(())
    }
}
