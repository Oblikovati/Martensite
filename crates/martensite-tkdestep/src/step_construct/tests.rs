//! Whole structures through the tools of the package, with no session.
use super::*;
use crate::de_step::DeStepWriteModeStepSchema;
use crate::step_ap203::StepAP203PersonOrganizationItem;
use crate::step_basic::{
    StepBasicApplicationContext, StepBasicConversionBasedUnitAndLengthUnit,
    StepBasicMechanicalContext, StepBasicNamedUnit,
    StepBasicProductDefinitionFormationWithSpecifiedSource, StepBasicProductType,
    StepBasicSiPrefix, StepBasicSiUnitAndLengthUnit, StepBasicSiUnitAndPlaneAngleUnit,
    StepBasicSiUnitName,
};
use crate::step_data::{StepDataFactors, StepDataSelectType, StepDataStepModel};
use crate::step_entity::step_array1_item;
use crate::step_repr::StepReprGlobalUnitAssignedContext;
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeRepresentation,
};
use crate::step_visual::{StepVisualColourRgb, StepVisualDraughtingPreDefinedColour};
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::HAsciiString;
use std::collections::HashMap;

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

fn text_of(value: &Handle<HAsciiString>) -> String {
    String::from_utf8_lossy(value.get().unwrap().string().as_bytes()).into_owned()
}

fn model_with_schema(schema: DeStepWriteModeStepSchema) -> Handle<StepDataStepModel> {
    let model = StepDataStepModel::new();
    model.get().unwrap().internal_parameters_mut().write_schema = schema;
    model
}

fn part_for(schema: DeStepWriteModeStepSchema) -> StepConstructPart {
    let application = Handle::new(StepBasicApplicationContext::new());
    application.init(&text("automotive"));
    let mut part = StepConstructPart::new();
    let sr = Handle::new(StepShapeShapeRepresentation::new());
    part.make_sdr(
        &sr,
        &text("widget"),
        &application,
        &model_with_schema(schema),
    )
    .unwrap();
    part
}

#[test]
fn a_part_of_ap214_is_reachable_through_every_accessor() {
    let part = part_for(DeStepWriteModeStepSchema::AP214_IS);
    assert!(part.is_done() && part.sr_value().unwrap().is_some());
    assert_eq!(text_of(&part.pid().unwrap()), "widget");
    assert_eq!(text_of(&part.pname().unwrap()), "widget");
    assert_eq!(text_of(&part.pc_discipline_type().unwrap()), "mechanical");
    assert_eq!(text_of(&part.ac_application().unwrap()), "automotive");
    assert_eq!(text_of(&part.pdc_name().unwrap()), "part definition");
    assert_eq!(text_of(&part.pdc_stage().unwrap()), "design");
    assert_eq!(text_of(&part.prpc_name().unwrap()), "part");
    assert!(
        part.pc()
            .unwrap()
            .cast::<StepBasicMechanicalContext>()
            .is_null()
    );
    assert!(part.pds().unwrap().is_some());
    part.set_pname(&text("renamed")).unwrap();
    assert_eq!(text_of(&part.pname().unwrap()), "renamed");
}

#[test]
fn a_part_takes_the_entities_of_its_schema() {
    let ap203 = part_for(DeStepWriteModeStepSchema::AP203);
    assert!(
        ap203
            .pc()
            .unwrap()
            .cast::<StepBasicMechanicalContext>()
            .is_some()
    );
    assert!(
        ap203
            .pdf()
            .unwrap()
            .cast::<StepBasicProductDefinitionFormationWithSpecifiedSource>()
            .is_some()
    );
    assert_eq!(text_of(&ap203.prpc_name().unwrap()), "detail");
    assert_eq!(text_of(&ap203.pdc_name().unwrap()), "");
    let cd = part_for(DeStepWriteModeStepSchema::AP214_CD);
    assert!(cd.prpc().cast::<StepBasicProductType>().is_some());
}

#[test]
fn reading_a_null_sdr_is_not_done_and_a_chain_is_a_failure() {
    let mut part = StepConstructPart::new();
    part.read_sdr(&Handle::default());
    assert!(!part.is_done());
    assert!(part.pname().is_err());
}

