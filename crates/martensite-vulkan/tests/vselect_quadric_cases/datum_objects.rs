//! Command-owned real AIS points and axes, including the original 2D point placement.
use crate::case_view::viewer_session::{ViewerSession, corner_persistence};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    resource::ResourceHost,
    standard::{Failure, Handle, SharedState},
    units::Units,
    units_api::UnitsApi,
};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomLine};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkservice::{
    aspect::{AspectTypeOfMarker, AspectTypeOfTriedronPosition},
    graphic3d::{GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags},
};
use martensite_tkv3d::{
    ais::{AisAxis, AisPoint},
    prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers},
};
use std::sync::Arc;

fn units() -> Arc<SharedState<UnitsApi>> {
    Arc::new(SharedState::new(UnitsApi::new(
        Arc::new(Units::new()),
        ResourceHost::native(),
    )))
}

/// The axis through two points; `line_axis(session, "a", [1.,-2.,2.], [1.,2.,2.])?`.
pub(crate) fn line_axis(
    session: &mut ViewerSession,
    name: &str,
    start: [f64; 3],
    end: [f64; 3],
) -> Result<(), Failure> {
    let a = Point3::new(start[0], start[1], start[2]);
    let direction = Direction3::new(end[0] - start[0], end[1] - start[1], end[2] - start[2])?;
    let line = Handle::new(GeomLine::from_point(a, direction));
    session.display_object(name, AisAxis::new(&line, units())?.cast())
}

/// A ray as the native pick command displays it; `ray_axis(session, "axis", origin, direction, -1., NamedColor::GREEN)?`.
pub(crate) fn ray_axis(
    session: &mut ViewerSession,
    name: &str,
    origin: [f64; 3],
    direction: [f64; 3],
    length: f64,
    color: NamedColor,
) -> Result<(), Failure> {
    let point = Point3::new(origin[0], origin[1], origin[2]);
    let dir = Direction3::new(direction[0], direction[1], direction[2])?;
    let axis = AisAxis::from_ray(&Axis1::new(point, dir), length, units())?;
    axis.set_color(Color::named(color))?;
    if length >= 0. {
        axis.set_infinite_state(false);
    }
    session.display_object(name, axis.cast())
}

/// A real command point with the requested marker and optional own colour; `point(session, "p", [0.;3], AspectTypeOfMarker::O, None)?`.
pub(crate) fn point(
    session: &mut ViewerSession,
    name: &str,
    xyz: [f64; 3],
    marker: AspectTypeOfMarker,
    color: Option<NamedColor>,
) -> Result<(), Failure> {
    let geom = Handle::new(GeomCartesianPoint::new(xyz[0], xyz[1], xyz[2]));
    let object = AisPoint::new(&geom.cast())?;
    object.set_marker(marker)?;
    if let Some(color) = color {
        object.set_color(Color::named(color))?;
    }
    session.display_object(name, object.cast())
}

/// The source 2D point flips Y and uses upper-left persistence/TopOSD; `point_2d(session, "p", [320.,318.])?`.
pub(crate) fn point_2d(
    session: &mut ViewerSession,
    name: &str,
    xy: [f64; 2],
) -> Result<(), Failure> {
    let geom = Handle::new(GeomCartesianPoint::new(xy[0], -xy[1], 0.));
    let object = AisPoint::new(&geom.cast())?;
    let persistence = corner_persistence(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        [0, 0],
    )?;
    object.set_transform_persistence(&persistence);
    object.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP_OSD);
    session.display_object(name, object.cast())
}
