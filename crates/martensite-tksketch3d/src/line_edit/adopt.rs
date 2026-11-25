// SPDX-License-Identifier: GPL-2.0-only
//! Replace only the selected line and retire its unneeded original endpoint.
use super::{
    EndpointProtection, LineEdit, LineEditFailure, LineEditOutcome, LineEndpoint, LineReplacement,
    LineReplacementIds, Portion, SupersededLineEndpoint, poll, select::Selection,
};
use crate::{
    CurveId, Edit, EditFailure, EntityId, LineGeometry, PointRole, SketchLine, SketchPoint,
    Snapshot,
};
use martensite_tkmath::gp::Point3;

impl Snapshot {
    pub(super) fn adopt_line_edit(
        &self,
        source: LineGeometry,
        selection: Selection,
        ids: LineReplacementIds,
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<LineEditOutcome, LineEditFailure> {
        poll(cancelled)?;
        let (boundary, hit, portion) = match selection {
            Selection::Unchanged(reason) => return Ok(LineEditOutcome::Unchanged(reason)),
            Selection::Hit {
                boundary,
                hit,
                portion,
            } => (boundary, hit, portion),
        };
        let superseded = self.superseded_endpoint(source.line, portion.endpoint(), cancelled)?;
        let contact = hit.placement()?;
        if contact.error_upper > limit {
            return Err(LineEditFailure::PositionError {
                upper_bound: contact.error_upper,
                limit,
            });
        }
        let endpoint = portion.endpoint().index();
        if contact.position == *source.points[endpoint].coordinates() {
            return Err(LineEditFailure::UnrepresentableExtent {
                source: source.line.id,
                endpoint: source.line.endpoints[endpoint],
                parameter: contact.parameters[0],
            });
        }
        let replacement = lineage(source.line, portion, ids, superseded, contact.parameters[0]);
        let sketch = self.apply_replacement(replacement, contact.position, cancelled)?;
        poll(cancelled)?;
        Ok(LineEditOutcome::Edited(Box::new(LineEdit {
            sketch,
            boundary,
            boundary_location: hit.parameters()[1].location(),
            contact,
            requested_error: limit,
            replacement,
        })))
    }

    fn superseded_endpoint(
        &self,
        source: SketchLine,
        endpoint: LineEndpoint,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<SupersededLineEndpoint, LineEditFailure> {
        let point = source.endpoints[endpoint.index()];
        if let Some(stamp) = self.frozen_source(point) {
            return Err(LineEditFailure::ProtectedEndpoint {
                point,
                protection: EndpointProtection::Frozen(stamp),
            });
        }
        let incident = self.incident_curves(point, cancelled)?;
        if let Some(other) = incident
            .into_iter()
            .find(|entity| *entity != EntityId::Line(source.id))
        {
            return Err(LineEditFailure::ProtectedEndpoint {
                point,
                protection: EndpointProtection::Shared(other),
            });
        }
        let value = self
            .point(point)
            .ok_or(EditFailure::MissingEditTarget(EntityId::Point(point)))?;
        Ok(match value.role {
            PointRole::Defining => SupersededLineEndpoint::Retired(point),
            _ => SupersededLineEndpoint::RetainedMarker(point),
        })
    }

    fn apply_replacement(
        &self,
        replacement: LineReplacement,
        [x, y, z]: [f64; 3],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, LineEditFailure> {
        let role = self
            .curve_role(CurveId::Line(replacement.source))
            .ok_or(LineEditFailure::MissingLine(replacement.source))?;
        let mut batch = vec![
            Edit::RemoveLine(replacement.source),
            Edit::InsertPoint(SketchPoint {
                id: replacement.added_endpoint,
                role: PointRole::Defining,
                position: Point3::new(x, y, z),
            }),
            Edit::InsertLine(replacement.line),
            Edit::SetCurveRole {
                curve: CurveId::Line(replacement.line.id),
                role,
            },
        ];
        if let SupersededLineEndpoint::Retired(point) = replacement.superseded_endpoint {
            batch.push(Edit::RemovePoint(point));
        }
        // This edit declares its specific retirement instead of applying global orphan cleanup.
        Ok(self.apply_targeted_edit_batch(&batch, cancelled)?)
    }
}

fn lineage(
    source: SketchLine,
    portion: Portion,
    ids: LineReplacementIds,
    superseded_endpoint: SupersededLineEndpoint,
    parameter: [f64; 2],
) -> LineReplacement {
    let endpoint = portion.endpoint().index();
    let mut endpoints = source.endpoints;
    endpoints[endpoint] = ids.endpoint;
    let original = [[0.; 2], [1.; 2]];
    let (retained_source_parameters, continuation_parameters) = match portion {
        Portion::Trim(LineEndpoint::Start) => ([parameter, original[1]], None),
        Portion::Trim(LineEndpoint::End) => ([original[0], parameter], None),
        Portion::Extend(LineEndpoint::Start) => (original, Some([parameter, original[0]])),
        Portion::Extend(LineEndpoint::End) => (original, Some([original[1], parameter])),
    };
    LineReplacement {
        source: source.id,
        line: SketchLine {
            id: ids.line,
            endpoints,
        },
        retained_endpoint: source.endpoints[1 - endpoint],
        superseded_endpoint,
        added_endpoint: ids.endpoint,
        retained_source_parameters,
        continuation_parameters,
    }
}
