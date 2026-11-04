//! Every named color owns a copied face and the authored twenty-six-column position.
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

pub(super) fn position(index: usize) -> [f64; 3] {
    [12. * (index % 26) as f64, 12. * (index / 26) as f64, 0.]
}
pub(crate) fn colors() -> Outcome {
    banner(&[
        "========",
        "0030930: Test Commands, ViewerCommands - add commands vlistcolors and vlistmaterials listing standard Colors and Materials",
        "Generate a table of predefined Colors (Quantity_NameOfColor enumeration)",
        "========",
    ]);
    let face = sixth_box_face()?;
    let mut run = SessionRun::new()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    println!("<a href=colors_table.html>Color Comparison Table</a>");
    populate(run.session(), &face)?;
    run.session().fit()?;
    run.dump("colors.png")?;
    color_table::write_table()?;
    run.finish()
}
fn sixth_box_face() -> Result<Shape, Failure> {
    let cube = crate::primitive_shapes::box_sized([10.; 3])?;
    let faces = crate::boolean_typed_explode::all_distinct(&cube, ShapeEnum::Face);
    faces.get(5).cloned().ok_or_else(|| {
        martensite_tkernel::standard::exceptions::program_error(&format!(
            "box faces={}; expected sixth face",
            faces.len()
        ))
    })
}
fn populate(session: &mut ViewerSession, face: &Shape) -> Result<(), Failure> {
    session.without_updates(|session| {
        for (index, color) in NamedColor::all().enumerate() {
            display_color(session, face, color, index)?;
        }
        Ok(())
    })
}
fn display_color(
    session: &mut ViewerSession,
    face: &Shape,
    color: NamedColor,
    index: usize,
) -> Result<(), Failure> {
    let name = Color::string_name(color);
    println!("{name}");
    let mut copy = BRepBuilderAPICopy::with_shape(face, true, false)?;
    session.display(
        name,
        copy.shape()?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.set_object_shading(name, Graphic3dTypeOfShadingModel::Unlit)?;
    session.set_color(name, Color::named(color))?;
    session.set_location(name, position(index))
}
