//! Info fields are collected before publishing any header metadata.
use super::{input_fields, input_source::BinaryReadSource};
use crate::ncollection::Sequence;
use crate::standard::{Failure, Handle};
use crate::storage::HeaderData;
use crate::tcollection::{AsciiString, ExtendedString};

pub(super) fn header_data<I: BinaryReadSource>(
    input: &mut I,
    header: &Handle<HeaderData>,
) -> Result<(), I::Error> {
    let info = InfoFields::read(input)?;
    let header = header.get().ok_or_else(null_header)?;
    info.store(header);
    Ok(())
}

/// The info section as the header-data read reads it into locals before storing it.
#[derive(Default)]
struct InfoFields {
    object_count: i32,
    texts: [AsciiString; 5],
    names: [ExtendedString; 2],
    user_info: Sequence<AsciiString>,
}

impl InfoFields {
    fn read<I: BinaryReadSource>(stream: &mut I) -> Result<Self, I::Error> {
        let mut info = Self::default();
        input_fields::integer(stream, &mut info.object_count)?;
        for text in info.texts.iter_mut().take(4) {
            input_fields::ascii(stream, text)?;
        }
        input_fields::extended(stream, &mut info.names[0])?;
        input_fields::ascii(stream, &mut info.texts[4])?;
        input_fields::extended(stream, &mut info.names[1])?;
        info.user_info = read_user_info(stream)?;
        Ok(info)
    }

    fn store(&self, header: &HeaderData) {
        let [
            version,
            date,
            schema_name,
            schema_version,
            application_version,
        ] = &self.texts;
        header.set_number_of_objects(self.object_count);
        header.set_storage_version(version);
        header.set_creation_date(date);
        header.set_schema_name(schema_name);
        header.set_schema_version(schema_version);
        header.set_application_name(&self.names[0]);
        header.set_application_version(application_version);
        header.set_data_type(&self.names[1]);
        for line in self.user_info.iter() {
            header.add_to_user_info(line);
        }
    }
}

fn read_user_info<I: BinaryReadSource>(stream: &mut I) -> Result<Sequence<AsciiString>, I::Error> {
    let mut length = 0;
    input_fields::integer(stream, &mut length)?;
    let mut lines = Sequence::default();
    let mut line = AsciiString::default();
    let mut index = 1;
    while index <= length && stream.good() {
        input_fields::ascii(stream, &mut line)?;
        lines.append(line.clone());
        index += 1;
    }
    Ok(lines)
}

fn null_header() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "FSD_BinaryFile::ReadHeaderData: null header; expected a Storage_HeaderData",
        "",
    )
}
