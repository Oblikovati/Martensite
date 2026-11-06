//! Tests of the RW classes over hand-built reader records and a real writer: what each one reads
//! into its entity, and the text it writes back.
use crate::step_rw::*;
use martensite_tkernel::resource::FormatType;
use martensite_tkxsbase::interface::{InterfaceParamType as Kind, InterfaceProtocol};

/// One parameter of a test record.
type TestParam<'a> = (&'a [u8], Kind);

/// A reader over records numbered from 1, each `(ident, type, parameters)`.
fn reader_over(records: &[(&str, &str, &[TestParam<'_>])]) -> Handle<StepDataStepReaderData> {
    let count = records.len() as i32;
    let params = records.iter().map(|record| record.2.len()).sum::<usize>() as i32;
    let data = Handle::new(StepDataStepReaderData::new(
        0,
        count,
        params,
        FormatType::Utf8,
    ));
    for (rank, (ident, record_type, list)) in records.iter().enumerate() {
        let num = rank as i32 + 1;
        data.set_record(
            num,
            ident.as_bytes(),
            record_type.as_bytes(),
            list.len() as i32,
        );
        for (value, kind) in *list {
            data.add_step_param(num, value, *kind, 0).unwrap();
        }
        data.init_params(num).unwrap();
    }
    data
}

/// What a writer makes of one entity whose parameters `write` sends.
fn written_text(
    source_type: &str,
    write: impl FnOnce(&mut StepDataStepWriter) -> Result<(), Failure>,
) -> String {
    let mut writer = StepDataStepWriter::new(&StepDataStepModel::new());
    step_start_entity(&mut writer, source_type).unwrap();
    write(&mut writer).unwrap();
    writer.end_entity().unwrap();
    let line = writer.line(1).unwrap();
    String::from_utf8_lossy(line.string().as_bytes())
        .trim()
        .to_string()
}

const POINT_LIST: [TestParam<'static>; 3] = [
    (b"1.", Kind::ParamReal),
    (b"2.", Kind::ParamReal),
    (b"3.", Kind::ParamReal),
];

#[test]
fn a_cartesian_point_reads_inline_coordinates_and_writes_them_back() {
    let name: &[u8] = b"'p'";
    let data = reader_over(&[
        ("$1", "(", &POINT_LIST),
        (
            "#2",
            "CARTESIAN_POINT",
            &[(name, Kind::ParamText), (b"$1", Kind::ParamSub)],
        ),
    ]);
    let check = Handle::new(InterfaceCheck::new());
    let point = Handle::new(StepGeomCartesianPoint::new());
    RwStepGeomRwCartesianPoint::new()
        .read_step(&data, 2, &check, &point)
        .unwrap();
    assert!(!check.has_failed(), "{:?}", check.nb_fails());
    assert_eq!(point.coordinates(), [1.0, 2.0, 3.0]);
    assert_eq!(point.nb_coordinates(), 3);
    assert_eq!(point.name().string().as_bytes(), b"p");
    let text = written_text("CARTESIAN_POINT", |writer| {
        RwStepGeomRwCartesianPoint::new().write_step(writer, &point)
    });
    assert_eq!(text, "CARTESIAN_POINT('p',(1.,2.,3.));");
}

#[test]
fn a_point_with_too_many_coordinates_warns_and_keeps_three() {
    let list: [TestParam<'_>; 4] = [
        (b"1.", Kind::ParamReal),
        (b"2.", Kind::ParamReal),
        (b"3.", Kind::ParamReal),
        (b"4.", Kind::ParamReal),
    ];
    let data = reader_over(&[
        ("$1", "(", &list),
        (
            "#2",
            "CARTESIAN_POINT",
            &[(b"'p'", Kind::ParamText), (b"$1", Kind::ParamSub)],
        ),
    ]);
    let check = Handle::new(InterfaceCheck::new());
    let point = Handle::new(StepGeomCartesianPoint::new());
    RwStepGeomRwCartesianPoint::new()
        .read_step(&data, 2, &check, &point)
        .unwrap();
    assert!(check.has_warnings());
    assert_eq!(point.nb_coordinates(), 3);
}

#[test]
fn a_wrong_parameter_count_fails_and_leaves_the_entity_unset() {
    let data = reader_over(&[("#1", "DIRECTION", &[(b"'d'", Kind::ParamText)])]);
    let check = Handle::new(InterfaceCheck::new());
    let direction = Handle::new(StepGeomDirection::new());
    RwStepGeomRwDirection::new()
        .read_step(&data, 1, &check, &direction)
        .unwrap();
    assert!(check.has_failed());
    assert_eq!(direction.nb_direction_ratios(), 0);
}

#[test]
fn an_si_unit_reads_its_enumerations_and_writes_the_derived_field() {
    let data = reader_over(&[(
        "#1",
        "SI_UNIT",
        &[
            (b"*", Kind::ParamMisc),
            (b".MILLI.", Kind::ParamEnum),
            (b".METRE.", Kind::ParamEnum),
        ],
    )]);
    let check = Handle::new(InterfaceCheck::new());
    let unit = Handle::new(StepBasicSiUnit::new());
    RwStepBasicRwSiUnit::new()
        .read_step(&data, 1, &check, &unit)
        .unwrap();
    assert!(!check.has_failed());
    assert!(unit.has_prefix());
    assert_eq!(unit.prefix(), StepBasicSiPrefix::SpMilli);
    assert_eq!(unit.name(), StepBasicSiUnitName::SunMetre);
    let text = written_text("SI_UNIT", |writer| {
        RwStepBasicRwSiUnit::new().write_step(writer, &unit)
    });
    assert_eq!(text, "SI_UNIT(*,.MILLI.,.METRE.);");
}

#[test]
fn an_unknown_enumeration_text_is_a_failure_and_keeps_the_default() {
    let data = reader_over(&[(
        "#1",
        "SI_UNIT",
        &[
            (b"*", Kind::ParamMisc),
            (b"$", Kind::ParamVoid),
            (b".PARSEC.", Kind::ParamEnum),
        ],
    )]);
    let check = Handle::new(InterfaceCheck::new());
    let unit = Handle::new(StepBasicSiUnit::new());
    RwStepBasicRwSiUnit::new()
        .read_step(&data, 1, &check, &unit)
        .unwrap();
    assert!(check.has_failed());
    assert!(!unit.has_prefix());
    assert_eq!(unit.name(), StepBasicSiUnitName::SunMetre);
}

#[test]
fn an_optional_field_left_undefined_is_written_undefined() {
    let data = reader_over(&[(
        "#1",
        "ACTION_METHOD",
        &[
            (b"'n'", Kind::ParamText),
            (b"$", Kind::ParamVoid),
            (b"'c'", Kind::ParamText),
            (b"'p'", Kind::ParamText),
        ],
    )]);
    let check = Handle::new(InterfaceCheck::new());
    let method = Handle::new(StepBasicActionMethod::new());
    RwStepBasicRwActionMethod::new()
        .read_step(&data, 1, &check, &method)
        .unwrap();
    assert!(!check.has_failed());
    assert!(!method.has_description());
    assert!(method.description().is_null());
    let text = written_text("ACTION_METHOD", |writer| {
        RwStepBasicRwActionMethod::new().write_step(writer, &method)
    });
    assert_eq!(text, "ACTION_METHOD('n',$,'c','p');");
}

#[test]
fn an_entity_reference_of_the_right_type_reads_and_shares() {
    let data = reader_over(&[
        ("#1", "DIRECTION", &[(b"''", Kind::ParamText)]),
        (
            "#2",
            "VECTOR",
            &[
                (b"'v'", Kind::ParamText),
                (b"#1", Kind::ParamIdent),
                (b"2.5", Kind::ParamReal),
            ],
        ),
    ]);
    let direction = Handle::new(StepGeomDirection::new());
    data.bind_entity(1, &direction.cast()).unwrap();
    let check = Handle::new(InterfaceCheck::new());
    let vector = Handle::new(StepGeomVector::new());
    let rw = RwStepGeomRwVector::new();
    rw.read_step(&data, 2, &check, &vector).unwrap();
    assert!(!check.has_failed());
    assert_eq!(vector.orientation(), direction);
    assert_eq!(vector.magnitude(), 2.5);
    let mut shared = InterfaceEntityIterator::new();
    rw.share(&vector, &mut shared).unwrap();
    assert_eq!(shared.nb_entities(), 1);
    assert_eq!(
        shared.value().unwrap(),
        direction.cast::<StandardTransient>()
    );
}

#[test]
fn a_reference_of_another_type_fails_and_stays_null() {
    let data = reader_over(&[
        ("#1", "DIRECTION", &[(b"''", Kind::ParamText)]),
        (
            "#2",
            "VECTOR",
            &[
                (b"'v'", Kind::ParamText),
                (b"#1", Kind::ParamIdent),
                (b"2.5", Kind::ParamReal),
            ],
        ),
    ]);
    data.bind_entity(1, &Handle::new(StepGeomVector::new()).cast())
        .unwrap();
    let check = Handle::new(InterfaceCheck::new());
    let vector = Handle::new(StepGeomVector::new());
    RwStepGeomRwVector::new()
        .read_step(&data, 2, &check, &vector)
        .unwrap();
    assert!(check.has_failed());
    assert!(vector.orientation().is_null());
}

#[test]
fn the_vector_check_refuses_a_null_magnitude() {
    let vector = Handle::new(StepGeomVector::new());
    vector.init(
        &Handle::new(HAsciiString::new("v")),
        &Handle::new(StepGeomDirection::new()),
        0.0,
    );
    let check = Handle::new(InterfaceCheck::new());
    RwStepGeomRwVector::new()
        .check(&vector, &empty_share_tool(), &check)
        .unwrap();
    assert!(check.has_failed());
}

/// A share tool over an empty model, for the checks that do not walk the graph.
fn empty_share_tool() -> InterfaceShareTool {
    let model = StepDataStepModel::new();
    let protocol: Handle<dyn InterfaceProtocol> = Handle::new(StepDataProtocol::default()).cast();
    InterfaceShareTool::with_protocol(&model.cast(), &protocol).unwrap()
}
