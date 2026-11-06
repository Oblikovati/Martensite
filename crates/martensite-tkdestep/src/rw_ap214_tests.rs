//! A whole STEP text through the AP214 modules: the records become their entity types with no
//! failure, and what is written back reads the same.
use crate::rw_step_ap214::RwStepAP214;
use crate::step_ap214::StepAP214;
use crate::step_basic::StepBasicProduct;
use crate::step_data::{
    StepDataProtocolOps, StepDataStepModel, StepDataStepWriter, StepDataUndefinedEntity,
};
use crate::step_dim_tol::StepDimTolGeometricToleranceWithDatumReference;
use crate::step_entity::*;
use crate::step_file::step_file_read;
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_kinematics::StepKinematicsRevolutePairWithRange;
use crate::step_visual::*;
use martensite_tkxsbase::interface::*;
use std::io::Cursor;

pub(crate) const HEADER: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('a'),'2;1');\nFILE_NAME('a','2026-01-01T00:00:00',(''),(''),'p','o','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));\nENDSEC;\nDATA;\n";
pub(crate) const FOOTER: &str = "ENDSEC;\nEND-ISO-10303-21;\n";

/// The AP214 protocol, with its modules installed.
fn protocol() -> Handle<dyn StepDataProtocolOps> {
    RwStepAP214::init();
    StepAP214::protocol().cast()
}

/// Reads STEP text into a new model.
pub(crate) fn model_of(text: &str) -> Handle<StepDataStepModel> {
    let model = StepDataStepModel::new();
    let mut stream = Cursor::new(text.as_bytes().to_vec());
    let status = step_file_read("in-memory", Some(&mut stream), &model, &protocol()).unwrap();
    assert_eq!(status, 0, "the text does not parse");
    model
}

/// The file a model writes.
pub(crate) fn written(model: &Handle<StepDataStepModel>) -> String {
    let mut writer = StepDataStepWriter::new(model);
    writer.send_model(&protocol(), false).unwrap();
    let mut out = Vec::new();
    writer.print(&mut out).unwrap();
    String::from_utf8(out).unwrap()
}

/// The failures the model reports, as their entity numbers and first texts.
pub(crate) fn failures(model: &Handle<StepDataStepModel>) -> Vec<String> {
    let live = model.get().unwrap();
    (1..=live.nb_entities())
        .filter_map(|num| {
            let check = live.check(num, true);
            let check = check.get()?;
            check.has_failed().then(|| {
                format!(
                    "#{num}: {}",
                    String::from_utf8_lossy(check.c_fail(1, true).unwrap().as_bytes())
                )
            })
        })
        .collect()
}

pub(crate) fn data_part(text: &str) -> &str {
    let start = text.find("DATA;").unwrap();
    &text[start..]
}

#[test]
fn records_of_each_package_become_their_entities_and_write_back_the_same() {
    let body = "#1=CARTESIAN_POINT('p',(1.,2.,3.));\n\
#2=COORDINATES_LIST('c',2,((0.,0.,0.),(1.,1.,1.)));\n\
#3=TESSELLATED_CURVE_SET('s',#2,((1,2),(2,1)));\n\
#4=DRAUGHTING_PRE_DEFINED_COLOUR('red');\n\
#5=PRESENTATION_STYLE_ASSIGNMENT((NULL_STYLE(.NULL.)));\n";
    let model = model_of(&format!("{HEADER}{body}{FOOTER}"));
    let live = model.get().unwrap();
    assert_eq!(live.nb_entities(), 5);
    assert_eq!(failures(&model), Vec::<String>::new());
    assert!(
        live.value(1)
            .unwrap()
            .is_kind(StepGeomCartesianPoint::static_type())
    );
    let coordinates = live.value(2).unwrap().cast::<StepVisualCoordinatesList>();
    let points = coordinates.get().unwrap().points();
    assert_eq!(points.get().unwrap().array1().length(), 2);
    assert_eq!(points.get().unwrap().array1().value(2).unwrap().x(), 1.0);
    let curves = live
        .value(3)
        .unwrap()
        .cast::<StepVisualTessellatedCurveSet>();
    assert_eq!(curves.get().unwrap().curves().get().unwrap().length(), 2);
    assert!(
        live.value(4)
            .unwrap()
            .is_kind(StepVisualDraughtingPreDefinedColour::static_type())
    );
    let styles = live
        .value(5)
        .unwrap()
        .cast::<StepVisualPresentationStyleAssignment>();
    assert_eq!(styles.get().unwrap().nb_styles().unwrap(), 1);
    let first = written(&model);
    let again = written(&model_of(&first));
    assert_eq!(data_part(&first), data_part(&again));
}

#[test]
fn complex_records_reach_their_dispatch_case() {
    let body = "#1=(GEOMETRIC_TOLERANCE('n','d',$,$)GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE(())MODIFIED_GEOMETRIC_TOLERANCE(.NONE.)POSITION_TOLERANCE());\n";
    let model = model_of(&format!("{HEADER}{body}{FOOTER}"));
    let live = model.get().unwrap();
    assert_eq!(live.nb_entities(), 1);
    let entity = live.value(1).unwrap();
    assert!(
        !entity.is_kind(StepDataUndefinedEntity::static_type()),
        "not recognized: {}",
        entity.dynamic_type().unwrap().name()
    );
    let _ = StepDimTolGeometricToleranceWithDatumReference::static_type();
}

#[test]
fn an_unknown_type_is_kept_as_an_undefined_entity() {
    let model = model_of(&format!("{HEADER}#1=NOT_A_STEP_TYPE(1);\n{FOOTER}"));
    let entity = model.get().unwrap().value(1).unwrap();
    assert!(entity.is_kind(StepDataUndefinedEntity::static_type()));
    let _ = StepKinematicsRevolutePairWithRange::static_type();
}

/// The bytes of a string are decoded as the UTF-8 iterator does, unchecked, even when they
/// are not UTF-8: these are Windows-1251 names of `bug31670_russian.stp`, with the result.
#[test]
fn a_name_in_another_code_page_decodes() {
    let mut text = HEADER.as_bytes().to_vec();
    text.extend_from_slice(
        b"#1=PRODUCT('\xc4\xe5\xf22','\xd1\xea\xf0\xf3\xe3\xeb\xe5\xed\xe8\xe53','',());\n",
    );
    text.extend_from_slice(FOOTER.as_bytes());
    let model = StepDataStepModel::new();
    let mut stream = Cursor::new(text);
    let status = step_file_read("in-memory", Some(&mut stream), &model, &protocol()).unwrap();
    assert_eq!(status, 0);
    assert!(
        written(&model).contains("PRODUCT('\u{165}','\u{4aa}\u{7492b}\u{6ba8}'"),
        "{}",
        written(&model)
    );
}

/// The array member's name goes through the derived class, which picks the case from it. Through
/// the embedded array class the case stays unset and the select refuses the member.
#[test]
fn a_derived_array_member_takes_the_case_of_its_name() {
    let body = "#1=FEA_LINEAR_ELASTICITY('m',FEA_ISOTROPIC_SYMMETRIC_TENSOR4_3D((6499999.77434735,0.35)));\n";
    let model = model_of(&format!("{HEADER}{body}{FOOTER}"));
    assert_eq!(failures(&model), Vec::<String>::new());
    let first = written(&model);
    assert!(
        first.contains("FEA_ISOTROPIC_SYMMETRIC_TENSOR4_3D((\n      6.499999774347E+06,0.35))"),
        "{first}"
    );
    assert_eq!(data_part(&first), data_part(&written(&model_of(&first))));
}

/// A name of `length` bytes reads exactly, and again from what the writer sends. A string
/// that fills the 16K scanner buffer fails the whole read: a deliberate difference.
fn long_name_round_trips(name: &str) {
    let model = model_of(&format!(
        "{HEADER}#1=PRODUCT('{name}','d','',());\n{FOOTER}"
    ));
    assert_eq!(failures(&model), Vec::<String>::new());
    let read_name = |model: &Handle<StepDataStepModel>| {
        let product = model.get().unwrap().value(1).unwrap();
        let product = product.cast::<StepBasicProduct>();
        let id = product.get().unwrap().id();
        id.get().unwrap().string().as_bytes().to_vec()
    };
    assert_eq!(read_name(&model), name.as_bytes());
    let first = written(&model);
    let reread = model_of(&first);
    assert_eq!(failures(&reread), Vec::<String>::new());
    assert_eq!(read_name(&reread), name.as_bytes());
    assert_eq!(data_part(&first), data_part(&written(&reread)));
}

#[test]
fn a_string_above_16k_reads_and_round_trips() {
    for length in [16_382, 16_383, 16_384, 20_000, 100_000] {
        long_name_round_trips(&"x".repeat(length));
    }
    long_name_round_trips("word ".repeat(4_000).trim_end());
}

/// Not a gate: walks the STEP files named in `MARTENSITE_STEP_SWEEP`, one per line.
#[test]
#[ignore = "reads files outside the repository"]
fn sweep_files_named_in_the_environment() {
    let list = std::env::var("MARTENSITE_STEP_SWEEP").unwrap();
    let out_dir = std::env::var("MARTENSITE_STEP_SWEEP_OUT").ok();
    for (rank, path) in std::fs::read_to_string(list).unwrap().lines().enumerate() {
        let model = StepDataStepModel::new();
        let mut stream = Cursor::new(std::fs::read(path).unwrap());
        match step_file_read("sweep", Some(&mut stream), &model, &protocol()) {
            Ok(0) => {}
            other => {
                println!("{path}: READ {other:?}");
                continue;
            }
        }
        let live = model.get().unwrap();
        let count = live.nb_entities();
        let fails = failures(&model);
        let undefined = (1..=count)
            .filter(|num| {
                live.value(*num)
                    .unwrap()
                    .is_kind(StepDataUndefinedEntity::static_type())
            })
            .count();
        let mut writer = StepDataStepWriter::new(&model);
        if let Err(failure) = writer.send_model(&protocol(), false) {
            println!(
                "{path}: WRITE entities={count} fails={} undefined={undefined} {failure}",
                fails.len()
            );
            continue;
        }
        let first = written(&model);
        if let Some(dir) = &out_dir {
            std::fs::write(format!("{dir}/{}.txt", rank + 1), &first).unwrap();
        }
        let reread = model_of(&first);
        let again = written(&reread);
        let difference = data_part(&first)
            .lines()
            .zip(data_part(&again).lines())
            .find(|(a, b)| a != b);
        let reread_count = reread.get().unwrap().nb_entities();
        println!(
            "{path}: entities={count} fails={} undefined={undefined} reread={reread_count} stable={} first_fail={:?} diff={difference:?}",
            fails.len(),
            difference.is_none(),
            fails.first()
        );
    }
}
