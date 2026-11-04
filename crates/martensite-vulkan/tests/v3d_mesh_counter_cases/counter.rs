//! The strip and points are culled by the actual retained view, never an expected-coordinate filter.
use super::*;
use crate::case_view::viewer_session::ViewParameters;
use crate::viewer_objects::{PrimitiveArrayObject, point_object};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{Graphic3dArrayOfTriangleStrips, Graphic3dCameraProjection};

const STRIP: [[f64; 3]; 4] = [
    [-97.9819, -175.225, -7.6706],
    [-97.9819, 194.163, -7.6706],
    [271.406, -175.225, -7.6706],
    [271.406, 194.163, -7.6706],
];
const POINTS: [(&str, [f64; 3]); 6] = [
    ("pl1", [-4., -17., 0.]),
    ("pl2", [-4., -10., 14.]),
    ("pl3", [0., -6., 17.]),
    ("pr1", [179., 17., 39.]),
    ("pr2", [174., -2., 12.]),
    ("pr3", [177., 0., 15.]),
];

fn source_strip(session: &mut ViewerSession) -> Result<(), Failure> {
    let array = Handle::new(Graphic3dArrayOfTriangleStrips::new(4, 0, 0)?);
    for [x, y, z] in STRIP {
        array.add_vertex_pnt(&Point3::new(x, y, z))?;
    }
    session.display_object("g", PrimitiveArrayObject::new(array.cast()).cast())
}

/// Build the actual source arrays and points; `prepare_counter_scene(&mut session)?`.
pub(super) fn prepare_counter_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    source_strip(session)?;
    for (name, [x, y, z]) in POINTS {
        session.display_object(name, point_object([x, y, z])?)?;
    }
    session.fit()?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    apply_source_camera(session)?;
    apply_source_camera(session)
}

fn apply_source_camera(session: &ViewerSession) -> Result<(), Failure> {
    let parameters = ViewParameters {
        scale: Some(5.),
        proj: Some([-0., -1., -0.]),
        up: Some([0., 0., 1.]),
        at: Some([-157., 9.5, 19.]),
        ..Default::default()
    };
    session.set_view_parameters(&parameters)
}

/// Parse the actual command's integer text; `profiler_integer("3 ", "points")?`.
pub(super) fn profiler_integer(text: &str, command: &str) -> Result<i64, Failure> {
    text.trim().parse().map_err(|_| {
        Failure::with_message(
            "Standard_ProgramError",
            &format!("vstatprofiler {command} returned {text:?}; expected an integer count"),
        )
    })
}

fn check_rendered_elements(session: &ViewerSession) -> Outcome {
    let points = profiler_integer(&session.rendered_points()?, "points")?;
    let triangles = profiler_integer(&session.rendered_triangles()?, "triangles")?;
    if points != 3 || triangles != 2 {
        println!("Error: unexpected number of culled elements");
    }
    Ok(())
}

const COUNTER_BANNER: [&str; 3] = [
    "=============",
    "0030437: Visualization, TKV3d - add Draw command to print rendering statistics",
    "=============",
];

/// Execute actual culling queries and the authored image; `culled_elements()?`.
pub(crate) fn culled_elements() -> Outcome {
    banner(&COUNTER_BANNER);
    let mut run = SessionRun::with_view_size(800, 400)?;
    prepare_counter_scene(run.session())?;
    check_rendered_elements(run.session())?;
    run.dump("bug30437.png")?;
    run.finish()
}
