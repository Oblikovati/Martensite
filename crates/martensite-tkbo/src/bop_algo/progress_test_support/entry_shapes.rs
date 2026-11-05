//! Retained source identities and settings are shared by concrete receiver witnesses.
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use crate::bop_algo::{BOPAlgoBuilder, BOPAlgoPaveFiller, SectionAttribute};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, Wire},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
/// Retain shared construction handles; `entry_sources(16)` shares an edge with its wire.
pub(crate) fn entry_sources(profile: usize) -> Vec<Shape> {
    let kind = match profile {
        28 => 6,
        29 => 16,
        20.. => 4,
        other => other,
    };
    match kind {
        0 => vec![],
        1 | 2 => vec![
            entry_vertex(0.),
            entry_vertex(if kind == 2 { 0. } else { 10. }),
        ],
        11 => vec![entry_vertex(0.)],
        12 => vec![entry_edge()],
        14 => vec![entry_empty(), entry_vertex(0.)],
        16 => shared_entry_wire(),
        17 => shared_entry_vertex(),
        3 | 9 => vec![
            entry_edge(),
            BRepBuilderAPIMakeEdge::from_points(Point3::new(0., -1., 0.), Point3::new(0., 1., 0.))
                .unwrap()
                .edge()
                .unwrap()
                .into_shape(),
        ],
        6 | 7 => vec![entry_box(0), entry_box(1)],
        15 => vec![entry_edge(), entry_plane(0)],
        _ => (0..if kind == 5 {
            3
        } else if kind == 13 {
            1
        } else {
            2
        })
            .map(|i| entry_plane(if kind == 10 { 0 } else { i }))
            .collect(),
    }
}
fn entry_vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn entry_edge() -> Shape {
    BRepBuilderAPIMakeEdge::from_points(Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.))
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}
fn entry_empty() -> Shape {
    let mut shape = Compound::default();
    BRepBuilder::default().make_compound(&mut shape);
    shape.into_shape()
}
fn shared_entry_wire() -> Vec<Shape> {
    let edge = entry_edge();
    let mut wire = Wire::default();
    let builder = BRepBuilder::default();
    builder.make_wire(&mut wire);
    builder.add(&mut wire, &edge).unwrap();
    vec![wire.into_shape(), edge]
}
fn shared_entry_vertex() -> Vec<Shape> {
    let vertex = entry_vertex(0.);
    let mut compound = Compound::default();
    let builder = BRepBuilder::default();
    builder.make_compound(&mut compound);
    builder.add(&mut compound, &vertex).unwrap();
    vec![compound.into_shape(), vertex]
}
fn entry_box(index: usize) -> Shape {
    let bytes: &[u8] = if index == 0 {
        include_bytes!("../builder/entry_progress/box0.brep")
    } else {
        include_bytes!("../builder/entry_progress/box1.brep")
    };
    read_owned_progress_shape(bytes, ShapeEnum::Solid)
}
fn entry_plane(index: usize) -> Shape {
    let bytes: &[u8] = match index {
        0 => include_bytes!("../pave_filler/ranged_intersection/plane0.brep"),
        1 => include_bytes!("../pave_filler/ranged_intersection/plane1.brep"),
        2 => include_bytes!("../pave_filler/ranged_intersection/plane2.brep"),
        actual => panic!("plane {actual}; expected 0..=2"),
    };
    read_owned_progress_shape(bytes, ShapeEnum::Face)
}
/// Preserve the same option/input matrix; `configure_entry_builder(builder, sources, kind, parallel)`.
pub(crate) fn configure_entry_builder(
    builder: &mut BOPAlgoBuilder,
    sources: &[Shape],
    kind: usize,
    parallel: bool,
) {
    builder.options_mut().set_run_parallel(parallel);
    builder.set_to_fill_history(kind != 7 && kind != 30);
    builder.set_non_destructive(kind == 8);
    if kind == 9 {
        builder.options_mut().set_fuzzy_value(1e-5);
        builder.options_mut().set_use_obb(true);
    }
    if kind == 10 {
        builder.set_glue(crate::bop_algo::Glue::Full);
    }
    for source in sources {
        builder.add_argument(source);
    }
}
/// Prepare caller-owned inputs without an active range; `prepared_entry_filler(builder, 20, false)`.
pub(crate) fn prepared_entry_filler(
    builder: &BOPAlgoBuilder,
    kind: usize,
    parallel: bool,
) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(builder.arguments());
    filler.options_mut().set_run_parallel(parallel);
    let mask = if kind <= 27 { kind - 20 } else { 7 };
    filler.set_section_attribute(&SectionAttribute::with_flags(
        mask & 1 != 0,
        mask & 2 != 0,
        mask & 4 != 0,
    ));
    filler.perform();
    filler
}
