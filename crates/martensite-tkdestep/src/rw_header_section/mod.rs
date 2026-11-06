//! `RWHeaderSection`: the reading, writing and general services of the header entities.
mod tools;

pub use tools::{
    RWHeaderSectionRWFileDescription, RWHeaderSectionRWFileName, RWHeaderSectionRWFileSchema,
};

use crate::header_section::{
    HeaderSection, HeaderSectionFileDescription, HeaderSectionFileName, HeaderSectionFileSchema,
    HeaderSectionProtocol, HeaderSectionTextList,
};
use crate::step_data::{
    StepData, StepDataGeneralModule, StepDataProtocolOps, StepDataReadWriteModule,
    StepDataStepReaderData, StepDataStepWriter, StepDataUndefinedEntity, StepDataWriterLib,
};
use martensite_tkernel::{
    impl_transient,
    ncollection::HArray1,
    standard::{Failure, Handle, StandardTransient, exceptions},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceGeneralLib,
    InterfaceGeneralModule, InterfaceProtocol, InterfaceReaderLib, InterfaceReaderModule,
    InterfaceShareTool,
};
use std::sync::OnceLock;

const RECO_FILE_NAME: &str = "FILE_NAME";
const RECO_FILE_DESCRIPTION: &str = "FILE_DESCRIPTION";
const RECO_FILE_SCHEMA: &str = "FILE_SCHEMA";

/// `RWHeaderSectionReadWriteModule`. Its constructor registers it in the libraries;
/// [`RWHeaderSection::init`] does this here, with the handle.
///
/// ```
/// use martensite_tkdestep::{rw_header_section::RWHeaderSectionReadWriteModule, step_data::StepDataReadWriteModule};
/// use martensite_tkernel::tcollection::AsciiString;
/// let module = RWHeaderSectionReadWriteModule::default();
/// assert_eq!(module.case_step(&AsciiString::new("FILE_SCHEMA")), 3);
/// assert_eq!(module.step_type(1), "FILE_NAME");
/// ```
#[derive(Default)]
pub struct RWHeaderSectionReadWriteModule {
    transient: StandardTransient,
}
impl_transient!(
    RWHeaderSectionReadWriteModule,
    "RWHeaderSectionReadWriteModule",
    dyn StepDataReadWriteModule,
    transient,
    interfaces: [dyn InterfaceReaderModule, dyn StepDataReadWriteModule]
);
crate::impl_step_read_write_module!(RWHeaderSectionReadWriteModule);

impl StepDataReadWriteModule for RWHeaderSectionReadWriteModule {
    fn case_step(&self, atype: &AsciiString) -> i32 {
        match atype.as_bytes() {
            b"FILE_NAME" => 1,
            b"FILE_DESCRIPTION" => 2,
            b"FILE_SCHEMA" => 3,
            _ => 0,
        }
    }
    fn step_type(&self, cn: i32) -> &'static str {
        match cn {
            1 => RECO_FILE_NAME,
            2 => RECO_FILE_DESCRIPTION,
            3 => RECO_FILE_SCHEMA,
            _ => "",
        }
    }
    fn read_step(
        &self,
        cn: i32,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &mut Handle<InterfaceCheck>,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        let mismatch = |ach: &Handle<InterfaceCheck>, what: &str| {
            ach.add_fail_text(&format!("Type Mismatch on {what}"), "");
            Ok(())
        };
        match cn {
            0 => Ok(()),
            1 => match ent.cast::<HeaderSectionFileName>().get() {
                Some(live) => RWHeaderSectionRWFileName.read_step(data, num, ach, live),
                None => mismatch(ach, "FileName"),
            },
            2 => match ent.cast::<HeaderSectionFileDescription>().get() {
                Some(live) => RWHeaderSectionRWFileDescription.read_step(data, num, ach, live),
                None => mismatch(ach, "FileDescription"),
            },
            3 => match ent.cast::<HeaderSectionFileSchema>().get() {
                Some(live) => RWHeaderSectionRWFileSchema.read_step(data, num, ach, live),
                None => mismatch(ach, "FileSchema"),
            },
            4 => read_undefined(data, num, ach, ent),
            _ => {
                ach.add_fail_text("Type Mismatch when reading - Entity", "");
                Ok(())
            }
        }
    }
    fn write_step(
        &self,
        cn: i32,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        match cn {
            1 => match ent.cast::<HeaderSectionFileName>().get() {
                Some(live) => RWHeaderSectionRWFileName.write_step(sw, live),
                None => Ok(()),
            },
            2 => match ent.cast::<HeaderSectionFileDescription>().get() {
                Some(live) => RWHeaderSectionRWFileDescription.write_step(sw, live),
                None => Ok(()),
            },
            3 => match ent.cast::<HeaderSectionFileSchema>().get() {
                Some(live) => RWHeaderSectionRWFileSchema.write_step(sw, live),
                None => Ok(()),
            },
            4 => write_undefined(sw, ent),
            _ => Ok(()),
        }
    }
}

