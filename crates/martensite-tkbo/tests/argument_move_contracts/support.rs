use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::ncollection::{BaseAllocator, List};
pub fn compound_shape() -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    compound.into_shape()
}
pub fn two_shapes(resource: &BaseAllocator) -> List<Shape> {
    let mut shapes = List::with_allocator(resource.clone());
    shapes.append(compound_shape());
    shapes.append(compound_shape());
    shapes
}

pub fn assert_shape_order(actual: &List<Shape>, expected: &[Shape]) {
    assert_eq!(actual.len(), expected.len());
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| actual.is_same(expected))
    );
}
