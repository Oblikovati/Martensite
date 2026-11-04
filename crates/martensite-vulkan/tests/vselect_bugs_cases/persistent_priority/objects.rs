//! Literal source geometry uses the same actual point/array/circle classes as its commands.
use super::*;
use crate::primitive_shapes::box_at;
use crate::viewer_objects::{circle_object, circle_through, point_object, triangle_object};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOP, GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dArrayOfPrimitives,
    Graphic3dArrayOfQuadrangles, Graphic3dArrayOfSegments,
};
use martensite_tkv3d::{
    ais::{AisPoint, AisShape, PrimitiveArrayPresentation},
    prs_mgr::PrsMgrPresentableObjectMembers as _,
};

pub(super) const CORNERS: [[f64; 3]; 4] = [
    [75., -250., 0.],
    [275., -250., 0.],
    [275., -50., 0.],
    [75., -50., 0.],
];
pub(super) const SEGMENTS: [[[f64; 3]; 2]; 4] = [
    [[76., -249., 0.], [274., -249., 0.]],
    [[274., -249., 0.], [274., -51., 0.]],
    [[274., -51., 0.], [76., -51., 0.]],
    [[76., -51., 0.], [76., -249., 0.]],
];
pub(super) const SEG_NAMES: [&str; 4] = ["seg1", "seg2", "seg3", "seg4"];

/// Retain the first source box position and dimensions; `first_box()?`.
pub(super) fn first_box() -> Result<Shape, Failure> {
    let origin = [75., -250., 0.];
    let size = [200., 200., 200.];
    box_at(origin, size)
}

/// Retain the overlapping source box without coordinate remapping; `second_box()?`.
pub(super) fn second_box() -> Result<Shape, Failure> {
    let origin = [175., -300., 0.];
    let size = [200., 200., 200.];
    box_at(origin, size)
}

/// Apply source persistence and own mode before the first display; `display_box(session,"box_1",shape)?`.
pub(super) fn display_box(
    session: &mut ViewerSession,
    name: &str,
    shape: &Shape,
) -> Result<(), Failure> {
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
    session
        .context()
        .set_z_layer(&object, GRAPHIC3D_ZLAYER_ID_TOP);
    session
        .context()
        .set_transform_persistence(&object, &commands::top_left()?)?;
    object.set_display_mode(1);
    session.display_object(name, object)
}

/// Keep real named operands until their consumers retain coordinates; `points(session,names,coords)?`.
pub(super) fn points(
    session: &mut ViewerSession,
    names: &[&str],
    coords: &[[f64; 3]],
) -> Result<(), Failure> {
    if names.len() != coords.len() {
        return Err(program_error(&format!(
            "point names={} coordinates={}; expected equal counts",
            names.len(),
            coords.len()
        )));
    }
    for (name, &position) in names.iter().zip(coords) {
        session.display_object(name, point_object(position)?)?;
    }
    Ok(())
}

fn named_point(session: &ViewerSession, name: &str) -> Result<[f64; 3], Failure> {
    let object = named(session, name)?.cast::<AisPoint>();
    let point = object
        .get()
        .ok_or_else(|| program_error(&format!("operand {name:?}; expected actual AIS point")))?;
    let p = point.component().pnt();
    Ok([p.x(), p.y(), p.z()])
}

/// Read real named point operands before constructing the array; `triangle(session,"tri1",["pt1","pt2","pt3"])?`.
pub(super) fn triangle(
    session: &mut ViewerSession,
    name: &str,
    operands: [&str; 3],
) -> Result<(), Failure> {
    let [a, b, c] = operands;
    let vertices = [
        named_point(session, a)?,
        named_point(session, b)?,
        named_point(session, c)?,
    ];
    session.display_object(name, triangle_object(vertices)?)
}

/// Keep the three-point circle construction and genuine filled selection; `filled_circle(session)?`.
pub(super) fn filled_circle(session: &mut ViewerSession) -> Result<(), Failure> {
    let points = [
        named_point(session, "pr1")?,
        named_point(session, "pr2")?,
        named_point(session, "pr3")?,
    ];
    let circle = circle_through(points)?;
    session.display_object("circle1", circle_object(&circle, true, None))
}

fn colored_array(
    array: &Graphic3dArrayOfPrimitives,
    coords: &[[f64; 3]],
    color: NamedColor,
) -> Result<(), Failure> {
    for &[x, y, z] in coords {
        array.add_vertex_pnt_color(&Point3::new(x, y, z), Color::named(color))?;
    }
    Ok(())
}

/// Retain explicit red vertex channels rather than line defaults; `segment_array(&ends)?`.
pub(super) fn segment_array(
    coords: &[[f64; 3]; 2],
) -> Result<Handle<Graphic3dArrayOfPrimitives>, Failure> {
    let array = Handle::new(Graphic3dArrayOfSegments::with_options(2, 0, true)?);
    colored_array(&array, coords, NamedColor::RED)?;
    Ok(array.cast())
}

/// Retain the original quad topology and blue vertex channels; `quad_array()?`.
pub(super) fn quad_array() -> Result<Handle<Graphic3dArrayOfPrimitives>, Failure> {
    let array = Handle::new(Graphic3dArrayOfQuadrangles::with_options(
        4, 0, false, true, false,
    )?);
    let vertices = [
        [175., -300., 0.],
        [375., -300., 0.],
        [375., -100., 0.],
        [175., -100., 0.],
    ];
    colored_array(&array, &vertices, NamedColor::BLUE)?;
    Ok(array.cast())
}

/// Use the command-owned real primitive object, including its selection; `display_array(session,"seg1",array)?`.
pub(super) fn display_array(
    session: &mut ViewerSession,
    name: &str,
    array: Handle<Graphic3dArrayOfPrimitives>,
) -> Result<(), Failure> {
    let object = PrimitiveArrayPresentation::from_array(array.clone());
    // Parsed colored commands own the aspect before Compute so later edits reach its group.
    if array.has_vertex_colors() {
        object.setup_vertex_color_shading()?;
    }
    session.display_object(name, object.cast())?;
    Ok(())
}

/// Apply VPoint pixel-coordinate conversion before displaying the datum; `screen_point(session)?`.
pub(super) fn screen_point(session: &mut ViewerSession) -> Result<(), Failure> {
    let object = point_object([175., -150., 0.])?; // VPoint -2d flips the authored Y before construction.
    session
        .context()
        .set_z_layer(&object, GRAPHIC3D_ZLAYER_ID_TOP_OSD);
    session
        .context()
        .set_transform_persistence(&object, &commands::top_left()?)?;
    session.display_object("p1", object)
}
