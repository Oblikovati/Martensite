//! `bcut_complex` bodies written by hand: they keep checks an earlier test made during the script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{prism, ttranslate},
    construction::copy,
    one_step::bcut,
    properties::Prop,
    restore::restore,
    session::Session,
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{BRepCheckAnalyzer, BRepCheckResult, BRepCheckStatus};

/// Beyond the script: a translation moves the location only, and each of the name's two moves
/// adds up; `ttranslate a a` names `a` twice.
fn translated_by(s: &dyn Session, name: &str, original: &str, expected_y: f64) {
    let shape = s.shape(name).expect("translated shape");
    let offset = shape.location().transformation().translation_part();
    assert_eq!(offset.y(), expected_y, "location of {name}");
    let source = s.shape(original).expect("restored shape");
    assert!(shape.is_partner(source), "{name} shares its restored shape");
}

pub fn K7(s: &mut dyn Session) {
    restore(s, "spo0139a.rle", "a");
    copy(s, "a", "restored_a");
    restore(s, "spo0139b.rle", "b");
    ttranslate(s, "a", [0., -60., 0.]);
    translated_by(s, "a", "restored_a", -60.);
    ttranslate(s, "a", [0., -60., 0.]);
    translated_by(s, "a", "restored_a", -120.);
    prism(s, "abox", "a", [0., 240., 0.]);
    bcut(s, "result", "b", "abox");
    checkprops(s, "result", Prop::Area, Expected::Mass(129086.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "abox"], Prop::Area),
    );
}

/// One fault in the whole result, of a shell in the context of the solid that holds it.
struct ContextFault {
    shell: Shape,
    solid: Shape,
    status: BRepCheckStatus,
}

fn context_faults(analyzer: &BRepCheckAnalyzer, shape: &Shape, faults: &mut Vec<ContextFault>) {
    let handle = analyzer.result(shape).unwrap();
    let Some(checker) = handle.get() else {
        return;
    };
    let own: Vec<BRepCheckStatus> = checker.status().unwrap().iter().copied().collect();
    assert!(
        own.iter().all(|status| *status == BRepCheckStatus::NoError),
        "expected only the shell-on-solid contextual fault, got {own:?}"
    );
    checker.init_context_iterator().unwrap();
    while checker.more_shape_in_context() {
        add_context_fault(checker, shape, faults);
        checker.next_shape_in_context();
    }
}

fn add_context_fault(checker: &dyn BRepCheckResult, shape: &Shape, faults: &mut Vec<ContextFault>) {
    let statuses = checker.status_on_shape().unwrap();
    for status in statuses.iter() {
        if *status != BRepCheckStatus::NoError {
            faults.push(ContextFault {
                shell: shape.clone(),
                solid: checker.contextual_shape().unwrap().clone(),
                status: *status,
            });
        }
    }
}

/// Beyond the script's REQUIRED line: which fault the two faulty shapes are.
fn one_shell_on_solid_fault(result: &Shape) {
    let analyzer = BRepCheckAnalyzer::new(result, true, false, false).unwrap();
    assert!(
        !analyzer.is_valid().unwrap(),
        "expected faulty_1 and faulty_2"
    );
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes(result, &mut shapes, true, true);
    let mut faults = Vec::new();
    for index in 1..=shapes.extent() {
        context_faults(&analyzer, shapes.find_key(index).unwrap(), &mut faults);
    }
    assert_eq!(faults.len(), 1, "expected exactly one faulty context");
    let fault = &faults[0];
    assert_eq!(fault.status, BRepCheckStatus::InvalidMultiConnexity);
    assert_eq!(fault.shell.shape_type().unwrap(), ShapeEnum::Shell);
    assert_eq!(fault.solid.shape_type().unwrap(), ShapeEnum::Solid);
    assert!(!fault.shell.is_same(&fault.solid), "two distinct shapes");
    let mut shells = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&fault.solid, ShapeEnum::Shell, &mut shells).unwrap();
    assert!(
        shells.contains(&fault.shell),
        "the shell belongs to the solid"
    );
}

pub fn O5(s: &mut dyn Session) {
    println!("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_2");
    restore(s, "CTO908_topo102-o.brep", "obj");
    restore(s, "CTO908_topo102-t.brep", "tool");
    bcut(s, "result", "obj", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(44135.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["obj", "tool"], Prop::Area),
    );
    let result = s.shape("result").expect("bcut result").clone();
    one_shell_on_solid_fault(&result);
}
