//! Canonical dyadic intervals are chosen before native construction, never by retries.
use super::*;

use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalFitChart, CylindricalFitQuery, CylindricalProfileError, CylindricalProfileInterval,
    plan_cylindrical_charts,
};

#[derive(Debug)]
pub(super) struct PlanningFailure {
    curve: SurfaceCurveId,
    cause: CylindricalProfileError,
}
impl PlanningFailure {
    pub(super) fn source_cause(&self) -> &CylindricalProfileError {
        &self.cause
    }
}
impl std::fmt::Display for PlanningFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "authored curve {:?}: {}", self.curve, self.cause)
    }
}

#[cfg(test)]
pub(super) fn prepare(request: &Request) -> Result<Request, PlanningFailure> {
    prepare_with_cancel(request, || false)
}
pub(super) fn prepare_with_cancel(
    request: &Request,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<Request, PlanningFailure> {
    prepare_with_fit_limit(request, default_fit_limit(request), cancelled)
}

pub(super) fn prepare_with_fit_limit(
    request: &Request,
    maximum_fit_error: f64,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<Request, PlanningFailure> {
    // Fit both rails together while preserving the independent modelling tolerance.
    let spatial_limit = if maximum_fit_error.is_finite() {
        maximum_fit_error.min(default_fit_limit(request))
    } else {
        maximum_fit_error
    };
    let mut prepared = request.clone();
    prepared.loops = request
        .loops
        .iter()
        .map(|contour| prepare_contour(contour, request, spatial_limit, &mut cancelled))
        .collect::<Result<_, _>>()?;
    Ok(prepared)
}

fn default_fit_limit(request: &Request) -> f64 {
    (request.tolerance / 4.).next_down().max(0.)
}

fn prepare_contour(
    contour: &[AuthoredPiece],
    request: &Request,
    spatial_limit: f64,
    cancelled: &mut CancelPoll<'_>,
) -> Result<Vec<AuthoredPiece>, PlanningFailure> {
    contour
        .iter()
        .map(|piece| prepare_curve(piece, request, spatial_limit, &mut *cancelled))
        .collect::<Result<Vec<_>, _>>()
        .map(|pieces| pieces.into_iter().flatten().collect())
}

fn prepare_curve(
    piece: &AuthoredPiece,
    request: &Request,
    spatial_limit: f64,
    cancelled: &mut CancelPoll<'_>,
) -> Result<Vec<AuthoredPiece>, PlanningFailure> {
    let query = planning_query(piece, request, spatial_limit)?;
    let plan = plan_cylindrical_charts(query, cancelled).map_err(|cause| PlanningFailure {
        curve: piece.span.curve,
        cause,
    })?;
    Ok(plan
        .charts()
        .iter()
        .flat_map(|chart| prepare_chart(piece, chart))
        .collect())
}

fn prepare_chart(piece: &AuthoredPiece, chart: &CylindricalFitChart) -> Vec<AuthoredPiece> {
    let interval = chart.interval();
    let mut source = restrict_piece(piece, interval.index(), interval.level());
    source.chart_center = chart.fit_plan().chart_center();
    let mut pieces = vec![source];
    for _ in 0..chart.fit_plan().subdivision_level() {
        pieces = pieces.iter().flat_map(split_piece).collect();
    }
    pieces
}

pub(super) fn restrict_piece(piece: &AuthoredPiece, index: u64, level: u8) -> AuthoredPiece {
    let mut source = piece.clone();
    for bit in (0..level - piece.span.level).rev() {
        let [left, right] = split_piece(&source);
        source = if index >> bit & 1 == 0 { left } else { right };
    }
    source
}

fn planning_query<'a>(
    piece: &'a AuthoredPiece,
    request: &Request,
    spatial_limit: f64,
) -> Result<CylindricalFitQuery<'a>, PlanningFailure> {
    Ok(CylindricalFitQuery {
        authored: &piece.poles,
        interval: whole_interval(piece.span.curve, spatial_limit)?,
        support_radius: request.radius,
        angular_origin: 0.,
        spatial_limit,
        max_level: request.max_fit_level,
        max_operations: 100_000,
    })
}

fn whole_interval(
    curve: SurfaceCurveId,
    spatial_limit: f64,
) -> Result<CylindricalProfileInterval, PlanningFailure> {
    CylindricalProfileInterval::new(0, 0).map_err(|cause| PlanningFailure {
        curve,
        cause: CylindricalProfileError {
            requested_options: Default::default(),
            requested_spatial_limit: spatial_limit,
            cause,
        },
    })
}

pub(super) fn split_piece(piece: &AuthoredPiece) -> [AuthoredPiece; 2] {
    let level = piece.span.level + 1;
    let first = piece.span.index * 2;
    let joint = CornerSource::Split {
        curve: piece.span.curve,
        numerator: first + 1,
        level,
    };
    let [left, right] = split_poles(&piece.poles);
    [
        split_half(piece, level, first, piece.start, joint, left),
        split_half(piece, level, first + 1, joint, piece.end, right),
    ]
}
fn split_half(
    piece: &AuthoredPiece,
    level: u8,
    index: u32,
    start: CornerSource,
    end: CornerSource,
    poles: Vec<[f64; 2]>,
) -> AuthoredPiece {
    AuthoredPiece {
        span: CurveSpan {
            curve: piece.span.curve,
            level,
            index,
        },
        start,
        end,
        poles,
        chart_center: piece.chart_center,
    }
}

fn split_poles(poles: &[[f64; 2]]) -> [Vec<[f64; 2]>; 2] {
    let mut work = poles.to_vec();
    let mut left = vec![work[0]];
    let mut right = vec![work[work.len() - 1]];
    for count in (1..work.len()).rev() {
        for i in 0..count {
            work[i] = std::array::from_fn(|k| work[i][k] / 2. + work[i + 1][k] / 2.);
        }
        left.push(work[0]);
        right.push(work[count - 1]);
    }
    right.reverse();
    [left, right]
}

#[cfg(test)]
#[path = "subdivision_tests.rs"]
pub(super) mod tests;
