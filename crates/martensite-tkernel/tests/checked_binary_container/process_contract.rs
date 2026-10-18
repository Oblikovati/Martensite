use super::support::driver;
use super::{fakes::FailingBufferedInput, fixture, support};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::Data;

#[test]
fn checked_metadata_serialization_is_stable() {
    let mut source = support::source(&fixture::metadata());
    let mut set = Handle::<Data>::default();
    let (state, count) = support::with_input(&mut source, |input| {
        driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap();
        (input.state(), input.gcount())
    });
    let trace = render(&source, set.get().unwrap(), state, count);
    assert_eq!(
        trace,
        "container_contract|position=157|state=0|count=4|objects=0|types=0|roots=0|schema=schema|units=[65, 937]|body=[98, 111, 100, 121]"
    );
    println!("{trace}");
}
fn render(source: &FailingBufferedInput, set: &Data, state: u8, count: usize) -> String {
    let header = set.header_data();
    let header = header.get().unwrap();
    let schema = header.schema_name();
    format!(
        "container_contract|position={}|state={state}|count={count}|objects={}|types={}|roots={}|schema={}|units={:?}|body={:?}",
        source.cursor,
        header.number_of_objects(),
        set.number_of_types(),
        set.number_of_roots(),
        std::str::from_utf8(schema.as_bytes()).unwrap(),
        header.application_name().as_utf16(),
        &source.bytes[source.cursor..]
    )
}
