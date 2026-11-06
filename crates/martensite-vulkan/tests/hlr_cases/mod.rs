//! Hidden-line regressions: a shape is shown in a 3D view, its visible lines are computed along
//! that view's projection, then their length is checked and they are dumped in a top view.
use crate::case_view::SourceViewOutput as _;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::dataset_shape::read_dataset_shape;
use crate::hidden_lines::{HiddenLineAlgorithm, visible_lines};
use crate::plane_capture::PlaneCapture;
use crate::session_run::banner;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use martensite_tkbrep::topods::Shape;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_lib::build_curves3d_default;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use martensite_vulkan::Graphic3dRenderDriver;
use std::error::Error;

mod fused_cylinders;

/// Where a regression's shape comes from.
#[derive(Clone, Copy, Debug)]
pub(crate) enum HlrInput {
    /// A dataset boundary-representation file.
    Dataset(&'static str),
    /// A torus of major radius 30 and minor radius 10 at the origin.
    Torus,
    /// Two coaxial cylinders (radius 10, height 30; radius 8, height 50 raised by 2) fused.
    FusedCylinders,
}

/// One hidden-line regression; `HlrCase { heading: &["OCC25908"], ... }`.
pub(crate) struct HlrCase {
    /// The banner lines between the separator lines.
    pub heading: &'static [&'static str],
    /// The shape.
    pub input: HlrInput,
    /// The standard view orientation set after display; `None` keeps the initial view.
    pub orientation: Option<V3dTypeOfOrientation>,
    /// The removal algorithm.
    pub algorithm: HiddenLineAlgorithm,
    /// The expected total length of the visible lines.
    pub length: f64,
    /// The image name of the top view dump.
    pub image: &'static str,
}

fn input_shape(input: HlrInput) -> Result<Shape, Box<dyn Error>> {
    Ok(match input {
        HlrInput::Dataset(name) => read_dataset_shape(name)?,
        HlrInput::Torus => BRepPrimAPIMakeTorus::new(30., 10.)?.shape()?.clone(),
        HlrInput::FusedCylinders => fused_cylinders::fused_cylinders()?,
    })
}

/// Run one hidden-line regression; `hidden_line_view(&case)?`.
pub(crate) fn hidden_line_view(case: &HlrCase) -> Result<(), Box<dyn Error>> {
    let separator = "=====================================";
    banner(&[separator]);
    banner(case.heading);
    banner(&[separator, ""]);
    let shape = input_shape(case.input)?;
    let mut plane = PlaneCapture::top()?;
    let driver = Graphic3dRenderDriver::new(crate::device_options::validation_options())?;
    let mut session = ViewerSession::new(&driver)?;
    session.display("a", &shape, &DisplayRequest::default())?;
    if let Some(orientation) = case.orientation {
        session.set_standard_projection(orientation)?;
    }
    let result = visible_lines(&shape, session.projection_frame()?, case.algorithm)?;
    plane.display(&result)?;
    plane.fit()?;
    if case.algorithm == HiddenLineAlgorithm::Exact {
        build_curves3d_default(&result)?;
    }
    let props = check_mass(&result, &length_check(case))?;
    plane.draw_inertia_axes(&props)?;
    plane.dump(case.image)?;
    println!("TEST COMPLETED");
    session.close();
    for diagnostic in driver.validation_errors()? {
        eprintln!("Error: Vulkan validation: {diagnostic}");
    }
    Ok(())
}

/// The grids' final `checkprops result -l $length`: the exact grid keeps the default relative
/// tolerance, the polygonal grid passes `-deps $depsilon` with `depsilon 1.e-7` (hlr/begin).
fn length_check(case: &HlrCase) -> MassCheck {
    let check = MassCheck::near(MassKind::Length, case.length);
    match case.algorithm {
        HiddenLineAlgorithm::Exact => check,
        HiddenLineAlgorithm::Polygonal => MassCheck {
            relative: 1.0e-7,
            ..check
        },
    }
}
