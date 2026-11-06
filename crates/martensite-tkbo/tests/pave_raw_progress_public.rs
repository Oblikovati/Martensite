//! Raw concrete weight callback preserves caller slots and observable iterator state.
use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, Glue, PISteps};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::fmt::Write;
#[path = "pave_raw_progress_public/fixture.rs"]
mod fixture;
const EXPECTED: [&str; 6] = [
    include_str!("pave_raw_progress_public/expected/recipe0.expected"),
    include_str!("pave_raw_progress_public/expected/recipe1.expected"),
    include_str!("pave_raw_progress_public/expected/recipe2.expected"),
    include_str!("pave_raw_progress_public/expected/recipe3.expected"),
    include_str!("pave_raw_progress_public/expected/recipe4.expected"),
    include_str!("pave_raw_progress_public/expected/recipe5.expected"),
];

#[test]
fn concrete_raw_callback_has_a_mutable_receiver_and_caller_owned_steps() {
    let _: fn(&mut BOPAlgoPaveFiller, &mut PISteps) -> Result<(), Failure> =
        BOPAlgoPaveFiller::fill_pi_steps;
}

#[test]
fn valid_profiles_match_all_raw_slot_bits_and_final_iterator_observations() {
    let mut count = 0;
    for (recipe, expected) in EXPECTED.iter().enumerate() {
        let mut actual = String::new();
        for (glue_id, glue) in fixture::GLUES.into_iter().enumerate() {
            append_glue_rows(&mut actual, recipe, glue_id, glue);
        }
        assert_eq!(
            actual, *expected,
            "recipe={recipe}; expected complete unchanged raw observations"
        );
        count += actual.lines().count();
        println!("RAW_PAVE_ROWS_BEGIN\n{actual}RAW_PAVE_ROWS_END");
    }
    assert_eq!(count, 576);
}

#[derive(Clone, Copy)]
struct RawConfiguration {
    recipe: usize,
    glue_id: usize,
    avoid: bool,
    parallel: bool,
}

impl RawConfiguration {
    fn prefix(self, bounds: (i32, i32)) -> String {
        let Self {
            recipe,
            glue_id,
            avoid,
            parallel,
        } = self;
        let (lower, upper) = bounds;
        let (avoid, parallel) = (u8::from(avoid), u8::from(parallel));
        format!("{recipe}|{glue_id}|{avoid}|{parallel}|{lower}|{upper}")
    }
}

fn append_glue_rows(rows: &mut String, recipe: usize, glue_id: usize, glue: Glue) {
    for (avoid, parallel) in [(false, false), (false, true), (true, false), (true, true)] {
        let configuration = RawConfiguration {
            recipe,
            glue_id,
            avoid,
            parallel,
        };
        append_configuration(rows, configuration, glue);
    }
}

fn append_configuration(rows: &mut String, configuration: RawConfiguration, glue: Glue) {
    let mut filler = fixture::prepared(
        configuration.recipe,
        glue,
        configuration.avoid,
        configuration.parallel,
    );
    for bounds in fixture::BOUNDS {
        append_raw_row(rows, &mut filler, configuration, bounds);
    }
}

fn append_raw_row(
    rows: &mut String,
    filler: &mut BOPAlgoPaveFiller,
    configuration: RawConfiguration,
    bounds: (i32, i32),
) {
    advance_vertex_edge_cursor(filler);
    let mut steps = fixture::seeded_steps(bounds.0, bounds.1);
    filler.fill_pi_steps(&mut steps).unwrap();
    let cursor = cursor_text(filler);
    let values = fixture::step_bits(&steps)
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    writeln!(rows, "{}|{cursor}|{values}", configuration.prefix(bounds)).unwrap();
}

fn advance_vertex_edge_cursor(filler: &mut BOPAlgoPaveFiller) {
    let iterator = filler.iterator_mut().unwrap();
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Edge);
    if iterator.more() {
        iterator.next();
    }
}

fn cursor_text(filler: &BOPAlgoPaveFiller) -> String {
    let iterator = filler.iterator().unwrap();
    let count = iterator.expected_length();
    let more = u8::from(iterator.more());
    let (first, second) = if iterator.more() {
        iterator.value().unwrap()
    } else {
        (-1, -1)
    };
    format!("{count}|{more}|{first},{second}")
}

#[test]
fn crossing_segments_have_independent_nonuniform_integer_and_fractional_weights() {
    let mut filler = fixture::prepared(2, Glue::Off, false, false);
    let mut steps = fixture::seeded_steps(0, 13);
    filler.fill_pi_steps(&mut steps).unwrap();
    let expected: [f64; 14] = [-7.25, 0., 0., 5., 0., 0., 0., 2., 0., 0., 1., 0., 0.2, 0.1];
    assert_eq!(fixture::step_bits(&steps), expected.map(f64::to_bits));
    assert_eq!(filler.iterator().unwrap().expected_length(), 0);
    assert!(!filler.iterator().unwrap().more());
}

#[test]
fn coincident_owned_vertices_reserve_no_normalization_or_constant_slot() {
    let mut filler = fixture::prepared(1, Glue::Off, false, false);
    let mut steps = fixture::seeded_steps(-2, 16);
    filler.fill_pi_steps(&mut steps).unwrap();
    assert_eq!(steps.get_step(0).to_bits(), (-7.25_f64).to_bits());
    assert_eq!(steps.get_step(1), 1.);
    assert_eq!(steps.get_step(6).to_bits(), 0.2_f64.to_bits());
    for untouched in [-2, -1, 14, 15, 16] {
        assert_eq!(steps.get_step(untouched).to_bits(), (-7.25_f64).to_bits());
    }
}

#[test]
fn raw_callback_rewinds_the_actual_face_face_cursor() {
    let mut filler = fixture::prepared(3, Glue::Off, false, false);
    let iterator = filler.iterator_mut().unwrap();
    iterator.initialize(ShapeEnum::Face, ShapeEnum::Face);
    let first = iterator.value().unwrap();
    let count = iterator.expected_length();
    iterator.next();
    assert_ne!(iterator.value().unwrap(), first);
    filler
        .fill_pi_steps(&mut fixture::seeded_steps(14, 16))
        .unwrap();
    let iterator = filler.iterator().unwrap();
    assert_eq!(iterator.expected_length(), count);
    assert_eq!(iterator.value().unwrap(), first);
}

#[test]
fn missing_iterator_returns_typed_failure_without_changing_steps_or_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let mut steps = fixture::seeded_steps(-2, 16);
    let before = fixture::step_bits(&steps);
    let failure = filler.fill_pi_steps(&mut steps).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(failure.message().contains("initialize()"));
    assert_eq!(fixture::step_bits(&steps), before);
    assert!(!filler.options().has_errors());
}

#[test]
fn raw_weight_callback_preserves_existing_reports_and_constant_policy() {
    use martensite_tkbo::bop_algo::AlertUserBreak;
    use martensite_tkernel::{message::AlertHandle, standard::RuntimeTyped};
    let mut filler = fixture::prepared(2, Glue::Off, false, false);
    filler.set_non_destructive(true);
    let alert: AlertHandle = std::sync::Arc::new(AlertUserBreak);
    filler.options().add_error(&alert);
    let mut steps = fixture::seeded_steps(0, 13);
    filler.fill_pi_steps(&mut steps).unwrap();
    assert!(filler.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(steps.get_step(0).to_bits(), (-7.25_f64).to_bits());
    assert_eq!(steps.get_step(3), 5.);
    assert!(filler.non_destructive());
}