#[test]
fn an_assembly_links_two_parts_through_a_nauo_and_a_cdsr() {
    let (assembly, component) = (
        part_for(DeStepWriteModeStepSchema::AP214_IS),
        part_for(DeStepWriteModeStepSchema::AP214_IS),
    );
    let mut link = StepConstructAssembly::new();
    let trsf = Handle::new(crate::step_geom::StepGeomCartesianTransformationOperator3d::new());
    link.init_trsf(&component.sdr_value(), &assembly.sdr_value(), &trsf)
        .unwrap();
    assert!(
        link.item_value()
            .cast::<StepShapeShapeRepresentation>()
            .is_some()
    );
    link.make_relationship().unwrap();
    let cdsr = link
        .item_value()
        .cast::<StepShapeContextDependentShapeRepresentation>();
    assert!(cdsr.is_some());
    let nauo = link.get_nauo().unwrap();
    let nauo = nauo.get().unwrap();
    assert!(nauo.relating_product_definition() == assembly.pd().unwrap());
    assert!(nauo.related_product_definition() == component.pd().unwrap());
}

#[test]
fn each_nauo_has_a_new_identifier() {
    let (first, second) = (
        part_for(DeStepWriteModeStepSchema::AP214_IS),
        part_for(DeStepWriteModeStepSchema::AP214_IS),
    );
    let trsf = Handle::new(crate::step_geom::StepGeomCartesianTransformationOperator3d::new());
    let ids: Vec<String> = (0..2)
        .map(|_| {
            let mut link = StepConstructAssembly::new();
            link.init_trsf(&first.sdr_value(), &second.sdr_value(), &trsf)
                .unwrap();
            link.make_relationship().unwrap();
            text_of(&link.get_nauo().unwrap().get().unwrap().id())
        })
        .collect();
    assert_ne!(ids[0], ids[1]);
}

#[test]
fn the_context_tool_gives_the_protocol_of_the_schema() {
    let cases = [
        (
            DeStepWriteModeStepSchema::AP214_CD,
            1997,
            "committee draft",
            "automotive_design",
            "core data for automotive mechanical design processes",
        ),
        (
            DeStepWriteModeStepSchema::AP214_DIS,
            1998,
            "draft international standard",
            "automotive_design",
            "core data for automotive mechanical design processes",
        ),
        (
            DeStepWriteModeStepSchema::AP203,
            1994,
            "international standard",
            "config_control_design",
            "configuration controlled 3D designs of mechanical parts and assemblies",
        ),
        (
            DeStepWriteModeStepSchema::AP214_IS,
            2000,
            "international standard",
            "automotive_design",
            "core data for automotive mechanical design processes",
        ),
        (
            DeStepWriteModeStepSchema::AP242_DIS,
            2013,
            "international standard",
            "ap242_managed_model_based_3d_engineering",
            "Managed model based 3d engineering",
        ),
    ];
    for (schema, year, status, schema_name, application) in cases {
        let mut tool = StepConstructContextTool::with_model(&model_with_schema(schema)).unwrap();
        tool.add_apd(false).unwrap();
        assert_eq!(tool.get_ac_year().unwrap(), year);
        assert_eq!(text_of(&tool.get_ac_status()), status);
        assert_eq!(text_of(&tool.get_ac_schema_name()), schema_name);
        assert_eq!(text_of(&tool.get_ac_name()), application);
        assert_eq!(tool.is_ap203(), schema == DeStepWriteModeStepSchema::AP203);
        assert_eq!(
            tool.is_ap242(),
            schema == DeStepWriteModeStepSchema::AP242_DIS
        );
    }
}

#[test]
fn the_product_name_follows_the_assembly_position() {
    let mut tool = StepConstructContextTool::new();
    assert_eq!(text_of(&tool.get_product_name()), "Product");
    tool.set_level(2);
    tool.set_index(7);
    assert_eq!(text_of(&tool.get_product_name()), "Product 1.7");
    tool.prev_level();
    assert_eq!(text_of(&tool.get_product_name()), "Product 1");
    tool.set_level(0);
    assert_eq!(tool.index(), 0);
}

