//! `BOPAlgoBuilder::FillImagesContainers`, `FillImagesContainer`, `FillImagesCompounds` and
//! `FillImagesCompound` (BOPAlgoBuilder_1).
use super::BOPAlgoBuilder;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::bop_ds::DataStructure;
use crate::bop_tools::{is_split_to_reverse_with_warning, make_container};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{List, Map};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoBuilder {
    /// Rebuilds every source container of `kind` with modified sub-shapes
    /// (`FillImagesContainers`); `builder.fill_images_containers(ShapeEnum::Wire, &ds)?`.
    pub fn fill_images_containers(
        &mut self,
        kind: ShapeEnum,
        structure: &DataStructure,
    ) -> Result<(), Failure> {
        let observed = ProgressCancellation::default();
        self.fill_containers_progress_raw(kind, structure, &ProgressRange::default(), &observed)
    }

    /// Preserves native per-source polls without inventing per-container advances.
    /// Example: `builder.fill_containers_progress_raw(kind, ds, range, observed)?;` is raw.
    pub(crate) fn fill_containers_progress_raw(
        &mut self,
        kind: ShapeEnum,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::with_literal(range, "Building splits of containers", 1.0, false);
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() == kind {
                self.fill_images_container(&info.shape().clone(), kind)?;
            }
            if observe_user_break(self.options(), &scope, observed) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// A new container of the splits of the sub-shapes, oriented as the originals, when any
    /// sub-shape was modified (`FillImagesContainer`).
    pub fn fill_images_container(&mut self, shape: &Shape, kind: ShapeEnum) -> Result<(), Failure> {
        if !self.has_modified_sub_shape(shape) {
            // None of the sub-shapes was modified: no new container.
            return Ok(());
        }
        let builder = BRepBuilder::default();
        let mut container = Shape::default();
        make_container(kind, &mut container);
        let context = self.context.clone();
        for sub_shape in ShapeIterator::new(shape, true, true) {
            let Some(images) = self.images.seek(&sub_shape) else {
                builder.add(&mut container, &sub_shape)?;
                continue;
            };
            for image in images.clone().iter() {
                let mut image = image.clone();
                if !image.is_equal(&sub_shape)
                    && is_split_to_reverse_with_warning(
                        &image,
                        &sub_shape,
                        super::required_context(&context)?,
                        Some(self.options().report()),
                    )?
                {
                    image.reverse();
                }
                builder.add(&mut container, &image)?;
            }
        }
        container.set_closed(BRepTool::is_closed(&container)?)?;
        let mut list = List::default();
        list.append(container);
        self.images.bound(shape.clone(), list);
        Ok(())
    }

    fn has_modified_sub_shape(&self, shape: &Shape) -> bool {
        ShapeIterator::new(shape, true, true).any(|sub_shape| {
            self.images.seek(&sub_shape).is_some_and(|images| {
                images.len() != 1
                    || !images
                        .iter()
                        .next()
                        .is_some_and(|first| first.is_same(&sub_shape))
            })
        })
    }

    /// Rebuilds every source compound with modified parts (`FillImagesCompounds`).
    /// `builder.fill_images_compounds(&ds)?`.
    pub fn fill_images_compounds(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        let observed = ProgressCancellation::default();
        self.fill_compounds_progress_raw(structure, &ProgressRange::default(), &observed)
    }

    /// Preserves native per-source compound polling and for-increment advances.
    /// Example: `builder.fill_compounds_progress_raw(ds, range, observed)?;` then classify its report.
    pub(crate) fn fill_compounds_progress_raw(
        &mut self,
        structure: &DataStructure,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let mut fence: Map<Shape, ShapeMapHasher> = Map::with_hasher(ShapeMapHasher, 100);
        let count = structure.nb_source_shapes() as f64;
        let scope =
            ProgressScope::with_literal(range, "Building splits of compounds", count, false);
        for index in 0..structure.nb_source_shapes() {
            self.bind_source_compound(structure, index, &mut fence)?;
            if observe_user_break(self.options(), &scope, observed) {
                return Ok(());
            }
            drop(scope.next(1.0));
        }
        Ok(())
    }

    fn bind_source_compound(
        &mut self,
        structure: &DataStructure,
        index: i32,
        fence: &mut Map<Shape, ShapeMapHasher>,
    ) -> Result<(), Failure> {
        let info = structure.shape_info(index)?;
        if info.shape_type() == ShapeEnum::Compound {
            self.fill_images_compound(&info.shape().clone(), fence)?;
        }
        Ok(())
    }

    /// A new compound of the images of the parts, nested compounds first, when any part has
    /// images (`FillImagesCompound`).
    pub fn fill_images_compound(
        &mut self,
        shape: &Shape,
        fence: &mut Map<Shape, ShapeMapHasher>,
    ) -> Result<(), Failure> {
        if !fence.add(shape.clone()) {
            return Ok(());
        }
        let mut interfered = false;
        for part in ShapeIterator::new(shape, true, true) {
            if part.shape_type()? == ShapeEnum::Compound {
                self.fill_images_compound(&part, fence)?;
            }
            if self.images.is_bound(&part) {
                interfered = true;
            }
        }
        if !interfered {
            return Ok(());
        }
        let builder = BRepBuilder::default();
        let mut compound = Shape::default();
        make_container(ShapeEnum::Compound, &mut compound);
        for part in ShapeIterator::new(shape, true, true) {
            let Some(images) = self.images.seek(&part) else {
                builder.add(&mut compound, &part)?;
                continue;
            };
            for image in images.iter() {
                builder.add(&mut compound, &image.oriented(part.orientation()))?;
            }
        }
        let mut list = List::default();
        list.append(compound);
        self.images.bind(shape.clone(), list);
        Ok(())
    }
}
