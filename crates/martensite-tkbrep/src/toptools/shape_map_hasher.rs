//! The shape hasher of the TopTools maps and the TopTools format versions.
use crate::topods::Shape;
use martensite_tkernel::ncollection::{HashPolicy, IndexedDataMap, IndexedMap, List, Map};

/// `TopTools_ShapeMapHasher`: hashes a shape with `std::hash<TopoDS_Shape>` and
/// compares with `IsSame`, so that the orientation is ignored.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkbrep::toptools::ShapeMapHasher;
/// use martensite_tkernel::ncollection::HashPolicy;
/// let null = Shape::default();
/// assert!(ShapeMapHasher.equal(&null, &null.reversed()));
/// assert_eq!(ShapeMapHasher.hash(&null), null.hash_code());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeMapHasher;

impl HashPolicy<Shape> for ShapeMapHasher {
    /// `operator()(theShape)`.
    fn hash(&self, key: &Shape) -> usize {
        key.hash_code()
    }
    /// `operator()(S1, S2)`: `S1.IsSame(S2)`.
    fn equal(&self, first: &Shape, second: &Shape) -> bool {
        first.is_same(second)
    }
    fn walks_in_order_added(&self) -> bool {
        true
    }
}

/// `NCollection_IndexedMap<TopoDS_Shape, TopTools_ShapeMapHasher>`.
pub type IndexedMapOfShape = IndexedMap<Shape, ShapeMapHasher>;
/// `NCollection_Map<TopoDS_Shape, TopTools_ShapeMapHasher>`.
pub type MapOfShape = Map<Shape, ShapeMapHasher>;
/// `NCollection_IndexedDataMap<TopoDS_Shape, NCollection_List<TopoDS_Shape>, TopTools_ShapeMapHasher>`.
pub type IndexedDataMapOfShapeListOfShape = IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>;

#[cfg(test)]
mod walk_tests {
    use super::*;
    use crate::brep::BRepBuilder;
    use crate::topods::Vertex;
    use martensite_tkernel::ncollection::{DataMap, DefaultHashPolicy, HashPolicy};

    fn vertices() -> Vec<Vertex> {
        (0..64)
            .map(|_| {
                let mut vertex = Vertex::default();
                BRepBuilder::default().make_vertex(&mut vertex);
                vertex
            })
            .collect()
    }

    fn walks_added<K: Clone + std::fmt::Debug, H: HashPolicy<K> + Default>(keys: &[K]) {
        let mut set = Map::<K, H>::default();
        let mut map = DataMap::<K, usize, H>::default();
        for (rank, key) in keys.iter().enumerate() {
            set.add(key.clone());
            map.bind(key.clone(), rank);
        }
        let policy = H::default();
        for (key, walked) in keys.iter().zip(set.iter()) {
            assert!(policy.equal(key, walked));
        }
        let ranks: Vec<usize> = map.items().map(|(_, rank)| *rank).collect();
        assert_eq!(ranks, (0..keys.len()).collect::<Vec<usize>>());
    }

    #[test]
    fn maps_of_shapes_walk_in_the_order_added_under_every_policy() {
        let vertices = vertices();
        let shapes: Vec<Shape> = vertices.iter().map(|v| v.clone().into_shape()).collect();
        walks_added::<Shape, ShapeMapHasher>(&shapes);
        walks_added::<Shape, DefaultHashPolicy>(&shapes);
        walks_added::<Vertex, DefaultHashPolicy>(&vertices);
    }
}
