// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use certification::bernstein_execution::BernsteinStoredReport;
use certification::conditioned_stage::ConditionedStage;

pub(super) fn result(result: Result<BoundaryCensus, CensusCause>) -> Vec<String> {
    match result {
        Ok(census) => census_rows(&census),
        Err(CensusCause::Cancelled {
            completed,
            next_domain,
            during_domain,
        }) => {
            let mut rows = census_rows(&completed);
            rows.push(format!("cancel {next_domain:?} during={during_domain}"));
            rows
        }
        Err(_) => panic!("unexpected pre-enumeration refusal"),
    }
}
pub(super) fn census_rows(census: &BoundaryCensus) -> Vec<String> {
    let mut rows = replay::census_snapshot(census);
    let paired = census.paired.as_ref().unwrap();
    rows.push(format!(
        "selector {:?} signs {:?}",
        paired.exclusion_version, paired.selected_signs
    ));
    rows.extend(paired.exclusions.iter().flat_map(record_rows));
    rows.push(format!("obligations {:?}", census.current_obligations()));
    rows
}
fn record_rows(record: &BernsteinStoredReport) -> Vec<String> {
    let mut rows = vec![format!(
        "sources {:?} mode {:?} effective {:?} work {}",
        record.sources,
        record.mode,
        record.effective,
        record.completed_work()
    )];
    match &record.outcome {
        Ok(execution) => {
            rows.extend(stage_rows(&execution.exclusion));
            if let Some(fallback) = &execution.fallback {
                rows.extend(replay::report_snapshot(fallback));
            }
        }
        Err(cause) => rows.push(format!("capture {cause:?}")),
    }
    rows
}
fn stage_rows(stage: &ConditionedStage) -> Vec<String> {
    let mut rows = match stage {
        ConditionedStage::Bernstein(report) => vec![format!("bernstein {report:?}")],
        ConditionedStage::Geometric(report) => vec![format!("geometric {report:?}")],
    };
    rows.push(format!(
        "policy {:?} work {:?} completion {:?}",
        stage.policy(),
        stage.work(),
        stage.completion()
    ));
    rows.extend(stage.source().captured_curves().map(replay::curve_snapshot));
    rows
}
