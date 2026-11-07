//! The selections, signatures and modifiers on models made by hand.
use super::{
    IgesSelectAutoCorrect, IgesSelectChangeLevelList, IgesSelectChangeLevelNumber,
    IgesSelectComputeStatus, IgesSelectCounterOfLevelNumber, IgesSelectFileModifier,
    IgesSelectFloatFormat, IgesSelectIGESName, IgesSelectIGESTypeForm, IgesSelectSelectBasicGeom,
    IgesSelectSelectBypassGroup, IgesSelectSelectBypassSubfigure, IgesSelectSelectFaces,
    IgesSelectSelectLevelNumber, IgesSelectSelectName, IgesSelectSelectPCurves,
    IgesSelectSelectSubordinate, IgesSelectSelectVisibleStatus, IgesSelectSetGlobalParameter,
    IgesSelectSetLabel, IgesSelectSetVersion5, IgesSelectSignColor, IgesSelectSignLevelNumber,
    IgesSelectSignStatus, IgesSelectSplineToBSpline, IgesSelectUpdateCreationDate,
    IgesSelectUpdateFileName, IgesSelectUpdateLastChange, IgesSelectViewSorter,
    IgesSelectWorkLibrary,
};
use crate::iges_basic::{IgesBasic, IgesBasicGroup};
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesModel, IgesDataProtocolOps, iges_data_protocol,
};
use crate::iges_geom::{IgesGeom, IgesGeomPlane};
use martensite_tkernel::{
    standard::{Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectContextModif, IFSelectGeneralModifier, IFSelectIntParam, IFSelectModifier,
        IFSelectSelectExplore, IFSelectSelectExtract, IFSelectSignCounterOps, IFSelectSignature,
        IFSelectSignatureListOps, IFSelectTransformer, IFSelectWorkLibrary,
    },
    interface::{
        InterfaceCheckIterator, InterfaceCopyTool, InterfaceEntityIterator, InterfaceGraph,
        InterfaceInterfaceModel, InterfaceSignType,
    },
};

pub(super) fn plain(type_number: i32) -> Handle<IgesDataIgesEntity> {
    let entity = Handle::new(IgesDataIgesEntity::default());
    entity.get().unwrap().init_type_and_form(type_number, 0);
    entity
}

pub(super) fn model_of(entities: &[&Handle<IgesDataIgesEntity>]) -> Handle<IgesDataIgesModel> {
    let model = IgesDataIgesModel::new();
    for entity in entities {
        model.get().unwrap().add_entity(&entity.cast()).unwrap();
    }
    model
}

fn no_model() -> Handle<dyn InterfaceInterfaceModel> {
    Handle::default()
}

fn int_param(value: i32) -> Handle<IFSelectIntParam> {
    let param = Handle::new(IFSelectIntParam::new());
    param.get().unwrap().set_value(value);
    param
}

/// A context of a whole model, with its protocol.
fn context_of(model: &Handle<IgesDataIgesModel>) -> (IFSelectContextModif, InterfaceCopyTool) {
    let protocol: Handle<dyn IgesDataProtocolOps> = iges_data_protocol().cast();
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let graph = InterfaceGraph::new(&general, true).unwrap();
    let mut ctx = IFSelectContextModif::new(&graph, "copy.igs").unwrap();
    ctx.set_protocol(&protocol.cast());
    let tc = InterfaceCopyTool::with_protocol(&general, &protocol.cast()).unwrap();
    (ctx, tc)
}

fn perform<M: IFSelectModifier>(
    modifier: &M,
    model: &Handle<IgesDataIgesModel>,
) -> IFSelectContextModif {
    let (mut ctx, mut tc) = context_of(model);
    let protocol = ctx.protocol();
    modifier
        .perform(&mut ctx, &model.cast(), &protocol, &mut tc)
        .unwrap();
    ctx
}

