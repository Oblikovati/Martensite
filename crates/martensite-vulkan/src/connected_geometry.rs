//! The outer render state applies once; an instance contributes geometry, not another Render call.
use super::Graphic3dRenderStructure;
use crate::{RenderError, error::rejected};
use martensite_tkernel::standard::exceptions::program_error;
use martensite_tkernel::standard::{Failure, Handle, WeakHandle, project_concrete};
use martensite_tkservice::graphic3d::*;
use std::collections::BTreeSet;
pub(crate) type GroupBatch = Vec<Handle<dyn Graphic3dGroup>>;

pub(crate) struct GeometryBatches {
    local: GroupBatch,
    references: Vec<GroupBatch>,
}

impl GeometryBatches {
    pub(crate) fn len(&self) -> usize {
        1 + self.references.len()
    }

    pub(crate) fn into_batches(self) -> impl DoubleEndedIterator<Item = GroupBatch> {
        std::iter::once(self.local).chain(self.references)
    }
}

impl Graphic3dRenderStructure {
    fn connection_target(
        &self,
        other: &dyn Graphic3dCStructure,
    ) -> Result<WeakHandle<Self>, Failure> {
        let target = project_concrete::<Self>(other).ok_or_else(|| {
            connected_failure(other.identification(), "a Martensite renderer structure")
        })?;
        self.require_connection_receiver()?;
        Ok(target.this.read().clone())
    }
    fn require_connection_receiver(&self) -> Result<(), Failure> {
        if self.parent.is_some() {
            return Err(connected_failure(
                self.identification(),
                "a non-shadow connection receiver",
            ));
        }
        Ok(())
    }
    // GlStructure::Connect permits one target; multiple children live in the outer graph.
    /// Retain one actual geometry target without owning it; `owner.connect_geometry(target)?`.
    pub(super) fn connect_geometry(&self, other: &dyn Graphic3dCStructure) -> Result<(), Failure> {
        let target = self.connection_target(other)?;
        let mut stored = self.connection.write();
        if stored.as_ref().is_some_and(|known| *known != target) {
            return Err(connected_failure(
                (
                    other.identification(),
                    stored.as_ref().map(WeakHandle::object_address),
                ),
                "the already connected target identity",
            ));
        }
        *stored = Some(target);
        Ok(())
    }
    /// Forget only the matching target identity; `owner.disconnect_geometry(target)?`.
    pub(super) fn disconnect_geometry(
        &self,
        other: &dyn Graphic3dCStructure,
    ) -> Result<(), Failure> {
        let target = self.connection_target(other)?;
        let mut stored = self.connection.write();
        if stored.as_ref() == Some(&target) {
            *stored = None;
        }
        Ok(())
    }
    /// Gather live referenced groups under the outer state; `owner.geometry_batches(local_groups)?`.
    pub(super) fn geometry_batches(
        &self,
        local: GroupBatch,
    ) -> Result<GeometryBatches, RenderError> {
        let mut batches = GeometryBatches {
            local,
            references: Vec::new(),
        };
        let Some(first) = self.geometry_reference()? else {
            return Ok(batches);
        };
        self.collect_reference_batches(&first, &mut batches.references)?;
        Ok(batches)
    }

    fn collect_reference_batches(
        &self,
        first: &Handle<Self>,
        batches: &mut Vec<GroupBatch>,
    ) -> Result<(), RenderError> {
        let root = self.this.read().object_address();
        let first_address = first.object_address();
        if first_address == root {
            return Err(first.connection_cycle());
        }
        batches.push(first.groups());
        if let Some(next) = first.geometry_reference()? {
            let mut ancestors = BTreeSet::from([root, first_address]);
            next.append_geometry_batches(&mut ancestors, batches)?;
        }
        Ok(())
    }

    fn append_geometry_batches(
        &self,
        ancestors: &mut BTreeSet<*const ()>,
        batches: &mut Vec<GroupBatch>,
    ) -> Result<(), RenderError> {
        let identity = self.this.read().object_address();
        if !ancestors.insert(identity) {
            return Err(self.connection_cycle());
        }
        batches.push(self.groups());
        // A renderer structure has at most one reference, so there is no sibling path.
        if let Some(reference) = self.geometry_reference()? {
            reference.append_geometry_batches(ancestors, batches)?;
        }
        Ok(())
    }

    fn connection_cycle(&self) -> RenderError {
        rejected(
            ("connection cycle", self.identification()),
            "an acyclic geometry reference path",
        )
    }
    pub(super) fn geometry_reference(&self) -> Result<Option<Handle<Self>>, RenderError> {
        if let Some(parent) = &self.parent {
            return Ok(Some(parent.clone()));
        }
        let stored = self.connection.read().clone();
        let Some(target) = stored else {
            return Ok(None);
        };
        let lease = target.upgrade();
        if lease.is_null() {
            return Err(rejected(
                ("expired connected target", target.object_address()),
                "a live connected geometry source",
            ));
        }
        Ok(Some(lease))
    }
}

fn connected_failure(value: impl std::fmt::Debug, expected: &str) -> Failure {
    let message = format!("connected geometry received {value:?}; expected {expected}");
    program_error(&message)
}
