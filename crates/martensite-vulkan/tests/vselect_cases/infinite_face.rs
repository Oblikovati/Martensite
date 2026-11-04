//! The complete infinite-cylinder face original, including both inherited view initializations.
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::SessionRun;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSurface};
use martensite_tkmath::gp::Axis3;
use martensite_tkservice::graphic3d::Graphic3dAspectsMembers as _;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tkv3d::{
    prs_mgr::PrsMgrPresentableObjectMembers as _,
    prs3d::{Prs3dDrawer, Prs3dIsoAspect},
};
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

/// Natural cylindrical face, without replacing infinite geometry by a finite trim.
fn infinite_cylinder_face() -> Result<Face, Failure> {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 10.)?).cast();
    BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?.face()
}

/// CopyIsoAspect preserves line appearance while giving this object its own count.
fn independent_face_iso(
    source: &Prs3dIsoAspect,
    count: i32,
) -> Result<Handle<Prs3dIsoAspect>, Failure> {
    let line = source.aspect();
    let copy = Prs3dIsoAspect::new(
        line.interior_color(),
        line.line_kind(),
        f64::from(line.width()),
        count,
    )?;
    Ok(Handle::new(copy))
}

/// Object-specific counts must not mutate the default drawer's shared iso aspects.
fn own_face_isolines(drawer: &Prs3dDrawer) -> Result<(), Failure> {
    let u = independent_face_iso(&drawer.u_iso_aspect(), 10)?;
    let v = independent_face_iso(&drawer.v_iso_aspect(), 4)?;
    drawer.set_u_iso_aspect(u);
    drawer.set_v_iso_aspect(v);
    Ok(())
}

/// A second view on the existing viewer and context, not a new viewer.
fn face_body_view(session: &mut ViewerSession) -> Result<(), Failure> {
    session.open_view(409, 409)?;
    Ok(())
}

/// Defaults are changed after first display, then existing objects are recomputed.
fn displayed_face_limit(session: &ViewerSession) -> Result<(), Failure> {
    let object = session
        .object("f")
        .cloned()
        .expect("object f; expected displayed infinite face");
    session.update_context(|context| {
        context.default_drawer().set_maximal_parameter_value(10.);
        context.redisplay(&object, false, false)
    })
}

/// Full isoline order, including context policy, local attributes and viewer update.
fn displayed_face_isolines(session: &ViewerSession) -> Result<(), Failure> {
    let object = session
        .object("f")
        .cloned()
        .expect("object f; expected displayed infinite face");
    println!("New number of isos : 10 4\nNew IsoOnPlane mode is ON");
    session.update_context(|context| {
        context.set_iso_on_plane(true);
        let drawer = object.attributes();
        own_face_isolines(&drawer)?;
        context.set_local_attributes(&object, &drawer, false)?;
        context.redisplay(&object, false, false)
    })
}

/// Complete G1 and inherited face end image; `infinite_face_isolines()`.
pub(crate) fn infinite_face_isolines() -> Outcome {
    let mut run = SessionRun::new()?; // face/begin creates View1 before the body.
    let face = infinite_cylinder_face()?;
    face_body_view(run.session())?;
    run.session().display(
        "f",
        &face.into_shape(),
        &DisplayRequest::default().with_display_mode(0),
    )?;
    displayed_face_limit(run.session())?;
    displayed_face_isolines(run.session())?;
    run.session().fit()?;
    run.dump("G1.png")?; // face/end, preceding the parent completion hook.
    run.finish()
}

#[cfg(test)]
#[path = "infinite_face_controls.rs"]
mod controls;