#[test]
fn the_extracts_sort_on_the_directory() {
    let entity = plain(110);
    let live = entity.get().unwrap();
    live.init_status(1, 3, 0, 0);
    let any: Handle<StandardTransient> = entity.cast();
    assert!(
        !IgesSelectSelectVisibleStatus::default()
            .sort(1, &any, &no_model())
            .unwrap()
    );
    live.init_status(0, 3, 0, 0);
    assert!(
        IgesSelectSelectVisibleStatus::default()
            .sort(1, &any, &no_model())
            .unwrap()
    );
    for (status, expected) in [(3, true), (2, false), (4, true), (5, true), (6, true)] {
        let select = IgesSelectSelectSubordinate::new(status);
        assert_eq!(
            select.sort(1, &any, &no_model()).unwrap(),
            expected,
            "{status}"
        );
    }
    let level = IgesSelectSelectLevelNumber::default();
    assert!(level.sort(1, &any, &no_model()).unwrap());
    level.set_level_number(&int_param(3));
    assert!(!level.sort(1, &any, &no_model()).unwrap());
    live.init_level(&Handle::default(), 3);
    assert!(level.sort(1, &any, &no_model()).unwrap());
    assert!(level.extract_label().as_bytes().ends_with(b"admitting 3"));
}

#[test]
fn a_name_is_compared_without_its_last_blanks() {
    let entity = plain(110);
    let any: Handle<StandardTransient> = entity.cast();
    let select = IgesSelectSelectName::default();
    assert!(!select.sort(1, &any, &no_model()).unwrap());
    entity
        .get()
        .unwrap()
        .set_label(&Handle::new(HAsciiString::new("AB  ")), -1);
    select.set_name(&Handle::new(HAsciiString::new("AB")));
    assert!(select.sort(1, &any, &no_model()).unwrap());
    select.set_name(&Handle::new(HAsciiString::new("AC")));
    assert!(!select.sort(1, &any, &no_model()).unwrap());
    assert!(select.extract_label().as_bytes().ends_with(b": AC"));
}

#[test]
fn the_signatures_say_the_directory() {
    let entity = plain(110);
    let any: Handle<StandardTransient> = entity.cast();
    let live = entity.get().unwrap();
    live.init_level(&Handle::default(), 12);
    assert_eq!(
        IgesSelectSignLevelNumber::new(true).value(&any, &no_model()),
        "     12"
    );
    assert_eq!(
        IgesSelectSignLevelNumber::new(false).value(&any, &no_model()),
        "/12/"
    );
    live.set_label(&Handle::new(HAsciiString::new("LBL")), -1);
    assert_eq!(IgesSelectIGESName::new().value(&any, &no_model()), "LBL");
    let mut type_form = IgesSelectIGESTypeForm::new(true);
    type_form.set_form(false);
    assert_eq!(type_form.value(&any, &no_model()), "110");
    let unknown: Handle<StandardTransient> =
        Handle::new(crate::iges_data::IgesDataUndefinedEntity::new()).cast();
    assert_eq!(type_form.value(&unknown, &no_model()), "0 (?)");
    live.init_status(1, 2, 3, 0);
    let status = IgesSelectSignStatus::new();
    let matches = |text: &str, exact: bool| {
        status
            .matches(&any, &no_model(), &AsciiString::new(text), exact)
            .unwrap()
    };
    assert!(matches("1,2,3,0", true));
    assert!(matches("B,L,3,0", true));
    assert!(!matches("0,2,3,0", true));
    assert!(matches("0,L", false));
    assert!(!matches("0,P", false));
}

#[test]
fn the_color_signature_knows_the_standard_colors() {
    let entity = plain(110);
    let any: Handle<StandardTransient> = entity.cast();
    entity.get().unwrap().init_color(&Handle::default(), 5);
    let value = |mode: i32| IgesSelectSignColor::new(mode).value(&any, &no_model());
    assert_eq!(value(1), "S5");
    assert_eq!(value(2), "YELLOW");
    assert_eq!(value(3), "100,100,0");
    assert_eq!(value(4), "100");
    assert_eq!(value(6), "0");
    entity.get().unwrap().init_color(&Handle::default(), 0);
    assert_eq!(value(1), "(none)");
    assert_eq!(value(3), "");
}

