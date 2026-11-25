// SPDX-License-Identifier: GPL-2.0-only
//! Stage values first so complete geometry, including shared endpoints, admits together.
use crate::{
    BatchPhase, Edit, EditFailure, Entity, EntityId, LineId, PointId, PointRole, SketchLine,
    Snapshot, admission,
};
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
        };
        let mut retired = RetiredIds::new();
        for &edit in edits {
            poll(&mut cancelled, BatchPhase::Staging)?;
            draft.stage(edit, &mut retired, &mut cancelled)?;
        }
        if !retired.is_empty() {
            draft.prune_defining_points(&mut cancelled)?;
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
            Edit::InsertPoint(point) => {
                admission::finite(point.id, point.position)?;
                self.insert(Entity::Point(point), retired)
            }
            Edit::InsertLine(line) => self.insert(Entity::Line(line), retired),
            Edit::MovePoint { point, position } => {
                admission::finite(point, position)?;
                let target = self
                    .entities
                    .get_mut(&point.get())
                    .and_then(as_point)
                    .ok_or(EditFailure::MissingEditTarget(EntityId::Point(point)))?;
                target.position = position;
                Ok(())
            }
            Edit::RemovePoint(point) => self.remove_point(point, retired, cancelled),
            Edit::RemoveLine(line) => self.remove(EntityId::Line(line), retired),
        }
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
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), EditFailure> {
        let unused = self.unused_defining_points(cancelled)?;
        for point in unused {
            poll(cancelled, BatchPhase::Staging)?;
            self.entities.remove(&point.get());
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

fn as_point(entity: &mut Entity) -> Option<&mut crate::SketchPoint> {
    match entity {
        Entity::Point(point) => Some(point),
        Entity::Line(_) => None,
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool, phase: BatchPhase) -> Result<(), EditFailure> {
    if cancelled() {
        return Err(EditFailure::Cancelled(phase));
    }
    Ok(())
}
