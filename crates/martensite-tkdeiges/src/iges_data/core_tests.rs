//! The parameter reader, the writer, the global section and the dump, on made-up records.
use super::{IgesDataParamCursor, IgesDataParamReader, iges_array1_item, iges_msg};
use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
use martensite_tkmath::gp::{Xy, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileParameter, InterfaceParamList, InterfaceParamType,
};

/// A reader over parameters given as a type and a text; the first one stands for the type number.
pub(super) fn reader_of(
    params: &[(InterfaceParamType, &str)],
) -> (IgesDataParamReader, Handle<InterfaceCheck>) {
    let list = Handle::new(InterfaceParamList::new(4));
    let live = list.get().unwrap();
    let mut head = InterfaceFileParameter::new();
    head.init_text("110", InterfaceParamType::ParamInteger);
    live.set_value(1, &head).unwrap();
    for (rank, (kind, text)) in params.iter().enumerate() {
        let mut parameter = InterfaceFileParameter::new();
        parameter.init_text(text, *kind);
        live.set_value(rank as i32 + 2, &parameter).unwrap();
    }
    let check = Handle::new(InterfaceCheck::new());
    let reader = IgesDataParamReader::new(&list, &check, 1, params.len() as i32 + 1, 0);
    (reader, check)
}

pub(super) const INT: InterfaceParamType = InterfaceParamType::ParamInteger;
const REAL: InterfaceParamType = InterfaceParamType::ParamReal;
const TEXT: InterfaceParamType = InterfaceParamType::ParamText;
const VOID: InterfaceParamType = InterfaceParamType::ParamVoid;
const ENUM: InterfaceParamType = InterfaceParamType::ParamEnum;

#[test]
fn integers_and_reals_advance_and_take_the_defaults() {
    let (mut pr, check) = reader_of(&[
        (INT, "42"),
        (VOID, ""),
        (REAL, "2.5D1"),
        (ENUM, "3E2"),
        (INT, "7"),
    ]);
    assert_eq!(pr.nb_params(), 5);
    let mut value = 0;
    assert!(pr.read_integer(&pr.current(), &mut value).unwrap());
    assert_eq!((value, pr.current_number()), (42, 2));
    assert!(
        pr.read_integer_text(&pr.current(), "void", &mut value)
            .unwrap()
    );
    assert_eq!(value, 0);
    let mut real = 0.0;
    assert!(pr.read_real(&pr.current(), &mut real).unwrap());
    assert_eq!(real, 25.0);
    assert!(pr.read_real_text(&pr.current(), "enum", &mut real).unwrap());
    assert_eq!(real, 300.0);
    assert!(pr.read_real(&pr.current(), &mut real).unwrap());
    assert_eq!(real, 7.0);
    assert!(pr.is_check_empty());
    assert_eq!(check.get().unwrap().nb_fails(), 0);
}

#[test]
fn a_wrong_parameter_is_reported_under_its_label() {
    let (mut pr, check) = reader_of(&[(TEXT, "3HABC"), (TEXT, "3HABC")]);
    let mut value = 0;
    assert!(
        !pr.read_integer_text(&pr.current(), "Count", &mut value)
            .unwrap()
    );
    assert!(pr.has_failed());
    let fail = check.get().unwrap().fail(1, true).unwrap();
    assert_eq!(
        fail.get().unwrap().string().as_bytes(),
        b"Count : not given as an Integer"
    );
    let mut real = 1.0;
    assert!(!pr.read_real_text(&pr.current(), "Size", &mut real).unwrap());
    assert_eq!(real, 0.0);
    let fail = check.get().unwrap().fail(2, true).unwrap();
    assert_eq!(
        fail.get().unwrap().string().as_bytes(),
        b"Size: not given as Real, rank 2"
    );
}

#[test]
fn reading_past_the_end_is_a_failure_under_its_label() {
    let (mut pr, check) = reader_of(&[(INT, "1")]);
    let mut value = 0;
    assert!(pr.read_integer(&pr.current(), &mut value).unwrap());
    assert!(!pr.read_integer(&pr.current(), &mut value).unwrap());
    assert_eq!(check.get().unwrap().nb_fails(), 0);
    assert!(
        !pr.read_integer_text(&pr.current(), "More", &mut value)
            .unwrap()
    );
    let fail = check.get().unwrap().fail(1, true).unwrap();
    assert_eq!(
        fail.get().unwrap().string().as_bytes(),
        b"More : Parameter number out of range"
    );
}

