// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkhelix::authored::*;
use martensite_tkmath::gp::Axis2;
use std::{
    f64::consts::{FRAC_1_PI, PI, TAU},
    num::NonZeroU64,
};

fn cylinder(rotation: HelixRotation) -> HelixSequencePlacement {
    HelixSequencePlacement {
        frame: Axis2::default(),
        radius: 4.,
        rotation,
    }
}

fn row(id: u64, pitch: f64, turns: f64) -> HelixPitchRow {
    HelixPitchRow {
        id: HelixRowId(NonZeroU64::new(id).unwrap()),
        pitch,
        turns,
    }
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-13, "{a:.16e} vs {b:.16e}");
}

#[test]
fn ordered_pitch_rows_preserve_owned_identity_and_one_sided_derivatives() {
    let mut rows = [row(9, 2., 1.), row(2, 4., 0.5)];
    let law = HelixSequence::pitch_table(cylinder(HelixRotation::Positive), &rows).unwrap();
    let original = rows;
    rows.reverse();
    rows[0].pitch = 7.;
    assert_ne!(rows, original);
    assert!(
        matches!(law.definition(), HelixSequenceDefinition::PitchTable(saved) if &**saved == original.as_slice())
    );
    let spans = law.spans().collect::<Vec<_>>();
    assert_eq!(spans[0].source, HelixSpanSource::Row(original[0].id));
    assert_eq!(spans[1].source, HelixSpanSource::Row(original[1].id));
    assert_eq!(spans[0].parameters, [0., TAU]);
    assert_eq!(spans[1].parameters, [TAU, 3. * PI]);
    assert_eq!(spans[0].elevations, [0., 2.]);
    assert_eq!(spans[1].elevations, [2., 4.]);

    let incoming = law.evaluate(TAU, HelixParameterSide::Before).unwrap();
    let outgoing = law.evaluate(TAU, HelixParameterSide::After).unwrap();
    assert_eq!(incoming.position, outgoing.position);
    assert_eq!(outgoing.position.x(), 4.);
    assert_eq!(outgoing.position.z(), 2.);
    near(outgoing.position.y(), 0.);
    assert_eq!(incoming.first.z(), FRAC_1_PI);
    assert_eq!(outgoing.first.z(), 2. * FRAC_1_PI);
    near(incoming.first.y(), 4.);
    near(outgoing.first.y(), 4.);
    let end = law.evaluate(3. * PI, HelixParameterSide::Before).unwrap();
    assert_eq!(end.position.x(), -4.);
    near(end.position.y(), 0.);
    assert_eq!(end.position.z(), 4.);
}

fn two_flat_ends() -> FlatEndHelixRequest {
    let zone = HelixEndZone {
        flat_turns: 0.125,
        transition_turns: 0.25,
    };
    FlatEndHelixRequest {
        placement: cylinder(HelixRotation::Negative),
        pitch: 2.,
        turns: 1.5,
        start: Some(zone),
        end: Some(zone),
    }
}

#[test]
fn flat_end_quartic_integrates_one_eighth_pitch_and_joins_c2() {
    let law = HelixSequence::flat_ends(two_flat_ends()).unwrap();
    let spans = law.spans().collect::<Vec<_>>();
    assert_eq!(
        spans.iter().map(|span| span.source).collect::<Vec<_>>(),
        [
            HelixSpanSource::StartFlat,
            HelixSpanSource::StartTransition,
            HelixSpanSource::Natural,
            HelixSpanSource::EndTransition,
            HelixSpanSource::EndFlat,
        ]
    );
    for (span, expected) in
        spans
            .iter()
            .zip([[0., 0.], [0., 0.25], [0.25, 1.75], [1.75, 2.], [2., 2.]])
    {
        assert_eq!(span.elevations, expected);
    }
    for span in &spans[..spans.len() - 1] {
        let t = span.parameters[1];
        assert_eq!(
            law.evaluate(t, HelixParameterSide::Before).unwrap(),
            law.evaluate(t, HelixParameterSide::After).unwrap(),
        );
    }
    let begin = law.evaluate(0., HelixParameterSide::After).unwrap();
    let final_sample = law.evaluate(3. * PI, HelixParameterSide::Before).unwrap();
    assert_eq!(begin.first.y(), -4.);
    assert_eq!(begin.first.z(), 0.);
    assert_eq!(final_sample.first.z(), 0.);
    assert_eq!(final_sample.position.z(), 2.);

    let rising = midpoint(&law, spans[1]);
    let falling = midpoint(&law, spans[3]);
    near(rising.position.z(), 3. / 64.);
    near(falling.position.z(), 2. - 3. / 64.);
    near(rising.first.z(), 0.5 * FRAC_1_PI);
    near(falling.first.z(), 0.5 * FRAC_1_PI);
    near(rising.second.z(), 3. * FRAC_1_PI * FRAC_1_PI);
    near(falling.second.z(), -3. * FRAC_1_PI * FRAC_1_PI);
}

