//! `STEPSelectionsCounter`: counts the solids, shells, faces, wires and edges of a transfer.
use super::entities_of;
use crate::step_basic::StepBasicProductRelatedProductCategory;
use crate::step_construct::StepConstructAssembly;
use crate::step_data::StepDataSelectType;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomCompositeCurve, StepGeomCurve, StepGeomSurface};
use crate::step_repr::{StepReprMappedItem, StepReprShapeRepresentationRelationship};
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeConnectedFaceSet,
    StepShapeContextDependentShapeRepresentation, StepShapeFaceSurface, StepShapeFacetedBrep,
    StepShapeGeometricSet, StepShapeManifoldSolidBrep, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation, StepShapeShellBasedSurfaceModel,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceGraph;
use std::collections::HashSet;

/// The entities a count met once, by address.
type EntitySet = HashSet<usize>;

fn key<T: ?Sized + martensite_tkernel::standard::RuntimeTyped>(entity: &Handle<T>) -> usize {
    entity.object_address() as usize
}

/// `STEPSelectionsCounter`. The `NbInstances*` counts each kind every time it is met, the
/// `Nb*` the distinct ones.
#[derive(Default)]
pub struct STEPSelectionsCounter {
    my_nb_faces: i32,
    my_nb_shells: i32,
    my_nb_solids: i32,
    my_nb_wires: i32,
    my_nb_edges: i32,
    my_map_of_faces: EntitySet,
    my_map_of_shells: EntitySet,
    my_map_of_solids: EntitySet,
    my_map_of_wires: EntitySet,
    my_map_of_edges: EntitySet,
}

