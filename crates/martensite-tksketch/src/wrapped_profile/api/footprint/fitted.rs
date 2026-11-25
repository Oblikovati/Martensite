//! Check controls that the extrusion has matched to both decoded cap pcurves.
use super::*;
use bounds::Intervals;

pub(super) fn check(
    tool: &WrappedChartNormalResult,
    band: &mut band::Band,
    exclusions: &mut holes::Exclusions,
    cancelled: &mut CancelPoll<'_>,
) -> FootprintChecked<usize> {
    let intervals = intervals(band, tool.support().chart_origin)?;
    let mut spans = 0;
    for region in tool.fitted_sources() {
        extrusion::poll(cancelled, WrappedStage::Publication)?;
        spans += check_region(region, intervals)?;
        check_full_trim(
            region,
            tool.support().chart_origin,
            band.reflect_axial,
            exclusions,
            &mut band.outer,
        )?;
    }
    Ok(spans)
}

fn check_full_trim(
    region: &Request,
    seam: f64,
    reflect_axial: bool,
    exclusions: &mut holes::Exclusions,
    outer: &mut Option<outer::OuterTrim>,
) -> FootprintChecked<()> {
    if !exclusions.present() && outer.is_none() {
        return Ok(());
    }
    let mut envelope = holes::TrimBox::empty();
    for &pole in region.loops.iter().flatten().flat_map(|piece| &piece.poles) {
        envelope.include(cap_coordinates::angular_pole(pole, region.radius));
    }
    bounds::check_trim(
        region.region,
        envelope,
        seam,
        reflect_axial,
        true,
        exclusions,
        outer,
    )
}

fn intervals(band: &band::Band, seam: f64) -> FootprintChecked<Intervals> {
    let angular = band
        .angular
        .map(|[first, last]| inward_angles(first, last, seam))
        .transpose()?;
    Ok([
        angular,
        Some(band::axial_interval(band.axial, band.reflect_axial)),
    ])
}

fn inward_angles(first: f64, last: f64, seam: f64) -> FootprintChecked<[f64; 2]> {
    let lower = (first - seam).next_up();
    let upper = (last - seam).next_down();
    if !lower.is_finite() || !upper.is_finite() || lower >= upper {
        return Err(unsupported("finite ordered cap angular trim bounds"));
    }
    Ok([lower, upper])
}

fn check_region(region: &Request, intervals: Intervals) -> FootprintChecked<usize> {
    let mut spans = 0;
    for piece in region.loops.iter().flatten() {
        for &pole in &piece.poles {
            let uv = cap_coordinates::angular_pole(pole, region.radius);
            check_pole(region.region, piece.span, uv, intervals)?;
        }
        spans += 1;
    }
    Ok(spans)
}

fn check_pole(
    region: SurfaceRegionId,
    span: CurveSpan,
    pole: [f64; 2],
    intervals: Intervals,
) -> FootprintChecked<()> {
    let Some((axis, required_interval)) = bounds::outside(pole, intervals) else {
        return Ok(());
    };
    Err(CylinderFootprintFailure::FittedControlOutside {
        region,
        curve: span.curve,
        fit_level: span.level,
        fit_index: span.index,
        axis,
        coordinate: pole[axis],
        required_interval,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitted_boundary_contact_reports_the_fitted_span_and_physical_axis() {
        let span = CurveSpan {
            curve: SurfaceCurveId(7),
            level: 2,
            index: 3,
        };
        let clip = [None, Some([0., 4.])];
        let error = check_pole(SurfaceRegionId(11), span, [-0.01, 4.], clip).unwrap_err();
        assert!(matches!(
            error,
            CylinderFootprintFailure::FittedControlOutside {
                region: SurfaceRegionId(11),
                curve: SurfaceCurveId(7),
                fit_level: 2,
                fit_index: 3,
                axis: 1,
                coordinate: 4.,
                required_interval: [0., 4.],
            }
        ));
        assert!(check_pole(SurfaceRegionId(11), span, [-0.01, 3.99], clip).is_ok());
    }
}
