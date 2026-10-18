//! A whole binary container read from a stream.
//!
//! The reader dispatches on `tellg()` against the section offsets of the header.
//! A position at the start of the data section, or of no section at all, would read
//! nothing and loop forever, so it returns a `Storage_StreamFormatError` instead.
//! The reference section creates objects with the callbacks registered by the type
//! section, and the type section registers none, so a non-empty reference section
//! has no callback to call and is reported as `Standard_NullObject`.
use super::input_source::{BinaryReadSource, OwnedReadSource};
use super::{BinaryFile, FileHeader, input_fields, input_info};
use crate::ncollection::{ArrayError, HArray1, Sequence};
use crate::standard::{Failure, Handle, StandardPersistent};
use crate::storage::PersistentNumbers;
use crate::storage::{CallBackTable, Data, InternalData, IoStream, Root, StreamException};
use crate::tcollection::{AsciiString, ExtendedString};

const MEMBER: &str = "FSD_BinaryFile::ReadCompleteInfo";

fn array_failure(error: ArrayError) -> Failure {
    let kind = match error {
        ArrayError::Bounds { .. } => "Standard_OutOfMemory",
        _ => "Standard_OutOfRange",
    };
    Failure::with_type(kind, &format!("{MEMBER}: {error}"), "")
}

fn null_failure(what: &str) -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        &format!("{MEMBER}: {what} is null; expected it to be read first"),
        "",
    )
}

/// The section a stream position starts, per the header offsets.
enum Section {
    End,
    Comment,
    Type,
    Root,
    Reference,
    Stuck(i32),
}

fn section_at(position: i32, offsets: &FileHeader) -> Section {
    if position >= offsets.end_data {
        Section::End
    } else if position == offsets.begin_comment {
        Section::Comment
    } else if position == offsets.begin_type {
        Section::Type
    } else if position == offsets.begin_root {
        Section::Root
    } else if position == offsets.begin_ref {
        Section::Reference
    } else {
        Section::Stuck(position)
    }
}

/// The parts of the data set the reader fills.
struct Target<'a> {
    set: &'a Data,
    internal: Handle<InternalData>,
    callbacks: Option<CallBackTable>,
}

impl BinaryFile {
    /// The body of the complete-info read; creates `data` when it is null.
    pub(crate) fn read_complete(
        &mut self,
        stream: &mut IoStream,
        data: &mut Handle<Data>,
    ) -> Result<(), Failure> {
        read_container(&mut OwnedReadSource(stream), data)
    }
}

pub(super) fn read_container<I: BinaryReadSource>(
    stream: &mut I,
    data: &mut Handle<Data>,
) -> Result<(), I::Error> {
    let mut offsets = FileHeader::default();
    input_fields::header(stream, &mut offsets)?;
    let mut target = read_target(stream, data)?;
    read_sections(stream, &offsets, &mut target)?;
    attach_roots(&target)?;
    if let Some(internal) = target.internal.get() {
        internal.clear();
    }
    Ok(())
}

fn read_target<'a, I: BinaryReadSource>(
    stream: &mut I,
    data: &'a mut Handle<Data>,
) -> Result<Target<'a>, I::Error> {
    if data.is_null() {
        *data = Handle::new(Data::default());
    }
    let set = data.get().ok_or_else(|| null_failure("the data set"))?;
    input_info::header_data(stream, &set.header_data())?;
    Ok(Target {
        set,
        internal: set.internal_data(),
        callbacks: None,
    })
}

/// Dispatches on `tellg()` until the end of the data section.
fn read_sections<I: BinaryReadSource>(
    stream: &mut I,
    offsets: &FileHeader,
    target: &mut Target<'_>,
) -> Result<(), I::Error> {
    while stream.good() && !stream.eof() {
        let position = stream.position()?;
        match section_at(position, offsets) {
            Section::End => break,
            Section::Comment => read_comments(stream, target)?,
            Section::Type => read_types(stream, target)?,
            Section::Root => read_roots(stream, target)?,
            Section::Reference => read_references(stream, target)?,
            Section::Stuck(position) => return Err(stuck(position).into()),
        }
    }
    Ok(())
}

