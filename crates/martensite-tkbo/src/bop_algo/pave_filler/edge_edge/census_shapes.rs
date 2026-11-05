//! Same literal edge arguments as the protected Native EE census, without importing its engine.
use super::super::BOPAlgoPaveFiller;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, Vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

/// Prepare one actual receiver before attaching progress; `configured_census("cross")`.
pub(super) fn configured_census(profile: &str) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for argument in census_arguments(profile) {
        filler.add_argument(&argument);
    }
    filler.options_mut().set_run_parallel(false);
    filler.options_mut().set_fuzzy_value(1.0e-6);
    filler.initialize().unwrap();
    if profile == "flagged" {
        flag_source_edges(&mut filler);
    }
    filler
}

fn census_arguments(profile: &str) -> Vec<Shape> {
    if profile == "shared" {
        return shared_arguments();
    }
    let offset = if profile == "far" { 1.0e9 } else { 0.0 };
    let x = offset + if profile == "none" { 10.0 } else { 0.0 };
    let y = if profile == "endpoint" { 0.0 } else { -1.0 };
    let mut arguments = vec![
        segment((offset - 1.0, 0.0), (offset + 1.0, 0.0)),
        segment((x, y), (x, 1.0)),
    ];
    if profile == "triple" {
        arguments.push(segment((offset - 1.0, -1.0), (offset + 1.0, 1.0)));
    }
    arguments
}

fn segment(first: (f64, f64), last: (f64, f64)) -> Shape {
    let edge = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(first.0, first.1, 0.0),
        Point3::new(last.0, last.1, 0.0),
    )
    .unwrap()
    .edge()
    .unwrap();
    argument(edge.into_shape())
}

fn argument(edge: Shape) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut shape = compound.into_shape();
    builder.add(&mut shape, &edge).unwrap();
    shape
}

fn shared_arguments() -> Vec<Shape> {
    let builder = BRepBuilder::default();
    let first = census_vertex(&builder, -1.0);
    let last = census_vertex(&builder, 1.0);
    (0..2)
        .map(|_| {
            let edge = BRepBuilderAPIMakeEdge::from_vertices(&first, &last)
                .unwrap()
                .edge()
                .unwrap();
            argument(edge.into_shape())
        })
        .collect()
}

fn census_vertex(builder: &BRepBuilder, x: f64) -> Vertex {
    let mut vertex = Vertex::default();
    builder
        .make_vertex_at(&mut vertex, Point3::new(x, 0.0, 0.0), 1.0e-7)
        .unwrap();
    vertex
}

fn flag_source_edges(filler: &mut BOPAlgoPaveFiller) {
    let structure = filler.data_structure_mut().unwrap();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() == ShapeEnum::Edge {
            structure.change_shape_info(index).unwrap().set_flag(1);
        }
    }
}
