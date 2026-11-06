//! Colours of selected and detected shapes: a recoloured selection, a highlighted planar face,
//! toggled faces and an infinite face trimmed to a large parameter range.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::ViewParameters;
use crate::dataset_shape::read_dataset_shape;
use crate::shape_checks::check_shape_valid;
use crate::shape_mesh::remove_triangulation;
use martensite_tkbrep::topods::{Edge, Shape, Vertex};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
    BRepBuilderAPIMakeWire,
};

/// `buc60587`: a selected dataset shape recoloured CYAN3 is drawn in that colour.
pub(crate) fn recolored_selection() -> Result<(), Box<dyn Error>> {
    banner(&["=========", "BUC60587", "=========", ""]);
    banner(&[
        "==================================",
        "It takes visual check for this BUG",
    ]);
    banner(&["=================================="]);
    let result = read_dataset_shape("buc60587a.brep")?;
    check_shape_valid(&result)?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.session().select_point(180, 180)?;
    run.session()
        .set_color("result", Color::named(NamedColor::CYAN3))?;
    let cyan3 = [0., 0.9098039269447327, 0.9098039269447327];
    if !run.session().check_color(120, 180, cyan3)? {
        println!("Error : Model has NOT CYAN colour.");
    }
    run.dump("buc60587.png")?;
    run.finish()
}

/// The vertices of the 3 mm square in the XZ plane, in the case's order.
fn square_vertices() -> Result<Vec<Vertex>, Box<dyn Error>> {
    let corners = [
        [0.003, 0., 0.],
        [0.003, 0., 0.003],
        [0., 0., 0.003],
        [0., 0., 0.],
    ];
    let mut vertices = Vec::new();
    for [x, y, z] in corners {
        vertices.push(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()?);
    }
    Ok(vertices)
}

/// `mkplane r w_1` of the wire of four edges joining the shared square vertices in turn.
fn square_face() -> Result<Shape, Box<dyn Error>> {
    let vertices = square_vertices()?;
    let mut wire = BRepBuilderAPIMakeWire::new();
    for index in 0..4 {
        let next = &vertices[(index + 1) % 4];
        let edge: Edge = BRepBuilderAPIMakeEdge::from_vertices(&vertices[index], next)?.edge()?;
        wire.add_edge(&edge)?;
    }
    let face = BRepBuilderAPIMakeFace::from_wire(&wire.wire()?, false)?.face()?;
    Ok(face.into_shape())
}

/// `buc60688`: a selected 3 mm square face stays highlighted after its lines are widened.
pub(crate) fn small_face_highlighted() -> Result<(), Box<dyn Error>> {
    banner(&["===========", "BUC60688", "==========="]);
    banner(&[
        "==================================",
        "It takes visual check for this BUG",
    ]);
    banner(&["=================================="]);
    let mut run = SessionRun::new()?;
    let r = square_face()?;
    run.session().display("r", &r, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().select_point(120, 22)?;
    banner(&["WARNING : The rectangular MUST be highlighted !", ""]);
    run.session().set_line_width("r", 5.)?;
    if run.session().pixel_color_name(88, 272)? != "GRAY80" {
        println!("Error : The rectangular is NOT highlighted.");
    }
    run.dump("buc60688.png")?;
    run.finish()
}

/// `bug113`: two faces of a box toggled into the selection are drawn selected, then
/// detected one after the other once the selection is cleared.
pub(crate) fn toggled_faces_detected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC113",
        "BUC60978",
        "================",
        "",
    ]);
    let (first, second) = ((29, 170), (204, 300));
    let mut run = SessionRun::new()?;
    let faces = all_distinct(&box_sized([10., 10., 10.])?, ShapeEnum::Face);
    run.session()
        .display("b_1", &faces[0], &DisplayRequest::default())?;
    run.session()
        .display("b_2", &faces[1], &DisplayRequest::default())?;
    run.session().fit()?;
    for (x, y) in [first, second] {
        run.session().check_color(x, y, [0., 1., 0.])?;
    }
    run.session().toggle_selected("b_1")?;
    run.session().toggle_selected("b_2")?;
    for (x, y) in [first, second] {
        run.session().check_color(x, y, [0.8, 0.8, 0.8])?;
    }
    run.session().move_cursor(first.0, first.1)?;
    run.session().select_point(10, 10)?;
    for (x, y) in [first, second] {
        run.session().move_cursor(x, y)?;
        run.session().check_color(x, y, [0., 1., 1.])?;
    }
    run.session().set_view_parameters(&bug113_camera())?;
    run.dump("bug113.png")?;
    run.finish()
}

/// The closing camera values of `bug113`.
fn bug113_camera() -> ViewParameters {
    ViewParameters {
        scale: Some(60.6309),
        proj: Some([0.479541, 0.586729, 0.652525]),
        up: Some([-0.838029, 0.0856396, 0.538863]),
        at: Some([3.61568258316782, 3.30626448080767, 3.11631746104816]),
        ..ViewParameters::default()
    }
}

/// `mkface ft ct` of the cylinder of radius 10 along +Z through (0, 0, 1), trimmed in V to
/// [-1e9, 1e9] (`trimv`).
fn trimmed_cylinder_face() -> Result<Shape, Box<dyn Error>> {
    let (z, x) = (Direction3::new(0., 0., 1.)?, Direction3::new(1., 0., 0.)?);
    let frame = Axis3::new(Point3::new(0., 0., 1.), z, x)?;
    let cylinder: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(frame, 10.)?).cast();
    let trimmed =
        GeomRectangularTrimmedSurface::with_one_direction(&cylinder, -1e9, 1e9, false, true)?;
    let surface: Handle<dyn GeomSurface> = Handle::new(trimmed).cast();
    let maker = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
    Ok(maker.face()?.into_shape())
}

/// Set the default drawer's maximum parameter value on a context without objects.
fn set_default_max_parameter(run: &mut SessionRun, value: f64) {
    run.session()
        .context()
        .default_drawer()
        .set_maximal_parameter_value(value);
}

/// `bug129_2`: the trimmed infinite cylinder drawn up to a 1e9 parameter limit is detected,
/// selected and deselected; the image is named after the view.
pub(crate) fn trimmed_cylinder_selected() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC129", "================", ""]);
    let ft = trimmed_cylinder_face()?;
    let mut run = SessionRun::new()?;
    set_default_max_parameter(&mut run, 1e9);
    run.session()
        .display("ft", &ft, &DisplayRequest::default())?;
    let (x1, y1, x2, y2) = (207, 184, 222, 175);
    run.session().move_cursor(x1, y1)?;
    run.session().check_color(x1, y1, [0., 1., 1.])?;
    run.session().select_point(x1, y1)?;
    run.session().check_color(x1, y1, [0.8, 0.8, 0.8])?;
    run.session().select_point(x2, y2)?;
    run.session().check_color(x1, y1, [1., 1., 0.])?;
    run.dump("bug129_2_Driver1_Viewer1_View1.png")?;
    set_default_max_parameter(&mut run, 500000.);
    run.finish()
}