fn read_undefined(
    data: &Handle<StepDataStepReaderData>,
    num: i32,
    ach: &Handle<InterfaceCheck>,
    ent: &Handle<StandardTransient>,
) -> Result<(), Failure> {
    let undefined = ent.cast::<StepDataUndefinedEntity>();
    let (Some(live), Some(reader)) = (undefined.get(), data.get()) else {
        ach.add_fail_text(
            "# Entity neither Recognized nor set as UndefinedEntity from StepData #",
            "",
        );
        return Ok(());
    };
    live.read_record(reader, num, ach)
}

fn write_undefined(
    sw: &mut StepDataStepWriter,
    ent: &Handle<StandardTransient>,
) -> Result<(), Failure> {
    let undefined = ent.cast::<StepDataUndefinedEntity>();
    let Some(live) = undefined.get() else {
        return Ok(());
    };
    if live.is_complex() {
        sw.start_complex()?;
    }
    live.write_params(sw)?;
    if live.is_complex() {
        sw.end_complex()?;
    }
    Ok(())
}

/// ```
/// use martensite_tkdestep::{header_section::HeaderSectionFileName, rw_header_section::RWHeaderSectionGeneralModule};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkxsbase::interface::InterfaceGeneralModule;
/// let mut ent = Handle::default();
/// assert!(RWHeaderSectionGeneralModule::default().new_void(1, &mut ent));
/// assert!(ent.cast::<HeaderSectionFileName>().is_some());
/// ```
#[derive(Default)]
pub struct RWHeaderSectionGeneralModule {
    transient: StandardTransient,
}
impl_transient!(
    RWHeaderSectionGeneralModule,
    "RWHeaderSectionGeneralModule",
    dyn StepDataGeneralModule,
    transient,
    interfaces: [dyn InterfaceGeneralModule, dyn StepDataGeneralModule]
);

impl StepDataGeneralModule for RWHeaderSectionGeneralModule {}

/// A new text with the content of `text`; null stays null (a null dereference).
fn copied_text(text: &Handle<HAsciiString>) -> Handle<HAsciiString> {
    text.get().map_or_else(Handle::default, |live| {
        Handle::new(HAsciiString::copied(live))
    })
}

/// A new list of `count` copied texts.
fn copied_list(
    count: i32,
    value: impl Fn(i32) -> Result<Handle<HAsciiString>, Failure>,
) -> Result<HeaderSectionTextList, Failure> {
    let list = HArray1::<Handle<HAsciiString>>::with_bounds(1, count)
        .map_err(|error| error.into_failure("RWHeaderSectionGeneralModule"))?;
    for rank in 1..=count {
        list.change_array1()
            .set_value(rank, copied_text(&value(rank)?))
            .map_err(|error| error.into_failure("RWHeaderSectionGeneralModule"))?;
    }
    Ok(Handle::new(list))
}

fn copy_file_name(from: &HeaderSectionFileName, to: &HeaderSectionFileName) -> Result<(), Failure> {
    let author = copied_list(from.nb_author(), |rank| from.author_value(rank))?;
    let organization = copied_list(from.nb_organization(), |rank| from.organization_value(rank))?;
    to.init(
        &copied_text(&from.name()),
        &copied_text(&from.time_stamp()),
        &author,
        &organization,
        &copied_text(&from.preprocessor_version()),
        &copied_text(&from.originating_system()),
        &copied_text(&from.authorisation()),
    );
    Ok(())
}