#[test]
fn the_level_counter_counts_lists_and_values() {
    let counter = IgesSelectCounterOfLevelNumber::new(false, false);
    let entity = plain(110);
    let any: Handle<StandardTransient> = entity.cast();
    entity.get().unwrap().init_level(&Handle::default(), 3);
    counter.add_sign(&any, &no_model()).unwrap();
    counter.add_level(&any, 250);
    counter.add_level(&any, -1);
    assert_eq!(counter.highest_level(), 250);
    assert_eq!(counter.nb_times_level(3), 1);
    assert_eq!(counter.nb_times_level(-1), 1);
    assert_eq!(counter.nb_times_level(251), 0);
    assert_eq!(
        counter
            .levels()
            .get()
            .unwrap()
            .sequence(|items| items.length()),
        2
    );
    let sign = counter.sign(&any, &no_model());
    assert_eq!(sign.get().unwrap().string().as_bytes(), b"      3");
    let mut text = Vec::new();
    counter.print_count(&mut text).unwrap();
    assert!(
        String::from_utf8(text)
            .unwrap()
            .contains("Highest value : 250")
    );
    counter.clear();
    assert_eq!(counter.highest_level(), 0);
}

#[test]
fn the_level_modifiers_change_numbers() {
    let entity = plain(110);
    entity.get().unwrap().init_level(&Handle::default(), 3);
    let model = model_of(&[&entity]);
    let modifier = IgesSelectChangeLevelNumber::new();
    modifier.set_old_number(&int_param(4));
    modifier.set_new_number(&int_param(9));
    assert_eq!(modifier.label().as_bytes(), b"Changes Level Number 4 to 9");
    perform(&modifier, &model);
    assert_eq!(entity.get().unwrap().level(), 3);
    modifier.set_old_number(&int_param(3));
    perform(&modifier, &model);
    assert_eq!(entity.get().unwrap().level(), 9);
    modifier.set_new_number(&int_param(-1));
    let ctx = perform(&modifier, &model);
    assert!(!ctx.check_list().is_empty(false));
    let list = IgesSelectChangeLevelList::new();
    list.set_old_number(&int_param(1));
    list.set_new_number(&int_param(2));
    assert!(list.has_old_number() && list.has_new_number());
    assert_eq!(
        list.label().as_bytes(),
        b"Changes Level Lists containing 1 to Number 2"
    );
    perform(&list, &model);
    assert_eq!(entity.get().unwrap().level(), 9);
}

#[test]
fn the_header_modifiers_edit_the_section() {
    let entity = plain(110);
    let model = model_of(&[&entity]);
    let live = model.get().unwrap();
    live.change_global_section(|header| {
        header.set_iges_version(5);
        header.set_resolution(0.1);
        header.set_unit_flag(2);
    });
    perform(&IgesSelectSetVersion5::new(), &model);
    assert_eq!(live.global_section().iges_version(), 9);
    assert!(live.global_section().has_last_change_date());
    perform(&IgesSelectUpdateFileName::new(), &model);
    let name = live.global_section().file_name();
    assert_eq!(name.get().unwrap().string().as_bytes(), b"copy.igs");
    perform(&IgesSelectUpdateCreationDate::new(), &model);
    assert!(live.global_section().date().is_some());
    perform(&IgesSelectUpdateLastChange::new(), &model);
    let parameter = IgesSelectSetGlobalParameter::new(4);
    perform(&parameter, &model);
    parameter.set_value(&Handle::new(HAsciiString::new("6HOTHERS")));
    assert_eq!(
        parameter.label().as_bytes(),
        b"Set IGES Global Parameter Number 4 to 6HOTHERS"
    );
    perform(&parameter, &model);
    let name = live.global_section().file_name();
    assert_eq!(name.get().unwrap().string().as_bytes(), b"OTHERS");
    let wrong = IgesSelectSetGlobalParameter::new(99);
    wrong.set_value(&Handle::new(HAsciiString::new("x")));
    assert!(!perform(&wrong, &model).check_list().is_empty(false));
}

