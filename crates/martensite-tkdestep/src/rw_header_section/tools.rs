//! The reading and writing of each header entity.
use crate::header_section::{
    HeaderSectionFileDescription, HeaderSectionFileName, HeaderSectionFileSchema,
    HeaderSectionTextList,
};
use crate::step_data::{StepDataStepReaderData, StepDataStepWriter};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceFileReaderDataParams};

/// Reads parameter `nump` as a list of strings; null with a fail when it is not a list.
/// `at_least_one` is the description's rule: an empty list stays null.
fn read_text_list(
    data: &StepDataStepReaderData,
    num: i32,
    nump: i32,
    mess: &str,
    ach: &Handle<InterfaceCheck>,
    at_least_one: bool,
) -> Result<HeaderSectionTextList, Failure> {
    let nsub = data.sub_list_number(num, nump, false)?;
    if nsub == 0 {
        ach.add_fail_text(&format!("Parameter #{nump} ({mess}) is not a LIST"), "");
        return Ok(Handle::default());
    }
    let count = data.nb_params(nsub)?;
    if at_least_one && count <= 0 {
        return Ok(Handle::default());
    }
    let list = HArray1::<Handle<HAsciiString>>::with_bounds(1, count)
        .map_err(|error| error.into_failure("RWHeaderSection"))?;
    let mut item = Handle::default();
    for rank in 1..=count {
        if data.read_string(nsub, rank, mess, ach, &mut item)? {
            list.change_array1()
                .set_value(rank, item.clone())
                .map_err(|error| error.into_failure("RWHeaderSection"))?;
        }
    }
    Ok(Handle::new(list))
}

/// Writes a list of texts as a sub-list.
fn write_text_list(
    sw: &mut StepDataStepWriter,
    count: i32,
    value: impl Fn(i32) -> Result<Handle<HAsciiString>, Failure>,
) -> Result<(), Failure> {
    sw.open_sub()?;
    for rank in 1..=count {
        sw.send_entity(&value(rank)?)?;
    }
    sw.close_sub()
}

/// RW file name.
#[derive(Clone, Copy, Debug, Default)]
pub struct RWHeaderSectionRWFileName;

impl RWHeaderSectionRWFileName {
    /// `read_step(data, num, ach, ent)`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &HeaderSectionFileName,
    ) -> Result<(), Failure> {
        let Some(reader) = data.get() else {
            return Ok(());
        };
        if !reader.check_nb_params(num, 7, ach, "file_name has not 7 parameter(s)")? {
            return Ok(());
        }
        let mut name = Handle::default();
        reader.read_string(num, 1, "name", ach, &mut name)?;
        let mut time_stamp = Handle::default();
        reader.read_string(num, 2, "time_stamp", ach, &mut time_stamp)?;
        let author = read_text_list(reader, num, 3, "author", ach, false)?;
        let organization = read_text_list(reader, num, 4, "organization", ach, false)?;
        let mut preprocessor = Handle::default();
        reader.read_string(num, 5, "preprocessor_version", ach, &mut preprocessor)?;
        let mut originating = Handle::default();
        reader.read_string(num, 6, "originating_system", ach, &mut originating)?;
        let mut authorisation = Handle::default();
        reader.read_string(num, 7, "authorisation", ach, &mut authorisation)?;
        if !ach.has_failed() {
            ent.init(
                &name,
                &time_stamp,
                &author,
                &organization,
                &preprocessor,
                &originating,
                &authorisation,
            );
        }
        Ok(())
    }
    /// `WriteStep(SW, ent)`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &HeaderSectionFileName,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.time_stamp())?;
        write_text_list(sw, ent.nb_author(), |rank| ent.author_value(rank))?;
        write_text_list(sw, ent.nb_organization(), |rank| {
            ent.organization_value(rank)
        })?;
        sw.send_entity(&ent.preprocessor_version())?;
        sw.send_entity(&ent.originating_system())?;
        sw.send_entity(&ent.authorisation())
    }
}

/// RW file schema.
#[derive(Clone, Copy, Debug, Default)]
pub struct RWHeaderSectionRWFileSchema;

impl RWHeaderSectionRWFileSchema {
    /// `read_step(data, num, ach, ent)`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &HeaderSectionFileSchema,
    ) -> Result<(), Failure> {
        let Some(reader) = data.get() else {
            return Ok(());
        };
        if !reader.check_nb_params(num, 1, ach, "file_schema has not 1 parameter(s)")? {
            return Ok(());
        }
        let list = read_text_list(reader, num, 1, "schema_identifiers", ach, false)?;
        if !ach.has_failed() {
            ent.init(&list);
        }
        Ok(())
    }
    /// `WriteStep(SW, ent)`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &HeaderSectionFileSchema,
    ) -> Result<(), Failure> {
        write_text_list(sw, ent.nb_schema_identifiers(), |rank| {
            ent.schema_identifiers_value(rank)
        })
    }
}

/// RW file description.
#[derive(Clone, Copy, Debug, Default)]
pub struct RWHeaderSectionRWFileDescription;

impl RWHeaderSectionRWFileDescription {
    /// `read_step(data, num, ach, ent)`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &HeaderSectionFileDescription,
    ) -> Result<(), Failure> {
        let Some(reader) = data.get() else {
            return Ok(());
        };
        if !reader.check_nb_params(num, 2, ach, "file_description has not 2 parameter(s)")? {
            return Ok(());
        }
        let list = read_text_list(reader, num, 1, "description", ach, true)?;
        let mut level = Handle::default();
        reader.read_string(num, 2, "implementation_level", ach, &mut level)?;
        if !ach.has_failed() {
            ent.init(&list, &level);
        }
        Ok(())
    }
    /// `WriteStep(SW, ent)`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &HeaderSectionFileDescription,
    ) -> Result<(), Failure> {
        write_text_list(sw, ent.nb_description(), |rank| ent.description_value(rank))?;
        sw.send_entity(&ent.implementation_level())
    }
}
