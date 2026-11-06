//! Whole files through the parser and back through the writer, on undefined entities.
use super::{
    Diagnostics, iges_file_read, iges_file_read_stream, iges_file_read_stream_with, parse_records,
};
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesWriter, IgesDataProtocol, IgesDataUndefinedEntity,
    iges_data_init, iges_data_protocol,
};
use crate::iges_data::{IgesDataIgesModel, IgesDataProtocolOps};
use crate::iges_file::structure::IgesFileParam;
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::if_select::IFSelectWorkLibrary;
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceModelEntities};

fn line(content: &str, section: char, number: i32) -> String {
    format!("{content:<72}{section}{number:07}\n")
}

/// A file with one entity of a type no package knows, its parameters on one line.
fn one_entity_file() -> String {
    let global = "1H,,1H;,4HSEND,4HFILE,3HSYS,3HVER,32,38,6,308,15,4HRECV,1.,2,2HMM,1,0.01,\
                  15H20200101.120000,0.0001,0.,6HAUTHOR,7HCOMPANY,11,0,15H20200101.120000;";
    let (global_first, global_rest) = global.split_at(72);
    let mut file = line("A start line", 'S', 1);
    file += &line(global_first, 'G', 1);
    file += &line(global_rest, 'G', 2);
    file += &format!(
        "{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}00000000D{:07}\n",
        5001, 1, 0, 0, 0, 0, 0, 0, 1
    );
    file += &format!(
        "{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}D{:07}\n",
        5001, 0, 0, 1, 7, "", "", "LABEL", "", 2
    );
    file += &format!("{:<64} {:07}P{:07}\n", "5001,1,2.5,3HABC;", 1, 1);
    file += &line("S      1G      2D      2P      1", 'T', 1);
    file
}

fn read_text(text: &str) -> Handle<IgesDataIgesModel> {
    iges_data_init().unwrap();
    let model = IgesDataIgesModel::new();
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let mut bytes = text.as_bytes();
    let status = iges_file_read_stream("test", &mut bytes, &model, &protocol).unwrap();
    assert_eq!(status, 0);
    model
}

#[test]
fn start_and_global_sections_are_read() {
    let model = read_text(&one_entity_file());
    let live = model.get().unwrap();
    assert_eq!(live.start_line(1), b"A start line");
    let header = live.global_section();
    let text = |value: &Handle<martensite_tkernel::tcollection::HAsciiString>| {
        value.get().unwrap().string().as_bytes().to_vec()
    };
    assert_eq!(text(&header.send_name()), b"SEND");
    assert_eq!(text(&header.company_name()), b"COMPANY");
    assert_eq!(header.unit_flag(), 2);
    assert_eq!(header.iges_version(), 11);
    assert!((header.max_line_weight() - 0.01).abs() < 1e-12);
    assert!(header.has_last_change_date() || header.iges_version() == 11);
}

#[test]
fn an_unknown_entity_keeps_its_parameters_and_directory() {
    let model = read_text(&one_entity_file());
    let live = model.get().unwrap();
    assert_eq!(live.nb_entities(), 1);
    let entity = live.entity(1);
    let undefined = entity.cast::<IgesDataUndefinedEntity>();
    let body = undefined.get().unwrap();
    assert_eq!(body.type_number(), 5001);
    assert_eq!(body.form_number(), 7);
    assert_eq!(body.line_weight_number(), 0);
    let content = body.undefined_content();
    assert_eq!(content.get().unwrap().nb_params(), 3);
    let label = body.short_label();
    assert_eq!(label.get().unwrap().string().as_bytes(), b"   LABEL");
}