#[test]
fn the_label_modifier_writes_directory_numbers() {
    let first = plain(110);
    let second = plain(110);
    second
        .get()
        .unwrap()
        .set_label(&Handle::new(HAsciiString::new("KEEP")), -1);
    let model = model_of(&[&first, &second]);
    perform(&IgesSelectSetLabel::new(1, false), &model);
    let label = |entity: &Handle<IgesDataIgesEntity>| {
        entity
            .get()
            .unwrap()
            .short_label()
            .get()
            .map(|text| text.string().as_bytes().to_vec())
    };
    assert_eq!(label(&first).unwrap(), b"DE1");
    assert_eq!(label(&second).unwrap(), b"KEEP");
    perform(&IgesSelectSetLabel::new(1, true), &model);
    assert_eq!(label(&second).unwrap(), b"DE3");
    perform(&IgesSelectSetLabel::new(0, false), &model);
    assert!(label(&first).is_none());
    assert_eq!(
        IgesSelectSetLabel::new(0, true).label().as_bytes(),
        b"Clear Short Label (enforced)"
    );
}

#[test]
fn correcting_and_computing_status_go_through_the_editor() {
    let entity = plain(110);
    let model = model_of(&[&entity]);
    perform(&IgesSelectAutoCorrect::new(), &model);
    perform(&IgesSelectComputeStatus::new(), &model);
    assert_eq!(entity.get().unwrap().subordinate_status(), 0);
}

#[test]
fn the_float_format_goes_to_the_writer() {
    let format = IgesSelectFloatFormat::new();
    format.set_zero_suppress(false);
    format.set_format("%g");
    format.set_format_for_range("%f", 1.0, 2.0);
    let values = format.format();
    assert!(!values.zerosup && values.hasrange);
    assert_eq!(values.mainform, "%g");
    let label = String::from_utf8_lossy(format.label().as_bytes()).into_owned();
    assert_eq!(label, "Float Format %g, in range 1. 2.:%f");
    format.set_format_for_range("", 1.0, 2.0);
    assert!(!format.format().hasrange);
    let model = IgesDataIgesModel::new();
    let mut writer = crate::iges_data::IgesDataIgesWriter::new(&model);
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let mut ctx = martensite_tkxsbase::if_select::IFSelectContextWrite::new(
        &general,
        &iges_data_protocol().cast(),
        &Handle::default(),
        "f.igs",
    );
    IgesSelectFileModifier::perform(&format, &mut ctx, &mut writer).unwrap();
    assert_eq!(writer.float_writer().main_format(), "%g");
}

#[test]
fn the_view_sorter_puts_entities_without_view_apart() {
    let entity = plain(110);
    let drawing = plain(404);
    let model = model_of(&[&entity, &drawing]);
    let sorter = IgesSelectViewSorter::default();
    sorter.set_model(&model);
    assert!(sorter.add(&model.cast()));
    assert_eq!(sorter.nb_entities(), 2);
    assert_eq!(sorter.nb_sets(false), 1);
    assert_eq!(sorter.set_item(1, false).unwrap(), drawing);
    sorter.sort_single_views(true);
    assert_eq!(sorter.nb_sets(true), 1);
    let sets = sorter.sets(true).unwrap();
    assert_eq!(sets.get().unwrap().nb_packets(), 1);
    assert_eq!(sets.get().unwrap().entities(1).unwrap().nb_entities(), 1);
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let graph = InterfaceGraph::new(&general, true).unwrap();
    sorter.sort_drawings(&graph).unwrap();
    assert_eq!(sorter.nb_sets(true), 1);
    sorter.clear();
    assert_eq!(sorter.nb_entities(), 0);
    assert!(!sorter.add(&Handle::default()));
}

