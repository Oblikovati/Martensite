//! The original two-box assembly input shared by isolated operation tests.
use super::{compound_shapes::compound_of, make_box};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::{gp::Transform3, toploc::Location};

pub(crate) fn two_boxes() -> (Shape, Shape) {
    let moved = make_box(1., 1., 1.)
        .moved(&Location::new(Transform3::translation([5., 0., 0.])), true)
        .unwrap();
    let compound = compound_of(&[make_box(1., 1., 1.), moved.clone()]);
    (compound, moved)
}
