// SPDX-License-Identifier: GPL-2.0-only
//! Classify authored endpoints against the same certified developed cut lattice.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    DevelopedPeriod, DevelopedScale, certify_developed_period,
};

// Keep one bit of headroom when locating adjacent sheet ordinals by binary64 division.
const EXACT_TURN_LIMIT: f64 = 4_503_599_627_370_496.;

pub(in crate::surface_sketch::full_turn) struct SeamCrossings {
    pub roots: [PeriodicCrossing; 2],
    pub angular_spans: [[f64; 2]; 2],
}

pub(super) struct CutPlacement {
    pub crossings: Option<SeamCrossings>,
    angular_clearance: f64,
}

impl CutPlacement {
    pub(super) fn check_linear(&self, radius: f64, floor: f64) -> Checked<()> {
        // For the nearest cut the physical angle is at most pi. Concavity
        // gives 2r sin(angle/2) >= (2/pi) r angle over that interval.
        let chord_factor = (2. / PI.next_up()).next_down();
        let chord = ((self.angular_clearance * chord_factor).next_down() * radius).next_down();
        cylinder_admission::require(
            chord > floor,
            "aperture seam fragment clearance",
            chord,
            "certified physical chord greater than effective spatial separation",
        )
    }
}

pub(super) struct CutLattice {
    developed: DevelopedPeriod,
    phase: f64,
    angular_margin: f64,
}

impl CutLattice {
    pub(super) fn new(support: FullTurnSupport, options: NormalExtrusionOptions) -> Checked<Self> {
        let (scale, phase) = match support {
            FullTurnSupport::Cylinder { support, .. } => (
                DevelopedScale::CylinderRadius(support.radius),
                support.chart_origin,
            ),
            FullTurnSupport::Cone { support, .. } => (
                DevelopedScale::ConeSemiAngle(support.semi_angle),
                support.chart_origin,
            ),
        };
        let developed = certify_developed_period(scale)
            .map_err(|cause| incomplete("certified developed period", format!("{cause:?}")))?;
        cylinder_admission::require(
            developed.scale[0] > 0.,
            "developed angular scale",
            developed.scale[0],
            "positive certified lower bound",
        )?;
        Ok(Self {
            developed,
            phase,
            angular_margin: effective_limits(options).angular_clearance,
        })
    }

    pub(super) fn classify(&self, contour: tool::Contour) -> Checked<CutPlacement> {
        let [first, last] = contour.lift;
        cylinder_admission::finite("aperture angular endpoints", &contour.lift)?;
        let margin = self.margin(contour.lift);
        let width = self.width(first, last, margin)?;
        let (a, clear_a) = self.sheet(first, margin)?;
        let (b, clear_b) = self.sheet(last, margin)?;
        let crossings = match b - a {
            0 => None,
            1 => Some(self.crossing(contour, b, width)?),
            _ => {
                return Err(incomplete(
                    "at most one transverse cut per aperture",
                    format!("sheets {a}..{b}"),
                ));
            }
        };
        let angular_clearance = (clear_a.min(clear_b) / self.developed.scale[1]).next_down();
        Ok(CutPlacement {
            crossings,
            angular_clearance,
        })
    }

    fn width(&self, first: f64, last: f64, margin: f64) -> Checked<[f64; 2]> {
        let width = difference(last, first);
        cylinder_admission::require(
            width[0] <= self.developed.period[1],
            "periodic aperture overlap",
            last - first,
            "width at most one developed period",
        )?;
        cylinder_admission::require(
            width[0] > 0. && (self.developed.period[0] - width[1]).next_down() > margin,
            "periodic aperture width",
            last - first,
            "positive width with certified clearance from a full period",
        )?;
        Ok(width)
    }

    fn crossing(
        &self,
        contour: tool::Contour,
        turn: i64,
        width: [f64; 2],
    ) -> Checked<SeamCrossings> {
        let [first, last] = contour.lift;
        let [lo, hi] = self.cut(turn);
        let left = [(lo - first).next_down(), (hi - first).next_up()];
        let right = [(last - hi).next_down(), (last - lo).next_up()];
        let parameters = [left, right].map(|distance| quotient(distance, width));
        let roots = names::crossings(contour, turn, parameters)?;
        let angular_spans = [left, right].map(|distance| quotient(distance, self.developed.scale));
        Ok(SeamCrossings {
            roots,
            angular_spans,
        })
    }

    fn margin(&self, lift: [f64; 2]) -> f64 {
        let angular_scale = lift[0].abs().max(lift[1].abs()) / self.developed.scale[0];
        let rounding = 64. * f64::EPSILON * (self.phase.abs() + angular_scale + TAU);
        ((self.angular_margin + rounding) * self.developed.scale[1]).next_up()
    }

    fn sheet(&self, coordinate: f64, margin: f64) -> Checked<(i64, f64)> {
        let estimate = (coordinate / self.developed.period[0]).floor();
        cylinder_admission::require(
            estimate.abs() < EXACT_TURN_LIMIT,
            "authored seam turn",
            estimate,
            "finite exactly representable adjacent sheet ordinals",
        )?;
        let estimate = estimate as i64;
        for candidate in [estimate, estimate - 1, estimate + 1] {
            let distance = self.sheet_boundary_distance(coordinate, candidate);
            if distance > margin {
                return Ok((candidate, distance));
            }
        }
        Err(crossing_clearance(coordinate))
    }

    fn sheet_boundary_distance(&self, coordinate: f64, sheet: i64) -> f64 {
        let lo = (coordinate - self.cut(sheet)[1]).next_down();
        let hi = (self.cut(sheet + 1)[0] - coordinate).next_down();
        lo.min(hi)
    }

    fn cut(&self, turn: i64) -> [f64; 2] {
        if turn == 0 {
            return [0.; 2];
        }
        let [a, b] = self.developed.period.map(|bound| bound * turn as f64);
        [a.min(b).next_down(), a.max(b).next_up()]
    }
}

pub(super) fn crossing_clearance(value: f64) -> SurfaceFailureCause {
    SurfaceFailureCause::InvalidInput {
        field: "authored seam crossing clearance",
        value,
        expected: "aperture endpoints strictly separated from the cut in their certified sheets",
    }
}

fn difference(last: f64, first: f64) -> [f64; 2] {
    let rounded = last - first;
    [rounded.next_down(), rounded.next_up()]
}

fn quotient([lo, hi]: [f64; 2], [a, b]: [f64; 2]) -> [f64; 2] {
    [(lo / b).next_down(), (hi / a).next_up()]
}
