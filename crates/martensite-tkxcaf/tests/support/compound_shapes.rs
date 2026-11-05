//! Compound construction shared by document fixtures.
use martensite_tkbrep::topods::{Builder, Compound, Shape};

pub(crate) fn compound_of(parts: &[Shape]) -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for part in parts {
        Builder.add(&mut compound, part).unwrap();
    }
    compound.into_shape()
}
