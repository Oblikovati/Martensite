//! Authored analytic arguments are owned by each callback test invocation.
use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, Glue, PISteps};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::{Array1, List};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};

pub(super) const GLUES: [Glue; 3] = [Glue::Off, Glue::Shift, Glue::Full];
pub(super) const BOUNDS: [(i32, i32); 8] = [
    (0, 13),
    (0, 16),
    (1, 3),
    (8, 10),
    (14, 16),
    (-2, 1),
    (0, 0),
    (5, 5),
];

pub(super) fn prepared(
    recipe: usize,
    glue: Glue,
    avoid: bool,
    parallel: bool,
) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments(recipe));
    filler.set_glue(glue);
    filler.set_avoid_build_pcurve(avoid);
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    assert!(!filler.options().has_errors());
    filler
}

fn arguments(recipe: usize) -> List<Shape> {
    match recipe {
        0 | 1 => [vertex(0.), vertex(if recipe == 0 { 10. } else { 0. })]
            .into_iter()
            .collect(),
        2 => [edge((-1., 0.), (1., 0.)), edge((0., -1.), (0., 1.))]
            .into_iter()
            .collect(),
        3 | 4 => boxes(recipe == 4),
        5 => [face(0., 2.), face(1., 3.)].into_iter().collect(),
        _ => panic!("raw weight recipe={recipe}; expected integer in 0..6"),
    }
}

fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

fn edge(first: (f64, f64), last: (f64, f64)) -> Shape {
    BRepBuilderAPIMakeEdge::from_points(
        Point3::new(first.0, first.1, 0.),
        Point3::new(last.0, last.1, 0.),
    )
    .unwrap()
    .edge()
    .unwrap()
    .into_shape()
}

fn boxes(diagonal: bool) -> List<Shape> {
    let coordinate = if diagonal { 0.5 } else { 0. };
    let mut first = BRepPrimAPIMakeBox::at_point(Point3::default(), 1., 1., 1.).unwrap();
    let mut second =
        BRepPrimAPIMakeBox::at_point(Point3::new(0.5, coordinate, coordinate), 1., 1., 1.).unwrap();
    [
        first.shape().unwrap().clone(),
        second.shape().unwrap().clone(),
    ]
    .into_iter()
    .collect()
}

fn face(first_u: f64, last_u: f64) -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (first_u, last_u, 0., 2.))
        .unwrap()
        .face()
        .unwrap()
        .clone()
        .into_shape()
}

pub(super) fn seeded_steps(lower: i32, upper: i32) -> PISteps {
    let mut steps = PISteps::new(1).unwrap();
    *steps.change_steps() = Array1::with_bounds(lower, upper).unwrap();
    steps.change_steps().init(-7.25);
    steps
}

pub(super) fn step_bits(steps: &PISteps) -> Vec<u64> {
    steps
        .steps()
        .as_slice()
        .iter()
        .map(|value| value.to_bits())
        .collect()
}
