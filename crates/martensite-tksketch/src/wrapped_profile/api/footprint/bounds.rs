//! A convex native band contains the filled footprint when it contains every source pole.
use super::*;

pub(super) fn check(request: &WrappedNormalRequest, band: &band::Band) -> FootprintChecked<()> {
    if request.selections.is_empty() {
        return Err(CylinderFootprintFailure::EmptySelection);
    }
    let arc = band
        .angular
        .map(|ranges| inward_arc(ranges, request.support))
        .transpose()?;
    let reader = profile_input::ProfileReader::new(&request.profile)
        .map_err(|cause| source_error(input::profile_failure(cause, None)))?;
    for selection in &request.selections {
        let region = input::selected(&reader, selection).map_err(source_error)?;
        check_region(region, [arc, Some(band.axial)])?;
    }
    Ok(())
}
fn source_error(cause: WrappedNormalFailure) -> CylinderFootprintFailure {
    CylinderFootprintFailure::Source(Box::new(cause))
}
fn inward_arc([first, last]: [f64; 2], support: CylinderSupport) -> FootprintChecked<[f64; 2]> {
    let lower = (support.radius * (first - support.chart_origin).next_up()).next_up();
    let upper = (support.radius * (last - support.chart_origin).next_down()).next_down();
    if support.radius <= 0. || !lower.is_finite() || !upper.is_finite() || lower >= upper {
        return Err(unsupported(
            "finite ordered developed bounds for the partial native band",
        ));
    }
    Ok([lower, upper])
}

fn check_region(
    region: profile_input::OrientedRegion,
    intervals: [Option<[f64; 2]>; 2],
) -> FootprintChecked<()> {
    for piece in region.loops.iter().flatten() {
        for &pole in &piece.poles {
            check_pole(region.region, piece.span.curve, pole, intervals)?;
        }
    }
    Ok(())
}
fn check_pole(
    region: SurfaceRegionId,
    curve: SurfaceCurveId,
    pole: [f64; 2],
    intervals: [Option<[f64; 2]>; 2],
) -> FootprintChecked<()> {
    for (axis, bound) in intervals.into_iter().enumerate() {
        let Some([lower, upper]) = bound else {
            continue;
        };
        if !(lower < pole[axis] && pole[axis] < upper) {
            return Err(CylinderFootprintFailure::ControlOutside {
                region,
                curve,
                axis,
                coordinate: pole[axis],
                required_interval: [lower, upper],
            });
        }
    }
    Ok(())
}
