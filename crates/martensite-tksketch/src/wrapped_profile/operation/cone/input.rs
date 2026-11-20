//! Capture directed solved sources before any fit, native owner or proof is created.
use super::*;
pub(in crate::surface_sketch::wrapped_profile) mod grant;
mod policy;
pub(super) fn poll(cancelled: &mut CancelPoll<'_>, stage: WrappedStage) -> Checked<()> {
    super::super::input::poll(cancelled, stage)
}
pub(super) fn capture(
    request: &WrappedConeNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    admit(request).map_err(ConeFailure::public)?;
    capture_admitted(request, cancelled)
}
pub(super) fn capture_with_selection(
    request: &WrappedConeNormalRequest,
    execution: WrappedConeExecutionSelection,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    if let WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 { domain } = execution {
        if domain == WrappedConeDomainPolicy::PositiveXRegionAtlasV5 {
            return Err(rounded_atlas().public());
        }
        return capture(request, cancelled);
    }
    admit_with_ceiling(request, grant::admission_ceiling(execution))
        .map_err(ConeFailure::public)?;
    // #563: reject the selected policy before source capture can observe a later stop.
    super::prepare::directed::admit(request, execution).map_err(ConeFailure::public)?;
    let mut inputs = capture_admitted(request, cancelled)?;
    inputs.grant =
        grant::capture(request, execution, &inputs.originals).map_err(ConeFailure::public)?;
    Ok(inputs)
}
fn rounded_atlas() -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Unsupported,
        ConeCause::Scalar {
            field: "atlas native qualification mode",
            actual: 0.,
            expected: "directed stored-solid separation",
        },
    )
    .at(WrappedStage::Admission)
}
fn capture_admitted(
    request: &WrappedConeNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    let mut inputs = Inputs {
        originals: Vec::new(),
        grant: None,
        directions: BTreeMap::new(),
    };
    super::super::source::read(
        &request.profile,
        &request.selections,
        cancelled,
        |selection, reader| push_region(&mut inputs, request, selection, reader),
    )?;
    Ok(inputs)
}
fn push_region(
    inputs: &mut Inputs,
    request: &WrappedConeNormalRequest,
    selection: &WrappedRegionSelection,
    reader: &profile_input::ProfileReader<'_>,
) -> Checked<()> {
    if inputs.directions.contains_key(&selection.region) {
        return Err(duplicate_region(selection.region).public());
    }
    let oriented = super::super::input::selected(reader, selection)?;
    let (source, directions) = oriented.into_developed(
        physical_depths(request),
        request.options.spatial_error,
        request.options.maximum_fit_level,
    );
    inputs.directions.insert(source.region, directions);
    inputs.originals.push(source);
    Ok(())
}
fn duplicate_region(region: SurfaceRegionId) -> ConeFailure {
    ConeFailure::new(
        Some(region),
        None,
        WrappedFailureClass::InvalidInput,
        ConeCause::Binding {
            field: "selected-region",
            actual: vec![region.0, region.0],
            expected: vec![region.0],
        },
    )
}

fn admit(request: &WrappedConeNormalRequest) -> ConeChecked<()> {
    admit_with_ceiling(request, 1_000_000)
}
fn admit_with_ceiling(request: &WrappedConeNormalRequest, ceiling: u32) -> ConeChecked<()> {
    admit_scalars(request)?;
    if ceiling == 1_000_000 {
        policy::admit(request.proof)?;
    } else {
        policy::admit_with_ceiling(request.proof, ceiling)?;
    }
    admit_volume(request.options.volume)?;
    admit_limits(request)?;
    admit_frame(request.support)?;
    Ok(())
}
fn admit_scalars(request: &WrappedConeNormalRequest) -> ConeChecked<()> {
    for (field, actual) in [
        ("semi-angle", request.support.semi_angle),
        ("spatial-error", request.options.spatial_error),
    ] {
        let valid = actual.is_finite()
            && actual > 0.
            && (field != "semi-angle" || actual < std::f64::consts::FRAC_PI_2);
        if !valid {
            return Err(scalar(
                field,
                actual,
                "positive finite value; semi-angle below pi/2",
            ));
        }
    }
    Ok(())
}

fn admit_limits(request: &WrappedConeNormalRequest) -> ConeChecked<()> {
    let [first, last] = request.depths;
    if !first.is_finite() || !last.is_finite() || first >= last {
        return Err(ConeFailure::new(
            None,
            None,
            WrappedFailureClass::InvalidInput,
            ConeCause::Depths(request.depths),
        )
        .at(WrappedStage::Admission));
    }
    if !request.support.chart_origin.is_finite() {
        return Err(scalar(
            "chart-origin",
            request.support.chart_origin,
            "finite angle",
        ));
    }
    admit_fit_level(request.options.maximum_fit_level)?;
    Ok(())
}
fn admit_fit_level(level: u8) -> ConeChecked<()> {
    if level > 8 {
        return Err(ConeFailure::new(
            None,
            None,
            WrappedFailureClass::InvalidInput,
            ConeCause::Limit {
                field: "maximum-fit-level",
                actual: u32::from(level),
                limit: 8,
            },
        ));
    }
    Ok(())
}

fn scalar(field: &'static str, actual: f64, expected: &'static str) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::InvalidInput,
        ConeCause::Scalar {
            field,
            actual,
            expected,
        },
    )
    .at(WrappedStage::Admission)
}
fn physical_depths(request: &WrappedConeNormalRequest) -> [f64; 2] {
    match request.support.normal {
        NormalSense::Outward => request.depths,
        NormalSense::Inward => [-request.depths[1], -request.depths[0]],
    }
}

fn admit_frame(support: ConeSupport) -> ConeChecked<()> {
    if support.apex == [0.; 3] && support.axis == [0., 0., 1.] && support.radial == [1., 0., 0.] {
        return Ok(());
    }
    Err(ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Unsupported,
        ConeCause::Support(Box::new(support)),
    )
    .at(WrappedStage::Admission))
}

fn admit_volume(policy: WrappedVolumePolicy) -> ConeChecked<()> {
    if let WrappedVolumePolicy::Refuse(value) = policy
        && (!value.is_finite() || value < 0.)
    {
        return Err(scalar(
            "volume-refusal",
            value,
            "finite nonnegative allowance",
        ));
    }
    Ok(())
}
