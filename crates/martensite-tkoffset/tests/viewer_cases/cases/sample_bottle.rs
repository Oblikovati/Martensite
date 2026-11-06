//! The tutorial bottle: a filleted prism with a neck, hollowed, with its threading lofted between
//! two cylinders and a word cut into the body.
use super::ENGRAVED_WORD;
use crate::commands::{check_shape, explode};
use crate::geometry::{
    Curve2d, cylinder, edge_of, edge_of_vertices, edge_on_surface_range, ellipse_2d, line_2d,
    mirrored, plane_face, prism, rotated, translated, trim_2d, vertex, wire_of,
};
use crate::grading::{DEMO, run_alone};
use crate::modelling::{
    TextRequest, blended, build_3d_curves, hollowed, loft, text_shape, two_step_fuse,
};
use crate::scene::{SHADED, ViewScene};
use crate::support::one_step::{bcut, bfuse};
use crate::support::rules::Verdict;
use crate::variables::Variables;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gc::MakeArcOfCircle;
use martensite_tkgeombase::geom_lib::Tool;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis as _};
use std::f64::consts::PI;

const HEIGHT: f64 = 70.;
const WIDTH: f64 = 50.;
const THICKNESS: f64 = 30.;

/// The half profile and its mirror image, closed into one wire.
fn body_profile() -> Shape {
    let (half, quarter) = (WIDTH / 2., THICKNESS / 4.);
    let first_side = edge_of_vertices(&vertex(-half, 0., 0.), &vertex(-half, -quarter, 0.));
    let arc = MakeArcOfCircle::through_points(
        Point3::new(-half, -quarter, 0.),
        Point3::new(0., -THICKNESS / 2., 0.),
        Point3::new(half, -quarter, 0.),
    )
    .unwrap()
    .value()
    .unwrap();
    let last_side = edge_of_vertices(&vertex(half, -quarter, 0.), &vertex(half, 0., 0.));
    let half_wire = wire_of(&[first_side, edge_of(&arc.cast()), last_side]);
    let other = mirrored(&half_wire, [0., 0., 0.], [0., 1., 0.]);
    wire_of(&[half_wire, other])
}

/// The prism, its 18 edges filleted, the neck fused on and the inside hollowed through its top.
fn hollow_body(variables: &mut Variables) {
    println!("Constructing bottle body...");
    variables.set("p", prism(&plane_face(&body_profile()), [0., 0., HEIGHT]));
    let count = explode(variables, "p", ShapeEnum::Edge);
    let edges: Vec<Shape> = (1..=count)
        .map(|i| variables.get(&format!("p_{i}")))
        .collect();
    let body = blended(&variables.get("p"), THICKNESS / 12., &edges);
    variables.set("b", body);
    let (neck_radius, neck_height) = (THICKNESS / 4., HEIGHT / 10.);
    let mut neck = BRepPrimAPIMakeCylinder::new(neck_radius, neck_height);
    let neck = neck.solid().unwrap().into_shape();
    variables.set("c", translated(&neck, [0., 0., HEIGHT]));
    bfuse(variables, "f", "b", "c");
    explode(variables, "c", ShapeEnum::Face);
    let hollow = hollowed(
        &variables.get("f"),
        -THICKNESS / 50.,
        1.0e-3,
        &variables.get("c_2"),
    );
    variables.set("body", hollow);
}

/// The parametric line of the threading between the ellipse ends, trimmed where the end lies.
fn closing_line(ellipse: &Curve2d) -> (Curve2d, f64) {
    let start = ellipse.value(0.).unwrap();
    let end = ellipse.value(PI).unwrap();
    let (along_x, along_y) = (end.x() - start.x(), end.y() - start.y());
    let line = line_2d(start.x(), start.y(), along_x, along_y);
    let found = Tool::parameter_2d(&line, Point2::new(end.x(), end.y()), 1.0e-9).unwrap();
    let last = found.expect("the end of the ellipse on its closing line");
    (trim_2d(&line, 0., last), last)
}

/// The threading: two parametric contours on two cylinders, lofted into a solid.
fn threading(variables: &mut Variables) {
    println!("Constructing threading...");
    let neck_radius = THICKNESS / 4.;
    let neck_height = HEIGHT / 10.;
    let inner = cylinder(&[0., 0., HEIGHT, 0., 0., 1.], neck_radius * 0.99);
    let outer = cylinder(&[0., 0., HEIGHT, 0., 0., 1.], neck_radius * 1.05);
    let (major, minor) = (2. * PI, neck_height / 10.);
    let centre = (2. * PI, neck_height / 2.);
    let along = (2. * PI, neck_height / 4.);
    let ellipse_1 = ellipse_2d(centre.0, centre.1, along, major, minor);
    let ellipse_2 = ellipse_2d(centre.0, centre.1, along, major, minor / 4.);
    let (arc_1, arc_2) = (trim_2d(&ellipse_1, 0., PI), trim_2d(&ellipse_2, 0., PI));
    let (segment, last) = closing_line(&ellipse_1);
    let wire_1 = wire_of(&[
        edge_on_surface_range(&arc_1, &inner, 0., PI),
        edge_on_surface_range(&segment, &inner, 0., last),
    ]);
    let wire_2 = wire_of(&[
        edge_on_surface_range(&arc_2, &outer, 0., PI),
        edge_on_surface_range(&segment, &outer, 0., last),
    ]);
    build_3d_curves(&wire_1, 1.0e-5);
    build_3d_curves(&wire_2, 1.0e-5);
    variables.set("thread", loft(&[wire_1, wire_2], true, false, false));
}

/// The word, extruded, turned and placed on the body's side, then cut from the body.
fn engrave(variables: &mut Variables) {
    println!("Putting together and writing the word...");
    let request = TextRequest {
        font: crate::modelling::SERIF_BOLD,
        height: 8.,
        ..TextRequest::default()
    };
    let text = prism(&text_shape(ENGRAVED_WORD, &request), [0., 0., 2.]);
    let text = rotated(&text, [0., 0., 0.], [0., 1., 0.], 90.);
    variables.set("text", translated(&text, [24.75, -2., 65.]));
    bcut(variables, "bodytext", "body", "text");
}

#[test]
fn bottle() {
    run_alone("cases::sample_bottle::bottle", &DEMO, Verdict::Ok, || {
        let mut variables = Variables::default();
        hollow_body(&mut variables);
        threading(&mut variables);
        engrave(&mut variables);
        two_step_fuse(&mut variables, "bottle", "bodytext", "thread");
        println!("Showing result...");
        let mut scene = ViewScene::open("bottle");
        scene.display("bottle", &variables.get("bottle"));
        scene.fit();
        scene.set_default_display_mode(SHADED);
        scene.show_iso_on_triangulation();
        check_shape(&variables, "bottle");
        assert!(scene.dump([0., 0., 0.]) > 0, "the view shows nothing");
        scene.close();
        println!("TEST COMPLETED");
    });
}
