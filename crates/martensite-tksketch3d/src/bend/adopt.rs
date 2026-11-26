// SPDX-License-Identifier: GPL-2.0-only
//! Bind the certified placement to actual stored line and arc values before adoption.
use super::{
    BendCheck, BendEvidence, BendFailure, BendHistory, BendRequest, BendResult, Corner, Leg, poll,
};
use crate::{
    ArcDefinition, ArcGeometry, CurveId, Edit, PointRole, SketchArc, SketchPoint, Snapshot,
    line_edit::SupersededLineEndpoint,
};
use martensite_tkmath::{
    gp::{Direction3, Point3},
    predicates::roots::metric_bounds::{BINARY64_QUARTER_ANGLE_ERROR, CardinalBendPlacement},
};
use std::f64::consts::FRAC_PI_2;

impl Snapshot {
    pub(super) fn adopt_bend(
        &self,
        request: BendRequest,
        corner: Corner,
        placement: CardinalBendPlacement,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<BendResult, BendFailure> {
        poll(cancelled)?;
        let history = history(request, &corner, placement)?;
        let edits = edits(&history, &corner, request.ids.center, placement);
        let sketch = self.apply_targeted_edit_batch(&edits, cancelled)?;
        poll(cancelled)?;
        let arc_fit_estimate = verify(&sketch, &corner, &history, placement, cancelled)?;
        poll(cancelled)?;
        let evidence = BendEvidence {
            stored_radius: placement.radius,
            radius_error_upper: placement.radius_error_upper,
            position_error_upper: placement.position_error_upper,
            tangent_angle_upper: [0., BINARY64_QUARTER_ANGLE_ERROR],
            nominal_terminal_error_upper: placement.terminal_error_upper,
            arc_fit_estimate,
        };
        Ok(BendResult {
            sketch,
            history,
            request,
            evidence,
        })
    }
}

fn history(
    request: BendRequest,
    corner: &Corner,
    placement: CardinalBendPlacement,
) -> Result<BendHistory, BendFailure> {
    let [nx, ny, nz] = placement.normal;
    let normal = Direction3::new(nx, ny, nz).map_err(|_| failure(BendCheck::ArcFrame))?;
    let [first, second] = corner.legs;
    let [line0, line1] = request.ids.lines;
    let [tangent0, tangent1] = request.ids.tangencies;
    let [cut0, cut1] = placement.legs;
    Ok(BendHistory {
        corner: corner.fate,
        legs: [
            first.history(line0, tangent0, cut0),
            second.history(line1, tangent1, cut1),
        ],
        arc: SketchArc {
            id: request.ids.arc,
            endpoints: request.ids.tangencies,
            definition: ArcDefinition::Center {
                center: request.ids.center,
                normal,
            },
            fit_tolerance: request.options.arc_fit,
        },
    })
}

fn edits(
    history: &BendHistory,
    corner: &Corner,
    center: crate::PointId,
    placement: CardinalBendPlacement,
) -> Vec<Edit> {
    let mut changes = Vec::with_capacity(12);
    for leg in history.legs {
        changes.push(Edit::RemoveLine(leg.source));
    }
    for (leg, cut) in history.legs.iter().zip(placement.legs) {
        changes.push(Edit::InsertPoint(SketchPoint {
            id: leg.tangency,
            role: PointRole::Defining,
            position: point(cut.tangent),
        }));
        changes.push(Edit::InsertLine(leg.line));
        changes.push(Edit::SetCurveRole {
            curve: CurveId::Line(leg.line.id),
            role: corner.role,
        });
    }
    changes.push(Edit::InsertPoint(SketchPoint {
        id: center,
        role: PointRole::CenterMarker,
        position: point(placement.center),
    }));
    changes.push(Edit::InsertArc(history.arc));
    changes.push(Edit::SetCurveRole {
        curve: CurveId::Arc(history.arc.id),
        role: corner.role,
    });
    if let SupersededLineEndpoint::Retired(point) = history.corner {
        changes.push(Edit::RemovePoint(point));
    }
    changes
}

fn verify(
    sketch: &Snapshot,
    corner: &Corner,
    history: &BendHistory,
    placement: CardinalBendPlacement,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<f64, BendFailure> {
    for i in 0..2 {
        poll(cancelled)?;
        let original = corner.legs[i];
        let trace = history.legs[i];
        let retained = sketch
            .line(trace.line.id)
            .ok_or(failure(BendCheck::LineTrace(trace.source)))?;
        if retained.line != trace.line || !line_trace(original, retained, placement.legs[i].tangent)
        {
            return Err(failure(BendCheck::LineTrace(trace.source)));
        }
    }
    let arc = sketch
        .arc(history.arc.id)
        .ok_or(failure(BendCheck::ArcPlacement))?;
    if arc.recipe() != history.arc {
        return Err(failure(BendCheck::ArcPlacement));
    }
    verify_arc(arc, placement)?;
    Ok(arc.deviation_estimate())
}

fn line_trace(original: Leg, retained: crate::LineGeometry, cut: [f64; 3]) -> bool {
    let mut expected = original.line.points.map(bits);
    expected[original.corner_index] = cut.map(f64::to_bits);
    retained.points.map(bits) == expected
        && retained.carrier.direction() == original.line.carrier.direction()
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn bits(point: Point3) -> [u64; 3] {
    point.coordinates().map(f64::to_bits)
}

fn verify_arc(arc: ArcGeometry, placement: CardinalBendPlacement) -> Result<(), BendFailure> {
    let [first, second] = placement.legs.map(|leg| leg.tangent.map(f64::to_bits));
    let center = placement.center.map(f64::to_bits);
    let frame = arc.frame();
    if arc.points().map(bits) != [first, second, center] || bits(frame.location()) != center {
        return Err(failure(BendCheck::ArcPlacement));
    }
    verify_frame(frame, placement)?;
    if arc.radius() != placement.radius {
        return Err(failure(BendCheck::ArcRadius));
    }
    if arc.range() != [0., FRAC_PI_2] {
        return Err(failure(BendCheck::ArcTrim));
    }
    Ok(())
}

fn verify_frame(
    frame: martensite_tkmath::gp::Axis2,
    placement: CardinalBendPlacement,
) -> Result<(), BendFailure> {
    let radial = [
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    ];
    // With this literal frame, the opening tangent is -u and the ideal closing tangent is +v.
    // The only analytic terminal mismatch comes from storing the quarter angle in binary64.
    if radial != placement.radial_axes || *frame.direction().coordinates() != placement.normal {
        return Err(failure(BendCheck::ArcFrame));
    }
    Ok(())
}

fn failure(check: BendCheck) -> BendFailure {
    BendFailure::StoredGeometry(check)
}
