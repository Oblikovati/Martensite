//! A convex native band contains the filled footprint when it contains every source pole.
use super::*;
pub(super) type Intervals = [Option<[f64; 2]>; 2];

pub(super) fn check(request: &WrappedNormalRequest, band: &band::Band) -> FootprintChecked<()> {
    let intervals = authored_intervals(request, band)?;
    let mut exclusions = holes::Exclusions::new(&band.holes)?;
    let reader = profile_input::ProfileReader::new(&request.profile)
        .map_err(|cause| source_error(input::profile_failure(cause, None)))?;
    for selection in &request.selections {
        let region = input::selected(&reader, selection).map_err(source_error)?;
        check_region(region.region, &region.loops, intervals)?;
        check_authored_holes(
            region.region,
            &region.loops,
            request.support,
            &mut exclusions,
        )?;
    }
    Ok(())
}

pub(super) fn check_authored_holes(
    region: SurfaceRegionId,
    loops: &[Vec<AuthoredPiece>],
    support: CylinderSupport,
    exclusions: &mut holes::Exclusions,
) -> FootprintChecked<()> {
    if !exclusions.present() {
        return Ok(());
    }
    let mut envelope = holes::TrimBox::empty();
    for &point in loops.iter().flatten().flat_map(|piece| &piece.poles) {
        envelope.include(point);
    }
    let arc = &mut envelope.0[0];
    *arc = [
        (arc[0] / support.radius).next_down(),
        (arc[1] / support.radius).next_up(),
    ];
    check_holes(region, envelope, support.chart_origin, false, exclusions)
}

pub(super) fn check_holes(
    region: SurfaceRegionId,
    mut bounds: holes::TrimBox,
    seam: f64,
    fitted: bool,
    exclusions: &mut holes::Exclusions,
) -> FootprintChecked<()> {
    let [lower, upper] = bounds.0[0];
    bounds.0[0] = [(lower + seam).next_down(), (upper + seam).next_up()];
    if let Some(wire) = exclusions.overlap(bounds)? {
        return Err(CylinderFootprintFailure::InnerTrimOverlap {
            region,
            wire,
            fitted,
        });
    }
    Ok(())
}
pub(super) fn authored_intervals(
    request: &WrappedNormalRequest,
    band: &band::Band,
) -> FootprintChecked<Intervals> {
    if request.selections.is_empty() {
        return Err(CylinderFootprintFailure::EmptySelection);
    }
    let arc = band
        .angular
        .map(|ranges| inward_arc(ranges, request.support))
        .transpose()?;
    Ok([arc, Some(band.axial)])
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

pub(super) fn check_region(
    region: SurfaceRegionId,
    loops: &[Vec<AuthoredPiece>],
    intervals: Intervals,
) -> FootprintChecked<()> {
    for piece in loops.iter().flatten() {
        for &pole in &piece.poles {
            check_pole(region, piece.span.curve, pole, intervals)?;
        }
    }
    Ok(())
}
fn check_pole(
    region: SurfaceRegionId,
    curve: SurfaceCurveId,
    pole: [f64; 2],
    intervals: Intervals,
) -> FootprintChecked<()> {
    let Some((axis, required_interval)) = outside(pole, intervals) else {
        return Ok(());
    };
    Err(CylinderFootprintFailure::ControlOutside {
        region,
        curve,
        axis,
        coordinate: pole[axis],
        required_interval,
    })
}

pub(super) fn outside(pole: [f64; 2], intervals: Intervals) -> Option<(usize, [f64; 2])> {
    intervals.into_iter().enumerate().find_map(|(axis, bound)| {
        let [lower, upper] = bound?;
        (!(lower < pole[axis] && pole[axis] < upper)).then_some((axis, [lower, upper]))
    })
}
