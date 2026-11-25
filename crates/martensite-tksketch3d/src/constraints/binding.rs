// SPDX-License-Identifier: GPL-2.0-only
//! Bind authored relations to solver columns and exact fixed coordinates.
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, Relation, RelationId, RelationSpec,
    residual::Equation,
};
use crate::{EntityId, LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use std::{collections::BTreeMap, sync::Arc};

pub(super) struct EquationBlock {
    pub(super) id: RelationId,
    pub(super) source: Arc<Equation>,
    pub(super) variables: Vec<usize>,
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
                let [a, b] = [
                    line_of(self.geometry, id, lines[0])?,
                    line_of(self.geometry, id, lines[1])?,
                ];
                let [a0, a1] = a.line.endpoints;
                let [b0, b1] = b.line.endpoints;
                self.equation(
                    id,
                    &[a0, a1, b0, b1],
                    Equation::Parallel {
                        direction,
                        reference: reference_axis(a),
                    },
                )
            }
        }
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
            variables,
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

fn reference_axis(line: LineGeometry) -> usize {
    let direction = line.carrier.direction();
    let [x, y, z] = direction.coordinates().map(f64::abs);
    if x <= y && x <= z {
        0
    } else if y <= z {
        1
    } else {
        2
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
        RelationSpec::Parallel([a, b], direction) => {
            distinct(id, EntityId::Line(a), EntityId::Line(b))?;
            line(a)?;
            line(b)?;
            Ok(Relation::Parallel([a, b], direction))
        }
        RelationSpec::Length(key, target) => {
            line(key)?;
            if target.value() < geometry.minimum.value() {
                return Err(Failure::LengthBelowMinimum(id, target.value()));
            }
            Ok(Relation::Length(key, target))
        }
    }
}

fn line_of(geometry: &Snapshot, id: RelationId, line: LineId) -> Result<LineGeometry, Failure> {
    geometry
        .line(line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(line)))
}

fn distinct(id: RelationId, a: EntityId, b: EntityId) -> Result<(), Failure> {
    if a == b {
        return Err(Failure::RepeatedOperand(id, a));
    }
    Ok(())
}
