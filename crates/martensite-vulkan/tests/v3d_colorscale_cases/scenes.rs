//! Colour scales over displayed shapes: a transparent shaded sphere, and a shaded box in a wide
//! view with the cursor over it.
use super::prelude::*;
use crate::case_view::viewer_session::DisplayRequest;
use crate::primitive_shapes::box_sized;
use crate::shape_checks::{MassCheck, MassKind, check_mass, check_shape_valid};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// A sphere of radius 30 on the plane through (100, 100, 100) with normal Z and X direction X.
fn sphere_on_plane() -> Result<Shape, Box<dyn Error>> {
    let axes = Axis2::with_x_direction(
        Point3::new(100., 100., 100.),
        Direction3::new(0., 0., 1.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    let mut maker = BRepPrimAPIMakeSphere::with_axis(axes, 30.)?;
    Ok(maker.shape()?.clone())
}

/// `bug22819`: a silver, 60% transparent sphere shaded under the demonstration scale.
pub(crate) fn transparent_sphere() -> Outcome {
    banner(&[
        "============",
        "OCC22819: Redesign of OpenGl driver(Tests ColorScale)",
        "============",
        "",
    ]);
    let result = sphere_on_plane()?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session()
        .set_material("result", Graphic3dNameOfMaterial::Silver)?;
    run.session().set_transparency("result", 0.6)?;
    color_scale(&mut run, "cs", &demo)?;
    run.session().set_object_display_mode("result", 1)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 11309.6))?;
    check_shape_valid(&result)?;
    run.dump("bug22819.png")?;
    run.finish()
}

/// `bug25027`: a 100-interval scale with right labels over a shaded box in a 1024x512 view.
pub(crate) fn wide_view_overlay() -> Outcome {
    banner(&[
        "============",
        "OCC25027: Visualization - 2D layer viewport is updated incorrectly on resize",
        "============",
        "",
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::with_view_size(1024, 512)?;
    run.session().set_display_mode(1)?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    color_scale(&mut run, "cs", &|s| {
        s.set_range(10., 20.);
        s.set_number_of_intervals(100);
        s.set_text_height(16);
        s.set_label_position(Pos::Right);
        s.set_position(0, 0);
    })?;
    run.session().move_cursor(512, 250)?;
    run.dump("bug25027.png")?;
    run.finish()
}
