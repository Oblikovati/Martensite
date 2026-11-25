// SPDX-License-Identifier: GPL-2.0-only
//! Bind authored relations to solver columns and exact fixed coordinates.
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, LineAngle, Relation, RelationId, RelationSpec,
    incidence::{SharedEnds, shared_ends},
    metric,
    residual::{Equation, FrameReference, axis_branch},
};
use crate::{EntityId, LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
pub(super) struct EquationBlock {
    pub(super) id: RelationId,
    pub(super) source: Arc<Equation>,
    pub(super) variables: Arc<[usize]>,
}

pub(super) struct Binding {
    pub(super) fixed: Vec<(usize, f64)>,
    pub(super) blocks: Vec<EquationBlock>,
    pub(super) authored_rows: usize,
    pub(super) frozen_coordinates: usize,
}

pub(super) fn bind(
    geometry: &Snapshot,
    relations: &BTreeMap<RelationId, Relation>,
    point_columns: &BTreeMap<PointId, usize>,
    policy: ConstraintPolicy,
) -> Result<Binding, Failure> {
    let mut builder = Layout {
        geometry,
        point_columns,
        fixed: BTreeMap::new(),
        blocks: Vec::new(),
        authored_rows: 0,
        scale: 1. / policy.length_scale.value(),
        angular_sine: policy.parallel_sine_tolerance,
    };
    for (&id, &start) in point_columns {
        if geometry.frozen_source(id).is_some() {
            let point = geometry.point(id).ok_or(Failure::MissingPoint(id))?;
            builder
                .fixed
                .extend((start..start + 3).zip(*point.position.coordinates()));
        }
    }
    let frozen_coordinates = builder.fixed.len();
    for (&id, &relation) in relations {
        builder.bind(id, relation)?;
    }
    let (blocks, authored_rows) = (builder.blocks, builder.authored_rows);
    let fixed = builder.fixed.into_iter().collect::<Vec<_>>();
    Ok(Binding {
        fixed,
        blocks,
        authored_rows,
        frozen_coordinates,
    })
}

struct Layout<'a> {
    geometry: &'a Snapshot,
    point_columns: &'a BTreeMap<PointId, usize>,
    fixed: BTreeMap<usize, f64>,
    blocks: Vec<EquationBlock>,
    authored_rows: usize,
    scale: f64,
    angular_sine: f64,
}

impl Layout<'_> {
    fn bind(&mut self, id: RelationId, relation: Relation) -> Result<(), Failure> {
        match relation {
            Relation::FixPoint(point, anchor) => self.fix(id, point, anchor),
            Relation::FixLine(line, anchors) => {
                let endpoints = line_of(self.geometry, id, line)?.line.endpoints;
                self.fix(id, endpoints[0], anchors[0])?;
                self.fix(id, endpoints[1], anchors[1])
            }
            Relation::Coincident(points) => {
                self.equation(id, &points, Equation::Coincident { scale: self.scale })
            }
            Relation::PointOnLine(point, line) => {
                let line = line_of(self.geometry, id, line)?;
                let [start, end] = line.line.endpoints;
                self.equation(
                    id,
                    &[start, end, point],
                    Equation::PointOnLine {
                        scale: self.scale,
                        reference: FrameReference::axis(line),
                    },
                )
            }
            Relation::Colinear(lines, direction) => {
                let pair = pair_of(self.geometry, id, lines)?;
                let independent_offset = match shared_ends(pair) {
                    SharedEnds::None => true,
                    SharedEnds::One => false,
                    SharedEnds::Both(_) => return Ok(()),
                };
                self.pair_equation(
                    id,
                    pair,
                    Equation::Colinear {
                        direction,
                        reference: FrameReference::pair(pair),
                        scale: self.scale,
                        independent_offset,
                    },
                )
            }
            Relation::Length(line, target) => {
                let points = line_of(self.geometry, id, line)?.line.endpoints;
                self.equation(
                    id,
                    &points,
                    Equation::Length {
                        scale: self.scale,
                        target: target.value(),
                    },
                )
            }
            Relation::Parallel(lines, direction) => {
                let [a, b] = pair_of(self.geometry, id, lines)?;
                self.pair_equation(
                    id,
                    [a, b],
                    Equation::Parallel {
                        direction,
                        reference: FrameReference::axis(a),
                    },
                )
            }
            Relation::ParallelToAxis(line, axis, direction) => {
                let endpoints = line_of(self.geometry, id, line)?.line.endpoints;
                self.equation(id, &endpoints, Equation::Axis(axis, direction))
            }
            Relation::ParallelToPlane(line, normal) => {
                let endpoints = line_of(self.geometry, id, line)?.line.endpoints;
                self.equation(id, &endpoints, Equation::Plane(*normal.coordinates()))
            }
            Relation::Perpendicular(lines) => {
                let lines = pair_of(self.geometry, id, lines)?;
                self.pair_equation(id, lines, Equation::Perpendicular)
            }
            Relation::EqualLength(lines) => {
                let lines = pair_of(self.geometry, id, lines)?;
                self.pair_equation(id, lines, Equation::EqualLength { scale: self.scale })
            }
            Relation::Angle(lines, angle) => self.angle(id, lines, angle),
        }
    }

    fn angle(
        &mut self,
        id: RelationId,
        lines: [LineId; 2],
        target: LineAngle,
    ) -> Result<(), Failure> {
        let tolerance = self.angular_sine;
        let sine = target.sin_cos().0;
        if sine.next_down() <= tolerance || !sine.recip().is_finite() {
            return Err(Failure::AngleTargetSingular(id, target, tolerance));
        }
        let geometry = pair_of(self.geometry, id, lines)?;
        if !metric::angle::nonsingular(geometry, tolerance) {
            return Err(Failure::AngleSeedSingular(id, lines, tolerance));
        }
        self.pair_equation(
            id,
            geometry,
            Equation::Angle {
                target,
                minimum_sine: tolerance,
            },
        )
    }

    fn pair_equation(
        &mut self,
        id: RelationId,
        [a, b]: [LineGeometry; 2],
        source: Equation,
    ) -> Result<(), Failure> {
        let [a0, a1] = a.line.endpoints;
        let [b0, b1] = b.line.endpoints;
        self.equation(id, &[a0, a1, b0, b1], source)
    }

    fn fix(&mut self, id: RelationId, point: PointId, position: Point3) -> Result<(), Failure> {
        let start = self.start(id, point)?;
        for (axis, &value) in position.coordinates().iter().enumerate() {
            if let Some(previous) = self.fixed.insert(start + axis, value)
                && previous.to_bits() != value.to_bits()
            {
                return Err(Failure::ConflictingFix(id, point));
            }
        }
        self.authored_rows += 3;
        Ok(())
    }

    fn equation(
        &mut self,
        id: RelationId,
        points: &[PointId],
        source: Equation,
    ) -> Result<(), Failure> {
        let mut variables = Vec::with_capacity(3 * points.len());
        for &point in points {
            let start = self.start(id, point)?;
            variables.extend(start..start + 3);
        }
        self.authored_rows += source.rows();
        self.blocks.push(EquationBlock {
            id,
            source: Arc::new(source),
            variables: variables.into(),
        });
        Ok(())
    }

    fn start(&self, id: RelationId, point: PointId) -> Result<usize, Failure> {
        self.point_columns
            .get(&point)
            .copied()
            .ok_or(Failure::MissingOperand(id, EntityId::Point(point)))
    }
}

