//! A 2D drawing: a snowflake built from planar booleans, a title block with text and a dimension,
//! seen from the top on white.
use super::ENGRAVED_WORD;
use crate::commands::{explode, unify_same_domain};
use crate::geometry::{
    bounded_extent, circle, compound_of, copied, edge_of, face_on, line, mirrored, plane, polyline,
    prism, reversed, rotated, scaled, shell_of, trim, whole_face, wire_of,
};
use crate::grading::{DEMO, run_alone};
use crate::modelling::{
    MONO_OBLIQUE, SANS_BOLD_OBLIQUE, SERIF, TextRequest, distance_between, text_shape,
};
use crate::scene::{LengthDimensionRequest, ViewScene};
use crate::support::one_step::{bcut, bfuse, bsection};
use crate::support::rules::Verdict;
use crate::variables::Variables;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkservice::graphic3d::Graphic3dVerticalTextAlignment;
use std::f64::consts::FRAC_PI_4;

const CENTRE: [f64; 3] = [-6., 0., 0.];
const Z_AXIS: [f64; 3] = [0., 0., 1.];

/// A trimmed circle of the ray, from the first angle to the second.
fn arc_edge(at: [f64; 3], radius: f64) -> Shape {
    edge_of(&trim(&circle(&at, radius), FRAC_PI_4, -FRAC_PI_4))
}

/// A trimmed line of the ray, between two distances from its point.
fn spoke_edge(at: [f64; 3], along: [f64; 3], first: f64, last: f64) -> Shape {
    edge_of(&trim(&line(&at, &along), first, last))
}

/// The ring between two arcs and two spokes.
fn arc_ring(at: [f64; 3], outer: f64, inner: f64) -> Shape {
    wire_of(&[
        arc_edge(at, outer),
        spoke_edge(at, [1., 1., 0.], inner, outer),
        arc_edge(at, inner),
        spoke_edge(at, [1., -1., 0.], inner, outer),
    ])
}

/// The three circular elements, the bar and its cut: half a ray's parts fused.
fn half_ray(variables: &mut Variables) {
    let support = plane(&[-6., 0., 0., 0., 0., 1.]);
    let outer = wire_of(&[edge_of(&circle(&[5., 5., 0.], 5.))]);
    let inner = reversed(&wire_of(&[edge_of(&circle(&[5., 5., 0.], 3.))]));
    let mut f1 = face_on(&support, &outer);
    BRepBuilder::default().add(&mut f1, &inner).unwrap();
    let f2 = face_on(&support, &arc_ring([18., 7., 0.], 7., 5.));
    let f3 = face_on(&support, &arc_ring([28.5, 5., 0.], 5., 3.));
    let l1 = trim(
        &line(&[-6., 0., 0.], &[0.86602540378443864, 0.5, 0.]),
        0.,
        30.,
    );
    let l2 = trim(&line(&[-6., 1., 0.], &[1., 0., 0.]), 0., 45.);
    variables.set("f5", prism(&edge_of(&l1), [-5., 8.6602540378443864, 0.]));
    variables.set("f4", prism(&edge_of(&l2), [0., -1., 0.]));
    variables.set("bc", compound_of(&[f1, f2, f3]));
    bfuse(variables, "r", "bc", "f4");
    bcut(variables, "r", "r", "f5");
}

/// One ray mirrored and six rays turned and fused, the faces then unified as one shell.
fn snowflake_contours(variables: &mut Variables) {
    half_ray(variables);
    variables.set(
        "r1",
        mirrored(&copied(&variables.get("r")), CENTRE, [0., 1., 0.]),
    );
    bfuse(variables, "w", "r", "r1");
    let ray = unify_same_domain(&variables.get("w"));
    variables.set("w", ray.clone());
    for step in 1..=5 {
        let turned = rotated(&copied(&ray), CENTRE, Z_AXIS, 60. * f64::from(step));
        variables.set("turned", turned);
        bfuse(variables, "w", "w", "turned");
    }
    let count = explode(variables, "w", ShapeEnum::Face);
    let faces: Vec<Shape> = (1..=count)
        .map(|i| variables.get(&format!("w_{i}")))
        .collect();
    variables.set("r", unify_same_domain(&shell_of(&faces)));
    let wires = wires_of(variables, "r");
    variables.set("snowflake", scaled(&compound_of(&wires), CENTRE, 1.5));
}

/// `eval compound [explode name w] name`: the shape's wires.
fn wires_of(variables: &mut Variables, name: &str) -> Vec<Shape> {
    let count = explode(variables, name, ShapeEnum::Wire);
    (1..=count)
        .map(|i| variables.get(&format!("{name}_{i}")))
        .collect()
}