fn stuck(position: i32) -> Failure {
    StreamException::Format.raise(&format!(
        "{MEMBER}: no section to read at offset {position}; the native loop repeats forever"
    ))
}

fn internal<'t>(target: &'t Target<'_>) -> Result<&'t InternalData, Failure> {
    target
        .internal
        .get()
        .ok_or_else(|| null_failure("the internal data"))
}

fn read_comments<I: BinaryReadSource>(stream: &mut I, target: &Target<'_>) -> Result<(), I::Error> {
    let mut comments: Sequence<ExtendedString> = Sequence::default();
    input_fields::comments(stream, &mut comments)?;
    let header = target.set.header_data();
    if let Some(header) = header.get() {
        for comment in comments.iter() {
            header.add_to_comments(comment);
        }
    }
    let array = HArray1::with_bounds(1, target.set.number_of_objects()).map_err(array_failure)?;
    internal(target)?.read_array(|slot| *slot = Handle::new(array));
    Ok(())
}

fn read_types<I: BinaryReadSource>(
    stream: &mut I,
    target: &mut Target<'_>,
) -> Result<(), I::Error> {
    let size = input_fields::section_size(stream)?;
    let mut callbacks = CallBackTable::new(size)?;
    let (mut name, mut number) = (AsciiString::default(), 0);
    let types = target.set.type_data();
    for _ in 1..=size {
        input_fields::type_entry(stream, &mut number, &mut name)?;
        if let Some(types) = types.get() {
            types.add_type(&name, number);
        }
        callbacks.set_value(number, &Handle::default())?;
    }
    target.callbacks = Some(callbacks);
    Ok(())
}

fn read_roots<I: BinaryReadSource>(stream: &mut I, target: &Target<'_>) -> Result<(), I::Error> {
    let size = input_fields::section_size(stream)?;
    let (mut name, mut type_name, mut reference) =
        (AsciiString::default(), AsciiString::default(), 0);
    let roots = target.set.root_data();
    for _ in 1..=size {
        input_fields::root_entry(stream, &mut name, &mut reference, &mut type_name)?;
        let root = Root::new(&name, &Handle::default());
        root.set_reference(reference);
        root.set_type(&type_name);
        if let Some(roots) = roots.get() {
            roots.add_root(&Handle::new(root));
        }
    }
    Ok(())
}

fn read_references<I: BinaryReadSource>(
    stream: &mut I,
    target: &Target<'_>,
) -> Result<(), I::Error> {
    let size = input_fields::section_size(stream)?;
    let (mut type_number, mut reference) = (0, 0);
    for _ in 1..=size {
        input_fields::reference_entry(stream, &mut reference, &mut type_number)?;
        let callbacks = target
            .callbacks
            .as_ref()
            .ok_or_else(|| null_failure("the callback table"))?;
        let object = callbacks.value(type_number, MEMBER)?.new_persistent();
        store_object(internal(target)?, reference, &object)?;
        if let Some(object) = object.get() {
            object.set_type_number(type_number);
        }
    }
    Ok(())
}

fn store_object(
    internal: &InternalData,
    reference: i32,
    object: &Handle<StandardPersistent>,
) -> Result<(), Failure> {
    internal.read_array(|array| {
        let array = array.get().ok_or_else(|| null_failure("the read array"))?;
        let mut values = array.change_array1();
        *values.change_value(reference).map_err(array_failure)? = object.clone();
        Ok(())
    })
}

/// Gives every root the object its reference designates in the read array.
fn attach_roots(target: &Target<'_>) -> Result<(), Failure> {
    let roots = target.set.root_data();
    let Some(roots) = roots.get() else {
        return Ok(());
    };
    let sequence = roots.roots();
    let listed: Vec<Handle<Root>> = sequence
        .get()
        .map(|items| items.sequence(|items| items.iter().cloned().collect()))
        .unwrap_or_default();
    for root in listed.iter().filter_map(Handle::get) {
        let object = internal(target)?.read_array(|array| {
            let array = array.get().ok_or_else(|| null_failure("the read array"))?;
            let values = array.array1();
            values
                .value(root.reference())
                .cloned()
                .map_err(array_failure)
        })?;
        roots.update_root(&root.name(), &object)?;
    }
    Ok(())
}
