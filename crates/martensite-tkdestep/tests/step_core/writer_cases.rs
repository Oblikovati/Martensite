//! `StepDataStepWriter`: the unsupported values, select arrays and `clean_text_for_send`.
use martensite_tkdestep::step_data::{
    StepDataField, StepDataFieldKind as Kind, StepDataSelectArrReal, StepDataSelectInt,
    StepDataSelectMember, StepDataStepModel, StepDataStepWriter,
};
use martensite_tkernel::{ncollection::HArray1, standard::Handle, tcollection::AsciiString};

fn line(writer: &StepDataStepWriter) -> Vec<u8> {
    writer.line(1).unwrap().string().as_bytes().to_vec()
}

fn write_field(field: &StepDataField) -> Vec<u8> {
    let model = StepDataStepModel::new();
    let mut writer = StepDataStepWriter::new(&model);
    writer.start_entity(&AsciiString::new("TEST")).unwrap();
    writer.send_field(field, &Handle::default()).unwrap();
    writer.end_entity().unwrap();
    line(&writer)
}

fn clean(text: &str) -> String {
    let cleaned = StepDataStepWriter::clean_text_for_send(&AsciiString::new(text));
    String::from_utf8_lossy(cleaned.as_bytes()).into_owned()
}

#[test]
fn send_field_unsupported_values_write_undefined() {
    let mut field = StepDataField::new();
    field.clear_int(10);
    assert_eq!(write_field(&field), b"TEST($);");
    field.clear(Kind::SELECT);
    assert_eq!(write_field(&field), b"TEST($);");
    field.set_enum(-1, b"");
    assert_eq!(write_field(&field), b"TEST($);");
}

#[test]
fn send_select_unsupported_kind_writes_undefined() {
    let select = Handle::new(StepDataSelectInt::default());
    select.set_kind(Kind::ANY.to_int());
    let mut field = StepDataField::new();
    field.set_select_member(&select.cast());
    assert_eq!(write_field(&field), b"TEST($);");
    select.set_kind(261);
    assert_eq!(write_field(&field), b"TEST($);");
}

#[test]
fn send_select_real_array_preserves_bounds_and_parameter_position() {
    let array = HArray1::<f64>::with_bounds(-2, 0).unwrap();
    array.change_array1().set_value(-2, 1.0).unwrap();
    array.change_array1().set_value(-1, 2.0).unwrap();
    array.change_array1().set_value(0, 3.0).unwrap();
    let select = Handle::new(StepDataSelectArrReal::default());
    select.set_arr_real(&Handle::new(array));
    let model = StepDataStepModel::new();
    let mut writer = StepDataStepWriter::new(&model);
    writer.start_entity(&AsciiString::new("TEST")).unwrap();
    writer.send_integer(9).unwrap();
    writer
        .send_select(&select.cast(), &Handle::default())
        .unwrap();
    writer.send_integer(10).unwrap();
    writer.end_entity().unwrap();
    assert_eq!(line(&writer), b"TEST(9,(1.,2.,3.),10);");
    select.set_name("VALUES");
    let mut named = StepDataStepWriter::new(&model);
    named.start_entity(&AsciiString::new("TEST")).unwrap();
    named
        .send_select(&select.cast(), &Handle::default())
        .unwrap();
    named.end_entity().unwrap();
    assert_eq!(line(&named), b"TEST(VALUES((1.,2.,3.)));");
}

#[test]
fn send_select_null_real_array_writes_undefined() {
    let select = Handle::new(StepDataSelectArrReal::default());
    let mut field = StepDataField::new();
    field.set_select_member(&select.cast());
    assert_eq!(write_field(&field), b"TEST($);");
}

#[test]
fn clean_text_for_send_basic_escaping() {
    assert_eq!(
        clean("text with 'single quotes'"),
        "text with ''single quotes''"
    );
    assert_eq!(
        clean("path\\with\\backslashes"),
        "path\\\\with\\\\backslashes"
    );
    assert_eq!(clean("line1\nline2"), "line1\\N\\line2");
    assert_eq!(clean("text\twith\ttabs"), "text\\T\\with\\T\\tabs");
}

#[test]
fn clean_text_for_send_control_directive_preservation() {
    assert_eq!(
        clean("text with \\XA7\\ section sign"),
        "text with \\XA7\\ section sign"
    );
    assert_eq!(clean("\\X2\\03C0\\X0\\ is pi"), "\\X2\\03C0\\X0\\ is pi");
    assert_eq!(
        clean("emoji \\X4\\001F600\\X0\\ face"),
        "emoji \\X4\\001F600\\X0\\ face"
    );
    assert_eq!(
        clean("text with \\S\\ directive"),
        "text with \\S\\ directive"
    );
    assert_eq!(
        clean("\\PA\\ code page setting"),
        "\\PA\\ code page setting"
    );
}

#[test]
fn clean_text_for_send_existing_directive_preservation() {
    assert_eq!(clean("line1\\N\\line2"), "line1\\N\\line2");
    assert_eq!(clean("text\\T\\with\\T\\tab"), "text\\T\\with\\T\\tab");
}

#[test]
fn clean_text_for_send_mixed_content() {
    assert_eq!(
        clean("see \\XA7\\ section and 'quotes'"),
        "see \\XA7\\ section and ''quotes''"
    );
    assert_eq!(clean("\\XA7\\ and path\\file"), "\\XA7\\ and path\\\\file");
    assert_eq!(
        clean("prefix \\X2\\03B103B2\\X0\\ 'text' with\ttab"),
        "prefix \\X2\\03B103B2\\X0\\ ''text'' with\\T\\tab"
    );
}

#[test]
fn clean_text_for_send_edge_cases() {
    assert_eq!(clean(""), "");
    assert_eq!(clean("''"), "''''");
    assert_eq!(clean("\\XA7\\"), "\\XA7\\");
    assert_eq!(clean("\\XA7\\\\XB6\\"), "\\XA7\\\\XB6\\");
}

#[test]
fn clean_text_for_send_malformed_input() {
    assert_eq!(
        clean("incomplete \\X and 'quotes'"),
        "incomplete \\\\X and ''quotes''"
    );
    assert_eq!(clean("partial \\XA and more"), "partial \\\\XA and more");
}

#[test]
fn clean_text_for_send_hex_sequence_detection() {
    assert_eq!(
        clean("\\X2\\03B103B203B3\\X0\\"),
        "\\X2\\03B103B203B3\\X0\\"
    );
    assert_eq!(
        clean("\\X4\\001F600001F638\\X0\\"),
        "\\X4\\001F600001F638\\X0\\"
    );
    assert_eq!(
        clean("start \\X2\\03C0\\X0\\ end"),
        "start \\X2\\03C0\\X0\\ end"
    );
}

#[test]
fn long_text_wraps_at_blanks_and_keeps_every_byte() {
    let model = StepDataStepModel::new();
    let mut writer = StepDataStepWriter::new(&model);
    writer.start_entity(&AsciiString::new("TEST")).unwrap();
    let words = "word ".repeat(40);
    writer.send_text(&AsciiString::new(&words)).unwrap();
    writer.end_entity().unwrap();
    let mut whole = Vec::new();
    for rank in 1..=writer.nb_lines() {
        let found = writer.line(rank).unwrap();
        assert!(found.length() <= 72, "line {rank} is above 72 characters");
        whole.extend_from_slice(found.string().as_bytes());
    }
    let expected = format!("TEST('{words}');");
    assert_eq!(String::from_utf8_lossy(&whole), expected);
}
