//! `BOPAlgoBuilder::FillImagesEdges` (BOPAlgoBuilder_1).
use super::BOPAlgoBuilder;
use super::images::append_listed;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::bop_ds::{DataStructure, PaveBlockHandle};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoBuilder {
    /// Each referenced source edge gets the edges of the real pave blocks of its pave blocks
    /// as images; small edges without pave blocks get an empty list and leave the result
    /// (`FillImagesEdges`); `builder.fill_images_edges(&structure)?`.
    pub fn fill_images_edges(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        let observed = ProgressCancellation::default();
        self.fill_edges_progress_raw(structure, &ProgressRange::default(), &observed)
    }

    /// Preserves the native post-edge poll and advances skipped source entries too.
    /// Example: `builder.fill_edges_progress_raw(ds, range, observed)?;` then classify its report.
    pub(crate) fn fill_edges_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let count = structure.nb_source_shapes() as f64;
        let scope = ProgressScope::with_literal(range, "Filling splits of edges", count, false);
        for index in 0..structure.nb_source_shapes() {
            if self.bind_source_edge_images(structure, index)?
                && observe_user_break(self.options(), &scope, observed)
            {
                return Ok(());
            }
            drop(scope.next(1.0));
        }
        Ok(())
    }

    fn bind_source_edge_images(
        &mut self,
        structure: &DataStructure,
        index: i32,
    ) -> Result<bool, Failure> {
        let info = structure.shape_info(index)?;
        if info.shape_type() != ShapeEnum::Edge || !info.has_reference() {
            return Ok(false);
        }
        let edge = info.shape().clone();
        self.images.bound(edge.clone(), List::default());
        for block in structure.pave_blocks(index)? {
            self.bind_edge_block_image(structure, &edge, block)?;
        }
        Ok(true)
    }

    fn bind_edge_block_image(
        &mut self,
        structure: &DataStructure,
        edge: &martensite_tkbrep::topods::Shape,
        block: &PaveBlockHandle,
    ) -> Result<(), Failure> {
        let real = structure.real_pave_block(block)?;
        let split = structure
            .shape(real.lock().expect("pave-block lock").edge())?
            .clone();
        if let Some(images) = self.images.change_seek(edge) {
            images.append(split.clone());
        }
        append_listed(&mut self.origins, &split, edge);
        if structure.is_common_block_on_edge(block) {
            let own = structure.shape(block.lock().expect("pave-block lock").edge())?;
            self.shapes_sd.bind(own.clone(), split);
        }
        Ok(())
    }
}
