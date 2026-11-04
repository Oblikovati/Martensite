//! Compounds of mixed sub-shapes: a vertex, an edge, a spherical patch and a box shaded with
//! one material, nested compounds of boxes coloured per sub-compound, and faces of a solid
//! regrouped into compounds.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::SubShapeStyle;
use crate::dataset_shape::read_dataset_shape;
use crate::primitive_shapes::compound_of;
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkernel::precision;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace,
    BRepBuilderAPIMakeShape as _, BRepBuilderAPIMakeVertex,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// A new vertex; `vertex_at([0., 1., 2.])?`.
fn vertex_at([x, y, z]: [f64; 3]) -> Result<Shape, Failure> {
    Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?
        .vertex()?
        .as_ref()
        .clone())
}

/// An edge between two new vertices; `edge_between([0.; 3], [1., 0., 0.])?`.
fn edge_between([a, b, c]: [f64; 3], [d, e, f]: [f64; 3]) -> Result<Shape, Failure> {
    let start = BRepBuilderAPIMakeVertex::new(Point3::new(a, b, c))?.vertex()?;
    let end = BRepBuilderAPIMakeVertex::new(Point3::new(d, e, f))?.vertex()?;
    let maker = BRepBuilderAPIMakeEdge::from_vertices(&start, &end)?;
    Ok(maker.edge()?.as_ref().clone())
}

/// A sphere of radius 150 at (-80, 0, 0), then a face on it: the patch of the sphere
/// between u 0.1..0.7 and v 0.2..0.9.
fn spherical_patch() -> Result<Shape, Failure> {
    let centre = Axis2::new(Point3::new(-80., 0., 0.), Direction3::new(0., 0., 1.)?)?;
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(Axis3::from_axis2(centre), 150.)?).cast();
    let bounds = (0.1, 0.7, 0.2, 0.9);
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, precision::confusion())?;
    Ok(maker.shape()?.clone())
}

/// `bug2883_1`: a JADE material set on a compound of a vertex, an edge, a spherical patch and
/// a box shades every face of it.
pub(crate) fn compound_material() -> Result<(), Box<dyn Error>> {
    banner(&[
        "==========",
        "OCC2883: It is impossible to set material, color and transparency to compound",
        "==========",
        "",
    ]);
    let parts = [
        vertex_at([-20., 10., -30.])?,
        edge_between([-10., 10., 10.], [0., 10., 10.])?,
        spherical_patch()?,
        box_at([-60., 0., 0.], [30., 60., 40.])?,
    ];
    let c = compound_of(&parts)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "c", &c)?;
    run.session().fit()?;
    run.session()
        .set_material("c", Graphic3dNameOfMaterial::Jade)?;
    run.dump_and_finish("bug2883_1.png", false)
}

/// The colours of the three boxes of each row, and the pixels of each colour checked after the
/// nested compound is coloured.
const ROW_COLORS: [NamedColor; 3] = [NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE];
const COLOR_PROBES: [(&str, [(i32, i32); 3]); 3] = [
    ("RED2", [(50, 250), (175, 175), (300, 100)]),
    ("GREEN2", [(100, 310), (200, 230), (320, 170)]),
    ("BLUE2", [(130, 280), (250, 200), (350, 150)]),
];

/// Rows `b<j>_<i>` of `bug28036_2`: `tcopy b<i> b<j>_<i>`, translated along y by -300, 0 and 300.
fn box_rows() -> Result<[[Shape; 3]; 3], Box<dyn Error>> {
    let boxes = [
        box_at([0., 0., 0.], [100., 200., 1.])?,
        box_at([150., 0., 0.], [100., 50., 1.])?,
        box_at([150., 50., 0.], [100., 150., 1.])?,
    ];
    let copy = |b: &Shape, dy: f64| -> Result<Shape, Failure> {
        let copied = BRepBuilderAPICopy::with_shape(b, true, false)?
            .shape()?
            .clone();
        translated(&copied, [0., dy, 0.])
    };
    let row = |dy: f64| -> Result<[Shape; 3], Failure> {
        Ok([
            copy(&boxes[0], dy)?,
            copy(&boxes[1], dy)?,
            copy(&boxes[2], dy)?,
        ])
    };
    Ok([row(-300.)?, row(0.)?, row(300.)?])
}