#[test]
fn the_roots_of_a_part_add_the_management_data_of_ap203_only() {
    let part = part_for(DeStepWriteModeStepSchema::AP203);
    let mut tool =
        StepConstructContextTool::with_model(&model_with_schema(DeStepWriteModeStepSchema::AP203))
            .unwrap();
    let roots = tool.get_roots_for_part(&part).unwrap();
    assert_eq!(roots.get().unwrap().sequence(|items| items.length()), 13);
    let creator = tool.ap203_context().get_creator();
    let items = creator.get().unwrap().items();
    let first: StepAP203PersonOrganizationItem = step_array1_item(&items, 1, "test").unwrap();
    assert!(
        first.value()
            == part
                .pdf()
                .unwrap()
                .cast::<martensite_tkernel::standard::StandardTransient>()
    );
    let plain = part_for(DeStepWriteModeStepSchema::AP214_IS);
    let mut tool = StepConstructContextTool::with_model(&model_with_schema(
        DeStepWriteModeStepSchema::AP214_IS,
    ))
    .unwrap();
    let roots = tool.get_roots_for_part(&plain).unwrap();
    assert_eq!(roots.get().unwrap().sequence(|items| items.length()), 2);
}

#[test]
fn a_clear_of_the_ap203_context_keeps_the_approver_until_the_approval_changes() {
    let part = part_for(DeStepWriteModeStepSchema::AP203);
    let mut context = StepConstructAP203Context::new();
    context.init(&part).unwrap();
    let approver = context.get_approver();
    context.clear();
    assert!(context.get_approver() == approver && context.get_approval().is_null());
    context.init(&part).unwrap();
    // The default approval is one, so the requisites of the same approval are not made again.
    assert!(context.get_approver() == approver);
}

#[test]
fn a_written_context_decodes_to_the_unit_it_was_written_in() {
    for (write_unit, factor) in [(1, 25.4), (2, 1.), (6, 1000.), (10, 10.)] {
        let model = StepDataStepModel::new();
        model.get().unwrap().internal_parameters_mut().write_unit = write_unit;
        let mut made = StepConstructUnitContext::new();
        made.init(0.01, &model, &StepDataFactors::default())
            .unwrap();
        assert!(made.is_done());
        let grc = made.value();
        let grc = grc.get().unwrap();
        let mut read = StepConstructUnitContext::new();
        assert_eq!(
            read.compute_factors(
                &grc.global_unit_assigned_context(),
                &StepDataFactors::default()
            )
            .unwrap(),
            0
        );
        assert!(read.length_done() && read.plane_angle_done() && read.solid_angle_done());
        assert!(
            (read.length_factor() - factor).abs() < 1e-9,
            "unit {write_unit}: {}",
            read.length_factor()
        );
        assert_eq!(read.plane_angle_factor(), 1.);
        assert_eq!(
            read.compute_tolerance(&grc.global_uncertainty_assigned_context())
                .unwrap(),
            0
        );
        assert!(read.has_uncertainty() && (read.uncertainty() - 0.01).abs() < 1e-12);
    }
}

#[test]
fn the_kind_of_a_written_length_unit_is_conversion_based_only_for_non_metric() {
    let inch = StepDataStepModel::new();
    inch.get().unwrap().internal_parameters_mut().write_unit = 1;
    let mut made = StepConstructUnitContext::new();
    made.init(0.01, &inch, &StepDataFactors::default()).unwrap();
    let context: Handle<StepReprGlobalUnitAssignedContext> =
        made.value().get().unwrap().global_unit_assigned_context();
    let unit: Handle<StepBasicNamedUnit> = context.get().unwrap().units_value(1).unwrap();
    assert!(
        unit.cast::<StepBasicConversionBasedUnitAndLengthUnit>()
            .is_some()
    );
    assert!(unit.cast::<StepBasicSiUnitAndLengthUnit>().is_null());
}

#[test]
fn factors_report_the_status_of_a_bad_unit_and_of_no_context() {
    let mut tool = StepConstructUnitContext::new();
    assert_eq!(
        tool.compute_factors(&Handle::default(), &StepDataFactors::default())
            .unwrap(),
        1
    );
    assert_eq!(tool.plane_angle_factor(), std::f64::consts::PI / 180.);
    assert_eq!(
        tool.compute_unit_factors(&Handle::default(), &StepDataFactors::default())
            .unwrap(),
        -1
    );
    let unit = Handle::new(StepBasicSiUnitAndPlaneAngleUnit::new());
    unit.init(
        true,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiUnitName::SunGram,
    );
    assert_eq!(
        tool.compute_unit_factors(&unit.cast(), &StepDataFactors::default())
            .unwrap(),
        11
    );
    assert_eq!(tool.compute_tolerance(&Handle::default()).unwrap(), 40);
    assert_eq!(
        tool.status_message(40),
        "Bad GlobalUncertaintyAssignedContext, default unit taken"
    );
    assert_eq!(
        tool.status_message(99),
        "Badly defined units, default taken"
    );
}

