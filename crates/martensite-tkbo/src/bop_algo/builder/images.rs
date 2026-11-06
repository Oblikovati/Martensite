//! `BOPAlgoBuilder::FillImagesVertices` (BOPAlgoBuilder_1).
use super::{BOPAlgoBuilder, ShapeImages};
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::{ncollection::List, standard::Failure};

impl BOPAlgoBuilder {
    /// Each vertex of a same-domain group gets its representative as image, same-domain
    /// shape and origin (`FillImagesVertices`); `builder.fill_images_vertices(&structure)?`.
    pub fn fill_images_vertices(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        let observed = ProgressCancellation::default();
        self.fill_vertices_progress_raw(structure, &ProgressRange::default(), &observed)
    }

    /// Runs actual direct-map bindings with the native pre-work poll.
    /// Example: `builder.fill_vertices_progress_raw(ds, range, observed)?;` is raw choreography.
    pub(crate) fn fill_vertices_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let count = structure.same_domain_binding_count() as f64;
        let scope = ProgressScope::with_literal(range, "Filling splits of vertices", count, false);
        for (source, representative) in structure.shape_sd_items() {
            if observe_user_break(self.options(), &scope, observed) {
                return Ok(());
            }
            self.bind_vertex_image(structure, *source, *representative)?;
            drop(scope.next(1.0));
        }
        Ok(())
    }

    fn bind_vertex_image(
        &mut self,
        structure: &DataStructure,
        source: i32,
        representative: i32,
    ) -> Result<(), Failure> {
        let vertex = structure.shape(source)?.clone();
        let representative = structure.shape(representative)?.clone();
        let mut image = List::default();
        image.append(representative.clone());
        self.images.bound(vertex.clone(), image);
        self.shapes_sd.bind(vertex.clone(), representative.clone());
        append_listed(&mut self.origins, &representative, &vertex);
        Ok(())
    }
}

/// Appends `value` to the list of `key`, binding an empty list first.
pub(super) fn append_listed(map: &mut ShapeImages, key: &Shape, value: &Shape) {
    if let Some(list) = map.change_seek(key) {
        list.append(value.clone());
        return;
    }
    map.bound(key.clone(), List::default())
        .append(value.clone());
}
