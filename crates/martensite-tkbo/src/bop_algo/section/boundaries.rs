//! Source-handle boundary incidence keeps each argument contribution distinct.
use super::*;
impl BOPAlgoSection {
    /// For each vertex and edge of the boundaries of the arguments (or of their splits), the
    /// number of arguments it bounds (`aMSI`); `section.boundary_counts_checked(cancelled)?`.
    pub(super) fn boundary_counts_checked(
        &self,
        cancelled: &mut impl FnMut(&Self) -> bool,
    ) -> Result<Option<IndexedDataMap<Shape, i32, ShapeMapHasher>>, Failure> {
        let mut unique = MapOfShape::default();
        let mut counts: IndexedDataMap<Shape, i32, ShapeMapHasher> =
            IndexedDataMap::with_hasher(ShapeMapHasher, 100);
        for argument in self.builder.arguments.iter() {
            if !unique.add(argument.clone()) {
                continue;
            }
            if cancelled(self) {
                return Ok(None);
            }
            for shape in self.boundary_splits(argument)?.iter() {
                match counts.change_seek(shape) {
                    Some(count) => *count += 1,
                    None => {
                        counts.add(shape.clone(), 1);
                    }
                }
            }
        }
        Ok(Some(counts))
    }

    /// The vertices and edges of the edge and vertex images of the argument.
    fn boundary_splits(&self, argument: &Shape) -> Result<IndexedMapOfShape, Failure> {
        let mut fence = MapOfShape::default();
        let mut parts = Vec::new();
        for kind in [ShapeEnum::Edge, ShapeEnum::Vertex] {
            for part in Explorer::new(argument, kind, ShapeEnum::Shape)? {
                if fence.add(part.clone()) {
                    parts.push(part);
                }
            }
        }
        let mut splits = IndexedMapOfShape::default();
        for part in parts {
            let images: Vec<Shape> = match self.builder.images().seek(&part) {
                Some(images) => images.iter().cloned().collect(),
                None => vec![part],
            };
            for image in images {
                topexp::map_shapes_of_type(&image, ShapeEnum::Vertex, &mut splits)?;
                topexp::map_shapes_of_type(&image, ShapeEnum::Edge, &mut splits)?;
            }
        }
        Ok(splits)
    }
}