/// The reference image: the nine boxes displayed separately, coloured red, green and blue.
fn dump_separate_boxes(run: &mut SessionRun, rows: &[[Shape; 3]; 3]) -> Result<(), Box<dyn Error>> {
    for (j, row) in rows.iter().enumerate() {
        for (i, (shape, color)) in row.iter().zip(ROW_COLORS).enumerate() {
            let name = format!("b{}_{}", j + 1, i + 1);
            show(run, &name, shape)?;
            run.session().set_color(&name, Color::named(color))?;
        }
    }
    run.session().fit()?;
    run.dump("bug28036_2_ref.png")?;
    Ok(())
}

/// Colour `part` of the displayed `b123_123`.
fn color_part(run: &mut SessionRun, part: &Shape, color: NamedColor) -> Result<(), Box<dyn Error>> {
    let style = SubShapeStyle {
        color: Some(Color::named(color)),
        width: None,
    };
    run.session().style_sub_shape("b123_123", part, &style)?;
    Ok(())
}

/// `bug28036_2`: colours set on nested sub-compounds of one shaded compound show as the
/// colours of the separately displayed boxes.
pub(crate) fn nested_compound_colors_shaded() -> Result<(), Box<dyn Error>> {
    banner(&["===========", "OCC28036"]);
    banner(&[
        "Visualization, AisColoredShape - handle correctly nested compounds within Shaded display mode",
        "===========",
        "",
    ]);
    let rows = box_rows()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    run.session().set_axonometric()?;
    dump_separate_boxes(&mut run, &rows)?;
    let pairs = rows
        .iter()
        .map(|[_, b, c]| compound_of(&[b.clone(), c.clone()]));
    let pairs: Vec<Shape> = pairs.collect::<Result<_, _>>()?;
    let triples = rows
        .iter()
        .zip(&pairs)
        .map(|([a, ..], bc)| compound_of(&[a.clone(), bc.clone()]));
    let triples: Vec<Shape> = triples.collect::<Result<_, _>>()?;
    let all = compound_of(&triples)?;
    run.session().clear()?;
    show(&mut run, "b123_123", &all)?;
    color_part(&mut run, &compound_of(&pairs)?, NamedColor::GREEN)?;
    for triple in &triples {
        color_part(&mut run, triple, NamedColor::RED)?;
    }
    color_part(&mut run, &rows[0][2], NamedColor::BLUE)?;
    let blue = compound_of(&[rows[1][2].clone(), rows[2][2].clone()])?;
    color_part(&mut run, &blue, NamedColor::BLUE)?;
    for (name, points) in COLOR_PROBES {
        for (x, y) in points {
            if run.session().pixel_color_name(x, y)? != name {
                println!("Error: wrong color");
            }
        }
    }
    run.dump_and_finish("bug28036_2.png", false)
}

/// `bug23191`: the second solid of the small `bug23191_irons.brep` and two compounds of its
/// faces, each shifted along x, shaded from the bottom with one compound red, without artifacts.
pub(crate) fn regrouped_faces_from_bottom() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===========",
        "OCC23191: Visualization - small objects displayed with visual artifacts",
        "===========",
        "",
    ]);
    let irons = read_dataset_shape("bug23191_irons.brep")?;
    let children: Vec<Shape> = ShapeIterator::new(&irons, true, true).collect();
    let second = children
        .get(1)
        .ok_or("explode i: expected a second sub-shape i_2")?;
    let faces = all_distinct(second, ShapeEnum::Face);
    if faces.len() < 5 {
        return Err(format!("explode i_2 F: {} faces; expected at least 5", faces.len()).into());
    }
    let first = compound_of(&[faces[0].clone(), faces[3].clone(), faces[4].clone()])?;
    let other = compound_of(&[faces[1].clone(), faces[2].clone()])?;
    let named = [
        ("i_2", translated(second, [0.003, 0., 0.])?),
        ("i2", translated(&first, [0.006, 0., 0.])?),
        ("i3", translated(&other, [0.006, 0., 0.])?),
    ];
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    for (name, shape) in &named {
        show(&mut run, name, shape)?;
    }
    run.session()
        .set_color("i3", Color::named(NamedColor::RED))?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_BOTTOM)?;
    run.session().fit()?;
    run.dump_and_finish("bug23191.png", false)
}
