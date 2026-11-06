//! Placement of the overlay: position and height changes, several views of one scale, and
//! back-face culling of the scale's own shading aspect.
use super::prelude::*;
use crate::case_view::viewer_session::SurfaceAspects;
use martensite_tkservice::graphic3d::Graphic3dTypeOfBackfacingModel;

/// `bug28813`: the title stays above the bar as the scale moves and changes height.
pub(crate) fn position_and_height() -> Outcome {
    banner(&[
        "========",
        "OCC28813: Visualization, AisColorScale - color scale title invalid placement",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    color_scale(&mut run, "cs", &|s| {
        demo(s);
        s.set_title(&ExtendedString::from_utf8("Test"));
    })?;
    run.dump("bug28813_1.png")?;
    let steps: [(ColorScaleSetting<'_>, &'static str); 7] = [
        (&|s| s.set_position(10, 20), "bug28813_2.png"),
        (&|s| s.set_height(150), "bug28813_3.png"),
        (&|s| s.set_height(250), "bug28813_4.png"),
        (&|s| s.set_position(10, 100), "bug28813_5.png"),
        (&|s| s.set_position(10, 150), "bug28813_6.png"),
        (&|s| s.set_height(150), "bug28813_7.png"),
        (&|s| s.set_height(250), "bug28813_8.png"),
    ];
    for (setting, image) in steps {
        color_scale(&mut run, "cs", setting)?;
        run.dump(image)?;
    }
    run.finish()
}

/// `bug26312`: one scale shown in three views of different sizes, fitted to each height.
pub(crate) fn multiple_views() -> Outcome {
    banner(&[
        "============",
        "OCC26312: ColorScale doesn't work as expected.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    color_scale(&mut run, "cs", &demo)?;
    run.dump("bug26312_View1.png")?;
    run.session().open_view(300, 500)?;
    run.session().open_view(500, 300)?;
    run.session().activate_view(1)?;
    color_scale(&mut run, "cs", &|s| s.set_height(500))?;
    run.dump("bug26312_View2.png")?;
    run.session().activate_view(2)?;
    color_scale(&mut run, "cs", &|s| s.set_height(300))?;
    run.dump("bug26312_View3.png")?;
    run.finish()
}

/// `colorscale_faceculling`: culling back faces keeps the scale's front-facing bar visible.
pub(crate) fn face_culled_scale() -> Outcome {
    banner(&[
        "============",
        "AisColorScale face-culling regression",
        "============",
        "",
    ]);
    let mut run = SessionRun::with_view_size(490, 409)?;
    color_scale(&mut run, "cs", &demo)?;
    let culled = SurfaceAspects {
        face_culling: Some(Graphic3dTypeOfBackfacingModel::BackCulled),
        ..SurfaceAspects::default()
    };
    run.session().set_surface_aspects("cs", &culled)?;
    if run.session().pixel_color_name(20, 25)? != "RED" {
        println!("Error: front face culled");
    }
    run.dump("colorscale_faceculling.png")?;
    run.finish()
}
