use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape, Vertex};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

pub fn vertex(x: f64) -> Vertex {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0.0, 0.0))
        .unwrap()
        .vertex()
        .unwrap()
}

pub fn compound(items: &[&Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for item in items {
        builder.add(&mut compound, item).unwrap();
    }
    compound.into_shape()
}
