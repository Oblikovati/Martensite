//! Authored rectangles, default Courier text, sphere and each vertex compound.
use super::*;
use crate::primitive_shapes::{box_at, compound_of, sphere, translated};
#[path = "../support/brep_text_shape.rs"]
mod brep_text_shape;
use brep_text_shape::{BRepText, text_shape};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_builder_api::BRepBuilderAPIMakeVertex};

pub(super) fn source_shapes() -> Result<Vec<(String, Shape)>, Box<dyn Error>> {
    let mut shapes = Vec::new();
    for (name, corner, size) in [
        ("b4", [-175., -125., -5.], [350., 250., 1.]),
        ("b3", [-150., -100., -4.], [300., 200., 1.]),
        ("b2", [-125., -75., -3.], [250., 150., 1.]),
        ("b1", [-100., -50., -2.], [200., 100., 1.]),
    ] {
        shapes.push((name.into(), box_at(corner, size)?));
    }
    shapes.push(("t".into(), centered_source_text()?));
    shapes.push(("s".into(), sphere(50.)?));
    for (index, origin) in MARK_ORIGINS.into_iter().enumerate() {
        shapes.push((format!("p{}", index + 1), clock_marks(index + 1, origin)?));
    }
    Ok(shapes)
}

fn centered_source_text() -> Result<Shape, Box<dyn Error>> {
    let shape = text_shape(
        "texT | Text\ntexT | Text",
        &BRepText {
            height: 50.,
            ..Default::default()
        },
    )?;
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&shape, &mut bounds, true)?;
    let values = bounds.get()?;
    // Authored aTX/aTY are maxima, not widths or the bounds' midpoint.
    Ok(translated(
        &shape,
        [-0.5 * values[3], -0.5 * values[4], 1.],
    )?)
}

const MARK_ORIGINS: [[f64; 2]; 12] = [
    [75., 86.],
    [130., 86.],
    [130., 0.],
    [120., -85.],
    [55., -85.],
    [-18., -85.],
    [-85., -85.],
    [-160., -85.],
    [-160., 0.],
    [-165., 86.],
    [-100., 86.],
    [-30., 86.],
];

fn clock_marks(count: usize, [x, y]: [f64; 2]) -> Result<Shape, Box<dyn Error>> {
    let mut vertices = Vec::new();
    for index in 1..=count {
        let point = Point3::new(x + 5. * index as f64, y, 1.);
        vertices.push(BRepBuilderAPIMakeVertex::new(point)?.vertex()?.into_shape());
    }
    Ok(compound_of(&vertices)?)
}
