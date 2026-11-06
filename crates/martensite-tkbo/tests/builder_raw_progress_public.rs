//! Raw General Fuse, Section and MakerVolume weight callbacks and shape counts, on the same real data
//! structures as the frozen native rows (the recorded answers).
use martensite_tkbo::bop_algo::{
    BOPAlgoBuilder, BOPAlgoMakerVolume, BOPAlgoPaveFiller, BOPAlgoSection, Glue, NbShapes, PISteps,
};
use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkernel::standard::Failure;
use std::fmt::Write;
#[allow(dead_code)]
#[path = "pave_raw_progress_public/fixture.rs"]
mod fixture;

const EXPECTED: &str = include_str!("builder_raw_progress_public/expected.txt");
type RawCallback<'a> = &'a dyn Fn(&DataStructure, &mut PISteps) -> Result<(), Failure>;

fn counts_text(counts: NbShapes) -> String {
    format!(
        "{},{},{},{},{},{},{},{}",
        counts.nb_vertices(),
        counts.nb_edges(),
        counts.nb_wires(),
        counts.nb_faces(),
        counts.nb_shells(),
        counts.nb_solids(),
        counts.nb_compsolids(),
        counts.nb_compounds()
    )
}

fn append_kind(
    rows: &mut String,
    prefix: &str,
    kind: &str,
    structure: &DataStructure,
    callback: RawCallback<'_>,
) {
    let counts = counts_text(BOPAlgoBuilder::new().get_nb_shapes(structure).unwrap());
    for (lower, upper) in fixture::BOUNDS {
        let mut steps = fixture::seeded_steps(lower, upper);
        callback(structure, &mut steps).unwrap();
        let values: Vec<String> = fixture::step_bits(&steps)
            .iter()
            .map(u64::to_string)
            .collect();
        let values = values.join(",");
        writeln!(rows, "{prefix}|{kind}|{lower}|{upper}|{counts}|{values}").unwrap();
    }
}

fn append_state(rows: &mut String, prefix: &str, filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    let builder = BOPAlgoBuilder::new();
    append_kind(rows, prefix, "builder", structure, &|ds, steps| {
        builder.fill_pi_steps(ds, steps)
    });
    let section = BOPAlgoSection::new();
    append_kind(rows, prefix, "section", structure, &|ds, steps| {
        section.fill_pi_steps(ds, steps)
    });
    for (intersect, kind) in [(false, "volume0"), (true, "volume1")] {
        let mut volume = BOPAlgoMakerVolume::new();
        volume.set_intersect(intersect);
        append_kind(rows, prefix, kind, structure, &|ds, steps| {
            volume.fill_pi_steps(ds, steps)
        });
    }
}

fn append_configuration(rows: &mut String, recipe: usize, glue_id: usize, glue: Glue) {
    let initialized = fixture::prepared(recipe, glue, false, false);
    append_state(
        rows,
        &format!("{recipe}|{glue_id}|initialized"),
        &initialized,
    );
    let mut performed = fixture::prepared(recipe, glue, false, false);
    performed.perform();
    append_state(rows, &format!("{recipe}|{glue_id}|performed"), &performed);
}

#[test]
fn raw_callbacks_and_counts_match_all_native_rows() {
    let mut actual = String::new();
    for recipe in 0..6 {
        for (glue_id, glue) in fixture::GLUES.into_iter().enumerate() {
            append_configuration(&mut actual, recipe, glue_id, glue);
        }
    }
    assert_eq!(actual.lines().count(), 1152);
    for (index, (actual, expected)) in actual.lines().zip(EXPECTED.lines()).enumerate() {
        assert_eq!(actual, expected, "row={index}");
    }
    assert_eq!(actual.lines().count(), EXPECTED.lines().count());
}

/// Two crossing edges, by hand: one new vertex shared by no source, two edges with pave blocks.
#[test]
fn crossing_edges_have_independent_counts_and_preserve_every_other_slot() {
    let mut filler = fixture::prepared(2, Glue::Off, false, false);
    filler.perform();
    let structure = filler.data_structure().unwrap();
    let counts = BOPAlgoBuilder::new().get_nb_shapes(structure).unwrap();
    assert_eq!(
        (counts.nb_edges(), counts.nb_faces(), counts.nb_solids()),
        (2, 0, 0)
    );
    let mut steps = fixture::seeded_steps(0, 11);
    let builder = BOPAlgoBuilder::new();
    builder.fill_pi_steps(structure, &mut steps).unwrap();
    let edges = f64::from(counts.nb_edges());
    assert_eq!(steps.get_step(1), edges);
    for untouched in [8, 9, 10, 11] {
        assert_eq!(steps.get_step(untouched), -7.25, "slot={untouched}");
    }
}

/// Face weights carry the native factors: 20 for General Fuse and 50 for a volume, whose solid slot
/// is written with or without intersection.
#[test]
fn face_weight_factors_follow_the_receiver() {
    let mut filler = fixture::prepared(4, Glue::Off, false, false);
    filler.perform();
    let structure = filler.data_structure().unwrap();
    let faces = f64::from(
        BOPAlgoBuilder::new()
            .get_nb_shapes(structure)
            .unwrap()
            .nb_faces(),
    );
    assert!(faces > 0.0, "overlapping boxes must have active faces");
    let mut steps = fixture::seeded_steps(0, 10);
    BOPAlgoBuilder::new()
        .fill_pi_steps(structure, &mut steps)
        .unwrap();
    assert_eq!(steps.get_step(3), 20.0 * faces);
    let mut volume = BOPAlgoMakerVolume::new();
    let mut steps = fixture::seeded_steps(0, 10);
    volume.set_intersect(false);
    volume.fill_pi_steps(structure, &mut steps).unwrap();
    assert_eq!(
        (steps.get_step(3), steps.get_step(10)),
        (-7.25, 50.0 * faces)
    );
    volume.set_intersect(true);
    volume.fill_pi_steps(structure, &mut steps).unwrap();
    assert_eq!(steps.get_step(3), 50.0 * faces);
}