impl STEPSelectionsCounter {
    /// `STEPSelectionsCounter()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `clear`.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    /// `add_shell(cfs)`.
    fn add_shell(&mut self, shell: &Handle<StepShapeConnectedFaceSet>) -> Result<(), Failure> {
        self.my_map_of_shells.insert(key(shell));
        self.my_nb_shells += 1;
        let live = step_live(shell, "STEPSelectionsCounter")?;
        let count = live.nb_cfs_faces()?;
        for rank in 1..=count {
            self.my_map_of_faces
                .insert(key(&live.cfs_faces_value(rank)?));
        }
        self.my_nb_faces += count;
        Ok(())
    }
    /// `add_composite_curve(ccurve)`.
    fn add_composite_curve(
        &mut self,
        curve: &Handle<StepGeomCompositeCurve>,
    ) -> Result<(), Failure> {
        let live = step_live(curve, "STEPSelectionsCounter")?;
        for rank in 1..=live.nb_segments() {
            let segment = live.segments_value(rank)?;
            let parent = step_live(&segment, "STEPSelectionsCounter")?.parent_curve();
            let composite: Handle<StepGeomCompositeCurve> = parent.cast();
            if composite.is_some() {
                self.add_composite_curve(&composite)?;
            } else {
                self.my_nb_edges += 1;
                self.my_map_of_edges.insert(key(&parent));
            }
        }
        Ok(())
    }
    /// A solid with its outer shell.
    fn add_solid(
        &mut self,
        start: &Handle<StandardTransient>,
        outer: &Handle<StepShapeConnectedFaceSet>,
    ) -> Result<(), Failure> {
        self.my_map_of_solids.insert(key(start));
        self.my_nb_solids += 1;
        self.add_shell(outer)
    }
    /// The shells and the curves of a brep model, true when `start` was one.
    fn count_brep(&mut self, start: &Handle<StandardTransient>) -> Result<bool, Failure> {
        let faceted: Handle<StepShapeFacetedBrep> = start.cast();
        if let Some(live) = faceted.get() {
            self.add_solid(start, &live.outer())?;
            return Ok(true);
        }
        let with_voids: Handle<StepShapeBrepWithVoids> = start.cast();
        if let Some(live) = with_voids.get() {
            self.add_solid(start, &live.outer())?;
            for rank in 1..=live.nb_voids() {
                self.add_shell(&live.voids_value(rank)?.cast())?;
            }
            return Ok(true);
        }
        let manifold: Handle<StepShapeManifoldSolidBrep> = start.cast();
        if let Some(live) = manifold.get() {
            // A faceted_brep_and_brep_with_voids stops here too, being a kind of
            // manifold_solid_brep: its branch after this one is never reached, and its voids
            // are not counted.
            self.add_solid(start, &live.outer())?;
            return Ok(true);
        }
        let surface_model: Handle<StepShapeShellBasedSurfaceModel> = start.cast();
        if let Some(live) = surface_model.get() {
            for rank in 1..=live.nb_sbsm_boundary()? {
                let select = live.sbsm_boundary_value(rank)?;
                let open = select.open_shell();
                if open.is_some() {
                    self.add_shell(&open.cast())?;
                }
                let closed = select.closed_shell();
                if closed.is_some() {
                    self.add_shell(&closed.cast())?;
                }
            }
            return Ok(true);
        }
        Ok(false)
    }
    /// The elements of a geometric set.
    fn count_geometric_set(&mut self, set: &StepShapeGeometricSet) -> Result<(), Failure> {
        for rank in 1..=set.nb_elements() {
            let ent = set.elements_value(rank)?.value();
            let composite: Handle<StepGeomCompositeCurve> = ent.cast();
            if composite.is_some() {
                self.my_nb_wires += 1;
                self.my_map_of_wires.insert(key(&composite));
                self.add_composite_curve(&composite)?;
            } else if ent.cast::<StepGeomCurve>().is_some() {
                self.my_nb_edges += 1;
                self.my_map_of_edges.insert(key(&ent));
            } else if ent.cast::<StepGeomSurface>().is_some() {
                self.my_nb_faces += 1;
                self.my_map_of_faces.insert(key(&ent));
            }
        }
        Ok(())
    }
    /// `count(graph, start)`.
    pub fn count(
        &mut self,
        graph: &InterfaceGraph,
        start: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        if start.is_null()
            || start
                .cast::<StepBasicProductRelatedProductCategory>()
                .is_some()
        {
            return Ok(());
        }
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> = start.cast();
        if let Some(live) = sdr.get() {
            self.count(graph, &live.used_representation().cast())?;
            for shared in entities_of(graph.shareds(start)?)? {
                if shared
                    .cast::<StepShapeContextDependentShapeRepresentation>()
                    .is_some()
                {
                    self.count(graph, &shared)?;
                }
            }
            return Ok(());
        }
        let rep: Handle<StepShapeShapeRepresentation> = start.cast();
        if let Some(live) = rep.get() {
            for rank in 1..=live.nb_items() {
                self.count(graph, &live.items_value(rank)?.cast())?;
            }
            return Ok(());
        }
        if self.count_brep(start)? {
            return Ok(());
        }
        let set: Handle<StepShapeGeometricSet> = start.cast();
        if let Some(live) = set.get() {
            // No return here; nothing below matches a geometric_set.
            return self.count_geometric_set(live);
        }
        let mapped: Handle<StepReprMappedItem> = start.cast();
        if let Some(live) = mapped.get() {
            self.count(graph, &live.mapping_target().cast())?;
            let map = live.mapping_source();
            let Some(live_map) = map.get() else {
                return Ok(());
            };
            return self.count(graph, &live_map.mapped_representation().cast());
        }
        if start.cast::<StepShapeFaceSurface>().is_some() {
            self.my_nb_faces += 1;
            self.my_map_of_faces.insert(key(start));
            return Ok(());
        }
        let cdsr: Handle<StepShapeContextDependentShapeRepresentation> = start.cast();
        if let Some(live) = cdsr.get() {
            let relation = live.representation_relation();
            let Some(live_relation) = relation.get() else {
                return Ok(());
            };
            let reversed = StepConstructAssembly::check_srr_reverses_nauo(graph, &cdsr)?;
            let rep = if reversed {
                live_relation.rep2()
            } else {
                live_relation.rep1()
            };
            for sharing in entities_of(graph.sharings(&rep.cast())?)? {
                if sharing
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .is_some()
                {
                    self.count(graph, &sharing)?;
                }
            }
            return Ok(());
        }
        let und: Handle<StepReprShapeRepresentationRelationship> = start.cast();
        if let Some(live) = und.get() {
            self.count(graph, &live.rep1().cast())?;
            self.count(graph, &live.rep2().cast())?;
        }
        Ok(())
    }
    /// `nb_instances_of_faces`.
    pub fn nb_instances_of_faces(&self) -> i32 {
        self.my_nb_faces
    }
    /// `nb_source_faces`.
    pub fn nb_source_faces(&self) -> i32 {
        self.my_map_of_faces.len() as i32
    }
    /// `nb_instances_of_shells`.
    pub fn nb_instances_of_shells(&self) -> i32 {
        self.my_nb_shells
    }
    /// `nb_source_shells`.
    pub fn nb_source_shells(&self) -> i32 {
        self.my_map_of_shells.len() as i32
    }
    /// `nb_instances_of_solids`.
    pub fn nb_instances_of_solids(&self) -> i32 {
        self.my_nb_solids
    }
    /// `nb_source_solids`.
    pub fn nb_source_solids(&self) -> i32 {
        self.my_map_of_solids.len() as i32
    }
    /// `nb_instances_of_edges`.
    pub fn nb_instances_of_edges(&self) -> i32 {
        self.my_nb_edges
    }
    /// `nb_source_edges`.
    pub fn nb_source_edges(&self) -> i32 {
        self.my_map_of_edges.len() as i32
    }
    /// `nb_instances_of_wires`.
    pub fn nb_instances_of_wires(&self) -> i32 {
        self.my_nb_wires
    }
    /// `nb_source_wires`.
    pub fn nb_source_wires(&self) -> i32 {
        self.my_map_of_wires.len() as i32
    }
}
