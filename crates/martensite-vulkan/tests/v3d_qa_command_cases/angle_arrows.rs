//! Each OCC301 call constructs two anonymous edges and its own dimension.
use super::Outcome;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::{
    quantity::Color,
    standard::{Failure, Handle},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisShape},
    prs_dim::{PrsDimAngleDimension, PrsDimDimensionMembers},
    prs3d::{Prs3dDimensionAspect, Prs3dDimensionTextHorizontalPosition},
    v3d::V3dTypeOfOrientation,
};

/// Execute the full original command sequence; `angle_arrows()?`.
pub(crate) fn angle_arrows() -> Outcome {
    banner(&["========", "OCC301", "SAM926", "========", ""]);
    let mut run = SessionRun::new()?;
    for (radius, arrow_size) in [(50., 5.), (30., 3.)] {
        display_angle(run.session().context(), radius, arrow_size)?;
    }
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().fit()?;
    println!("Angle dimensions have different arrow sizes");
    run.dump("bug301.png")?;
    run.finish()
}

fn display_angle(
    context: &AisInteractiveContext,
    radius: f64,
    arrow_size: f64,
) -> Result<(), Failure> {
    display_edges(context)?;
    // RegressionCommands constructs this unused support plane before constructing the angle.
    let _plane = martensite_tkg3d::geom::GeomPlane::from_point(
        Point3::new(0., 0., 0.),
        martensite_tkmath::gp::Direction3::new(0., 0., 1.)?,
    )?;
    let dimension = make_angle(radius, arrow_size)?;
    context.display(&dimension.cast(), false)
}

fn display_edges(context: &AisInteractiveContext) -> Result<(), Failure> {
    let points = [
        Point3::new(10., 10., 0.),
        Point3::new(50., 10., 0.),
        Point3::new(50., 50., 0.),
    ];
    for (ends, update) in [
        (points[..2].try_into().unwrap(), false),
        (points[1..].try_into().unwrap(), true),
    ] {
        let [first, last]: [Point3; 2] = ends;
        let mut edge = BRepBuilderAPIMakeEdge::from_points(first, last)?;
        context.display(&AisShape::new(edge.shape()?).cast(), update)?;
    }
    Ok(())
}

fn make_angle(radius: f64, arrow_size: f64) -> Result<Handle<PrsDimAngleDimension>, Failure> {
    let dimension = PrsDimAngleDimension::from_points(
        &Point3::new(90., 10., 0.),
        &Point3::new(50., 10., 0.),
        &Point3::new(50., 50., 0.),
    )?;
    let aspect = Handle::new(Prs3dDimensionAspect::new()?);
    aspect.make_arrows_3d(true);
    aspect.arrow_aspect().set_length(arrow_size);
    aspect.set_text_horizontal_position(Prs3dDimensionTextHorizontalPosition::Right);
    aspect
        .text_aspect()
        .set_color(Color::from_name("YELLOW").expect("YELLOW is a named color"));
    dimension.set_dimension_aspect(&aspect);
    dimension.set_flyout(radius);
    Ok(dimension)
}

#[test]
fn two_angle_calls_keep_distinct_arrow_sizes_and_flyouts() {
    let first = make_angle(50., 5.).unwrap();
    let second = make_angle(30., 3.).unwrap();
    assert_ne!(first.object_address(), second.object_address());
    assert_eq!((first.get_flyout(), second.get_flyout()), (50., 30.));
    assert_eq!(first.dimension_aspect().arrow_aspect().length(), 5.);
    assert_eq!(second.dimension_aspect().arrow_aspect().length(), 3.);
}

#[test]
fn two_calls_display_six_distinct_anonymous_presentations() {
    use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    display_angle(session.context(), 50., 5.).unwrap();
    display_angle(session.context(), 30., 3.).unwrap();
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 6);
    session.close();
}
