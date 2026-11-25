// SPDX-License-Identifier: GPL-2.0-only
//! Atomic circular Bend for two connected cardinal sketch lines.
mod adopt;
mod failure;

use crate::{
    ArcFitTolerance, ArcId, CurveId, CurveRole, EntityId, LineGeometry, LineId, PointId, PointRole,
    SketchArc, SketchLine, Snapshot,
    line_edit::{EndpointProtection, SupersededLineEndpoint},
};
pub use failure::{BendCheck, BendFailure, BendQuantity};
use martensite_tkmath::predicates::roots::metric_bounds::{
    BINARY64_QUARTER_ANGLE_ERROR, CardinalBendPlacement, CardinalLegCut, cardinal_bend_placement,
};

/// Fresh identities the caller reserves for both replacement legs and their joining arc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BendIds {
    /// New straight leg identities in requested order.
    pub lines: [LineId; 2],
    /// New defining points shared by the legs and the arc.
    pub tangencies: [PointId; 2],
    /// New standalone center marker for the joining arc.
    pub center: PointId,
    /// New circular arc from the first to second tangency.
    pub arc: ArcId,
}

/// Independent modelling limits for stored quarter-circle placement and its trim.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendOptions {
    /// Finite nonnegative allowance on the ideal placement of any new point, in model units.
    pub max_position_error: f64,
    /// Finite nonnegative allowance on stored versus requested radius, in model units.
    pub max_radius_error: f64,
    /// Finite nonnegative allowance on joining tangent angle, in radians.
    pub max_tangent_angle: f64,
    /// Declared arc fit limit, also applied to the certified nominal-terminal discrepancy.
    pub arc_fit: ArcFitTolerance,
}

/// An explicit two-line Bend at their shared point identity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendRequest {
    /// Ordered pair of lines, from the first arc tangent to the second.
    pub lines: [LineId; 2],
    /// The shared original corner.
    pub corner: PointId,
    /// Positive finite requested radius in model units.
    pub radius: f64,
    /// Reserved identities for all created values.
    pub ids: BendIds,
    /// Independently requested quality limits.
    pub options: BendOptions,
}

/// One replaced authored line and the original directed interval kept by its replacement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendLegHistory {
    /// Retired original line.
    pub source: LineId,
    /// Derived line in original traversal orientation.
    pub line: SketchLine,
    /// Original far endpoint, retained unchanged.
    pub retained_endpoint: PointId,
    /// New defining point shared with the joining arc.
    pub tangency: PointId,
    /// Outward bounds for both endpoints of the retained interval in original directed coordinates.
    pub retained_source_parameters: [[f64; 2]; 2],
}

/// Complete retiring and derived identities for one Bend.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendHistory {
    /// The original corner, retired if defining or retained as a standalone marker.
    pub corner: SupersededLineEndpoint,
    /// Both parents of the arc and final straight spans.
    pub legs: [BendLegHistory; 2],
    /// Joining arc derived from the two original lines and their corner.
    pub arc: SketchArc,
}

/// Independent achieved evidence on the stored arc and its two line joins.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendEvidence {
    /// Exact common radius of the two stored tangencies, checked against the stored arc.
    pub stored_radius: f64,
    /// Certified upper deviation from the requested radius, in model units.
    pub radius_error_upper: f64,
    /// Certified upper point displacement relative to ideal placement, in model units.
    pub position_error_upper: f64,
    /// Certified upper angular error at each analytic tangent, in radians.
    pub tangent_angle_upper: [f64; 2],
    /// Certified analytic end-position discrepancy from the stored second tangency.
    pub nominal_terminal_error_upper: f64,
    /// Rounded sample estimate used by the ordinary arc admission for its placement points.
    pub arc_fit_estimate: f64,
}

/// Admitted immutable geometry and the source lineage and quality checks of the edit.
#[derive(Debug, PartialEq)]
pub struct BendResult {
    /// The complete ordered edit adopted into a new snapshot.
    pub sketch: Snapshot,
    /// Actual source identities and normalized retained spans.
    pub history: BendHistory,
    /// Original request captured for replay and comparison against the achieved evidence.
    pub request: BendRequest,
    /// Evidence for this stored arc and its line joins.
    pub evidence: BendEvidence,
}

#[derive(Clone, Copy)]
struct Leg {
    line: LineGeometry,
    corner_index: usize,
}

impl Leg {
    fn far(self) -> [f64; 3] {
        *self.line.points[1 - self.corner_index].coordinates()
    }

    fn history(self, id: LineId, tangent: PointId, cut: CardinalLegCut) -> BendLegHistory {
        let mut endpoints = self.line.line.endpoints;
        let retained_endpoint = endpoints[1 - self.corner_index];
        endpoints[self.corner_index] = tangent;
        let retained_source_parameters = if self.corner_index == 0 {
            [cut.from_corner, [1.; 2]]
        } else {
            [[0.; 2], cut.to_corner]
        };
        BendLegHistory {
            source: self.line.line.id,
            line: SketchLine { id, endpoints },
            retained_endpoint,
            tangency: tangent,
            retained_source_parameters,
        }
    }
}

