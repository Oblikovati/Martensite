// SPDX-License-Identifier: GPL-2.0-only
//! Explicit V5 loans share the existing admission, proof and construction core.
use super::*;
use crate::profile_extrusion as feature;
/// Lend a finite operation integration allowance to an explicitly selected V5 graph.
/// Example: `build_planar_regions_with_integration_budget_and_cancel(&profile, resolution, work, version5, 8192, || false)?`.
pub fn build_planar_regions_with_integration_budget_and_cancel(
    profile: &SketchProfile,
    resolution: Resolution,
    requested: feature::PlanarProofWorkLimit,
    version: feature::PlanarProofVersion,
    remaining_integration_cells: u64,
    mut cancelled: impl FnMut() -> bool + Send,
) -> feature::PlanarRegionWorkResult {
    if version.integral_options().is_none() {
        return Err(unsupported_policy(
            requested,
            version,
            remaining_integration_cells,
        ));
    }
    build_selected(
        profile,
        resolution,
        requested,
        version,
        Some(remaining_integration_cells),
        ProfileTopologyMode::Legacy,
        &mut cancelled,
    )
}
fn effective_version(
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
) -> feature::PlanarProofVersion {
    match (version.integral_options(), allowance) {
        (Some(options), Some(allowance)) => {
            version.with_integration_limit(options.max_total_subintervals.min(allowance))
        }
        _ => version,
    }
}

pub(super) fn build_selected(
    source: &SketchProfile,
    resolution: Resolution,
    requested: feature::PlanarProofWorkLimit,
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
    mode: ProfileTopologyMode,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> feature::PlanarRegionWorkResult {
    if cancelled() {
        return Err(cancelled_failure(requested, Stage::Admission, None));
    }
    if matches!(
        version,
        feature::PlanarProofVersion::DirectedSelectedCapV1 { .. }
    ) {
        return Err(unsupported_policy(
            requested,
            version,
            allowance.unwrap_or(0),
        ));
    }
    validate_requested(requested, version, allowance)?;
    let profile = admit_closed_profile(source, resolution, requested)?;
    let effective = effective_version(version, allowance);
    let certified = certified_for_work_with_cancel(&profile, requested, effective, &mut *cancelled)
        .map_err(|failure| attach_program_policy(failure, version, allowance))?;
    let built = construct_selected(&profile, &certified, requested, mode, cancelled);
    let mut built = built
        .map_err(|failure| attach_completed_program(failure, &certified, version, allowance))?;
    built.integral = issued_program(&certified, source, version, allowance);
    Ok(built)
}
fn construct_selected(
    profile: &AdmittedProfile,
    certified: &CertifiedRegions,
    requested: feature::PlanarProofWorkLimit,
    mode: ProfileTopologyMode,
    cancelled: &mut impl FnMut() -> bool,
) -> feature::PlanarRegionWorkResult {
    if cancelled() {
        return Err(cancelled_failure(
            requested,
            Stage::Construction,
            Some(certified.work),
        ));
    }
    construct_regions_with_cancel(profile, certified, requested, mode, cancelled)
}
fn issued_program(
    certified: &CertifiedRegions,
    source: &SketchProfile,
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
) -> Option<feature::PlanarIntegralEvidence> {
    Some(feature::PlanarIntegralEvidence::issue(
        version,
        allowance?,
        std::sync::Arc::clone(certified.programs.as_ref()?),
        source,
        certified.parents.clone(),
    ))
}
fn attach_program_policy(
    mut failure: feature::PlanarRegionWorkFailure,
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
) -> feature::PlanarRegionWorkFailure {
    if let Some(program) = failure.integral.as_mut() {
        program.requested = version;
        program.entry_allowance = allowance.unwrap_or(program.entry_allowance);
    }
    failure
}
fn attach_completed_program(
    failure: feature::PlanarRegionWorkFailure,
    certified: &CertifiedRegions,
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
) -> feature::PlanarRegionWorkFailure {
    let Some(programs) = &certified.programs else {
        return failure;
    };
    failure.with_completed_integral(
        version,
        allowance.unwrap_or(0),
        programs.options,
        programs.consumed,
    )
}

fn unsupported_policy(
    requested: feature::PlanarProofWorkLimit,
    version: feature::PlanarProofVersion,
    remaining: u64,
) -> feature::PlanarRegionWorkFailure {
    let cause = feature::DeclineCause::IncompleteCertification {
        reason: format!(
            "integration loan {remaining} requires FilteredCompositeBernsteinV5, actual {version:?}"
        ),
    };
    feature::PlanarRegionWorkFailure::new(requested, DECLINE.at(Stage::Admission, cause), None)
        .with_integral_policy(version, remaining)
}
pub(super) fn refusal_program(
    version: feature::PlanarProofVersion,
    failure: &feature::proof::RegionProofFailure,
) -> Option<feature::PlanarIntegralFailure> {
    let program = failure.program.as_ref()?;
    Some(feature::PlanarIntegralFailure {
        requested: version,
        entry_allowance: version.integral_options()?.max_total_subintervals,
        source: failure.program_source,
        options: program.options,
        consumed: program.consumed,
        failure: Some(program.clone()),
    })
}

fn validate_requested(
    work: feature::PlanarProofWorkLimit,
    version: feature::PlanarProofVersion,
    allowance: Option<u64>,
) -> Result<(), feature::PlanarRegionWorkFailure> {
    let Some(options) = version.integral_options() else {
        return Ok(());
    };
    let invalid = invalid_scalar(options);
    let Some(invalid) = invalid else {
        return Ok(());
    };
    Err(invalid_policy(
        work,
        version,
        allowance.unwrap_or(options.max_total_subintervals),
        invalid,
    ))
}
fn invalid_scalar(
    options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
) -> Option<(&'static str, u64, u64, u64)> {
    let maximum =
        u64::from(options.integration.max_curves) * u64::from(options.integration.max_subintervals);
    if options.max_components == 0 || options.max_components > 64 {
        Some(("max_components", u64::from(options.max_components), 1, 64))
    } else if options.max_total_subintervals > maximum {
        Some((
            "max_total_subintervals",
            options.max_total_subintervals,
            0,
            maximum,
        ))
    } else {
        None
    }
}
fn invalid_policy(
    work: feature::PlanarProofWorkLimit,
    version: feature::PlanarProofVersion,
    allowance: u64,
    invalid: (&'static str, u64, u64, u64),
) -> feature::PlanarRegionWorkFailure {
    let cause = feature::DeclineCause::IncompleteCertification {
        reason: format!(
            "original V5 {}={} outside {}..={}",
            invalid.0, invalid.1, invalid.2, invalid.3
        ),
    };
    feature::PlanarRegionWorkFailure::new(work, DECLINE.at(Stage::Admission, cause), None)
        .with_invalid_integral_policy(version, allowance, invalid)
}
