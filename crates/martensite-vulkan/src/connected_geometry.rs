//! The outer render state applies once; an instance contributes geometry, not another Render call.
use super::Graphic3dRenderStructure;
use crate::{RenderError, error::rejected};
use martensite_tkernel::standard::exceptions::program_error;
use martensite_tkernel::standard::{Failure, Handle, WeakHandle, project_concrete};
use martensite_tkservice::graphic3d::*;
use std::collections::BTreeSet;
type GeometryBatches = Vec<Vec<Handle<dyn Graphic3dGroup>>>;

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
    /// Gather live referenced groups under the outer state; `owner.geometry_batches()?`.
    pub(super) fn geometry_batches(&self) -> Result<GeometryBatches, RenderError> {
        let mut ancestors = BTreeSet::new();
        self.collect_geometry_batches(&mut ancestors)
    }
    fn collect_geometry_batches(
        &self,
        ancestors: &mut BTreeSet<*const ()>,
    ) -> Result<GeometryBatches, RenderError> {
        let identity = self.this.read().object_address();
        if !ancestors.insert(identity) {
            return Err(rejected(
                ("connection cycle", self.identification()),
                "an acyclic geometry reference path",
            ));
        }
        let outcome = self.geometry_reference_batches(ancestors);
        ancestors.remove(&identity);
        outcome
    }
    fn geometry_reference_batches(
        &self,
        ancestors: &mut BTreeSet<*const ()>,
    ) -> Result<GeometryBatches, RenderError> {
        // CappingScene reverses batches for ordinary drawing, preserving instance-before-local order.
        let mut batches = vec![self.groups()];
        let reference = self.geometry_reference()?;
        if let Some(reference) = reference {
            batches.extend(reference.collect_geometry_batches(ancestors)?);
        }
        Ok(batches)
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