/// The frame and the title block's lines.
fn frame_lines() -> Shape {
    let pieces: [&[[f64; 3]]; 10] = [
        &[
            [-100., -100., 0.],
            [172., -100., 0.],
            [172., 100., 0.],
            [-100., 100., 0.],
            [-100., -100., 0.],
        ],
        &[[52., -100., 0.], [52., -45., 0.], [172., -45., 0.]],
        &[[52., -60., 0.], [172., -60., 0.]],
        &[[52., -85., 0.], [172., -85., 0.]],
        &[[122., -100., 0.], [122., -60., 0.]],
        &[[122., -80., 0.], [172., -80., 0.]],
        &[[122., -65., 0.], [172., -65., 0.]],
        &[[142., -80., 0.], [142., -85., 0.]],
        &[[137., -80., 0.], [137., -60., 0.]],
        &[[154., -80., 0.], [154., -60., 0.]],
    ];
    let shapes: Vec<Shape> = pieces.iter().map(|points| polyline(points)).collect();
    compound_of(&shapes)
}

/// A title block entry: the text's wires as a compound.
fn text_block(
    variables: &mut Variables,
    name: &str,
    text: &str,
    request: &TextRequest<'_>,
) -> Shape {
    variables.set(name, text_shape(text, request));
    compound_of(&wires_of(variables, name))
}

fn title_block(variables: &mut Variables) -> Shape {
    let at = |x: f64, y: f64| [x, y, 0.];
    let courier = |height: f64, position: [f64; 3]| TextRequest {
        font: MONO_OBLIQUE,
        height,
        position,
        ..TextRequest::default()
    };
    let mut column = courier(3.5, at(157., -63.));
    column.vertical = Graphic3dVerticalTextAlignment::Topfirstline;
    let mut mass = column;
    mass.position = at(140., -63.);
    let entries = [
        (
            "sample",
            "SAMPLE",
            TextRequest {
                font: SANS_BOLD_OBLIQUE,
                height: 10.,
                position: at(90., -55.),
                ..TextRequest::default()
            },
        ),
        (
            "word",
            ENGRAVED_WORD,
            TextRequest {
                font: SERIF,
                height: 6.,
                position: at(125., -95.),
                ..TextRequest::default()
            },
        ),
        ("name", "Snowflake", courier(7., at(65., -75.))),
        ("material", "Ice", courier(7., at(75., -95.))),
        ("sheets", "Sheets 1", courier(3.5, at(145., -83.))),
        ("scale", "Scale\n\n1:100", column),
        ("mass", "Mass\n\n1 mg", mass),
    ];
    let parts: Vec<Shape> = entries
        .iter()
        .map(|(name, text, request)| text_block(variables, name, text, request))
        .collect();
    compound_of(&parts)
}

/// A vertex of the section of the snowflake with `x = value`: of its two, the one nearer `fup`.
fn upper_section_vertex(variables: &mut Variables, name: &str, x: f64, fup: &Shape) -> Shape {
    variables.set("cross", whole_face(&plane(&[x, 0., 0., 1., 0., 0.])));
    bsection(variables, name, "snowflake", "cross");
    explode(variables, name, ShapeEnum::Vertex);
    let (mut nearest, mut smallest) = (Shape::default(), 1e10);
    for index in 1..=2 {
        let candidate = variables.get(&format!("{name}_{index}"));
        let distance = distance_between(&candidate, fup);
        if smallest > distance {
            smallest = distance;
            nearest = candidate;
        }
    }
    nearest
}

/// The drawing's three objects, black on white, seen from the top and fitted.
fn drawing_scene(variables: &Variables, lines: &Shape, text: &Shape) -> ViewScene {
    let mut scene = ViewScene::open_sized("snowflake", (1024, 768));
    scene.display("snowflake", &variables.get("snowflake"));
    scene.display("lines", lines);
    scene.display("text", text);
    scene.set_sample_count(8);
    for name in ["snowflake", "lines", "text"] {
        scene.set_color(name, NamedColor::BLACK);
    }
    scene.set_background(Color::named(NamedColor::WHITE));
    scene.top();
    scene.fit();
    scene
}

/// The dimension between the two section vertices nearest the top of the snowflake's box.
fn add_width_dimension(
    scene: &mut ViewScene,
    variables: &mut Variables,
    (min, max): ([f64; 3], [f64; 3]),
) {
    let fup = whole_face(&plane(&[0., max[1], 0., 0., 1., 0.]));
    let first = upper_section_vertex(variables, "s1", min[0], &fup);
    let second = upper_section_vertex(variables, "s2", max[0], &fup);
    let request = LengthDimensionRequest {
        value: 0.001,
        units: "mm",
        flyout: 70.,
        color: NamedColor::BLACK,
        text_height: 5.,
    };
    scene.display_length_dimension("length", (&first, &second), &request);
}

#[test]
fn snowflake() {
    run_alone(
        "cases::sample_snowflake::snowflake",
        &DEMO,
        Verdict::Ok,
        || {
            println!("Generating sample drawing of snowflake...");
            let mut variables = Variables::default();
            snowflake_contours(&mut variables);
            let lines = frame_lines();
            let text = title_block(&mut variables);
            // Before the display, whose triangulation the box would read.
            let extent = bounded_extent(&variables.get("snowflake"));
            let mut scene = drawing_scene(&variables, &lines, &text);
            add_width_dimension(&mut scene, &mut variables, extent);
            assert!(scene.dump([1., 1., 1.]) > 0, "the view shows nothing");
            scene.close();
            println!("TEST COMPLETED");
        },
    );
}
