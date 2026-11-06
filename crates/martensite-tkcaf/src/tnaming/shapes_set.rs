//! `TNamingShapesSet`: a set of shapes (orientation ignored) with set algebra, and its
//! iterator.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::ncollection::MapIterator;
use martensite_tkernel::standard::{Failure, exceptions};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `TNamingShapesSet`.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkcaf::tnaming::TNamingShapesSet;
/// let set = TNamingShapesSet::new();
/// assert!(set.is_empty());
/// assert_eq!(TNamingShapesSet::from_shape(&Shape::default(), None).unwrap().nb_shapes(), 0);
/// ```
#[derive(Default)]
pub struct TNamingShapesSet {
    map: MapOfShape,
}

impl TNamingShapesSet {
    /// `TNamingShapesSet()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TNamingShapesSet(S, Type)` (`None` is the default `TopAbsSHAPE`): without a type,
    /// a solid, face, edge or vertex itself, else its direct sub-shapes; with a type below
    /// the shape's, its sub-shapes of that type, else the shape itself (vro 13.09.00).
    pub fn from_shape(shape: &Shape, shape_type: Option<ShapeEnum>) -> Result<Self, Failure> {
        let mut set = Self::new();
        if shape.is_null() {
            return Ok(set);
        }
        let kind = shape.shape_type()?;
        match shape_type.filter(|&t| t != ShapeEnum::Shape) {
            None if is_unit(kind) => {
                set.add(shape);
            }
            None => {
                for child in ShapeIterator::new(shape, true, true) {
                    set.add(&child);
                }
            }
            Some(wanted) if wanted as i32 > kind as i32 => set.add_explored(shape, wanted)?,
            Some(_) => {
                set.add(shape);
            }
        }
        Ok(set)
    }
    fn add_explored(&mut self, shape: &Shape, wanted: ShapeEnum) -> Result<(), Failure> {
        let mut explorer = Explorer::new(shape, wanted, ShapeEnum::Shape)?;
        while explorer.more() {
            self.add(explorer.current());
            explorer.next()?;
        }
        Ok(())
    }
    /// `Clear`.
    pub fn clear(&mut self) {
        self.map.clear(false);
    }
    /// `Add(S)`: true when the shape was not in the set.
    pub fn add(&mut self, shape: &Shape) -> bool {
        self.map.add(shape.clone())
    }
    /// `Contains(S)`.
    pub fn contains(&self, shape: &Shape) -> bool {
        self.map.contains(shape)
    }
    /// `Remove(S)`: true when the shape was in the set.
    pub fn remove(&mut self, shape: &Shape) -> bool {
        self.map.remove(shape)
    }
    /// `Add(Shapes)`: the union.
    pub fn add_set(&mut self, shapes: &TNamingShapesSet) {
        for shape in shapes.map.iter() {
            self.map.add(shape.clone());
        }
    }
    /// `Filter(Shapes)`: the intersection.
    pub fn filter(&mut self, shapes: &TNamingShapesSet) {
        let mut to_remove = TNamingShapesSet::new();
        for shape in self.map.iter() {
            if !shapes.contains(shape) {
                to_remove.add(shape);
            }
        }
        self.remove_set(&to_remove);
    }
    /// `Remove(Shapes)`: the difference.
    pub fn remove_set(&mut self, shapes: &TNamingShapesSet) {
        for shape in shapes.map.iter() {
            self.map.remove(shape);
        }
    }
    /// `IsEmpty`.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// `NbShapes`.
    pub fn nb_shapes(&self) -> i32 {
        i32::try_from(self.map.extent()).unwrap_or(i32::MAX)
    }
    /// `ChangeMap`.
    pub fn change_map(&mut self) -> &mut MapOfShape {
        &mut self.map
    }
    /// `Map`.
    pub fn map(&self) -> &MapOfShape {
        &self.map
    }
}

/// The types the default constructor keeps whole.
fn is_unit(kind: ShapeEnum) -> bool {
    matches!(
        kind,
        ShapeEnum::Solid | ShapeEnum::Face | ShapeEnum::Edge | ShapeEnum::Vertex
    )
}

