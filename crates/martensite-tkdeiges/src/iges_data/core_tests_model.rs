//! The writer, the global section, the entities, the undefined entities and the dump.
use super::core_tests::{INT, reader_of};
use super::{
    IgesDataDefList, IgesDataDefType, IgesDataDirPart, IgesDataDirValues, IgesDataFreeFormatEntity,
    IgesDataGlobalSection, IgesDataIgesDumper, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataProtocolOps, IgesDataUndefinedEntity,
    iges_data_init, iges_data_protocol, iges_dump_list_header, iges_dump_rect_vals,
    iges_dump_string, iges_dump_vals, iges_dump_xyz_l,
};
use martensite_tkernel::{
    standard::{Handle, RuntimeTyped},
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::{GeneralTransform3, TransformForm, Xy, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceEntityList, InterfaceInterfaceModel, InterfaceParamSet,
    InterfaceParamType,
};

const VOID: InterfaceParamType = InterfaceParamType::ParamVoid;
const TEXT: InterfaceParamType = InterfaceParamType::ParamText;

fn entity_in(model: &Handle<IgesDataIgesModel>, type_number: i32) -> Handle<IgesDataIgesEntity> {
    let entity = Handle::new(IgesDataIgesEntity::default());
    entity.get().unwrap().init_type_and_form(type_number, 0);
    model.get().unwrap().add_entity(&entity.cast()).unwrap();
    entity
}

#[test]
fn the_writer_sends_values_and_cuts_lines() {
    let model = IgesDataIgesModel::new();
    let entity = entity_in(&model, 5001);
    let mut writer = IgesDataIgesWriter::new(&model);
    writer.section_s().unwrap();
    writer.send_start_line(&[b'a'; 100]);
    writer.section_g(&IgesDataGlobalSection::new()).unwrap();
    writer.sections_dp().unwrap();
    writer.dir_part(&entity).unwrap();
    writer.own_params(&entity).unwrap();
    writer.send(12).unwrap();
    writer.send(0.5).unwrap();
    writer.send_boolean(true).unwrap();
    writer.send_void().unwrap();
    writer.send(Handle::new(HAsciiString::new("ABC"))).unwrap();
    writer.send_entity(&entity, true).unwrap();
    writer.send(&entity).unwrap();
    writer.send(Xy::new(1.0, 2.0)).unwrap();
    writer.send(Xyz::new(1.0, 2.0, 3.0)).unwrap();
    writer
        .send_string(&Handle::new(HAsciiString::new("RAW")))
        .unwrap();
    for _ in 0..40 {
        writer.send(1000).unwrap();
    }
    writer.associativities(&entity).unwrap();
    writer.properties(&entity).unwrap();
    writer.end_entity().unwrap();
    writer.section_t().unwrap();
    let start = writer.section_strings(1);
    assert_eq!(start.get().unwrap().sequence(|lines| lines.length()), 2);
    let params = writer.section_strings(3);
    let params = params.get().unwrap();
    let first = params.sequence(|lines| lines.value(1).unwrap().clone());
    let line = first.get().unwrap().string().as_bytes().to_vec();
    assert!(
        String::from_utf8_lossy(&line).starts_with("5001,12,0.5,1,,3HABC,-1,1,1.,2.,1.,2.,3.,RAW,"),
        "{}",
        String::from_utf8_lossy(&line)
    );
    assert!(params.sequence(|lines| lines.length()) >= 2);
    let mut file = Vec::new();
    assert!(writer.print(&mut file).unwrap());
    let text = String::from_utf8(file).unwrap();
    assert!(text.lines().all(|row| row.len() == 80), "{text}");
    assert!(text.contains("D0000001"));
    assert!(text.lines().last().unwrap().ends_with("T0000001"));
}

#[test]
fn the_sections_must_come_in_order() {
    let model = IgesDataIgesModel::new();
    let mut writer = IgesDataIgesWriter::new(&model);
    assert!(writer.section_g(&IgesDataGlobalSection::new()).is_err());
    let mut file = Vec::new();
    assert!(writer.print(&mut file).is_err());
}

#[test]
fn the_enciphered_form_is_written_on_demand() {
    let model = IgesDataIgesModel::new();
    let mut writer = IgesDataIgesWriter::new(&model);
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    writer.send_model(&protocol).unwrap();
    *writer.write_mode() = 10;
    let mut file = Vec::new();
    assert!(writer.print(&mut file).unwrap());
    assert!(file.starts_with(b"                              ***  EUCLID/STRIM"));
}

fn section_with_values() -> IgesDataGlobalSection {
    let mut section = IgesDataGlobalSection::new();
    section.set_send_name(&Handle::new(HAsciiString::new("4HSEND")));
    section.set_scale(2.0);
    section.set_unit_flag(2);
    section.set_unit_name(&Handle::new(HAsciiString::new("MM")));
    section.set_max_line_weight(0.5);
    section.set_resolution(0.001);
    section.set_max_coord(10.0);
    section.set_date(&Handle::new(HAsciiString::new("20200102.030405")));
    section.set_last_change_date(&Handle::new(HAsciiString::new("20200102.030405")));
    section
}

#[test]
fn the_global_section_comes_back_from_its_parameters() {
    let section = section_with_values();
    let sender = section.send_name();
    assert_eq!(sender.get().unwrap().string().as_bytes(), b"SEND");
    let params = section.params().unwrap();
    assert_eq!(params.get().unwrap().nb_params(), 25);
    let mut again = IgesDataGlobalSection::new();
    let check = Handle::new(InterfaceCheck::new());
    again.init(&params, &check).unwrap();
    assert_eq!(check.get().unwrap().nb_fails(), 0);
    assert_eq!(again.scale(), 2.0);
    assert_eq!(again.unit_flag(), 2);
    assert_eq!(again.max_coord(), 10.0);
    assert_eq!(again.max_line_weight(), 0.5);
    let date = again.last_change_date();
    assert_eq!(date.get().unwrap().string().as_bytes(), b"20200102.030405");
}

#[test]
fn a_short_section_fails_and_a_missing_unit_flag_comes_from_its_name() {
    let set = Handle::new(InterfaceParamSet::new(20, 1));
    let live = set.get().unwrap();
    for rank in 1..=15 {
        let (text, kind): (&[u8], _) = if rank == 15 {
            (b"2HCM", TEXT)
        } else {
            (b"", VOID)
        };
        live.append(text, text.len() as i32, kind, 0).unwrap();
    }
    let mut section = IgesDataGlobalSection::new();
    let check = Handle::new(InterfaceCheck::new());
    section.init(&set, &check).unwrap();
    assert_eq!(check.get().unwrap().nb_fails(), 1);
    assert_eq!(section.unit_flag(), 10);
}

#[test]
fn dates_convert_between_their_forms() {
    let text = |date: &Handle<HAsciiString>| date.get().unwrap().string().as_bytes().to_vec();
    let long = IgesDataGlobalSection::new_date_string(2020, 1, 2, 3, 4, 5, -1);
    assert_eq!(text(&long), b"20200102.030405");
    let short = IgesDataGlobalSection::new_date_string(1999, 1, 2, 3, 4, 5, 0);
    assert_eq!(text(&short), b"990102.030405");
    let early = IgesDataGlobalSection::new_date_string(2005, 1, 2, 3, 4, 5, 0);
    assert_eq!(text(&early), b"050102.030405");
    let readable = IgesDataGlobalSection::new_date_string_from(&long, 1);
    assert_eq!(text(&readable), b"2020-01-02:03-04-05");
    let from_short = IgesDataGlobalSection::new_date_string_from(&short, 1);
    assert_eq!(text(&from_short), b"1999-01-02:03-04-05");
    let not_a_date = Handle::new(HAsciiString::new("nope"));
    assert_eq!(
        text(&IgesDataGlobalSection::new_date_string_from(&not_a_date, 1)),
        b"nope"
    );
}

#[test]
fn the_maximum_coordinate_only_grows() {
    let mut section = IgesDataGlobalSection::new();
    assert!(!section.has_max_coord());
    section.max_max_coords(&Xyz::new(1.0, -3.0, 2.0));
    assert_eq!(section.max_coord(), 3.0);
    section.max_max_coord(2.0);
    assert_eq!(section.max_coord(), 3.0);
    section.set_max_coord(-1.0);
    assert!(!section.has_max_coord());
}

#[test]
fn the_directory_of_an_entity_keeps_what_is_set() {
    let entity = Handle::new(IgesDataIgesEntity::default());
    let live = entity.get().unwrap();
    live.init_type_and_form(110, 3);
    live.init_status(1, 2, 3, 2);
    assert_eq!(
        (
            live.blank_status(),
            live.subordinate_status(),
            live.use_flag(),
            live.hierarchy_status()
        ),
        (1, 2, 3, 2)
    );
    live.init_line_font(&Handle::default(), 4);
    assert_eq!(
        (live.def_line_font(), live.rank_line_font()),
        (IgesDataDefType::DefValue, 4)
    );
    live.init_color(&Handle::default(), 2);
    assert_eq!(live.def_color(), IgesDataDefType::DefValue);
    live.init_level(&Handle::default(), 7);
    assert_eq!(
        (live.def_level(), live.level()),
        (IgesDataDefList::DefOne, 7)
    );
    assert_eq!(live.def_view(), IgesDataDefList::DefNone);
    assert!(live.single_view().is_null() && live.view_list().is_null());
    live.set_label(&Handle::new(HAsciiString::new("NAME")), 12);
    assert!(live.has_short_label() && live.has_name());
    let name = live.name_value().unwrap();
    assert_eq!(name.get().unwrap().string().as_bytes(), b"NAME(12)");
    assert!(live.has_sub_script_number());
    live.set_label(&Handle::new(HAsciiString::new("NAME")), -1);
    assert_eq!(live.sub_script_number(), 0);
    let name = live.name_value().unwrap();
    assert_eq!(name.get().unwrap().string().as_bytes(), b"NAME");
    live.init_misc(&Handle::default(), &Handle::default(), 3);
    live.set_line_weight(1.0, 10.0, 5);
    assert_eq!(live.line_weight(), 6.0);
    live.set_line_weight(1.0, 10.0, 1);
    assert_eq!(live.line_weight(), 30.0);
    assert_eq!(live.location().unwrap().form(), TransformForm::Identity);
    assert_eq!(
        live.vector_location().unwrap().form(),
        TransformForm::Identity
    );
    assert_eq!(
        live.compound_location().unwrap().form(),
        TransformForm::Identity
    );
}

#[test]
fn lists_of_an_entity_stay_apart_and_are_marked_when_read() {
    let owner = Handle::new(IgesDataIgesEntity::default());
    let other = Handle::new(IgesDataIgesEntity::default());
    let live = owner.get().unwrap();
    assert!(!live.are_present_properties());
    live.add_property(&other).unwrap();
    assert_eq!(live.nb_properties(), 1);
    assert_eq!(
        live.nb_typed_properties(IgesDataIgesEntity::static_type()),
        1
    );
    assert_eq!(
        live.typed_property(IgesDataIgesEntity::static_type(), 0)
            .unwrap(),
        other
    );
    live.remove_property(&other).unwrap();
    assert_eq!(live.nb_properties(), 0);
    IgesDataIgesEntity::associate(&other, &owner).unwrap();
    assert_eq!(live.nb_associativities(), 1);
    assert_eq!(live.associativities().nb_entities(), 1);
    IgesDataIgesEntity::dissociate(&other, &owner).unwrap();
    assert_eq!(live.nb_associativities(), 0);
    let mut read = InterfaceEntityList::new();
    read.append(&other.cast()).unwrap();
    live.load_properties(&read);
    live.load_associativities(&InterfaceEntityList::new());
    assert!(live.are_present_properties() && live.are_present_associativities());
    live.clear_properties();
    live.clear_associativities();
    assert!(live.are_present_properties());
    assert!(!live.has_one_parent());
    assert!(live.unique_parent().is_err());
}

#[test]
fn the_undefined_entity_writes_back_what_it_read() {
    let (mut pr, _check) = reader_of(&[(INT, "3"), (TEXT, "2HAB"), (VOID, "")]);
    let entity = IgesDataUndefinedEntity::new();
    let data = Handle::new(IgesDataIgesReaderData::new(1, 3));
    entity.read_own_params(&data, &mut pr).unwrap();
    assert_eq!(pr.current_number(), 4);
    let model = IgesDataIgesModel::new();
    let mut writer = IgesDataIgesWriter::new(&model);
    writer.section_s().unwrap();
    writer.section_g(&IgesDataGlobalSection::new()).unwrap();
    writer.sections_dp().unwrap();
    entity.write_own_params(&mut writer).unwrap();
    writer.end_entity().unwrap();
    let params = writer.section_strings(3);
    let first = params
        .get()
        .unwrap()
        .sequence(|lines| lines.value(1).unwrap().clone());
    assert_eq!(first.get().unwrap().string().as_bytes(), b",3,2HAB,;");
}

#[test]
fn directory_errors_of_an_undefined_entity_show_in_its_definitions() {
    let entity = IgesDataUndefinedEntity::new();
    let data = Handle::new(IgesDataIgesReaderData::new(2, 3));
    let mut part = IgesDataDirPart::default();
    let mut fields = IgesDataDirValues::default();
    fields.values[3] = -100;
    fields.values[14] = 100;
    fields.values[5] = 1000;
    fields.subscript = *b"AB      \0";
    part.init(&fields);
    let check = Handle::new(InterfaceCheck::new());
    assert!(!entity.read_dir(&data, &mut part, &check).unwrap());
    assert!(!entity.is_ok_dir_part());
    assert_eq!(entity.def_line_font(), IgesDataDefType::ErrorRef);
    assert_eq!(entity.def_view(), IgesDataDefList::ErrorSeveral);
    assert_eq!(entity.def_color(), IgesDataDefType::ErrorRef);
    assert!(!entity.has_sub_script_number());
    assert_eq!(part.values().subscript, *b"        \0");
    assert_eq!(check.get().unwrap().nb_fails(), 4);
    entity.set_ok_dir_part();
    assert_eq!(entity.dir_status(), 0);
}

#[test]
fn a_free_format_entity_writes_its_negative_pointers() {
    let model = IgesDataIgesModel::new();
    let target = entity_in(&model, 7);
    let free = IgesDataFreeFormatEntity::new();
    free.set_type_number(9);
    free.set_form_number(2);
    assert_eq!((free.type_number(), free.form_number()), (9, 2));
    free.add_literal_text(INT, "12").unwrap();
    free.add_entity(InterfaceParamType::ParamIdent, &target, true)
        .unwrap();
    free.add_entity(InterfaceParamType::ParamIdent, &target, false)
        .unwrap();
    assert!(free.is_negative_pointer(2) && !free.is_negative_pointer(3));
    assert!(free.is_param_entity(3).unwrap());
    let (is_entity, _, entity, _) = free.param_data(2).unwrap();
    assert!(is_entity && entity == target);
    let mut writer = IgesDataIgesWriter::new(&model);
    writer.section_s().unwrap();
    writer.section_g(&IgesDataGlobalSection::new()).unwrap();
    writer.sections_dp().unwrap();
    free.write_own_params(&mut writer).unwrap();
    writer.end_entity().unwrap();
    let params = writer.section_strings(3);
    let first = params
        .get()
        .unwrap()
        .sequence(|lines| lines.value(1).unwrap().clone());
    assert_eq!(first.get().unwrap().string().as_bytes(), b",12,-1,1;");
    free.clear_negative_pointers();
    assert!(free.negative_pointers().is_null());
}

#[test]
fn the_dump_says_what_it_can_at_each_level() {
    let model = IgesDataIgesModel::new();
    let entity = entity_in(&model, 110);
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let dumper = IgesDataIgesDumper::new(&model, &protocol);
    let text = |level: i32| {
        let mut out = Vec::new();
        dumper.dump(&entity, &mut out, level, -1).unwrap();
        String::from_utf8(out).unwrap()
    };
    assert_eq!(text(-1), "");
    assert!(text(0).contains("1:D1"));
    assert!(text(1).contains("Type:110"));
    assert!(text(3).contains("Complete Directory Part"));
    assert!(text(4).contains("Dump Impossible, n0:id:1:D1"));
    let mut null = Vec::new();
    dumper
        .dump(&Handle::<IgesDataIgesEntity>::default(), &mut null, 3, -1)
        .unwrap();
    assert_eq!(String::from_utf8(null).unwrap(), "\n(Null)\n");
}

#[test]
fn the_dump_of_lists_depends_on_the_level() {
    let text = |level: i32| {
        let mut out = Vec::new();
        iges_dump_vals(&mut out, level, 1, 2, |rank| Ok(f64::from(rank) / 2.0)).unwrap();
        String::from_utf8(out).unwrap()
    };
    assert_eq!(text(4), " (Count : 2) [content : ask level > 4]");
    assert_eq!(text(5), " (Count : 2) : 0.5 1");
    assert_eq!(text(-1), " (Count : 2)");
    let mut out = Vec::new();
    iges_dump_rect_vals(&mut out, 5, (1, 2), (1, 1), |col, row| Ok(col + row)).unwrap();
    assert_eq!(
        String::from_utf8(out).unwrap(),
        " (Row :1 - 1 ; Col :1 - 2)\nRow 1:[ 2 3 ]\n"
    );
    let mut out = Vec::new();
    let trsf = GeneralTransform3::default();
    iges_dump_xyz_l(&mut out, 6, &Xyz::new(1.0, 2.0, 3.0), &trsf).unwrap();
    iges_dump_list_header(&mut out, 2, 1).unwrap();
    iges_dump_string(&mut out, &Handle::default()).unwrap();
    assert_eq!(
        String::from_utf8(out).unwrap(),
        " (1,2,3) (Empty List)(undefined)"
    );
}

#[test]
fn the_template_model_says_it_comes_from_this_kernel() {
    iges_data_init().unwrap();
    let template = martensite_tkxsbase::interface::InterfaceModelBase::template("iges").unwrap();
    let model = template.cast::<IgesDataIgesModel>();
    let header = model.get().unwrap().global_section();
    let system = header.system_id();
    let text = system.get().unwrap().string().as_bytes().to_vec();
    assert!(
        text.starts_with(b"Martensite "),
        "{}",
        String::from_utf8_lossy(&text)
    );
    let processor = header.interface_version();
    assert!(
        processor
            .get()
            .unwrap()
            .string()
            .as_bytes()
            .starts_with(b"Martensite IGES processor")
    );
    assert_eq!(header.iges_version(), 11);
    let author = header.author_name();
    assert_eq!(author.get().unwrap().string().as_bytes(), b"Martensite");
    let mut dump = Vec::new();
    model.get().unwrap().dump_header(&mut dump, 0).unwrap();
    let text = String::from_utf8(dump).unwrap();
    assert!(text.contains("Martensite"));
    assert!(!text.contains("CASCADE"));
    let mut check = Handle::new(InterfaceCheck::new());
    model.get().unwrap().verify_check(&mut check);
    let fails = check.get().unwrap().fails(true);
    let texts: Vec<_> = fails
        .iter()
        .map(|fail| fail.get().unwrap().string().as_bytes().to_vec())
        .collect();
    assert!(
        texts.is_empty(),
        "{:?}",
        texts
            .iter()
            .map(|t| String::from_utf8_lossy(t).into_owned())
            .collect::<Vec<_>>()
    );
}
