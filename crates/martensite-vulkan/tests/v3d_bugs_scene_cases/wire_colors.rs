//! Colours of free edges and compounds in shading mode, and depth fitting of a zoom-persistent
//! box.
use super::prelude::*;
use crate::primitive_shapes::compound_of;
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};

/// `vertex name x y z`; `vertex([20., 0., 0.])?`.
fn vertex([x, y, z]: [f64; 3]) -> Result<Vertex, Box<dyn Error>> {
    Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()?)
}

/// `edge name v1 v2`: the straight edge between two existing vertices.
fn edge(from: &Vertex, to: &Vertex) -> Result<Shape, Box<dyn Error>> {
    let edge = BRepBuilderAPIMakeEdge::from_vertices(from, to)?.edge()?;
    Ok(edge.as_ref().clone())
}

/// `bug5990`: a box, a free edge and a compound of edges all turn blue in shading mode.
pub(crate) fn wire_colors_in_shading() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC5990: SetColor does not change color of wireframe entities in shading mode",
    ]);
    banner(&["================", ""]);
    let b1 = box_sized([10., 10., 10.])?;
    let [v1, v2, v3, v4, v5] = [
        [20., 0., 0.],
        [20., 0., 10.],
        [30., 0., 0.],
        [30., 0., 10.],
        [30., 10., 10.],
    ]
    .map(vertex);
    let e1 = edge(&v1?, &v2?)?;
    let v4 = v4?;
    let c1 = compound_of(&[edge(&v3?, &v4)?, edge(&v4, &v5?)?])?;
    let mut run = SessionRun::new()?;
    for (name, shape) in [("b1", &b1), ("e1", &e1), ("c1", &c1)] {
        show(&mut run, name, shape)?;
    }
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    for name in ["b1", "e1", "c1"] {
        run.session()
            .set_color(name, Color::named(NamedColor::BLUE))?;
    }
    run.session().check_color(162, 159, [0., 0., 0.83])?;
    run.session().check_color(204, 269, [0., 0., 1.])?;
    run.session().check_color(348, 238, [0., 0., 1.])?;
    run.dump("bug5990.png")?;
    run.finish()
}

/// `bug22900`: a compound of a cone and an edge recoloured before and after shading is red.
pub(crate) fn recolored_compound_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC22900: Problem with shape displaying in shading mode",
    ]);
    banner(&["================", ""]);
    let e1 = edge(&vertex([0., 0., 0.])?, &vertex([0., 0., 30.])?)?;
    let con1 = translated(&cone(5., 0., 10.)?, [0., 0., 30.])?;
    let c1 = compound_of(&[con1, e1])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "c1", &c1)?;
    run.session().fit()?;
    run.session()
        .set_color("c1", Color::named(NamedColor::RED))?;
    run.session()
        .set_color("c1", Color::named(NamedColor::GREEN))?;
    run.session().set_display_mode(1)?;
    run.session()
        .set_color("c1", Color::named(NamedColor::RED))?;
    run.session().check_color(204, 202, [1., 0., 0.])?;
    run.dump("bug22900.png")?;
    run.finish()
}

/// `bug26792`: a zoom-persistent box keeps its depth range after a strong zoom out.
pub(crate) fn persistent_box_z_fitted() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================================================================",
        "CR26792",
    ]);
    banner(&[
        "Visualization, Graphic3d - Z-fit support for transform persistence is missing after removing Visual3dView",
    ]);
    banner(&[
        "================================================================",
        "",
    ]);
    let b = box_sized([50., 50., 50.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .display("b", &b, &DisplayRequest::zoom_persistent([0.; 3]))?;
    run.session().zoom(0.01)?;
    run.session().z_fit()?;
    for (x, y) in [(204, 184), (238, 205), (275, 182), (239, 184)] {
        run.session().check_color(x, y, [1., 1., 0.])?;
    }
    run.dump("bug26792.png")?;
    run.finish()
}
