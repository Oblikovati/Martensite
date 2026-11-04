//! Triangulated quadrics: cylinder, cone and frustum; sphere; torus, pipe and ring segments.
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use crate::triangle_array_object::TriangleArrayObject;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Transform3;
use martensite_tkservice::graphic3d::Graphic3dArrayOfTriangles;
use martensite_tkv3d::prs3d::{Prs3dToolCylinder, Prs3dToolSphere, Prs3dToolTorus};
use std::error::Error;
use std::f64::consts::PI;

type Outcome = Result<(), Box<dyn Error>>;

/// Display `triangles` as a primitive-array object `name`, translated by `offset` when given.
fn show(
    session: &mut ViewerSession,
    name: &str,
    triangles: Handle<Graphic3dArrayOfTriangles>,
    offset: Option<[f64; 3]>,
) -> Outcome {
    session.display_object(name, TriangleArrayObject::new(triangles).cast())?;
    if let Some(offset) = offset {
        session.translate_location(name, offset)?;
    }
    Ok(())
}

/// A cylinder of radii `r1`/`r2` and `height` with the default 100 slices and one stack.
fn cylinder(
    r1: f64,
    r2: f64,
    height: f64,
) -> Result<Handle<Graphic3dArrayOfTriangles>, Box<dyn Error>> {
    Ok(Prs3dToolCylinder::create(
        r1,
        r2,
        height,
        100,
        1,
        &Transform3::default(),
    )?)
}

/// A torus segment with angles in degrees, converted as the viewer converts them.
fn torus(
    segment: (f64, f64),
    pipe: f64,
) -> Result<Handle<Graphic3dArrayOfTriangles>, Box<dyn Error>> {
    let radians = |degrees: f64| degrees * (PI / 180.0);
    let (first, last) = (radians(segment.0), radians(segment.1));
    let trsf = Transform3::default();
    Ok(Prs3dToolTorus::create(
        10.0,
        5.0,
        first,
        last,
        radians(pipe),
        100,
        100,
        &trsf,
    )?)
}

/// `cylinder`: a cylinder, a cone and a cone frustum at three locations.
pub(crate) fn cylinder_cone_frustum() -> Outcome {
    banner(&[
        "====================================",
        "Prs3dToolCylinder - create cylinder",
        "====================================",
    ]);
    let mut run = SessionRun::new()?;
    show(
        run.session(),
        "c1",
        cylinder(8.0, 8.0, 20.0)?,
        Some([0., 0., 20.]),
    )?;
    show(
        run.session(),
        "c2",
        cylinder(8.0, 0.0, 20.0)?,
        Some([-20., -20., -20.]),
    )?;
    show(
        run.session(),
        "c3",
        cylinder(8.0, 4.0, 10.0)?,
        Some([20., 20., -20.]),
    )?;
    run.session().fit()?;
    run.dump("cylinder.png")?;
    run.finish()
}

/// `sphere`: a complete sphere of radius 10 with 100 slices and stacks.
pub(crate) fn complete_sphere() -> Outcome {
    banner(&[
        "================================",
        "Prs3dToolSphere - create sphere",
        "================================",
    ]);
    let mut run = SessionRun::new()?;
    let sphere = Prs3dToolSphere::create(10.0, 100, 100, &Transform3::default())?;
    show(run.session(), "s", sphere, None)?;
    run.session().fit()?;
    run.dump("sphere.png")?;
    run.finish()
}

/// `torus`: a complete torus, a 270 degree pipe segment and a half ring segment.
pub(crate) fn torus_segments() -> Outcome {
    banner(&[
        "==============================",
        "Prs3dToolTorus - create torus",
        "==============================",
    ]);
    let mut run = SessionRun::new()?;
    show(
        run.session(),
        "t1",
        torus((0., 360.), 360.)?,
        Some([0., 0., 20.]),
    )?;
    show(
        run.session(),
        "t2",
        torus((0., 360.), 270.)?,
        Some([-20., -20., -20.]),
    )?;
    show(
        run.session(),
        "t3",
        torus((180., 360.), 360.)?,
        Some([20., 20., -20.]),
    )?;
    run.session().fit()?;
    run.dump("torus.png")?;
    run.finish()
}
