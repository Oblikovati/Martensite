//! Reference replay of the bug23380 filling tolerance regression.

use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools,
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Shape, ShapeIterator},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, standard::Failure, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkoffset::brep_offset_api::{BRepOffsetAPIMakeFilling, FillingParameters};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("23380 fixture: expected BRep header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("23380 fixture: expected readable shape");
    shape
}

fn direct_members(shape: &Shape) -> Vec<Shape> {
    ShapeIterator::new(shape, false, true).collect()
}

fn edges_of(shape: &Shape) -> Result<Vec<Edge>, Failure> {
    Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?
        .map(|edge| topods::edge(&edge))
        .collect()
}

fn count(shape: &Shape, kind: ShapeEnum) -> Result<usize, Failure> {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found)?;
    Ok(found.extent())
}

fn max_tolerance(shape: &Shape) -> Result<f64, Failure> {
    [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .into_iter()
        .map(|kind| BRepTool::max_tolerance(shape, kind))
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.into_iter().fold(0., f64::max))
}

#[test]
#[allow(non_snake_case)]
fn Filling__Bug23380() -> Result<(), Failure> {
    let shape = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug23380.brep"
    ));
    let members = direct_members(&shape);
    assert_eq!(members.len(), 5, "23380 fixture: expected five members");
    let first_face = topods::face(&members[0])?;
    let initial_face = topods::face(&members[3])?;
    let last_face = topods::face(&members[4])?;
    let first_boundary = edges_of(first_face.as_ref())?.remove(0);
    let last_boundary = edges_of(last_face.as_ref())?.remove(2);
    let second_boundary = topods::edge(&members[1])?;
    let third_boundary = topods::edge(&members[2])?;
    let result = fill(
        &initial_face,
        &first_boundary,
        &first_face,
        &last_boundary,
        &last_face,
        &second_boundary,
        &third_boundary,
    )?;
    assert_eq!(count(&result, ShapeEnum::Face)?, 1);
    assert_eq!(count(&result, ShapeEnum::Wire)?, 1);
    assert_eq!(count(&result, ShapeEnum::Edge)?, 4);
    assert_eq!(count(&result, ShapeEnum::Vertex)?, 4);
    assert!(max_tolerance(&result)? <= 230., "23380: high tolerance");
    Ok(())
}

fn fill(
    initial: &Face,
    first: &Edge,
    first_support: &Face,
    last: &Edge,
    last_support: &Face,
    second: &Edge,
    third: &Edge,
) -> Result<Shape, Failure> {
    let mut filling = BRepOffsetAPIMakeFilling::new(FillingParameters {
        nb_pts_on_cur: 10,
        nb_iter: 3,
        ..FillingParameters::default()
    });
    filling.load_init_surface(initial);
    filling.add_edge_on_face(first, first_support, Continuity::G1, true);
    filling.add_edge_on_face(last, last_support, Continuity::G1, true);
    filling.add_edge(second, Continuity::C0, true);
    filling.add_edge(third, Continuity::C0, true);
    filling.build(&ProgressRange::default())?;
    assert!(filling.is_done(), "23380: filling failed");
    Ok(filling.shape()?.clone())
}
