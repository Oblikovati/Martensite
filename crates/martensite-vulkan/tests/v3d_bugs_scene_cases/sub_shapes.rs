//! Custom colours of sub-shapes: nested compounds, back-face
//! culling of coloured solids, untriangulated cones and transparency round trips.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::{SubShapeStyle, ViewParameters};
use crate::primitive_shapes::compound_of;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkg3d::top_abs::ShapeEnum;

/// A colour-only sub-shape style.
fn colored(color: NamedColor) -> SubShapeStyle {
    SubShapeStyle {
        color: Some(Color::named(color)),
        width: None,
    }
}

/// Print `message` unless pixel (x, y) has the named colour.
fn expect_named(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    name: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(x, y)? != name {
        println!("{message}");
    }
    Ok(())
}

const OCC28036: [&str; 5] = [
    "===========",
    "OCC28036",
    "Visualization, AisColoredShape - handle correctly nested compounds within Shaded display mode",
    "===========",
    "",
];

/// `bug28036_1`: a box keeps its own colour inside a coloured nested compound.
pub(crate) fn nested_compound_colors() -> Result<(), Box<dyn Error>> {
    banner(&OCC28036);
    let b1 = box_at([0., 0., 0.], [1., 2., 3.])?;
    let b2 = box_at([0., 3., 0.], [2., 1., 3.])?;
    let b3 = box_at([3., 0., 0.], [3., 2., 1.])?;
    let c12 = compound_of(&[b1.clone(), b2])?;
    let c = compound_of(&[c12.clone(), b3])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("c", &c, &shaded)?;
    run.session().fit()?;
    run.session()
        .set_color("c", Color::named(NamedColor::BLUE))?;
    run.session()
        .style_sub_shape("c", &b1, &colored(NamedColor::RED))?;
    run.session()
        .style_sub_shape("c", &c12, &colored(NamedColor::GREEN))?;
    for (pixel, name) in [
        ((100, 100), "RED2"),
        ((200, 100), "GREEN2"),
        ((200, 300), "BLUE2"),
    ] {
        expect_named(&mut run, pixel, name, "Error: wrong subshape color")?;
    }
    run.dump("bug28036_1.png")?;
    run.finish()
}

/// `bug25687_1`: a solid with one coloured face keeps back faces culled.
pub(crate) fn colored_solid_culled() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC25687",
        "AisColoredShape should activate back-face culling for valid Solids.",
    ]);
    banner(&["============", ""]);
    let b = box_sized([10., 10., 10.])?;
    let faces = all_distinct(&b, ShapeEnum::Face);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "b", &b)?;
    let proj = ViewParameters {
        proj: Some([0.5, 0.3, 0.1]),
        ..ViewParameters::default()
    };
    run.session().set_view_parameters(&proj)?;
    run.session().fit()?;
    run.session()
        .style_sub_shape("b", &faces[0], &colored(NamedColor::GREEN))?;
    if run.session().pixel_color_name(225, 32)? == "GREEN" {
        println!("Error: back-face culling should be turned ON, but back-face is still visible.");
    }
    run.dump("bug25687_1.png")?;
    run.finish()
}

/// `bug25687_2`: a solid with one coloured and one hidden face shows its back faces through
/// the hole, so the coloured face is seen at the view centre.
pub(crate) fn hidden_face_disables_culling() -> Result<(), Box<dyn Error>> {
    banner(&["============", "OCC25687"]);
    banner(&[
        "AisColoredShape should disable back-face culling for Solids with hidden Face",
        "============",
        "",
    ]);
    let b = box_sized([10., 10., 10.])?;
    let faces = all_distinct(&b, ShapeEnum::Face);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "b", &b)?;
    let proj = ViewParameters {
        proj: Some([0.5, 0.3, 0.1]),
        ..ViewParameters::default()
    };
    run.session().set_view_parameters(&proj)?;
    run.session().fit()?;
    run.session()
        .style_sub_shape("b", &faces[0], &colored(NamedColor::GREEN))?;
    run.session().hide_sub_shape("b", &faces[1])?;
    let message = "Error: back-face culling should be disabled, but seems to be not.";
    expect_named(&mut run, (200, 200), "GREEN", message)?;
    run.dump("bug25687_2.png")?;
    run.finish()
}

