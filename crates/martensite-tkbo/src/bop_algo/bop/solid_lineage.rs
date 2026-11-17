//! Compose actual selected-cell ancestry at Fuse assembly, before history filtering (#456).
use super::BOPAlgoBOP;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::Shape,
    toptools::{IndexedDataMapOfShapeListOfShape, MapOfShape},
};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoBOP {
    pub(super) fn retain_assembled_solid_images(
        &mut self,
        areas: &List<Shape>,
        faces: &IndexedDataMapOfShapeListOfShape,
        sources: &MapOfShape,
    ) -> Result<(), Failure> {
        if !self.builder().has_history() {
            return Ok(());
        }
        let mut images = IndexedDataMapOfShapeListOfShape::default();
        for area in areas.iter() {
            for cell in assembled_cells(area, faces)? {
                self.record_cell_origins(&cell, area, sources, &mut images);
            }
        }
        self.bind_assembled_images(&images)
    }
    fn record_cell_origins(
        &self,
        cell: &Shape,
        area: &Shape,
        sources: &MapOfShape,
        images: &mut IndexedDataMapOfShapeListOfShape,
    ) {
        if sources.contains(cell) {
            append_image(cell, area, images);
        }
        if let Some(origins) = self.builder().origins().seek(cell) {
            for source in origins.iter() {
                append_image(source, area, images);
            }
        }
    }
    fn bind_assembled_images(
        &mut self,
        images: &IndexedDataMapOfShapeListOfShape,
    ) -> Result<(), Failure> {
        for index in 1..=images.extent() {
            let source = images.find_key(index)?;
            let children = images.find_from_index(index)?;
            self.bind_assembled_origins(source, children);
            self.provider
                .builder
                .images
                .bound(source.clone(), children.clone());
        }
        Ok(())
    }
    fn bind_assembled_origins(&mut self, source: &Shape, children: &List<Shape>) {
        for child in children.iter() {
            let origins = self
                .provider
                .builder
                .origins
                .try_bound(child.clone(), List::default());
            if !origins.iter().any(|origin| origin.is_same(source)) {
                origins.append(source.clone());
            }
        }
    }
}
fn append_image(source: &Shape, area: &Shape, images: &mut IndexedDataMapOfShapeListOfShape) {
    if source.is_same(area) {
        return;
    }
    if let Some(children) = images.change_seek(source) {
        if !children.iter().any(|child| child.is_same(area)) {
            children.append(area.clone());
        }
        return;
    }
    let mut children = List::default();
    children.append(area.clone());
    images.add(source.clone(), children);
}
fn assembled_cells(
    area: &Shape,
    faces: &IndexedDataMapOfShapeListOfShape,
) -> Result<Vec<Shape>, Failure> {
    let mut visited = MapOfShape::default();
    let mut queue = Vec::new();
    seed_cells(area, faces, &mut visited, &mut queue)?;
    let mut current = 0;
    while current < queue.len() {
        for owners in Explorer::new(&queue[current], ShapeEnum::Face, ShapeEnum::Shape)?
            .filter_map(|face| faces.seek(&face).filter(|owners| owners.len() > 1))
        {
            append_cells(owners, &mut visited, &mut queue);
        }
        current += 1;
    }
    Ok(queue)
}
fn seed_cells(
    area: &Shape,
    faces: &IndexedDataMapOfShapeListOfShape,
    visited: &mut MapOfShape,
    queue: &mut Vec<Shape>,
) -> Result<(), Failure> {
    for face in Explorer::new(area, ShapeEnum::Face, ShapeEnum::Shape)? {
        if let Some(owners) = faces.seek(&face).filter(|owners| owners.len() == 1) {
            append_cells(owners, visited, queue);
        }
    }
    Ok(())
}
fn append_cells(owners: &List<Shape>, visited: &mut MapOfShape, queue: &mut Vec<Shape>) {
    for owner in owners.iter() {
        if visited.add(owner.clone()) {
            queue.push(owner.clone());
        }
    }
}