#[test]
fn the_explorations_go_down_the_groups_and_stop_on_geometry() {
    IgesGeom::init();
    IgesBasic::init();
    let group = Handle::new(IgesBasicGroup::new());
    let member = plain(110);
    let members = crate::iges_data::iges_new_array1(1, 1).unwrap();
    crate::iges_data::iges_array1_set(&members, 1, member.clone(), "test").unwrap();
    group.get().unwrap().init(&members).unwrap();
    let model = model_of(&[&member]);
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let graph = InterfaceGraph::new(&general, true).unwrap();
    let any: Handle<StandardTransient> = group.cast();
    let mut explored = InterfaceEntityIterator::new();
    assert!(
        IgesSelectSelectBypassGroup::new(0)
            .explore(1, &any, &graph, &mut explored)
            .unwrap()
    );
    assert_eq!(explored.nb_entities(), 1);
    let mut explored = InterfaceEntityIterator::new();
    let faces = IgesSelectSelectFaces::new();
    assert!(faces.explore(1, &any, &graph, &mut explored).unwrap());
    assert_eq!(explored.nb_entities(), 1);
    let geom = IgesSelectSelectBasicGeom::new(0);
    let line: Handle<StandardTransient> = member.cast();
    assert!(geom.explore(1, &line, &graph, &mut explored).unwrap());
    assert!(
        !IgesSelectSelectBasicGeom::new(-1)
            .explore(1, &line, &graph, &mut explored)
            .unwrap()
    );
    let plane: Handle<StandardTransient> = Handle::new(IgesGeomPlane::new()).cast();
    assert!(!faces.explore(1, &plane, &graph, &mut explored).is_err());
    assert!(
        !IgesSelectSelectPCurves::new(false)
            .explore(1, &line, &graph, &mut explored)
            .unwrap()
    );
    assert!(
        IgesSelectSelectPCurves::new(false)
            .explore(3, &line, &graph, &mut explored)
            .unwrap()
    );
    assert!(
        IgesSelectSelectBypassSubfigure::new(0)
            .explore(1, &line, &graph, &mut explored)
            .unwrap()
    );
    let mut curves = InterfaceEntityIterator::new();
    assert!(IgesSelectSelectBasicGeom::sub_curves(&member, &mut curves).unwrap());
    assert!(!IgesSelectSelectBasicGeom::sub_curves(&Handle::default(), &mut curves).unwrap());
}

#[test]
fn the_spline_transformer_refuses_what_it_finds() {
    let spline = plain(112);
    let model = model_of(&[&spline]);
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let graph = InterfaceGraph::new(&general, true).unwrap();
    let transformer = IgesSelectSplineToBSpline::new(false);
    let mut checks = InterfaceCheckIterator::new();
    let mut newmod = Handle::default();
    assert!(
        !transformer
            .perform(
                &graph,
                &iges_data_protocol().cast(),
                &mut checks,
                &mut newmod
            )
            .unwrap()
    );
    assert!(newmod.is_null());
    let any: Handle<StandardTransient> = spline.cast();
    let mut updated = Handle::default();
    assert!(!transformer.updated(&any, &mut updated));
    let empty = model_of(&[]);
    let graph = InterfaceGraph::new(&empty.cast(), true).unwrap();
    assert!(
        transformer
            .perform(
                &graph,
                &iges_data_protocol().cast(),
                &mut checks,
                &mut newmod
            )
            .unwrap()
    );
    assert!(transformer.updated(&any, &mut updated) && updated == any);
}

#[test]
fn the_work_library_reads_and_dumps_what_it_writes() {
    let library = IgesSelectWorkLibrary::new(false);
    let protocol = IgesSelectWorkLibrary::define_protocol();
    let mut read = Handle::default();
    assert_eq!(
        library
            .read_file("/nonexistent/none.igs", &mut read, &protocol.cast())
            .unwrap(),
        -1
    );
    let entity = plain(110);
    let model = model_of(&[&entity]);
    let mut out = Vec::new();
    library
        .dump_entity(&model.cast(), &protocol.cast(), &entity.cast(), &mut out, 4)
        .unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(
        text.starts_with(" --- Entity 1 Type cdl : IgesDataIgesEntity"),
        "{text}"
    );
}
