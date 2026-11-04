//! Both buffer settings are exercised. Vulkan uploads buffers in both cases; its documented
//! enable_vbo no-op is a backend adaptation, not evidence of two Vulkan storage paths.
use super::*;
use crate::viewer_objects::PrimitiveArrayObject;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR, Graphic3dArrayOfPrimitives, Graphic3dTypeOfPrimitiveArray,
};

/// The four authored vertices and one red bound; `red_bound_polyline()?`.
pub(super) fn red_bound_polyline() -> Result<Handle<Graphic3dArrayOfPrimitives>, Failure> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(
        Graphic3dTypeOfPrimitiveArray::Polylines,
        4,
        1,
        0,
        GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR,
    )?);
    for [x, y, z] in [[0., 0., 0.], [0., 10., 0.], [0., 10., 10.], [0., 0., 0.]] {
        array.add_vertex(x, y, z)?;
    }
    array.add_bound_rgb(4, 1., 0., 0.)?;
    Ok(array)
}

/// Request the legacy mode and display a fresh array; `bound_stage(&mut session, false)?`.
pub(super) fn bound_stage(
    session: &mut ViewerSession,
    enabled: bool,
) -> Result<Handle<Graphic3dArrayOfPrimitives>, Failure> {
    session.view().viewer().driver().enable_vbo(enabled);
    let array = red_bound_polyline()?;
    session.display_object("pl01", PrimitiveArrayObject::new(array.clone()).cast())?;
    Ok(array)
}

const BOUND_BANNER: [&str; 5] = [
    "==========",
    "OCC23883",
    "#################################################################################",
    "Bound color in the primitive array do not work when displayed using VBO",
    "#################################################################################",
];

/// Complete `bug23883`, keeping both checks and only its final dump; `bound_colour()?`.
pub(crate) fn bound_colour() -> Outcome {
    banner(&BOUND_BANNER);
    let mut run = SessionRun::new()?;
    bound_stage(run.session(), false)?;
    run.session().fit()?;
    run.session().check_color(321, 200, [1., 0., 0.])?;
    run.session().clear()?;
    bound_stage(run.session(), true)?;
    run.session().check_color(321, 200, [1., 0., 0.])?;
    run.dump("bug23883.png")?;
    run.finish()
}
