//! Dimensions on edges: highlighted flyouts, a box edge in a local selection, collinear
//! edges, interior and exterior angles, and lengths from circular edges.
use super::dimension_params::finish_change;
use super::prelude::*;
use crate::primitive_shapes::{box_at, box_sized, cylinder};
use martensite_tkg3d::geom::{GeomCurve as _, GeomLine};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use martensite_tkv3d::prs_dim::{
    PrsDimAngleDimension, PrsDimTypeOfAngle, PrsDimTypeOfAngleArrowVisibility as Arrows,
};

const RULE: &str = "============";

/// Sub-shape `index` (1-based) of `kind`; `nth(&b, ShapeEnum::Edge, 9)`.
fn nth(shape: &Shape, kind: ShapeEnum, index: usize) -> Shape {
    crate::boolean_typed_explode::all_distinct(shape, kind)[index - 1].clone()
}

/// `bug24293`: the flyout lines belong to the highlighted dimension.
pub(crate) fn highlighted_flyout() -> Outcome {
    banner(&[
        RULE,
        "CR24293: Dimension flyout lines don't belong to the dimension sensitive entity.",
        RULE,
        "",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    display_point(&mut run, "lengthP1", [0., 0., 0.])?;
    display_point(&mut run, "lengthP2", [10., 10., 10.])?;
    let shapes = [Measured::Object("lengthP1"), Measured::Object("lengthP2")];
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Xoy),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &shapes)
    };
    display_dimension(run.session(), "dim1", &request)?;
    run.session().fit()?;
    run.session().move_cursor(202, 191)?;
    if !run.session().check_color(272, 198, [0., 1., 1.])? {
        println!("Error : Highlighting of length dimension is wrong.");
    }
    run.dump("bug24293.png")?;
    run.finish()
}

/// `bug24374`: a flipped 3D-text length of a box edge highlights in edge selection mode.
pub(crate) fn flipped_edge_highlight() -> Outcome {
    banner(&[
        RULE,
        "CR24374: Flipping affects highlight presentation of dimension",
        RULE,
        "",
    ]);
    register_sans_font()?;
    let b = box_sized([100., 100., 100.])?;
    let b9 = nth(&b, ShapeEnum::Edge, 9);
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session()
        .display("b_9", &b9, &DisplayRequest::default())?;
    let request = DimensionRequest {
        text_3d: Some(true),
        plane: Some(WorkingPlane::Zox),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &[Measured::Shape(&b9)])
    };
    display_dimension(run.session(), "dim1", &request)?;
    run.session().set_object_selection_mode("b", 2, true)?;
    run.session().set_object_selection_mode("dim1", 2, true)?;
    run.session().fit()?;
    run.session().move_cursor(130, 352)?;
    if !run.session().check_color(130, 352, [0., 1., 1.])? {
        println!("Error : Highlighting of dimension with flipping in local context failed.");
    }
    run.dump("bug24374.png")?;
    run.finish()
}

/// An edge of `line` between parameters `first` and `last` (`mkedge e line first last`).
fn line_edge(line: &Handle<GeomLine>, first: f64, last: f64) -> Result<Shape, Failure> {
    let maker = BRepBuilderAPIMakeEdge::from_curve_params(&line.clone().cast(), first, last)?;
    Ok(maker.edge()?.as_ref().clone())
}

/// `bug25549_1` (same line, other ends) and `_2` (opposite lines): no angle crash.
pub(crate) fn collinear_angle(opposite: bool, title: &str, image: &'static str) -> Outcome {
    banner(&[
        RULE,
        "CR25549",
        "Visualization - do not crash on attempt to display the Angle dimension between two parallel lines",
        title,
        RULE,
        "",
    ]);
    let line = || {
        Handle::new(GeomLine::from_point(
            Point3::new(0., 0., 0.),
            Direction3::new(1., 0., 0.).expect("unit X"),
        ))
    };
    let (first, second) = (line(), line());
    let (e1, e2) = if opposite {
        first.reverse()?;
        (line_edge(&first, 0., 50.)?, line_edge(&second, 0., 50.)?)
    } else {
        (line_edge(&first, 50., 100.)?, line_edge(&first, 50., 0.)?)
    };
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .display("e1", &e1, &DisplayRequest::default())?;
    run.session()
        .display("e2", &e2, &DisplayRequest::default())?;
    let request = DimensionRequest::new(
        DimensionKind::Angle,
        &[Measured::Shape(&e1), Measured::Shape(&e2)],
    );
    display_dimension(run.session(), "ad", &request)?;
    run.session().fit()?;
    dump_coincident(&mut run, image, "ad", &["e1", "e2"])?;
    run.finish()
}

