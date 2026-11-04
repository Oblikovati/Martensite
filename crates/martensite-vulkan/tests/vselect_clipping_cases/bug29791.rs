//! The authentic bottle, orthogonal plane, then the original negative-depth camera regime.
use super::prelude::*;
use crate::case_view::viewer_session::ViewParameters;
use crate::dataset_shape::{locate_dataset_file, read_shape_bytes};
use martensite_tkbrep::topods::Shape;

// Compile-input guards bind these actual fixture bytes to SHA256 7b59eb71563f1a1908d0e717733dd4c4d4295fc52bf642cafb3e204ff790811b.
const BOTTLE_BYTES: &[u8] =
    include_bytes!("../../../../external/martensite-test-data/occt-source-data/occ/bottle.brep");

const BUG29791_BANNER: &[&str] = &[
    "============",
    "0029791: Visualization - Wrong result of SelectMgrRectangularFrustum::IsClipped",
    "============",
    "",
];

pub(crate) fn orthogonal_and_negative_depth() -> Result<(), Box<dyn Error>> {
    banner(BUG29791_BANNER);
    let bottle = read_pinned_bottle()?;
    let mut run = SessionRun::new()?;
    bottle_scene(run.session(), &bottle)?;
    let mut planes = ClippingPlanes::new(run.session().view());
    planes.ensure("p")?;
    planes.equation("p", [1., 0., 0., -20.])?;
    planes.on_view("p", run.session().view())?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    run.session().fit()?;
    observe_bottle(&mut run, 350, "GOLDENROD", "bug29791_1.png")?;
    run.session()
        .set_view_parameters(&negative_depth_camera())?;
    observe_bottle(&mut run, 300, "GOLDENROD3", "bug29791_2.png")?;
    run.finish()
}

pub(super) fn negative_depth_camera() -> ViewParameters {
    ViewParameters {
        scale: Some(14.7435),
        proj: Some([0.193921, -0.891229, 0.410007]),
        up: Some([-0.0205984, 0.414149, 0.909976]),
        at: Some([11.3689, 1.32152, 24.9954]),
        ..Default::default()
    }
}

fn observe_bottle(
    run: &mut SessionRun,
    x: i32,
    color: &str,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(150, 200)?;
    print_entity_state(run.session())?;
    run.session().select_point(150, 200)?;
    check_bottle_pixels(run.session(), x, color)?;
    run.dump(image)
}

fn print_entity_state(session: &ViewerSession) -> Result<(), Failure> {
    print!("{}", session.detected_entities_response()?);
    Ok(())
}

pub(super) fn read_pinned_bottle() -> Result<Shape, Box<dyn Error>> {
    let path = locate_dataset_file("occ/bottle.brep")?;
    let bytes = std::fs::read(&path)?;
    verified_bottle_bytes(&bytes)?;
    println!("clipping input: {} ({} bytes)", path.display(), bytes.len());
    Ok(read_shape_bytes(&bytes)?)
}

pub(super) fn verified_bottle_bytes(bytes: &[u8]) -> Result<(), Failure> {
    if bytes == BOTTLE_BYTES {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_ProgramError",
        &format!(
            "occ/bottle.brep selected bytes={} differ; expected exact 401311-byte SHA256 7b59eb71563f1a1908d0e717733dd4c4d4295fc52bf642cafb3e204ff790811b input",
            bytes.len()
        ),
    ))
}

fn check_bottle_pixels(session: &ViewerSession, x: i32, color: &str) -> Result<(), Failure> {
    pixel(session, 150, 200, "BLACK", "Error: object is not clipped")?;
    pixel(
        session,
        x,
        200,
        color,
        "Error: object should NOT be selected",
    )?;
    Ok(())
}

pub(super) fn bottle_scene(session: &mut ViewerSession, bottle: &Shape) -> Result<(), Failure> {
    session.clear()?;
    session.display("b", bottle, &shaded_highlighted())?;
    session.fit()?;
    session.show_view_trihedron(&Default::default())?;
    Ok(())
}