/// `TNamingIteratorOnShapesSet`: the shapes of a set in map order.
///
/// ```
/// use martensite_tkcaf::tnaming::{TNamingIteratorOnShapesSet, TNamingShapesSet};
/// let set = TNamingShapesSet::new();
/// let iterator = TNamingIteratorOnShapesSet::new(&set);
/// assert!(!iterator.more());
/// ```
#[derive(Default)]
pub struct TNamingIteratorOnShapesSet<'a> {
    cursor: Option<MapIterator<'a, Shape>>,
}

impl<'a> TNamingIteratorOnShapesSet<'a> {
    /// `TNamingIteratorOnShapesSet(S)`.
    pub fn new(set: &'a TNamingShapesSet) -> Self {
        let mut iterator = Self::default();
        iterator.init(set);
        iterator
    }
    /// `Init(S)`.
    pub fn init(&mut self, set: &'a TNamingShapesSet) {
        self.cursor = Some(set.map.cursor());
    }
    /// `More`.
    pub fn more(&self) -> bool {
        self.cursor.as_ref().is_some_and(MapIterator::more)
    }
    /// `Next`.
    #[allow(clippy::should_implement_trait)] // A More/Next iteration protocol.
    pub fn next(&mut self) {
        if let Some(cursor) = &mut self.cursor {
            cursor.next();
        }
    }
    /// `Value`: `StandardNoSuchObject` past the end.
    pub fn value(&self) -> Result<&'a Shape, Failure> {
        match &self.cursor {
            Some(cursor) => cursor.key(),
            None => Err(exceptions::no_such_object(
                "TNamingIteratorOnShapesSet::Value: the iterator is not initialized; expected Init",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbrep::topods::{Builder, Compound, Vertex};

    fn vertex() -> Shape {
        let mut vertex = Vertex::default();
        martensite_tkbrep::brep::BRepBuilder::default().make_vertex(&mut vertex);
        vertex.into_shape()
    }

    #[test]
    fn set_algebra_follows_the_native_members() {
        let (first, second, third) = (vertex(), vertex(), vertex());
        let mut set = TNamingShapesSet::new();
        assert!(set.add(&first) && set.add(&second) && !set.add(&first.reversed()));
        let mut other = TNamingShapesSet::new();
        other.add(&second);
        other.add(&third);
        let mut union = TNamingShapesSet::new();
        union.add_set(&set);
        union.add_set(&other);
        assert_eq!(union.nb_shapes(), 3);
        set.filter(&other);
        assert_eq!(set.nb_shapes(), 1);
        assert!(set.contains(&second));
        union.remove_set(&other);
        assert_eq!(union.nb_shapes(), 1);
        assert!(union.remove(&first) && union.is_empty());
    }

    #[test]
    fn the_shape_constructor_splits_containers() {
        let mut compound = Compound::default();
        Builder.make_compound(&mut compound);
        let mut compound = compound.into_shape();
        let (first, second) = (vertex(), vertex());
        Builder.add(&mut compound, &first).unwrap();
        Builder.add(&mut compound, &second).unwrap();
        let whole = TNamingShapesSet::from_shape(&first, None).unwrap();
        assert!(whole.contains(&first) && whole.nb_shapes() == 1);
        let split = TNamingShapesSet::from_shape(&compound, None).unwrap();
        assert_eq!(split.nb_shapes(), 2);
        let explored = TNamingShapesSet::from_shape(&compound, Some(ShapeEnum::Vertex)).unwrap();
        assert_eq!(explored.nb_shapes(), 2);
        let kept = TNamingShapesSet::from_shape(&first, Some(ShapeEnum::Compound)).unwrap();
        assert!(kept.contains(&first));
        let mut iterator = TNamingIteratorOnShapesSet::new(&split);
        let mut count = 0;
        while iterator.more() {
            assert!(split.contains(iterator.value().unwrap()));
            iterator.next();
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(iterator.value().is_err());
    }
}