pub(super) fn capture(
    geometry: &Snapshot,
    id: RelationId,
    spec: RelationSpec,
) -> Result<Relation, Failure> {
    let point = |key| {
        geometry
            .point(key)
            .map(|point| point.position)
            .ok_or(Failure::MissingOperand(id, EntityId::Point(key)))
    };
    let line = |key| line_of(geometry, id, key);
    match spec {
        RelationSpec::FixPoint(key) => Ok(Relation::FixPoint(key, point(key)?)),
        RelationSpec::FixLine(key) => Ok(Relation::FixLine(key, line(key)?.points)),
        RelationSpec::Coincident([a, b]) => {
            distinct(id, EntityId::Point(a), EntityId::Point(b))?;
            point(a)?;
            point(b)?;
            Ok(Relation::Coincident([a, b]))
        }
        RelationSpec::PointOnLine(key, support) => {
            point(key)?;
            if line(support)?.line.endpoints.contains(&key) {
                return Err(Failure::PointIsLineEndpoint(id, key, support));
            }
            Ok(Relation::PointOnLine(key, support))
        }
        RelationSpec::Colinear(lines, direction) => {
            let pair = pair_of(geometry, id, lines)?;
            if !metric::line_branch(pair, direction) {
                return Err(Failure::ColinearSeedBranch(id, lines, direction));
            }
            Ok(Relation::Colinear(lines, direction))
        }
        RelationSpec::Parallel(lines, direction) => {
            pair_of(geometry, id, lines)?;
            Ok(Relation::Parallel(lines, direction))
        }
        RelationSpec::Length(key, target) => {
            line(key)?;
            if target.value() < geometry.minimum.value() {
                return Err(Failure::LengthBelowMinimum(id, target.value()));
            }
            Ok(Relation::Length(key, target))
        }
        RelationSpec::ParallelToAxis(key, axis, direction) => {
            let [start, end] = line(key)?.points;
            let component = end.coordinates()[axis.index()] - start.coordinates()[axis.index()];
            if !axis_branch(direction, component) {
                return Err(Failure::AxisSeedBranch(id, key, axis, direction));
            }
            Ok(Relation::ParallelToAxis(key, axis, direction))
        }
        RelationSpec::ParallelToPlane(key, normal) => {
            line(key)?;
            let coordinates = *normal.coordinates();
            if coordinates.iter().any(|value| !value.is_finite()) || coordinates == [0.; 3] {
                return Err(Failure::InvalidPlaneNormal(id, coordinates));
            }
            Ok(Relation::ParallelToPlane(key, normal))
        }
        RelationSpec::Perpendicular(lines) => {
            pair_of(geometry, id, lines)?;
            Ok(Relation::Perpendicular(lines))
        }
        RelationSpec::EqualLength(lines) => {
            pair_of(geometry, id, lines)?;
            Ok(Relation::EqualLength(lines))
        }
        RelationSpec::Angle(lines, target) => {
            pair_of(geometry, id, lines)?;
            Ok(Relation::Angle(lines, target))
        }
    }
}

fn line_of(geometry: &Snapshot, id: RelationId, line: LineId) -> Result<LineGeometry, Failure> {
    geometry
        .line(line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(line)))
}

fn pair_of(
    geometry: &Snapshot,
    id: RelationId,
    [first, second]: [LineId; 2],
) -> Result<[LineGeometry; 2], Failure> {
    distinct(id, EntityId::Line(first), EntityId::Line(second))?;
    Ok([
        line_of(geometry, id, first)?,
        line_of(geometry, id, second)?,
    ])
}

fn distinct(id: RelationId, a: EntityId, b: EntityId) -> Result<(), Failure> {
    if a == b {
        return Err(Failure::RepeatedOperand(id, a));
    }
    Ok(())
}