#[test]
fn booleans_are_exact_or_warned() {
    let (mut pr, check) = reader_of(&[(INT, "1"), (INT, "5"), (INT, "5"), (VOID, "")]);
    let mut value = false;
    assert!(
        pr.read_boolean(&pr.current(), "flag", &mut value, true)
            .unwrap()
    );
    assert!(value);
    assert!(
        !pr.read_boolean(&pr.current(), "flag", &mut value, true)
            .unwrap()
    );
    assert_eq!(check.get().unwrap().nb_fails(), 1);
    assert!(
        pr.read_boolean(&pr.current(), "flag", &mut value, false)
            .unwrap()
    );
    assert!(value);
    assert_eq!(check.get().unwrap().nb_warnings(), 1);
    assert!(
        pr.read_boolean(&pr.current(), "flag", &mut value, true)
            .unwrap()
    );
    assert!(!value);
}

#[test]
fn coordinates_are_read_together() {
    let (mut pr, _check) = reader_of(&[
        (REAL, "1."),
        (REAL, "2."),
        (REAL, "3."),
        (REAL, "4."),
        (REAL, "5."),
    ]);
    let mut xy = Xy::new(0.0, 0.0);
    assert!(
        pr.read_xy(&pr.current_list(1, 2), "point", &mut xy)
            .unwrap()
    );
    assert_eq!((xy.x(), xy.y()), (1.0, 2.0));
    let mut xyz = Xyz::new(0.0, 0.0, 0.0);
    let msg = iges_msg("XSTEP_89");
    assert!(pr.read_xyz(&pr.current_list(1, 3), &msg, &mut xyz).unwrap());
    assert_eq!((xyz.x(), xyz.y(), xyz.z()), (3.0, 4.0, 5.0));
    assert_eq!(pr.current_number(), 6);
}

#[test]
fn counted_texts_keep_their_content_and_warn_on_a_wrong_count() {
    let (mut pr, check) = reader_of(&[(TEXT, "3HABC"), (TEXT, "9HABC"), (TEXT, "ABC"), (VOID, "")]);
    let mut text = Handle::<HAsciiString>::default();
    assert!(pr.read_text(&pr.current(), "name", &mut text).unwrap());
    assert_eq!(text.get().unwrap().string().as_bytes(), b"ABC");
    assert!(pr.read_text(&pr.current(), "name", &mut text).unwrap());
    assert_eq!(text.get().unwrap().string().as_bytes(), b"ABC");
    assert_eq!(check.get().unwrap().nb_warnings(), 1);
    assert!(!pr.read_text(&pr.current(), "name", &mut text).unwrap());
    assert_eq!(check.get().unwrap().nb_fails(), 1);
    assert!(pr.read_text(&pr.current(), "name", &mut text).unwrap());
    assert_eq!(text.get().unwrap().string().as_bytes(), b"");
}

#[test]
fn lists_fill_arrays_from_their_index() {
    let (mut pr, _check) = reader_of(&[
        (INT, "1"),
        (VOID, ""),
        (REAL, "1.5"),
        (REAL, "2."),
        (TEXT, "1HA"),
        (VOID, ""),
    ]);
    let mut ints = Handle::default();
    assert!(
        pr.read_ints(&pr.current_list(2, 1), "ints", &mut ints, 1)
            .unwrap()
    );
    assert_eq!(iges_array1_item(&ints, 2, "test").unwrap(), 0);
    let mut reals = Handle::default();
    assert!(
        pr.read_reals(&pr.current_list(2, 1), "reals", &mut reals, 3)
            .unwrap()
    );
    assert_eq!(iges_array1_item(&reals, 4, "test").unwrap(), 2.0);
    let mut texts = Handle::default();
    assert!(
        pr.read_texts(&pr.current_list(2, 1), "texts", &mut texts, 1)
            .unwrap()
    );
    let first = iges_array1_item(&texts, 1, "test").unwrap();
    assert_eq!(first.get().unwrap().string().as_bytes(), b"A");
}

#[test]
fn a_term_goes_after_the_whole_item_a_list_starts_with() {
    let mut cursor = IgesDataParamCursor::with_items(1, 2, 3);
    assert!(cursor.set_xyz(true).is_err());
    let mut cursor = IgesDataParamCursor::with_items(1, 2, 0);
    cursor.set_one(false).unwrap_err();
    let mut cursor = IgesDataParamCursor::new(4);
    cursor.set_advance(false);
    assert!(!cursor.advance());
    assert_eq!((cursor.start(), cursor.limit(), cursor.count()), (4, 5, 1));
    assert_eq!(IgesDataParamCursor::from(4).item_size(), 1);
}