/// `bug26014`: without automatic triangulation a shaded cone shows a red wide face outline
/// and no shading.
pub(crate) fn untriangulated_colored_cone() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26014",
        "Check that the colored shading shape doesn't have a shading presentation",
    ]);
    banner(&[
        "due to its triangulation isn't computed in case of disabled auto triangulation feature.",
    ]);
    banner(&[
        "But in this case the shape should have special wireframe presentation.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    run.session().set_auto_triangulation(false);
    let c = cone(0., 5., 10.)?;
    show(&mut run, "c", &c)?;
    run.session().fit()?;
    let face = all_distinct(&c, ShapeEnum::Face)[1].clone();
    let style = SubShapeStyle {
        color: Some(Color::named(NamedColor::RED)),
        width: Some(4.),
    };
    run.session().style_sub_shape("c", &face, &style)?;
    let wireframe = run.session().pixel_color_name(200, 31)?;
    let shaded = run.session().pixel_color_name(180, 60)?;
    run.dump("bug26014_shape_pres.png")?;
    if wireframe != "RED" || shaded != "BLACK" {
        println!("Error: presentation of shape is incorrect");
    }
    run.finish()
}

/// Set transparency 0.8 on `b`, then unset it, returning the colour name at
/// (250, 250) before the round trip and after it.
fn transparency_round_trip(run: &mut SessionRun) -> Result<[String; 2], Box<dyn Error>> {
    let before = run.session().pixel_color_name(250, 250)?;
    run.session().set_transparency("b", 0.8)?;
    let object = run.session().named_objects(&["b"])?.remove(0);
    run.session()
        .update_context(|context| context.unset_transparency(&object, false))?;
    Ok([before, run.session().pixel_color_name(250, 250)?])
}

/// `bug28621`: unsetting transparency restores the colour of a plain and of a coloured shape.
pub(crate) fn transparency_unset_restores_color() -> Result<(), Box<dyn Error>> {
    let rule = "=================================================================================================";
    banner(&[
        rule,
        "0028621: Visualization - AisColoredShape::UnsetTransparency() is not implemented",
        rule,
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b", &b, &shaded)?;
    run.session().fit()?;
    let [first, second] = transparency_round_trip(&mut run)?;
    if first != second {
        println!("Error: color is not set back");
    }
    let face = all_distinct(&b, ShapeEnum::Face)[1].clone();
    run.session()
        .style_sub_shape("b", &face, &colored(NamedColor::RED))?;
    let [first, second] = transparency_round_trip(&mut run)?;
    if first != second {
        println!("Error: color is not set back");
    }
    run.dump("bug28621.png")?;
    run.finish()
}

/// A colour and width sub-shape style.
fn colored_wide(color: NamedColor, width: f64) -> SubShapeStyle {
    SubShapeStyle {
        color: Some(Color::named(color)),
        width: Some(width),
    }
}

/// The custom vertex, edge, wire and face aspects of box `b` in `bug24762_coloredshape`.
fn style_box_parts(run: &mut SessionRun, b: &Shape) -> Result<(), Box<dyn Error>> {
    let part = |kind, index: usize| all_distinct(b, kind)[index - 1].clone();
    let session = run.session();
    session.style_sub_shape(
        "b",
        &part(ShapeEnum::Vertex, 1),
        &colored(NamedColor::GREEN),
    )?;
    for index in [6, 12] {
        let style = colored_wide(NamedColor::RED, 6.);
        session.style_sub_shape("b", &part(ShapeEnum::Edge, index), &style)?;
    }
    let pink = colored_wide(NamedColor::HOTPINK, 4.);
    session.style_sub_shape("b", &part(ShapeEnum::Wire, 2), &pink)?;
    session.style_sub_shape("b", &part(ShapeEnum::Face, 3), &colored(NamedColor::GRAY))?;
    Ok(())
}

/// `bug24762_coloredshape`: custom colours and widths of a box's vertex, edges, wire and face,
/// and a green box with one red face switched to wireframe.
pub(crate) fn customized_sub_shapes() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC24762 new interactive object AisColoredShape with customized subshapes presentations",
    ]);
    banner(&["========"]);
    let b = box_at([0., 0., 0.], [1., 2., 3.])?;
    let bb = box_at([3., 0., 0.], [2., 3., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    run.session().set_axonometric()?;
    show(&mut run, "b", &b)?;
    show(&mut run, "bb", &bb)?;
    run.session().fit()?;
    style_box_parts(&mut run, &b)?;
    let bb_6 = all_distinct(&bb, ShapeEnum::Face)[5].clone();
    // One command: the object becomes a coloured shape, turns green, then its face red.
    run.session()
        .style_sub_shape("bb", &bb_6, &SubShapeStyle::default())?;
    run.session()
        .set_color("bb", Color::named(NamedColor::GREEN))?;
    run.session()
        .style_sub_shape("bb", &bb_6, &colored(NamedColor::RED))?;
    run.session().set_object_display_mode("bb", 0)?;
    run.dump("bug24762_coloredshape.png")?;
    let checks = [
        ((54, 150), "HOTPINK", "Wire"),
        ((100, 90), "RED", "Edge"),
        ((30, 200), "GRAY69", "Face"),
    ];
    for (pixel, name, part) in checks {
        expect_named(&mut run, pixel, name, &format!("Error: wrong {part} color"))?;
    }
    run.finish()
}

/// `bug25459_coloredshape`: a material set on a coloured shape keeps its red face.
pub(crate) fn material_keeps_face_color() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC25459 AisColoredShape::SetMaterial() should not reset custom colors",
    ]);
    banner(&["========"]);
    let b = box_at([0., 0., 0.], [1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b", &b, &shaded)?;
    run.session().fit()?;
    let face = all_distinct(&b, ShapeEnum::Face)[2].clone();
    run.session()
        .style_sub_shape("b", &face, &colored(NamedColor::RED))?;
    run.session().set_material(
        "b",
        martensite_tkservice::graphic3d::Graphic3dNameOfMaterial::Plastified,
    )?;
    expect_named(&mut run, (128, 256), "RED4", "Error: wrong Face color")?;
    run.dump("bug25459_coloredshape.png")?;
    run.finish()
}