/// Set the angle type and arrow visibility of `name`, as the angle parameter command does.
fn angle_params(
    run: &mut SessionRun,
    name: &str,
    kind: PrsDimTypeOfAngle,
    arrows: Arrows,
) -> Outcome {
    let object = run.session().object(name).cloned().unwrap_or_default();
    let angle = object.cast::<PrsDimAngleDimension>();
    let value = angle.get().ok_or("an angle dimension")?;
    value.set_type(kind);
    value.set_arrows_visibility(arrows);
    finish_change(run.session(), &angle.cast())
}

/// `bug27692`: exterior and interior angles with first, no and second arrows.
pub(crate) fn angle_arrows() -> Outcome {
    banner(&[
        "========",
        "Visualization, AisAngleDimension - exterior angle and arrows visibility improvements",
        "Test case creates three angle dimensions with exterior/interior presentation for the angle",
        "and different visual state of presentation arrows",
        "========",
    ]);
    let mut run = SessionRun::new()?;
    let triples = [
        (
            [0., 0., 0.],
            [10., 0., 0.],
            [10., 5., 0.],
            PrsDimTypeOfAngle::Exterior,
            Arrows::First,
        ),
        (
            [50., 0., 0.],
            [60., 0., 0.],
            [60., 5., 0.],
            PrsDimTypeOfAngle::Interior,
            Arrows::None,
        ),
        (
            [30., -40., 0.],
            [40., -40., 0.],
            [50., -35., 0.],
            PrsDimTypeOfAngle::Exterior,
            Arrows::Second,
        ),
    ];
    for (index, (a, b, c, kind, arrows)) in triples.into_iter().enumerate() {
        let names = [1, 2, 3].map(|k| format!("p{}", index * 3 + k));
        for (name, p) in names.iter().zip([a, b, c]) {
            display_point(&mut run, name, p)?;
        }
        let shapes = names.each_ref().map(|n| Measured::Object(n.as_str()));
        let request = DimensionRequest {
            plane: Some(WorkingPlane::Xoy),
            ..DimensionRequest::new(DimensionKind::Angle, &shapes)
        };
        let dim = format!("dim{}", index + 1);
        display_dimension(run.session(), &dim, &request)?;
        angle_params(&mut run, &dim, kind, arrows)?;
    }
    run.session().fit()?;
    run.dump("bug27692.png")?;
    run.finish()
}

/// `bug32351`: lengths from a circular cylinder edge to a box vertex and a box edge.
pub(crate) fn circular_edge_lengths() -> Outcome {
    banner(&[
        "========",
        "0032351: Visualization, PrsDimLengthDimension - add NULL checks for handling non-Line curves",
        "========",
    ]);
    let b = box_at([200., 0., 0.], [100., 200., 300.])?;
    let c = cylinder(100., 200.)?;
    let mut run = SessionRun::new()?;
    let wire = DisplayRequest::default().with_display_mode(0);
    run.session().display("b", &b, &wire)?;
    run.session().display("c", &c, &wire)?;
    run.session().fit()?;
    let (c1, b_vertex) = (nth(&c, ShapeEnum::Edge, 1), nth(&b, ShapeEnum::Vertex, 1));
    run.session()
        .display("c_1", &c1, &DisplayRequest::default())?;
    run.session()
        .display("b_1", &b_vertex, &DisplayRequest::default())?;
    let first = DimensionRequest {
        plane: Some(WorkingPlane::Zox),
        ..DimensionRequest::new(
            DimensionKind::Length,
            &[Measured::Shape(&c1), Measured::Shape(&b_vertex)],
        )
    };
    display_dimension(run.session(), "d1", &first)?;
    let b3 = nth(&b, ShapeEnum::Edge, 3);
    run.session()
        .display("c_1", &c1, &DisplayRequest::default())?;
    run.session()
        .display("b_3", &b3, &DisplayRequest::default())?;
    let second = DimensionRequest::new(
        DimensionKind::Length,
        &[Measured::Shape(&c1), Measured::Shape(&b3)],
    );
    display_dimension(run.session(), "d2", &second)?;
    run.session().fit()?;
    run.dump("bug32351.png")?;
    run.finish()
}
