//! Stored reference records, preserving delimiter selection, warnings and lexical paths.
use super::{PcdmReadWriterBase, PcdmReference, file_header::native_driver, reference_paths};
use martensite_tkernel::{
    message::{Gravity, Messenger},
    osd::CadPath,
    standard::Failure,
    storage::{BaseDriver, HeaderData, OpenMode, StorageError},
    tcollection::{AsciiString, ExtendedString},
};
use std::sync::Arc;

pub(super) fn references(
    file: &ExtendedString,
    messenger: Option<&Arc<Messenger>>,
    mut append: impl FnMut(PcdmReference),
) -> Result<(), Failure> {
    let records = reference_records(file)?;
    let (directory, _) =
        CadPath::folder_and_file_from_path(AsciiString::from_extended(file, None).as_bytes());
    for record in records {
        if let Some(reference) = decode_reference_record(&record, &directory, messenger) {
            append(reference);
        }
    }
    Ok(())
}

fn reference_records(file: &ExtendedString) -> Result<Vec<ExtendedString>, Failure> {
    let Some(driver) = native_driver(file)? else {
        return Ok(Vec::new());
    };
    records_from_driver(driver, file)
}

fn records_from_driver(
    mut driver: Box<dyn BaseDriver>,
    file: &ExtendedString,
) -> Result<Vec<ExtendedString>, Failure> {
    open_header(&mut *driver, file)?;
    let header = HeaderData::default();
    header.read(&mut *driver)?;
    let records = delimited_records(header.user_info().iter());
    driver.close()?;
    Ok(records)
}

fn open_header(driver: &mut dyn BaseDriver, file: &ExtendedString) -> Result<(), Failure> {
    let status = driver.open(&AsciiString::from_extended(file, None), OpenMode::Read)?;
    if status == StorageError::Ok {
        return Ok(());
    }
    let detail = match status {
        StorageError::OpenError => "; file was not found or permission denied",
        StorageError::AlreadyOpen => "; file was already opened",
        _ => "",
    };
    Err(Failure::new(&format!(
        "could not open the file: {}{detail}",
        file.to_utf8()
    )))
}

fn delimited_records<'a>(lines: impl Iterator<Item = &'a AsciiString>) -> Vec<ExtendedString> {
    let lines: Vec<&AsciiString> = lines.collect();
    let begin = lines
        .iter()
        .rposition(|line| line.as_bytes() == b"START_REF");
    let end = lines.iter().rposition(|line| line.as_bytes() == b"END_REF");
    let Some((begin, end)) = begin.zip(end).filter(|(begin, end)| begin < end) else {
        return Vec::new();
    };
    lines[begin + 1..end]
        .iter()
        .map(|line| ExtendedString::from_ascii_string(line, true))
        .collect()
}

fn decode_reference_record(
    record: &ExtendedString,
    directory: &AsciiString,
    messenger: Option<&Arc<Messenger>>,
) -> Option<PcdmReference> {
    let (mut identifier, mut version, mut file) = (0, 0, ExtendedString::default());
    if !PcdmReadWriterBase::parse_reference(record, &mut identifier, &mut version, &mut file) {
        let mut warning = ExtendedString::new("Warning: invalid document reference: ");
        warning.assign_cat(record.clone());
        emit_reference_warning(messenger, &warning);
        return None;
    }
    let path = reference_paths::resolved_path(directory, &AsciiString::from_extended(&file, None));
    let path = ExtendedString::from_ascii_string(&path, true);
    found_warning(messenger, identifier, &path, version);
    Some(PcdmReference::new(identifier, &path, version))
}

fn found_warning(messenger: Option<&Arc<Messenger>>, id: i32, path: &ExtendedString, version: i32) {
    let mut warning = ExtendedString::new("Warning: reference found; ReferenceIdentifier:  ");
    for part in [
        ExtendedString::from_integer(id),
        ExtendedString::new("; File:"),
        path.clone(),
        ExtendedString::new(", version:"),
        ExtendedString::from_integer(version),
    ] {
        warning.assign_cat(part);
    }
    emit_reference_warning(messenger, &warning);
}

fn emit_reference_warning(messenger: Option<&Arc<Messenger>>, warning: &ExtendedString) {
    if let Some(messenger) = messenger {
        messenger.send_extended(warning, Gravity::Warning);
    }
}

#[cfg(test)]
#[path = "header_reference_tests.rs"]
mod tests;
