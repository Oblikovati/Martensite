// SPDX-License-Identifier: GPL-2.0-only
use super::{
    PROFILE_USES, SECTION, SPINE, SPINE_KEYS, ShapeEnum, SweepFailure, archive, options, point_id,
    profile, spine, uses,
};
use martensite_tkbo::measured::Evidence;
use martensite_tkgeombase::gprop::GProps;
use martensite_tksketch::spatial_sweep::{
    DetachedSolidSweep, SolidSweepHistory, SolidSweepImage, SolidSweepOptions, SolidSweepResult,
    sweep_rounded_solid,
};
use martensite_tktopalgo::brep_gprop::BRepGProp;

static_assertions::assert_impl_all!(SolidSweepOptions: Send, Sync);
static_assertions::assert_impl_all!(SolidSweepResult: Send, Sync);
static_assertions::assert_impl_all!(SolidSweepHistory: Send, Sync);
static_assertions::assert_impl_all!(SolidSweepImage: Send, Sync);
static_assertions::assert_impl_all!(DetachedSolidSweep: Send, Sync);

fn solid_options() -> SolidSweepOptions {
    SolidSweepOptions {
        sweep: options(),
        volume_error_limit: 1e-9,
    }
}

#[test]
fn e10_closed_sweep_binds_actual_cap_and_encloses_the_independent_volume() {
    let result =
        sweep_rounded_solid(&spine(SPINE), &profile(SECTION), solid_options(), || false).unwrap();
    assert_volume(&result, 11.5 + std::f64::consts::PI / 8.);
    let detached = result.export_brep(|| false).unwrap();
    assert_eq!(detached.solid.shape_type().unwrap(), ShapeEnum::Solid);
    assert!(result.validation().is_valid());
    assert_eq!(result.history().len(), 86);
    let census = &result.validation().census[0];
    assert_eq!(
        (census.vertices, census.edges, census.faces, census.genus),
        (24, 42, 20, Some(0))
    );
    assert_caps(&detached);
}

fn assert_volume(result: &SolidSweepResult, expected: f64) {
    let detached = result.export_brep(|| false).unwrap();
    let mut independent = GProps::default();
    BRepGProp::volume_properties(&detached.solid, &mut independent, true, false, false).unwrap();
    let actual = independent.mass();
    assert!((actual - expected).abs() < 1e-10);
    let enclosure = result.volume();
    assert_eq!(enclosure.evidence(), Evidence::Proven);
    let support = result.support_evidence();
    assert!(
        support.achieved_trace_bound > 0. && support.achieved_trace_bound < support.trace_limit
    );
    let closure = result.closure_volume_error_bound();
    assert!(closure > 0. && closure <= enclosure.error_bound && closure < 1e-9);
    assert!(
        enclosure.encloses(expected) && enclosure.encloses(actual),
        "{expected}, {actual}, {enclosure:?}"
    );
}

fn assert_caps(detached: &DetachedSolidSweep) {
    for station in [point_id(SPINE_KEYS[0]), point_id(SPINE_KEYS[3])] {
        let line = uses(&PROFILE_USES);
        let cap = detached.images.iter().find(|image| {
            matches!(&image.history, SolidSweepHistory::Cap { station: point, profile } if *point == station && profile.as_slice() == line)
        }).unwrap();
        let mut area = GProps::default();
        BRepGProp::surface_properties(&cap.shape, &mut area, false, false).unwrap();
        assert!((area.mass() - 1.).abs() <= 1e-10);
    }
}

#[test]
fn enclosure_translates_with_actual_body_and_refuses_an_insufficient_volume_budget() {
    let tiny = SolidSweepOptions {
        volume_error_limit: f64::EPSILON,
        ..solid_options()
    };
    assert!(
        matches!(sweep_rounded_solid(&spine(SPINE), &profile(SECTION), tiny, || false),
        Err(SweepFailure::VolumeErrorBound { bound, limit }) if bound > limit)
    );
    let along = spine(SPINE.map(translate));
    let section = profile(SECTION.map(translate));
    let solid = sweep_rounded_solid(&along, &section, solid_options(), || false).unwrap();
    assert_volume(&solid, 11.5 + std::f64::consts::PI / 8.);
    let again = sweep_rounded_solid(&along, &section, solid_options(), || false).unwrap();
    assert_eq!(
        solid.history().collect::<Vec<_>>(),
        again.history().collect::<Vec<_>>()
    );
    assert_eq!(
        archive(&solid.export_brep(|| false).unwrap().solid),
        archive(&again.export_brep(|| false).unwrap().solid)
    );
}

fn translate(p: [f64; 3]) -> [f64; 3] {
    [p[0] + 8., p[1] - 16., p[2] + 32.]
}

#[test]
fn non_square_section_integrates_distinct_actual_turn_radii() {
    let section = profile(SECTION.map(|[x, y, z]| [x, 2. * y, z]));
    let result = sweep_rounded_solid(&spine(SPINE), &section, solid_options(), || false).unwrap();
    assert_volume(&result, 22.5 + 3. * std::f64::consts::PI / 8.);
}

#[test]
fn independent_parallel_operations_match_a_serial_native_archive_and_history() {
    use std::{sync::Barrier, thread};

    let path = spine(SPINE);
    let profile = profile(SECTION);
    let serial = sweep_rounded_solid(&path, &profile, solid_options(), || false).unwrap();
    let serial_state = solid_state(&serial);
    let barrier = Barrier::new(4);
    thread::scope(|scope| {
        let workers = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    let result =
                        sweep_rounded_solid(&path, &profile, solid_options(), || false).unwrap();
                    solid_state(&result)
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial_state);
        }
    });
}

fn solid_state(result: &SolidSweepResult) -> (Vec<u8>, Vec<SolidSweepHistory>, [f64; 4]) {
    let detached = result.export_brep(|| false).unwrap();
    let history = result.history().cloned().collect();
    let volume = result.volume();
    (
        archive(&detached.solid),
        history,
        [
            volume.value,
            volume.error_bound,
            result.closure_volume_error_bound(),
            result.support_evidence().achieved_trace_bound,
        ],
    )
}

#[test]
fn cancelling_late_in_volume_phase_refuses_publication() {
    let path = spine(SPINE);
    let profile = profile(SECTION);
    let mut checkpoints = 0;
    sweep_rounded_solid(&path, &profile, solid_options(), || {
        checkpoints += 1;
        false
    })
    .unwrap();
    let mut active = 0;
    assert!(matches!(
        sweep_rounded_solid(&path, &profile, solid_options(), || {
            active += 1;
            active >= checkpoints - 5
        }),
        Err(SweepFailure::Cancelled(
            martensite_tksketch::spatial_sweep::SweepStage::Integration
        ))
    ));
}
