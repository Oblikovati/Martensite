//! Filename driver selection and format extraction, using the released persistence drivers.
use martensite_tkernel::{
    fsd::{BinaryFile, CmpFile, DriverFileSystem, File, NativeDriverFileSystem},
    standard::Failure,
    storage::{BaseDriver, HeaderData, OpenMode, StorageError, TypeData},
    tcollection::{AsciiString, ExtendedString},
};
use std::rc::Rc;

pub(super) fn file_format(name: &ExtendedString) -> Result<ExtendedString, Failure> {
    match native_driver(name)? {
        Some(mut driver) => format_from_driver(&mut *driver, name),
        None => Ok(super::xml_header::file_format(name)),
    }
}

pub(super) fn native_driver(name: &ExtendedString) -> Result<Option<Box<dyn BaseDriver>>, Failure> {
    select_driver(
        &AsciiString::from_extended(name, None),
        Rc::new(NativeDriverFileSystem),
    )
}

pub(super) fn select_driver(
    name: &AsciiString,
    files: Rc<dyn DriverFileSystem>,
) -> Result<Option<Box<dyn BaseDriver>>, Failure> {
    if CmpFile::is_good_file_type(files.clone(), name)? == StorageError::Ok {
        return Ok(Some(Box::new(CmpFile::new(files))));
    }
    if File::is_good_file_type(files.clone(), name)? == StorageError::Ok {
        return Ok(Some(Box::new(File::new(files))));
    }
    if BinaryFile::is_good_file_type(files.clone(), name)? == StorageError::Ok {
        return Ok(Some(Box::new(BinaryFile::new(files))));
    }
    Ok(None)
}

pub(super) fn format_from_driver(
    driver: &mut dyn BaseDriver,
    name: &ExtendedString,
) -> Result<ExtendedString, Failure> {
    query_catching_read_failure(driver, name, ExtendedString::default(), read_format)
}

pub(super) fn query_catching_read_failure<T>(
    driver: &mut dyn BaseDriver,
    name: &ExtendedString,
    mut result: T,
    read: impl FnOnce(&mut dyn BaseDriver, &mut T) -> Result<(), Failure>,
) -> Result<T, Failure> {
    // Selection is outside the native catch, reopening/reading inside, closing outside.
    if driver
        .open(&AsciiString::from_extended(name, None), OpenMode::Read)
        .is_ok_and(|status| status == StorageError::Ok)
    {
        let _ = read(driver, &mut result);
        driver.close()?;
    }
    Ok(result)
}

fn read_format(driver: &mut dyn BaseDriver, found: &mut ExtendedString) -> Result<(), Failure> {
    let header = HeaderData::default();
    // The native method ignores the boolean status: user info survives a later
    // comment-section failure and still determines the format.
    header.read(driver)?;
    for line in header.user_info().iter() {
        if line.search(b"FILE_FORMAT: ") != -1 {
            *found = ExtendedString::from_ascii_string(&line.token(b" ", 2), true);
            return Ok(());
        }
    }
    *found = first_stored_type(driver)?;
    Ok(())
}

fn first_stored_type(driver: &mut dyn BaseDriver) -> Result<ExtendedString, Failure> {
    let types = TypeData::default();
    types.read(driver)?;
    types.types().sequence(|items| {
        items
            .value_signed(1)
            .map(|name| ExtendedString::from_ascii_string(name, true))
    })
}

#[cfg(test)]
#[path = "file_header_tests.rs"]
mod tests;
