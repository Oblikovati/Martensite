#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwPerson, "RwStepBasicRwPerson");
impl RwStepBasicRwPerson {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicPerson>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "person")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "id", ach, &mut a_id)?;
        let mut a_last_name: Handle<HAsciiString> = Default::default();
        let mut has_alast_name: bool = true;
        if data.is_param_defined(num, 2)? {
            data.read_string(num, 2, "last_name", ach, &mut a_last_name)?;
        } else {
            has_alast_name = false;
            a_last_name = Default::default();
        }
        let mut a_first_name: Handle<HAsciiString> = Default::default();
        let mut has_afirst_name: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_string(num, 3, "first_name", ach, &mut a_first_name)?;
        } else {
            has_afirst_name = false;
            a_first_name = Default::default();
        }
        let mut a_middle_names: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut has_amiddle_names: bool = true;
        if data.is_param_defined(num, 4)? {
            let mut a_middle_names_item: Handle<HAsciiString> = Default::default();
            let mut nsub4: i32 = Default::default();
            if data.read_sub_list(num, 4, "middle_names", ach, &mut nsub4, false, 0, 0)? {
                let nb4: i32 = data.nb_params(nsub4)?;
                a_middle_names = step_new_array1::<Handle<HAsciiString>>(1, nb4)?;
                for i4 in 1..=nb4 {
                    if data.read_string(nsub4, i4, "middle_names", ach, &mut a_middle_names_item)? {
                        step_array1_set(
                            &a_middle_names,
                            i4,
                            a_middle_names_item.clone(),
                            "ReadStep",
                        )?;
                    }
                }
            }
        } else {
            has_amiddle_names = false;
            a_middle_names = Default::default();
        }
        let mut a_prefix_titles: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut has_aprefix_titles: bool = true;
        if data.is_param_defined(num, 5)? {
            let mut a_prefix_titles_item: Handle<HAsciiString> = Default::default();
            let mut nsub5: i32 = Default::default();
            if data.read_sub_list(num, 5, "prefix_titles", ach, &mut nsub5, false, 0, 0)? {
                let nb5: i32 = data.nb_params(nsub5)?;
                a_prefix_titles = step_new_array1::<Handle<HAsciiString>>(1, nb5)?;
                for i5 in 1..=nb5 {
                    if data.read_string(
                        nsub5,
                        i5,
                        "prefix_titles",
                        ach,
                        &mut a_prefix_titles_item,
                    )? {
                        step_array1_set(
                            &a_prefix_titles,
                            i5,
                            a_prefix_titles_item.clone(),
                            "ReadStep",
                        )?;
                    }
                }
            }
        } else {
            has_aprefix_titles = false;
            a_prefix_titles = Default::default();
        }
        let mut a_suffix_titles: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut has_asuffix_titles: bool = true;
        if data.is_param_defined(num, 6)? {
            let mut a_suffix_titles_item: Handle<HAsciiString> = Default::default();
            let mut nsub6: i32 = Default::default();
            if data.read_sub_list(num, 6, "suffix_titles", ach, &mut nsub6, false, 0, 0)? {
                let nb6: i32 = data.nb_params(nsub6)?;
                a_suffix_titles = step_new_array1::<Handle<HAsciiString>>(1, nb6)?;
                for i6 in 1..=nb6 {
                    if data.read_string(
                        nsub6,
                        i6,
                        "suffix_titles",
                        ach,
                        &mut a_suffix_titles_item,
                    )? {
                        step_array1_set(
                            &a_suffix_titles,
                            i6,
                            a_suffix_titles_item.clone(),
                            "ReadStep",
                        )?;
                    }
                }
            }
        } else {
            has_asuffix_titles = false;
            a_suffix_titles = Default::default();
        }
        ent.init(
            &a_id,
            has_alast_name,
            &a_last_name,
            has_afirst_name,
            &a_first_name,
            has_amiddle_names,
            &a_middle_names,
            has_aprefix_titles,
            &a_prefix_titles,
            has_asuffix_titles,
            &a_suffix_titles,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicPerson>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        let has_alast_name: bool = ent.has_last_name();
        if has_alast_name {
            sw.send_entity(&ent.last_name())?;
        } else {
            sw.send_undef()?;
        }
        let has_afirst_name: bool = ent.has_first_name();
        if has_afirst_name {
            sw.send_entity(&ent.first_name())?;
        } else {
            sw.send_undef()?;
        }
        let has_amiddle_names: bool = ent.has_middle_names();
        if has_amiddle_names {
            sw.open_sub()?;
            for i4 in 1..=ent.nb_middle_names() {
                sw.send_entity(&ent.middle_names_value(i4)?)?;
            }
            sw.close_sub()?;
        } else {
            sw.send_undef()?;
        }
        let has_aprefix_titles: bool = ent.has_prefix_titles();
        if has_aprefix_titles {
            sw.open_sub()?;
            for i5 in 1..=ent.nb_prefix_titles() {
                sw.send_entity(&ent.prefix_titles_value(i5)?)?;
            }
            sw.close_sub()?;
        } else {
            sw.send_undef()?;
        }
        let has_asuffix_titles: bool = ent.has_suffix_titles();
        if has_asuffix_titles {
            sw.open_sub()?;
            for i6 in 1..=ent.nb_suffix_titles() {
                sw.send_entity(&ent.suffix_titles_value(i6)?)?;
            }
            sw.close_sub()?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
}