fn copy_error(cn: i32) -> Failure {
    exceptions::null_object(&format!(
        "RWHeaderSectionGeneralModule::CopyCase {cn}: expected two entities of this case"
    ))
}

impl InterfaceGeneralModule for RWHeaderSectionGeneralModule {
    fn fill_shared_case(
        &self,
        cn: i32,
        ent: &Handle<StandardTransient>,
        iter: &mut InterfaceEntityIterator,
    ) {
        if cn != 4 {
            return;
        }
        if let Some(live) = ent.cast::<StepDataUndefinedEntity>().get() {
            // The trait gives no result: a malformed content shares what was reached.
            let _ = live.fill_shared(iter);
        }
    }
    fn check_case(
        &self,
        _cn: i32,
        _ent: &Handle<StandardTransient>,
        _shares: &InterfaceShareTool,
        _ach: &mut Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    fn copy_case(
        &self,
        cn: i32,
        entfrom: &Handle<StandardTransient>,
        entto: &Handle<StandardTransient>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        match cn {
            1 => {
                let (from, to) = (
                    entfrom.cast::<HeaderSectionFileName>(),
                    entto.cast::<HeaderSectionFileName>(),
                );
                let (Some(from), Some(to)) = (from.get(), to.get()) else {
                    return Err(copy_error(cn));
                };
                copy_file_name(from, to)
            }
            2 => {
                let (from, to) = (
                    entfrom.cast::<HeaderSectionFileDescription>(),
                    entto.cast::<HeaderSectionFileDescription>(),
                );
                let (Some(from), Some(to)) = (from.get(), to.get()) else {
                    return Err(copy_error(cn));
                };
                let list = copied_list(from.nb_description(), |rank| from.description_value(rank))?;
                to.init(&list, &copied_text(&from.implementation_level()));
                Ok(())
            }
            3 => {
                let (from, to) = (
                    entfrom.cast::<HeaderSectionFileSchema>(),
                    entto.cast::<HeaderSectionFileSchema>(),
                );
                let (Some(from), Some(to)) = (from.get(), to.get()) else {
                    return Err(copy_error(cn));
                };
                let list = copied_list(from.nb_schema_identifiers(), |rank| {
                    from.schema_identifiers_value(rank)
                })?;
                to.init(&list);
                Ok(())
            }
            4 => {
                let to = entto.cast::<StepDataUndefinedEntity>();
                let live = to.get().ok_or_else(|| copy_error(cn))?;
                live.get_from_another(&entfrom.cast(), tc)
            }
            _ => Ok(()),
        }
    }
    fn new_void(&self, cn: i32, ent: &mut Handle<StandardTransient>) -> bool {
        *ent = match cn {
            1 => Handle::new(HeaderSectionFileName::default()).cast(),
            2 => Handle::new(HeaderSectionFileDescription::default()).cast(),
            3 => Handle::new(HeaderSectionFileSchema::default()).cast(),
            4 => Handle::new(StepDataUndefinedEntity::new(false)).cast(),
            _ => return false,
        };
        true
    }
}

/// `RWHeaderSection`.
pub struct RWHeaderSection;

impl RWHeaderSection {
    /// `init`: records the header protocol and its modules, once. The read-write
    /// module registers with a new protocol of its own; here with the shared one, which is
    /// of the same type, the criterion of the libraries.
    ///
    /// ```
    /// use martensite_tkdestep::{rw_header_section::RWHeaderSection, step_data::StepData};
    /// RWHeaderSection::init();
    /// assert!(StepData::header_protocol().is_some());
    /// ```
    pub fn init() {
        static DONE: OnceLock<()> = OnceLock::new();
        DONE.get_or_init(|| {
            let protocol: Handle<HeaderSectionProtocol> = HeaderSection::protocol();
            let step: Handle<dyn StepDataProtocolOps> = protocol.cast();
            StepData::add_header_protocol(&step);
            let module = Handle::new(RWHeaderSectionReadWriteModule::default());
            StepDataWriterLib::set_global(&module.cast(), &step);
            let any: Handle<dyn InterfaceProtocol> = protocol.cast();
            InterfaceReaderLib::set_global(&module.cast(), &any);
            let general = Handle::new(RWHeaderSectionGeneralModule::default());
            InterfaceGeneralLib::set_global(&general.cast(), &any);
        });
    }
}
