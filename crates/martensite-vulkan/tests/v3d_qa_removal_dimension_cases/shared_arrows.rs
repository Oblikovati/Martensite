//! BUC60915 mutates a single aspect between real dimension Display calls.
use super::{Outcome, shared_arrow_shapes as shapes};
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tkv3d::{
    ais::AisInteractiveContext,
    prs_dim::{PrsDimDimensionMembers, PrsDimLengthDimension, PrsDimRadiusDimension},
    prs3d::Prs3dDimensionAspect,
};

pub(super) struct ArrowDimensions {
    pub lengths: [Handle<PrsDimLengthDimension>; 3],
    pub radii: [Handle<PrsDimRadiusDimension>; 3],
    pub aspect: Handle<Prs3dDimensionAspect>,
    pub vertices: Vec<Shape>,
}

pub(super) fn display_dimensions(
    context: &AisInteractiveContext,
) -> Result<ArrowDimensions, Failure> {
    let vertices = shapes::vertices()?;
    let plane = GeomPlane::from_point(Point3::new(0., 0., 0.), Direction3::new(0., 0., 1.)?)?;
    let aspect = Handle::new(Prs3dDimensionAspect::new()?);
    aspect.make_arrows_3d(true);
    aspect
        .arrow_aspect()
        .set_angle(std::f64::consts::PI / 180. * 10.)?;
    let lengths = display_lengths(context, &vertices, &plane.pln(), &aspect)?;
    let radii = display_radii(context, &aspect)?;
    Ok(ArrowDimensions {
        lengths,
        radii,
        aspect,
        vertices,
    })
}

fn display_lengths(
    context: &AisInteractiveContext,
    vertices: &[Shape],
    plane: &Plane3,
    aspect: &Handle<Prs3dDimensionAspect>,
) -> Result<[Handle<PrsDimLengthDimension>; 3], Failure> {
    let first = PrsDimLengthDimension::from_shapes(&vertices[1], &vertices[2], plane)?;
    aspect.arrow_aspect().set_length(30.);
    first.set_dimension_aspect(aspect);
    context.display(&first.cast(), false)?;
    let second = PrsDimLengthDimension::from_shapes(&vertices[6], &vertices[3], plane)?;
    second.set_dimension_aspect(aspect);
    second.set_flyout(30.);
    aspect.arrow_aspect().set_length(100.);
    context.display(&second.cast(), false)?;
    let third = PrsDimLengthDimension::from_shapes(&vertices[0], &vertices[1], plane)?;
    third.set_dimension_aspect(aspect);
    context.display(&third.cast(), false)?;
    Ok([first, second, third])
}

fn display_radii(
    context: &AisInteractiveContext,
    aspect: &Handle<Prs3dDimensionAspect>,
) -> Result<[Handle<PrsDimRadiusDimension>; 3], Failure> {
    let first = PrsDimRadiusDimension::from_shape(&shapes::first_radius_edge()?)?;
    first.set_dimension_aspect(aspect);
    context.display(&first.cast(), false)?;
    let second = complete_radius([191.09, -88.58, 0.], 1., 43.80)?;
    aspect.arrow_aspect().set_length(60.);
    second.set_dimension_aspect(aspect);
    context.display(&second.cast(), false)?;
    let third = complete_radius([566.11, -88.58, 0.], -1., 17.86)?;
    aspect.arrow_aspect().set_length(40.);
    third.set_dimension_aspect(aspect);
    context.display(&third.cast(), true)?;
    Ok([first, second, third])
}

fn complete_radius(
    center: [f64; 3],
    normal_z: f64,
    radius: f64,
) -> Result<Handle<PrsDimRadiusDimension>, Failure> {
    let edge = shapes::complete_radius_edge(center, normal_z, radius)?;
    PrsDimRadiusDimension::from_shape(&edge)
}

/// Full original without an added FitAll or projection change; `shared_dimension_arrows()?`.
pub(crate) fn shared_dimension_arrows() -> Outcome {
    banner(&["===========", "BUC60915", "==========="]);
    let mut run = SessionRun::new()?;
    display_dimensions(run.session().context())?;
    if !run.session().check_color(300, 164, [0.51, 0.98, 0.])? {
        println!(
            "Error : Cannot set the arrow size when using AisLengthDimension or AisRadiusDimensin."
        );
    }
    run.dump("buc60915.png")?;
    run.finish()
}