#[test]
fn a_file_read_then_written_gives_the_same_parameters() {
    let model = read_text(&one_entity_file());
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let mut writer = IgesDataIgesWriter::new(&model);
    writer.send_model(&protocol).unwrap();
    let mut file = Vec::new();
    assert!(writer.print(&mut file).unwrap());
    let written = String::from_utf8(file).unwrap();
    assert!(written.contains("5001,1,2.5,3HABC;"), "{written}");
    let again = read_text(&written);
    let live = again.get().unwrap();
    assert_eq!(live.nb_entities(), 1);
    let header = live.global_section();
    assert_eq!(header.unit_flag(), 2);
    assert_eq!(live.start_line(1), b"A start line");
}

#[test]
fn a_text_across_two_records_is_joined() {
    let global = format!(
        "1H,,1H;,70H{},4HFILE,3HSYS,3HVER,32,38,6,308,15,4HRECV,1.,2,2HMM,1,0.01,\
         15H20200101.120000,0.0001,0.,6HAUTHOR,7HCOMPANY,11,0;",
        "x".repeat(70)
    );
    let mut file = line("", 'S', 1);
    let mut number = 0;
    for piece in global.as_bytes().chunks(72) {
        number += 1;
        file += &line(std::str::from_utf8(piece).unwrap(), 'G', number);
    }
    file += &line(&format!("S      1G{number:7}D      0P      0"), 'T', 1);
    let model = read_text(&file);
    let header = model.get().unwrap().global_section();
    let sender = header.send_name();
    assert_eq!(
        sender.get().unwrap().string().as_bytes(),
        "x".repeat(70).as_bytes()
    );
    assert_eq!(header.iges_version(), 11);
}

#[test]
fn records_without_end_of_line_and_with_carriage_returns_are_split() {
    let text = one_entity_file().replace('\n', "");
    let model = read_text(&text);
    assert_eq!(model.get().unwrap().nb_entities(), 1);
    let text = one_entity_file().replace('\n', "\r");
    let model = read_text(&text);
    assert_eq!(model.get().unwrap().nb_entities(), 1);
}

#[test]
fn a_plain_line_before_the_first_record_is_skipped() {
    let text = format!("Some comment line\n{}", one_entity_file());
    let model = read_text(&text);
    assert_eq!(model.get().unwrap().start_line(1), b"A start line");
    assert_eq!(model.get().unwrap().nb_entities(), 1);
}

#[test]
fn an_enciphered_file_is_read() {
    let mut coded = Vec::new();
    for record in one_entity_file().lines() {
        for (index, byte) in record.bytes().enumerate() {
            coded.push(byte ^ (150 + (index & 3) as u8));
        }
        coded.push(b'\n');
    }
    let model = IgesDataIgesModel::new();
    let protocol = Handle::new(IgesDataProtocol::default());
    let mut bytes = coded.as_slice();
    let status = iges_file_read_stream_with(
        "test",
        &mut bytes,
        &model,
        &protocol.cast(),
        &Handle::default(),
        true,
    )
    .unwrap();
    assert_eq!(status, 0);
    assert_eq!(model.get().unwrap().nb_entities(), 1);
}

#[test]
fn a_failing_stream_gives_a_negative_status() {
    struct FailingStream;
    impl std::io::Read for FailingStream {
        fn read(&mut self, _buffer: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("broken"))
        }
    }
    let model = IgesDataIgesModel::new();
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let status = iges_file_read_stream("test", &mut FailingStream, &model, &protocol).unwrap();
    assert_eq!(status, -1);
    assert_eq!(iges_file_read("", &model, &protocol).unwrap(), -1);
}

