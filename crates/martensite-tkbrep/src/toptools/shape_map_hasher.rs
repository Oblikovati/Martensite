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
}

/// `NCollection_IndexedMap<TopoDS_Shape, TopTools_ShapeMapHasher>`.
pub type IndexedMapOfShape = IndexedMap<Shape, ShapeMapHasher>;
/// `NCollection_Map<TopoDS_Shape, TopTools_ShapeMapHasher>`.
pub type MapOfShape = Map<Shape, ShapeMapHasher>;
/// `NCollection_IndexedDataMap<TopoDS_Shape, NCollection_List<TopoDS_Shape>, TopTools_ShapeMapHasher>`.
pub type IndexedDataMapOfShapeListOfShape = IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>;
