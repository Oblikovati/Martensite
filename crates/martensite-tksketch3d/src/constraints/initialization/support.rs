// SPDX-License-Identifier: GPL-2.0-only
//! Qualify a private authoring seed against its actual incident supports and equation domains.
use super::{ConstraintFailure, Coordinate, EquationBlock, point, poll, read};
use crate::{MinimumLength, PointId, SketchLine, Snapshot, admission};
use martensite_tksolve::{ad::DualNumber, residual::ResidualSource};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct LineSlots {
    source: SketchLine,
    slots: [usize; 2],
}

pub(super) struct SeedSupport<'a> {
    minimum: MinimumLength,
    lines: BTreeMap<usize, Vec<LineSlots>>,
    equations: BTreeMap<usize, Vec<usize>>,
    blocks: &'a [EquationBlock],
}

impl<'a> SeedSupport<'a> {
    pub(super) fn new(
        geometry: &Snapshot,
        point_columns: &BTreeMap<PointId, usize>,
        blocks: &'a [EquationBlock],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, ConstraintFailure> {
        let mut lines: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for line in geometry.geometry.values() {
            poll(cancelled)?;
            let [a, b] = line.line.endpoints;
            let slots = [column(point_columns, a)?, column(point_columns, b)?];
            let entry = LineSlots {
                source: line.line,
                slots,
            };
            for slot in slots {
                lines.entry(slot).or_default().push(entry);
            }
        }
        let equations = equation_index(blocks, cancelled)?;
        Ok(Self {
            minimum: geometry.minimum,
            lines,
            equations,
            blocks,
        })
    }

    pub(super) fn degree(&self, slot: usize) -> usize {
        self.lines.get(&slot).map_or(0, Vec::len)
    }

    pub(super) fn admits(
        &self,
        values: &[f64],
        slot: usize,
        candidate: Coordinate,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, ConstraintFailure> {
        for line in self.lines.get(&slot).into_iter().flatten() {
            poll(cancelled)?;
            let points = line.slots.map(|at| {
                point(if at == slot {
                    candidate
                } else {
                    read(values, at)
                })
            });
            if admission::line_from_points(line.source, points, self.minimum).is_err() {
                return Ok(false);
            }
        }
        for &index in self.equations.get(&slot).into_iter().flatten() {
            poll(cancelled)?;
            if !evaluates(&self.blocks[index], values, slot, candidate) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn column(columns: &BTreeMap<PointId, usize>, id: PointId) -> Result<usize, ConstraintFailure> {
    columns
        .get(&id)
        .copied()
        .ok_or(ConstraintFailure::MissingPoint(id))
}

fn equation_index(
    blocks: &[EquationBlock],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BTreeMap<usize, Vec<usize>>, ConstraintFailure> {
    let mut equations: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, block) in blocks.iter().enumerate() {
        poll(cancelled)?;
        for point in block.variables.as_chunks::<3>().0 {
            let incident = equations.entry(point[0]).or_default();
            if incident.last() != Some(&index) {
                incident.push(index);
            }
        }
    }
    Ok(equations)
}

fn evaluates(block: &EquationBlock, values: &[f64], slot: usize, candidate: Coordinate) -> bool {
    // The residual source's scalar evaluation includes its angle, frame and sign domain
    // checks. The solver remains responsible for derivatives and equation feasibility.
    let scalars = block
        .variables
        .iter()
        .map(|&column| {
            let value = if (slot..slot + 3).contains(&column) {
                candidate[column - slot]
            } else {
                values[column]
            };
            DualNumber::constant(value)
        })
        .collect::<Result<Vec<_>, _>>();
    scalars.is_ok_and(|scalars| block.source.evaluate(&scalars).is_ok())
}
