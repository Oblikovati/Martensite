//! Stream container detection consumes only magic and the XML declaration.
use super::{PcdmInputStream, PcdmStreamFailure, PcdmTypeOfFileDriver};
use martensite_tkernel::{
    fsd::{BinaryFile, CmpFile, File, NativeDriverFileSystem},
    storage::BaseDriver,
};
use std::{io::Read, rc::Rc};

/// Persistent-container driver selection; `Pcdm::file_driver_type_stream(&mut input, &mut driver)`.
pub struct Pcdm;

impl Pcdm {
    /// Select a driver with explicit stream state and exception masks.
    /// XML preserves the existing output driver; `Pcdm::file_driver_type_input(&mut input, &mut driver)?`.
    pub fn file_driver_type_input(
        input: &mut PcdmInputStream<'_>,
        driver: &mut Option<Box<dyn BaseDriver>>,
    ) -> Result<PcdmTypeOfFileDriver, PcdmStreamFailure> {
        probe_driver(input, driver)
            .map(StreamDriverKind::public_kind)
            .map_err(PcdmStreamFailure::in_driver_probe)
    }

    /// Detect with the default non-throwing mask; use explicit input to retain error state.
    /// Example: `Pcdm::file_driver_type_stream(&mut input, &mut driver)`.
    pub fn file_driver_type_stream(
        input: &mut dyn Read,
        driver: &mut Option<Box<dyn BaseDriver>>,
    ) -> PcdmTypeOfFileDriver {
        // A zero mask records failures without throwing, so the error arm is defensive only.
        Self::file_driver_type_input(&mut PcdmInputStream::new(input), driver)
            .unwrap_or(PcdmTypeOfFileDriver::Unknown)
    }
}

#[derive(Clone, Copy)]
pub(super) enum StreamDriverKind {
    Text,
    Compatible,
    Binary,
    Xml,
    Unknown,
}

impl StreamDriverKind {
    fn public_kind(self) -> PcdmTypeOfFileDriver {
        match self {
            Self::Text | Self::Binary => PcdmTypeOfFileDriver::File,
            Self::Compatible => PcdmTypeOfFileDriver::CmpFile,
            Self::Xml => PcdmTypeOfFileDriver::XmlFile,
            Self::Unknown => PcdmTypeOfFileDriver::Unknown,
        }
    }
}

fn finish_noncontainer(
    input: &mut PcdmInputStream<'_>,
    driver: &mut Option<Box<dyn BaseDriver>>,
    magic: &[u8],
) -> Result<StreamDriverKind, PcdmStreamFailure> {
    if magic.windows(5).any(|part| part == b"<?xml") {
        input.consume_xml_declaration()?;
        return Ok(StreamDriverKind::Xml);
    }
    *driver = None;
    Ok(StreamDriverKind::Unknown)
}

pub(super) fn probe_driver(
    input: &mut PcdmInputStream<'_>,
    driver: &mut Option<Box<dyn BaseDriver>>,
) -> Result<StreamDriverKind, PcdmStreamFailure> {
    let magic = input.read_magic_number()?;
    if let Some((kind, found)) = container_driver(magic.as_bytes()) {
        *driver = Some(found);
        return Ok(kind);
    }
    finish_noncontainer(input, driver, magic.as_bytes())
}

pub(super) fn container_driver(magic: &[u8]) -> Option<(StreamDriverKind, Box<dyn BaseDriver>)> {
    let files = Rc::new(NativeDriverFileSystem);
    let (kind, driver): (StreamDriverKind, Box<dyn BaseDriver>) = match magic {
        b"CMPFILE" => (StreamDriverKind::Compatible, Box::new(CmpFile::new(files))),
        b"FSDFILE" => (StreamDriverKind::Text, Box::new(File::new(files))),
        b"BINFILE" => (StreamDriverKind::Binary, Box::new(BinaryFile::new(files))),
        _ => return None,
    };
    Some((kind, driver))
}
