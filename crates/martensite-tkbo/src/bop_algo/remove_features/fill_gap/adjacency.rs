//! Feature incidence discovery retains source identities and input traversal order.
use super::{Connections, FillGap};
use crate::bop_algo::ProgressCancellation;
use crate::bop_algo::remove_features::helpers::{children, explore};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

impl FillGap {
    /// The faces sharing an edge with the feature, and the solids holding the feature or a
    /// face adjacent to an adjacent face (`FindAdjacentFaces`).
    #[cfg(test)]
    pub(super) fn find_adjacent_faces(
        &mut self,
        connections: &Connections<'_>,
    ) -> Result<IndexedMapOfShape, Failure> {
        self.find_adjacent_faces_on_progress(
            connections,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
    }

    /// Collect actual feature incidences in the caller range; cancellation remains silent.
    /// Example: `gap.find_adjacent_faces_on_progress(connections, range, observed)?` retains partial diagnostic state.
    pub(super) fn find_adjacent_faces_on_progress(
        &mut self,
        connections: &Connections<'_>,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<IndexedMapOfShape, Failure> {
        self.cache_feature_faces();
        let outer = ProgressScope::unnamed(range, 2., false);
        self.collect_adjacency_in_scope(connections, &outer, observed)
    }

    fn collect_adjacency_in_scope(
        &mut self,
        connections: &Connections<'_>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<IndexedMapOfShape, Failure> {
        // Keep this scope alive through solid collection to preserve close-callback order.
        let face_range = outer.next(1.);
        let faces =
            ProgressScope::with_literal(&face_range, "Looking for adjacent faces", 1., true);
        let mut adjacent = IndexedMapOfShape::default();
        if !self.collect_feature_neighbors(connections, &mut adjacent, &faces, observed)? {
            return Ok(adjacent);
        }
        self.collect_adjacent_solids(connections, &adjacent, &outer.next(1.), observed)?;
        Ok(adjacent)
    }

    fn collect_feature_neighbors(
        &mut self,
        connections: &Connections<'_>,
        adjacent: &mut IndexedMapOfShape,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        for face in children(&self.feature) {
            if !adjacency_more(scope, observed) {
                return Ok(false);
            }
            self.collect_face_neighbors(&face, connections, adjacent)?;
            self.add_solids_of(&face, connections);
            drop(scope.next(1.));
        }
        Ok(true)
    }

    fn collect_adjacent_solids(
        &mut self,
        connections: &Connections<'_>,
        adjacent: &IndexedMapOfShape,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::with_literal(
            range,
            "Looking for adjacent solids",
            adjacent.extent() as f64,
            false,
        );
        for face in adjacent.iter() {
            if !adjacency_more(&scope, observed) {
                return Ok(());
            }
            self.collect_neighbor_solids(face, connections)?;
            drop(scope.next(1.));
        }
        Ok(())
    }

    fn cache_feature_faces(&mut self) {
        for face in children(&self.feature) {
            self.feature_faces.add(face);
        }
    }

    fn collect_face_neighbors(
        &self,
        face: &Shape,
        connections: &Connections<'_>,
        adjacent: &mut IndexedMapOfShape,
    ) -> Result<(), Failure> {
        for edge in explore(face, ShapeEnum::Edge)? {
            self.retain_edge_neighbors(&edge, connections, adjacent);
        }
        Ok(())
    }

    fn retain_edge_neighbors(
        &self,
        edge: &Shape,
        connections: &Connections<'_>,
        adjacent: &mut IndexedMapOfShape,
    ) {
        for neighbor in connections
            .edge_faces
            .seek(edge)
            .into_iter()
            .flat_map(|faces| faces.iter())
        {
            if !self.feature_faces.contains(neighbor) {
                adjacent.add(neighbor.clone());
            }
        }
    }

    fn collect_neighbor_solids(
        &mut self,
        face: &Shape,
        connections: &Connections<'_>,
    ) -> Result<(), Failure> {
        for edge in explore(face, ShapeEnum::Edge)? {
            self.collect_edge_solids(&edge, connections);
        }
        Ok(())
    }

    fn collect_edge_solids(&mut self, edge: &Shape, connections: &Connections<'_>) {
        for neighbor in connections
            .edge_faces
            .seek(edge)
            .into_iter()
            .flat_map(|faces| faces.iter())
        {
            self.add_solids_of(neighbor, connections);
        }
    }

    fn add_solids_of(&mut self, face: &Shape, connections: &Connections<'_>) {
        for solid in connections
            .face_solids
            .seek(face)
            .into_iter()
            .flat_map(|solids| solids.iter())
        {
            self.solids.add(solid.clone());
        }
    }
}

#[cfg(test)]
#[path = "adjacency_tests.rs"]
mod tests;

fn adjacency_more(scope: &ProgressScope<'_>, observed: &ProgressCancellation) -> bool {
    let more = scope.more();
    if !more {
        observed.record_cancelled();
    }
    more
}