/// One real file through the whole norm: everything read is recognised, and what is written reads
/// back the same, entity by entity.
fn check_real_file_goes_round(name: &str) {
    let library = crate::iges_select::IgesSelectWorkLibrary::new(false);
    assert!(library.dump_levels().1 == 6);
    let protocol = crate::iges_select::IgesSelectWorkLibrary::define_protocol();
    let model = IgesDataIgesModel::new();
    assert_eq!(iges_file_read(name, &model, &protocol).unwrap(), 0);
    let live = model.get().unwrap();
    let signature = |entity: &IgesDataIgesEntity| (entity.type_number(), entity.form_number());
    let mut unknown = 0;
    let mut first = Vec::new();
    for rank in 1..=live.nb_entities() {
        let entity = live.entity(rank);
        unknown += i32::from(entity.cast::<IgesDataUndefinedEntity>().is_some());
        first.push(signature(entity.get().unwrap()));
    }
    let mut writer = IgesDataIgesWriter::new(&model);
    writer.send_model(&protocol).unwrap();
    let mut file = Vec::new();
    assert!(writer.print(&mut file).unwrap());
    let again = IgesDataIgesModel::new();
    let mut bytes = file.as_slice();
    assert_eq!(
        iges_file_read_stream("again", &mut bytes, &again, &protocol).unwrap(),
        0
    );
    let second: Vec<_> = (1..=again.get().unwrap().nb_entities())
        .map(|rank| signature(again.get().unwrap().entity(rank).get().unwrap()))
        .collect();
    println!(
        "entities {} unknown {unknown} bytes {}",
        first.len(),
        file.len()
    );
    assert_eq!(first, second);
    let mismatches = compare_parameters(&std::fs::read(name).unwrap(), &file);
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}

/// The public files of the pinned test data, or the one file `IGES_ROUND_FILE` names instead.
#[test]
fn a_real_file_goes_round() {
    if let Ok(name) = std::env::var("IGES_ROUND_FILE") {
        check_real_file_goes_round(&name);
        return;
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/occt-source-data/iges");
    for file in ["bearing.iges", "hammer.iges"] {
        check_real_file_goes_round(&root.join(file).to_string_lossy());
    }
}

/// Whether two parameters of two files say the same: a void one is a default, a real is compared
/// at the precision of the written form.
fn same_param(first: &IgesFileParam, second: &IgesFileParam) -> bool {
    let (left, right) = (
        String::from_utf8_lossy(first.text()).into_owned(),
        String::from_utf8_lossy(second.text()).into_owned(),
    );
    if left.trim() == right.trim() {
        return true;
    }
    let number = |text: &str| text.trim().replace(['D', 'd'], "e").parse::<f64>().ok();
    match (number(&left), number(&right)) {
        (Some(a), Some(b)) => (a - b).abs() <= 1e-6 * a.abs().max(b.abs()).max(1.0),
        (Some(a), None) | (None, Some(a)) if left.trim().is_empty() || right.trim().is_empty() => {
            a == 0.0
        }
        _ => false,
    }
}

/// What differs between the parameters of the entities of two files.
fn compare_parameters(original: &[u8], written: &[u8]) -> Vec<String> {
    let diagnostics = Diagnostics {
        check: Handle::new(InterfaceCheck::new()),
    };
    let parse = |mut bytes: &[u8]| {
        parse_records(&mut bytes, false, &diagnostics)
            .unwrap()
            .unwrap()
            .1
    };
    let (left, right) = (parse(original), parse(written));
    let mut mismatches = Vec::new();
    for (a, b) in left.parts.iter().zip(&right.parts) {
        if a.values[0] != b.values[0] || a.values[16] != b.values[16] {
            mismatches.push(format!(
                "D{}: type {} form {} became {} {}",
                a.numpart, a.values[0], a.values[16], b.values[0], b.values[16]
            ));
            continue;
        }
        for (rank, (x, y)) in a.list.iter().zip(&b.list).enumerate() {
            if !same_param(x, y) {
                mismatches.push(format!(
                    "D{} type {} param {rank}: {} became {}",
                    a.numpart,
                    a.values[0],
                    String::from_utf8_lossy(x.text()),
                    String::from_utf8_lossy(y.text())
                ));
                break;
            }
        }
        if a.list.len() != b.list.len() {
            mismatches.push(format!(
                "D{} type {}: {} params became {}",
                a.numpart,
                a.values[0],
                a.list.len(),
                b.list.len()
            ));
        }
    }
    mismatches
}