#[test]
fn colours_predefined_in_step_encode_to_a_name_and_decode_back() {
    for (named, _) in [
        (NamedColor::RED, "red"),
        (NamedColor::GREEN, "green"),
        (NamedColor::WHITE, "white"),
    ] {
        let colour = StepConstructStyles::encode_color(&Color::named(named)).unwrap();
        assert!(
            colour
                .cast::<StepVisualDraughtingPreDefinedColour>()
                .is_some()
        );
        let mut decoded = Color::default();
        assert!(StepConstructStyles::decode_color(&colour, &mut decoded).unwrap());
        assert!(decoded == Color::named(named));
    }
}

#[test]
fn other_colours_encode_to_srgb_and_a_range_over_one_is_normalized() {
    let color = Color::new(0.25, 0.5, 0.75, ColorSpace::Srgb).unwrap();
    let colour = StepConstructStyles::encode_color(&color).unwrap();
    let rgb = colour.cast::<StepVisualColourRgb>();
    assert!((rgb.get().unwrap().green() - 0.5).abs() < 1e-5);
    let large = Handle::new(StepVisualColourRgb::new());
    large.init(&text(""), 100., 200., 255.);
    let mut decoded = Color::default();
    assert!(StepConstructStyles::decode_color(&large.cast(), &mut decoded).unwrap());
    let [_, green, blue] = decoded.values(ColorSpace::Srgb);
    assert!((blue - 1.).abs() < 1e-5 && (green - 200. / 255.).abs() < 1e-5);
}

#[test]
fn the_shared_encoding_makes_one_entity_per_colour() {
    let (mut named, mut rgbs) = (HashMap::new(), HashMap::new());
    let color = Color::new(0.25, 0.5, 0.75, ColorSpace::Srgb).unwrap();
    let first = StepConstructStyles::encode_color_shared(&color, &mut named, &mut rgbs).unwrap();
    let second = StepConstructStyles::encode_color_shared(&color, &mut named, &mut rgbs).unwrap();
    assert!(first == second && rgbs.len() == 1 && named.is_empty());
    let red = Color::named(NamedColor::RED);
    let first = StepConstructStyles::encode_color_shared(&red, &mut named, &mut rgbs).unwrap();
    let second = StepConstructStyles::encode_color_shared(&red, &mut named, &mut rgbs).unwrap();
    assert!(first == second && named.len() == 1);
}

#[test]
fn a_colour_makes_a_style_with_surface_and_curve_parts_and_reads_back() {
    let mut styles = StepConstructStyles::new();
    let colour = StepConstructStyles::encode_color(&Color::named(NamedColor::RED)).unwrap();
    let psa = styles.get_color_psa(&Handle::default(), &colour).unwrap();
    // The PSA of a colour is remembered.
    assert!(styles.get_color_psa(&Handle::default(), &colour).unwrap() == psa);
    assert_eq!(psa.get().unwrap().nb_styles().unwrap(), 2);
    let item = Handle::new(crate::step_geom::StepGeomCartesianPoint::new());
    let style = styles
        .add_style_for_item(&item.cast(), &psa, &Handle::default())
        .unwrap();
    assert_eq!(styles.nb_styles(), 1);
    let (mut surface, mut boundary, mut curve) =
        (Handle::default(), Handle::default(), Handle::default());
    let mut rendering = StepConstructRenderingProperties::new();
    let mut is_component = false;
    assert!(
        styles
            .get_colors(
                &style,
                &mut surface,
                &mut boundary,
                &mut curve,
                &mut rendering,
                &mut is_component
            )
            .unwrap()
    );
    assert!(surface == colour && curve == colour && boundary.is_null() && is_component);
}

#[test]
fn an_empty_psa_is_null_and_a_style_by_context_comes_for_a_nauo() {
    let styles = StepConstructStyles::new();
    let none = StepConstructRenderingProperties::new();
    let empty = styles
        .make_color_psa(
            &Handle::default(),
            &Handle::default(),
            &Handle::default(),
            &none,
            false,
        )
        .unwrap();
    assert!(empty.is_null());
    let colour = StepConstructStyles::encode_color(&Color::named(NamedColor::BLACK)).unwrap();
    let psa = styles
        .make_color_psa(&Handle::default(), &colour, &Handle::default(), &none, true)
        .unwrap();
    assert!(
        psa.cast::<crate::step_visual::StepVisualPresentationStyleByContext>()
            .is_some()
    );
}