fn midpoint(law: &HelixSequence, span: HelixSequenceSpan) -> HelixEvaluation {
    let [start, end] = span.parameters;
    law.evaluate(start.midpoint(end), HelixParameterSide::After)
        .unwrap()
}

#[test]
fn exact_zone_overlap_and_collapsed_row_stations_are_typed() {
    let zone = HelixEndZone {
        flat_turns: 0.5,
        transition_turns: 0.5_f64.next_up(),
    };
    assert_eq!(zone.flat_turns + zone.transition_turns, 1.);
    assert!(matches!(
        HelixSequence::flat_ends(FlatEndHelixRequest {
            start: Some(zone),
            end: None,
            turns: 1.,
            ..two_flat_ends()
        }),
        Err(HelixSequenceError::OverlappingEndZones)
    ));
    let zone = HelixEndZone {
        flat_turns: 1.,
        transition_turns: 0.125,
    };
    assert!(matches!(
        HelixSequence::flat_ends(FlatEndHelixRequest {
            start: Some(zone),
            ..two_flat_ends()
        }),
        Err(HelixSequenceError::EndZone { start: true, .. })
    ));
    let rows = [row(8, 2., 1e16), row(7, 2., 0.01)];
    assert!(matches!(
        HelixSequence::pitch_table(cylinder(HelixRotation::Positive), &rows),
        Err(HelixSequenceError::UnrepresentableSpan(HelixSpanSource::Row(id))) if id == rows[1].id
    ));
    assert!(matches!(
        HelixSequence::pitch_table(cylinder(HelixRotation::Positive), &[rows[0]; 2]),
        Err(HelixSequenceError::DuplicateRow(id)) if id == rows[0].id
    ));
    assert!(matches!(
        HelixSequence::pitch_table(cylinder(HelixRotation::Positive), &[]),
        Err(HelixSequenceError::RowCount(0))
    ));
}

#[test]
fn touching_transition_zones_keep_order_and_admit_only_available_sides() {
    let zone = HelixEndZone {
        flat_turns: 0.,
        transition_turns: 0.5,
    };
    let law = HelixSequence::flat_ends(FlatEndHelixRequest {
        start: Some(zone),
        end: Some(zone),
        turns: 1.,
        ..two_flat_ends()
    })
    .unwrap();
    assert_eq!(law.spans().len(), 2);
    assert_eq!(
        law.evaluate(PI, HelixParameterSide::After).unwrap(),
        law.evaluate(PI, HelixParameterSide::Before).unwrap(),
    );
    for (parameter, side) in [
        (0., HelixParameterSide::Before),
        (TAU, HelixParameterSide::After),
        (f64::NAN, HelixParameterSide::After),
    ] {
        assert!(matches!(
            law.evaluate(parameter, side),
            Err(HelixSequenceError::ParameterSide { .. })
        ));
    }
    assert_eq!(
        law.evaluate(TAU, HelixParameterSide::Before)
            .unwrap()
            .position
            .z(),
        1.
    );
}

#[path = "helix_sequence/native.rs"]
mod native;
