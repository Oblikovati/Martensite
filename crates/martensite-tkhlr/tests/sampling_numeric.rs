//! Whole original controlled-adaptor range corpus, historical and explicit local modes.
#[path = "replay_support/mod.rs"]
mod replay_support;
use martensite_tkhlr::contap::ContapSamplingContext;
use replay_support::{check_request_slice, check_request_slice_with, replay};
use std::{ops::Range, sync::Mutex};
// This test owns historical range reports only; serialization proves no production safety.
static HISTORICAL_SAMPLE_REPORT_CONTEXT: Mutex<()> = Mutex::new(());
const REQUESTS: &str = include_str!("fixtures/sampling_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/sampling_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    let _context = HISTORICAL_SAMPLE_REPORT_CONTEXT.lock().unwrap();
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn all_original_sampling_requests_reset_the_same_valid_baseline() {
    assert_eq!(REQUESTS.lines().count(), 2772);
    assert_eq!(EXPECTATIONS.lines().count(), 2772);
    let mut requests = REQUESTS
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>());
    let baseline = requests.next().unwrap();
    assert_eq!(baseline[12], "0.0");
    for request in requests {
        assert_eq!(&request[..13], &baseline[..13]);
    }
}

#[test]
fn local_sampling_context_preserves_every_frozen_native_observation() {
    let mut context = ContapSamplingContext::new();
    for (request, expected) in REQUESTS
        .split_inclusive('\n')
        .zip(EXPECTATIONS.split_inclusive('\n'))
    {
        check_request_slice_with(request, expected, |line| {
            replay::execute_request_with_context(line, &mut context)
        });
    }
}

#[test]
fn parallel_local_sampling_reports_match_serial_bytes() {
    let requests: Vec<_> = REQUESTS.split_inclusive('\n').collect();
    let serial = local_outputs(&requests, 0, 1);
    let mut parallel = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|worker| {
                let requests = &requests;
                scope.spawn(move || local_outputs(requests, worker, 4))
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    parallel.sort_by_key(|(index, _)| *index);
    assert_eq!(parallel, serial);
}

fn local_outputs(requests: &[&str], first: usize, step: usize) -> Vec<(usize, String)> {
    let mut context = ContapSamplingContext::new();
    (first..requests.len())
        .step_by(step)
        .map(|index| {
            (
                index,
                replay::execute_request_with_context(requests[index], &mut context).unwrap(),
            )
        })
        .collect()
}

#[path = "sampling_numeric/cases_00.rs"]
mod cases_00;

#[path = "sampling_numeric/cases_01.rs"]
mod cases_01;

#[path = "sampling_numeric/cases_02.rs"]
mod cases_02;

#[path = "sampling_numeric/cases_03.rs"]
mod cases_03;

#[path = "sampling_numeric/cases_04.rs"]
mod cases_04;

#[path = "sampling_numeric/cases_05.rs"]
mod cases_05;

#[path = "sampling_numeric/cases_06.rs"]
mod cases_06;

#[path = "sampling_numeric/cases_07.rs"]
mod cases_07;

#[path = "sampling_numeric/cases_08.rs"]
mod cases_08;

#[path = "sampling_numeric/cases_09.rs"]
mod cases_09;

#[path = "sampling_numeric/cases_10.rs"]
mod cases_10;

#[path = "sampling_numeric/cases_11.rs"]
mod cases_11;

#[path = "sampling_numeric/cases_12.rs"]
mod cases_12;

#[path = "sampling_numeric/cases_13.rs"]
mod cases_13;

#[path = "sampling_numeric/cases_14.rs"]
mod cases_14;

#[path = "sampling_numeric/cases_15.rs"]
mod cases_15;

#[path = "sampling_numeric/cases_16.rs"]
mod cases_16;

#[path = "sampling_numeric/cases_17.rs"]
mod cases_17;

#[path = "sampling_numeric/cases_18.rs"]
mod cases_18;

#[path = "sampling_numeric/cases_19.rs"]
mod cases_19;

#[path = "sampling_numeric/cases_20.rs"]
mod cases_20;

#[path = "sampling_numeric/cases_21.rs"]
mod cases_21;

#[path = "sampling_numeric/cases_22.rs"]
mod cases_22;

#[path = "sampling_numeric/cases_23.rs"]
mod cases_23;

#[path = "sampling_numeric/cases_24.rs"]
mod cases_24;

#[path = "sampling_numeric/cases_25.rs"]
mod cases_25;

#[path = "sampling_numeric/cases_26.rs"]
mod cases_26;

#[path = "sampling_numeric/cases_27.rs"]
mod cases_27;

#[path = "sampling_numeric/cases_28.rs"]
mod cases_28;

#[path = "sampling_numeric/cases_29.rs"]
mod cases_29;

#[path = "sampling_numeric/cases_30.rs"]
mod cases_30;

#[path = "sampling_numeric/cases_31.rs"]
mod cases_31;

#[path = "sampling_numeric/cases_32.rs"]
mod cases_32;

#[path = "sampling_numeric/cases_33.rs"]
mod cases_33;

#[path = "sampling_numeric/cases_34.rs"]
mod cases_34;
