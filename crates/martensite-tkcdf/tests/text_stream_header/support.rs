use martensite_tkernel::{standard::Handle, storage::Data, tcollection::AsciiString};

pub const HEADERS: [(&[u8], &str); 12] = [
    (include_bytes!("fixtures/00.header"), ""),
    (include_bytes!("fixtures/01.header"), "First"),
    (include_bytes!("fixtures/02.header"), "Second"),
    (include_bytes!("fixtures/03.header"), ""),
    (include_bytes!("fixtures/04.header"), "FILE_FORMAT:"),
    (include_bytes!("fixtures/05.header"), "Tabbed"),
    (include_bytes!("fixtures/06.header"), "Spaced"),
    (include_bytes!("fixtures/07.header"), "café"),
    (include_bytes!("fixtures/08.header"), "\tTabbed"),
    (include_bytes!("fixtures/09.header"), "Final"),
    (include_bytes!("fixtures/10.header"), ""),
    (include_bytes!("fixtures/11.header"), "First"),
];

pub fn storage_header(bytes: &[u8]) -> Handle<Data> {
    let storage = Data::default();
    for line in std::str::from_utf8(bytes).unwrap().split_terminator('\n') {
        storage.add_to_user_info(&AsciiString::from_bytes(line.as_bytes()));
    }
    Handle::new(storage)
}
