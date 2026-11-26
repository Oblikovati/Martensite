// SPDX-License-Identifier: GPL-2.0-only
//! Bind authored relations to solver columns and exact fixed coordinates.
mod arc;
use super::{
    ConstraintFailure as Failure, ConstraintPolicy, ContinuityJoin, LineAngle, Relation,
    RelationId, RelationSpec,
    arc_layout::ArcLayout,
    continuity,
    incidence::{SharedEnds, shared_ends},
    metric, poll,
    residual::{Equation, FrameReference, axis_branch},
};
use crate::{ArcId, EntityId, LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Copy)]
pub(super) enum BlockOwner {
    Relation(RelationId),
    IntrinsicArc(ArcId),
}

#[derive(Clone)]
pub(super) struct EquationBlock {
    pub(super) owner: BlockOwner,
    pub(super) source: Arc<Equation>,
    pub(super) variables: Arc<[usize]>,
}

impl EquationBlock {
    pub(super) fn relation_id(&self) -> Option<RelationId> {
        match self.owner {
            BlockOwner::Relation(id) => Some(id),
            BlockOwner::IntrinsicArc(_) => None,
        }
    }
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
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Binding, Failure> {
    let arcs = geometry
        .arcs
        .iter()
        .map(|(&id, &arc)| {
            poll(cancelled)?;
            Ok((id, ArcLayout::capture(arc, policy)?))
        })
        .collect::<Result<BTreeMap<_, _>, Failure>>()?;
    let mut builder = Layout {
        arcs,
        geometry,
        point_columns,
        fixed: BTreeMap::new(),
        blocks: Vec::new(),
        authored_rows: 0,
        scale: 1. / policy.length_scale.value(),
        length_scale: policy.length_scale.value(),
        angular_sine: policy.parallel_sine_tolerance,
    };
    for (&id, &start) in point_columns {
        if geometry.point_is_read_only(id) {
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
    builder.arc_intrinsics()?;
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
    arcs: BTreeMap<ArcId, ArcLayout>,
    geometry: &'a Snapshot,
    point_columns: &'a BTreeMap<PointId, usize>,
    fixed: BTreeMap<usize, f64>,
    blocks: Vec<EquationBlock>,
    authored_rows: usize,
    scale: f64,
    length_scale: f64,
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
            Relation::FixArc(arc, anchors, branch) => self.fix_arc(id, arc, anchors, branch),
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
            Relation::Radius(arc, target, branch) => self.radius(id, arc, target, branch),
            Relation::EqualRadius(arcs, branches) => self.equal_radius(id, arcs, branches),
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
            Relation::Tangent(join) => self.continuity(id, join, false),
            Relation::G2(join, _) => self.continuity(id, join, true),
        }
    }

    fn continuity(
        &mut self,
        id: RelationId,
        join: ContinuityJoin,
        g2: bool,
    ) -> Result<(), Failure> {
        let stencil = continuity::bind(self.geometry, id, join, g2)?;
        let reference = join.reference().chart_direction(self.geometry, id)?;
        self.equation(
            id,
            stencil.points(),
            Equation::Continuity {
                reference: FrameReference::from_direction(reference),
                cubic_reference: matches!(join, ContinuityJoin::CubicCubic(_)),
                curvature_scale: g2.then_some(self.length_scale),
            },
        )
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
        self.pin(id, point, position)?;
        self.authored_rows += 3;
        Ok(())
    }

    fn pin(&mut self, id: RelationId, point: PointId, position: Point3) -> Result<(), Failure> {
        let start = self.start(id, point)?;
        for (axis, &value) in position.coordinates().iter().enumerate() {
            if let Some(previous) = self.fixed.insert(start + axis, value)
                && previous.to_bits() != value.to_bits()
            {
                return Err(Failure::ConflictingFix(id, point));
            }
        }
        Ok(())
    }

    fn equation(
        &mut self,
        id: RelationId,
        points: &[PointId],
        source: Equation,
    ) -> Result<(), Failure> {
        let variables = slots(points, |point| self.start(id, point))?;
        self.authored_rows += source.rows();
        self.blocks.push(EquationBlock {
            owner: BlockOwner::Relation(id),
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

fn slots(
    points: &[PointId],
    mut start: impl FnMut(PointId) -> Result<usize, Failure>,
) -> Result<Vec<usize>, Failure> {
    let mut variables = Vec::with_capacity(3 * points.len());
    for &point in points {
        let base = start(point)?;
        variables.extend(base..base + 3);
    }
    Ok(variables)
}

pub(super) fn capture(
    geometry: &Snapshot,
    id: RelationId,
    spec: RelationSpec,
    policy: ConstraintPolicy,
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
        RelationSpec::FixArc(key) => arc::capture_fix(geometry, id, key, policy),
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
        RelationSpec::Radius(key, target) => arc::capture_radius(geometry, id, key, target, policy),
        RelationSpec::EqualRadius(arcs) => arc::capture_equal(geometry, id, arcs, policy),
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
        RelationSpec::Tangent(join) => {
            continuity::check_seed(geometry, id, join, false)?;
            Ok(Relation::Tangent(join))
        }
        RelationSpec::G2(join, cap) => {
            continuity::check_seed(geometry, id, join, true)?;
            Ok(Relation::G2(join, cap))
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