struct Corner {
    legs: [Leg; 2],
    position: [f64; 3],
    fate: SupersededLineEndpoint,
    role: CurveRole,
}

impl Snapshot {
    /// Place a tangent quarter-circle between two exclusive cardinal line incidences.
    /// Example: `let applied = sketch.bend_lines(request, || false)?;`.
    pub fn bend_lines(
        &self,
        request: BendRequest,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<BendResult, BendFailure> {
        poll(&mut cancelled)?;
        validate_limits(request.options)?;
        let corner = self.bend_corner(request, &mut cancelled)?;
        let placement =
            cardinal_bend_placement(corner.position, corner.legs.map(Leg::far), request.radius)?;
        check_bounds(request.options, placement)?;
        self.adopt_bend(request, corner, placement, &mut cancelled)
    }

    fn bend_corner(
        &self,
        request: BendRequest,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Corner, BendFailure> {
        let [first, second] = request.lines;
        if first == second {
            return Err(BendFailure::RepeatedLine(first));
        }
        let point = self
            .point(request.corner)
            .ok_or(BendFailure::MissingCorner(request.corner))?;
        let legs = [
            self.bend_leg(first, request.corner)?,
            self.bend_leg(second, request.corner)?,
        ];
        let role = self.bend_role(request.lines)?;
        self.check_bend_incidence(request, cancelled)?;
        let fate = match point.role {
            PointRole::Defining => SupersededLineEndpoint::Retired(point.id),
            _ => SupersededLineEndpoint::RetainedMarker(point.id),
        };
        Ok(Corner {
            legs,
            position: *point.position.coordinates(),
            fate,
            role,
        })
    }

    fn bend_leg(&self, id: LineId, corner: PointId) -> Result<Leg, BendFailure> {
        let line = self.line(id).ok_or(BendFailure::MissingLine(id))?;
        let corner_index = line
            .line
            .endpoints
            .iter()
            .position(|id| *id == corner)
            .ok_or(BendFailure::CornerIncidence { corner, line: id })?;
        Ok(Leg { line, corner_index })
    }

    fn bend_role(&self, [first, second]: [LineId; 2]) -> Result<CurveRole, BendFailure> {
        let role = |id| {
            self.curve_role(CurveId::Line(id))
                .ok_or(BendFailure::MissingLine(id))
        };
        let roles = [role(first)?, role(second)?];
        if roles[0] != roles[1] {
            return Err(BendFailure::MixedParticipation(roles));
        }
        Ok(roles[0])
    }

    fn check_bend_incidence(
        &self,
        request: BendRequest,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), BendFailure> {
        let corner = request.corner;
        if let Some(source) = self.frozen_source(corner) {
            return Err(BendFailure::ProtectedCorner {
                corner,
                protection: EndpointProtection::Frozen(source),
            });
        }
        if let Some(source) = self.included_point_source(corner) {
            return Err(BendFailure::ProtectedCorner {
                corner,
                protection: EndpointProtection::Included(source),
            });
        }
        let selected = request.lines.map(EntityId::Line);
        for incident in self.incident_curves(corner, cancelled)? {
            if !selected.contains(&incident) {
                return Err(BendFailure::ProtectedCorner {
                    corner,
                    protection: EndpointProtection::Shared(incident),
                });
            }
        }
        Ok(())
    }
}

fn validate_limits(options: BendOptions) -> Result<(), BendFailure> {
    for (quantity, value) in [
        (BendQuantity::Placement, options.max_position_error),
        (BendQuantity::Radius, options.max_radius_error),
        (BendQuantity::TangentAngle, options.max_tangent_angle),
    ] {
        if !value.is_finite() || value < 0. {
            return Err(BendFailure::InvalidLimit { quantity, value });
        }
    }
    Ok(())
}

fn check_bounds(options: BendOptions, placement: CardinalBendPlacement) -> Result<(), BendFailure> {
    for (quantity, achieved, limit) in [
        (
            BendQuantity::Placement,
            placement.position_error_upper,
            options.max_position_error,
        ),
        (
            BendQuantity::Radius,
            placement.radius_error_upper,
            options.max_radius_error,
        ),
        (
            BendQuantity::TangentAngle,
            BINARY64_QUARTER_ANGLE_ERROR,
            options.max_tangent_angle,
        ),
        (
            BendQuantity::NominalTerminal,
            placement.terminal_error_upper,
            options.arc_fit.value(),
        ),
    ] {
        if achieved > limit {
            return Err(BendFailure::Bound {
                quantity,
                achieved,
                limit,
            });
        }
    }
    Ok(())
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), BendFailure> {
    if cancelled() {
        return Err(BendFailure::Cancelled);
    }
    Ok(())
}
