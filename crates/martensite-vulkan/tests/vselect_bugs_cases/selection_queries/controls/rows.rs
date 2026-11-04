use super::*;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::ais::{AisShapeParameters, AisShapeSource};

#[test]
fn real_local_face_owner_matches_distinct_explorer_face_two() {
    let mut session = boxes();
    pick_face(&mut session);
    assert_face_identity(&session);
    observations::record_scene("local-face", &session, &["b1", "b2"]);
    let text = objects::state_text(&session, &["b1", "b2"], false).unwrap();
    assert!(predicates::first_face_errors(&text).is_empty(), "{text:?}");
}

fn assert_face_identity(session: &ViewerSession) {
    let operands = selected::selected_operands(session.context()).unwrap();
    let object = query_named_object(session, "b1")
        .unwrap()
        .cast::<dyn AisShapeSource>();
    let faces: Vec<_> = Explorer::new(&object.shape(), ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let picked = operands[0]
        .shape
        .as_ref()
        .expect("real local owner has a face");
    assert_eq!(picked.shape_type().unwrap(), ShapeEnum::Face);
    assert!(picked.is_same(&faces[1]));
    let global_address = object.global_sel_owner().object_address();
    assert_ne!(operands[0].owner.object_address(), global_address);
    let rank = crate::vselect_clipping_cases::occurrence_rank(&object.shape(), picked).unwrap();
    assert_eq!(rank, Some(2));
    observations::record_rank(&operands[0], rank.unwrap());
}

#[test]
fn real_mixed_local_global_owners_supply_live_flags_and_detected_membership() {
    let mut session = boxes();
    pick_face(&mut session);
    pick_world(&mut session, [6., 5.5, 5.5], AisSelectionScheme::Xor);
    let operands = selected::selected_operands(session.context()).unwrap();
    assert_eq!(operands.len(), 2);
    assert_live_flags(&session);
    observations::record_scene("mixed-owners", &session, &["b1", "b2"]);
    let text = objects::state_text(&session, &["b1", "b2"], true).unwrap();
    let (lines, errors) = predicates::mixed_report(&text);
    assert!(errors.is_empty(), "{text:?}: {errors:?}");
    assert_eq!(lines[2], "6");
}

fn assert_live_flags(session: &ViewerSession) {
    let b1 = query_named_object(session, "b1").unwrap();
    let b2 = query_named_object(session, "b2").unwrap();
    let detected = selected::detected_objects(session.context()).unwrap();
    assert!(!session.context().is_selected_object(&b1));
    assert!(session.context().is_selected_object(&b2));
    assert!(!detected.contains(&b1) && detected.contains(&b2));
    let current: AisSelectedOwner = {
        let lease = session.context().detected_owner();
        (*lease).clone()
    };
    assert_eq!(current.selectable().object_address(), b2.object_address());
    assert!(session.context().is_displayed(&b1) && session.context().is_displayed(&b2));
}

fn fixture_mixed(first: &str, second: &str, face: &str) -> String {
    format!(
        "Neutral-point state:\n  {first}\n  {second}\nLocally selected sub-shapes within b1:\n       {face}\n"
    )
}

#[test]
fn full_text_predicates_keep_trailing_rows_two_orders_spacing_and_error_order() {
    use predicates::{B1_ROW, B2_ROW, first_face_errors, mixed_report};
    let first = "Locally selected sub-shapes within b1:\n       Face #2\n";
    assert!(first_face_errors(first).is_empty());
    assert!(!first_face_errors(first.trim_end()).is_empty());
    for (a, b) in [(B1_ROW, B2_ROW), (B2_ROW, B1_ROW)] {
        let text = fixture_mixed(a, b, "Face #2");
        let (printed, errors) = mixed_report(&text);
        assert_eq!(printed, [a.to_owned(), b.to_owned(), "6".to_owned()]);
        assert!(errors.is_empty());
        assert_eq!(
            mixed_report(text.trim_end()).1,
            ["ERROR: vstate output for shift selection is incorrect"]
        );
    }
    check_independent_text_failures(first);
    assert_report_order();
    assert!(!first_face_errors(&first.replace("within b1:", "within other:")).is_empty());
    assert_eq!(mixed_report("").0, ["", "", "0"]);
}

fn check_independent_text_failures(first: &str) {
    assert!(!predicates::first_face_errors(&first.replace("Face #2", "Face #1")).is_empty());
    let text = fixture_mixed("wrong b1", "wrong b2", "Face #1");
    let errors = predicates::mixed_report(&("extra\n".to_owned() + &text)).1;
    assert_eq!(
        errors,
        [
            "ERROR: vstate output for shift selection is incorrect",
            "ERROR: no info about object b1 in vstate output",
            "ERROR: no info about object b2 in vstate output",
            "ERROR: vstate output for selected face in test part 2 is incorrect"
        ]
    );
    check_spacing_and_flag();
}

#[derive(Default)]
struct NamedQueryOutput(Vec<String>);
impl super::super::wire::QueryOutputLines for NamedQueryOutput {
    fn line(&mut self, text: &str) {
        // Named fake owns only output; every geometry/state observation remains genuine.
        self.0.push(text.to_owned());
    }
}

fn assert_report_order() {
    let text = fixture_mixed("wrong b1", "wrong b2", "Face #1");
    let (printed, errors) = predicates::mixed_report(&("extra\n".to_owned() + &text));
    let mut output = NamedQueryOutput::default();
    super::super::wire::report_to(&mut output, &printed, &errors);
    let expected: Vec<String> = printed
        .into_iter()
        .chain(errors.into_iter().map(str::to_owned))
        .collect();
    assert_eq!(output.0, expected);
}

fn check_spacing_and_flag() {
    let changed = predicates::B2_ROW.replacen("Selected", "selected", 1);
    assert_eq!(
        predicates::mixed_report(&fixture_mixed(predicates::B1_ROW, &changed, "Face #2")).1,
        ["ERROR: no info about object b2 in vstate output"]
    );
    let changed = predicates::B1_ROW.replacen("                   ", "                  ", 1);
    let errors =
        predicates::mixed_report(&fixture_mixed(&changed, predicates::B2_ROW, "Face #2")).1;
    assert_eq!(errors, ["ERROR: no info about object b1 in vstate output"]);
}
