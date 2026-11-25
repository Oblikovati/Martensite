// SPDX-License-Identifier: GPL-2.0-only
//! Stage values first so complete geometry, including shared endpoints, admits together.
use crate::{
    BatchPhase, Edit, EditFailure, Entity, EntityId, LineId, PointId, PointRole, SketchLine,
    SketchPoint, Snapshot, admission,
};
use martensite_tkmath::gp::Point3;
use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Entry},
    num::NonZeroU64,
};

type RetiredIds = BTreeMap<NonZeroU64, EntityId>;

impl Snapshot {
    /// Adopt one atomic batch on owned values. Example: `snapshot.apply(&edits, || false)?`.
    ///
    /// This operation builds and validates a whole snapshot for adoption. It may traverse
    /// all entities, and the caller can cancel during staging and line validation.
    /// IDs can be checked against the current snapshot and the current batch. Callers
    /// reserve fresh IDs across earlier removals, snapshots, and undo.
    pub fn apply(
        &self,
        edits: &[Edit],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, EditFailure> {
        poll(&mut cancelled, BatchPhase::BeforeStaging)?;
        let mut draft = Self {
            minimum: self.minimum,
            entities: self.entities.clone(),
            geometry: Default::default(),
            frozen: self.frozen.clone(),
        };
        let mut retired = RetiredIds::new();
        for &edit in edits {
            poll(&mut cancelled, BatchPhase::Staging)?;
            draft.stage(edit, &mut retired, &mut cancelled)?;
        }
        if !retired.is_empty() {
            draft.prune_defining_points(&mut retired, &mut cancelled)?;
        }
        // This draft has the same minimum as its source, so matching inputs retain their proof.
        draft.geometry = draft.check_lines(&self.geometry, &mut cancelled)?;
        poll(&mut cancelled, BatchPhase::Completion)?;
        Ok(draft)
    }

    fn stage(
        &mut self,
        edit: Edit,
        retired: &mut RetiredIds,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), EditFailure> {
        match edit {
            Edit::InsertPoint(point) => self.insert_point(point, retired),
            Edit::InsertFrozenPoint { point, source } => {
                self.insert_point(point, retired)?;
                self.frozen.insert(point.id, source);
                Ok(())
            }
            Edit::InsertLine(line) => self.insert(Entity::Line(line), retired),
            Edit::MovePoint { point, position } => self.move_point(point, position),
            Edit::DetachFrozenPoint(point) => self.detach_frozen_point(point),
            Edit::RemovePoint(point) => self.remove_point(point, retired, cancelled),
            Edit::RemoveLine(line) => self.remove(EntityId::Line(line), retired),
        }
    }

    fn insert_point(
        &mut self,
        point: SketchPoint,
        retired: &RetiredIds,
    ) -> Result<(), EditFailure> {
        admission::finite(point.id, point.position)?;
        self.insert(Entity::Point(point), retired)
    }

    fn move_point(&mut self, point: PointId, position: Point3) -> Result<(), EditFailure> {
        if let Some(source) = self.frozen_source(point) {
            return Err(EditFailure::FrozenPointMove { point, source });
        }
        admission::finite(point, position)?;
        let target = match self.entities.get_mut(&point.get()) {
            Some(Entity::Point(target)) => target,
            _ => return Err(EditFailure::MissingEditTarget(EntityId::Point(point))),
        };
        target.position = position;
        Ok(())
    }

    fn detach_frozen_point(&mut self, point: PointId) -> Result<(), EditFailure> {
        if self.point(point).is_none() {
            return Err(EditFailure::MissingEditTarget(EntityId::Point(point)));
        }
        self.frozen
            .remove(&point)
            .map(|_| ())
            .ok_or(EditFailure::PointIsNotFrozen(point))
    }

    fn insert(&mut self, entity: Entity, retired: &RetiredIds) -> Result<(), EditFailure> {
        let attempted = entity.id();
        if let Some(&existing) = retired.get(&attempted.raw()) {
            return Err(EditFailure::DuplicateIdentity {
                existing,
                attempted,
            });
        }
        match self.entities.entry(attempted.raw()) {
            Entry::Occupied(existing) => Err(EditFailure::DuplicateIdentity {
                existing: existing.get().id(),
                attempted,
            }),
            Entry::Vacant(entry) => {
                entry.insert(entity);
                Ok(())
            }
        }
    }

    fn remove_point(
        &mut self,
        point: PointId,
        retired: &mut RetiredIds,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), EditFailure> {
        self.remove(EntityId::Point(point), retired)?;
        for line in self.incident_lines(point, cancelled)? {
            poll(cancelled, BatchPhase::Staging)?;
            self.remove(EntityId::Line(line), retired)?;
        }
        Ok(())
    }

    fn incident_lines(
        &self,
        point: PointId,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<LineId>, EditFailure> {
        let mut incident = Vec::new();
        for entity in self.entities.values() {
            poll(cancelled, BatchPhase::Staging)?;
            if let Entity::Line(line) = entity
                && line.endpoints.contains(&point)
            {
                incident.push(line.id);
            }
        }
        Ok(incident)
    }

    fn remove(&mut self, key: EntityId, retired: &mut RetiredIds) -> Result<(), EditFailure> {
        if let Entry::Occupied(entry) = self.entities.entry(key.raw())
            && entry.get().id() == key
        {
            retired.insert(key.raw(), key);
            entry.remove();
            if let EntityId::Point(point) = key {
                self.frozen.remove(&point);
            }
            return Ok(());
        }
        Err(EditFailure::MissingEditTarget(key))
    }

    fn lines(&self) -> impl Iterator<Item = SketchLine> + '_ {
        self.entities.values().filter_map(|entity| match entity {
            Entity::Line(line) => Some(*line),
            Entity::Point(_) => None,
        })
    }

    fn prune_defining_points(
        &mut self,
        retired: &mut RetiredIds,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), EditFailure> {
        let unused = self.unused_defining_points(cancelled)?;
        for point in unused {
            poll(cancelled, BatchPhase::Staging)?;
            self.remove(EntityId::Point(point), retired)?;
        }
        Ok(())
    }

    fn unused_defining_points(
        &self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<PointId>, EditFailure> {
        let used = self.used_points(cancelled)?;
        let mut unused = Vec::new();
        for entity in self.entities.values() {
            poll(cancelled, BatchPhase::Staging)?;
            if let Entity::Point(point) = entity
                && point.role == PointRole::Defining
                && !used.contains(&point.id)
            {
                unused.push(point.id);
            }
        }
        Ok(unused)
    }

    fn used_points(
        &self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<BTreeSet<PointId>, EditFailure> {
        let mut used = BTreeSet::new();
        for entity in self.entities.values() {
            poll(cancelled, BatchPhase::Staging)?;
            if let Entity::Line(line) = entity {
                used.extend(line.endpoints);
            }
        }
        Ok(used)
    }

    fn check_lines(
        &self,
        previous: &BTreeMap<LineId, crate::LineGeometry>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<BTreeMap<LineId, crate::LineGeometry>, EditFailure> {
        self.lines()
            .map(|line| {
                poll(cancelled, BatchPhase::Validation)?;
                Ok((
                    line.id,
                    admission::line(self, line, previous.get(&line.id))?,
                ))
            })
            .collect()
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool, phase: BatchPhase) -> Result<(), EditFailure> {
    if cancelled() {
        return Err(EditFailure::Cancelled(phase));
    }
    Ok(())
}
