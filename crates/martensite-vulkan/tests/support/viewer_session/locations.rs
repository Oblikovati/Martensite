//! Local transformations of named objects, composed with their current location.
use super::*;
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

/// One change of an object's local transformation; angles are in degrees.
/// Example: `LocationChange::PreScale { center: [0.; 3], factor: 0.5 }`.
#[derive(Clone, Copy, Debug)]
pub enum LocationChange<'a> {
    /// Translation applied after the current transformation.
    Translate([f64; 3]),
    /// Rotation about an axis applied before the current transformation.
    PreRotate {
        /// A point of the axis.
        origin: [f64; 3],
        /// The axis direction.
        axis: [f64; 3],
        /// The angle in degrees.
        degrees: f64,
    },
    /// Rotation about an axis applied after the current transformation.
    Rotate {
        /// A point of the axis.
        origin: [f64; 3],
        /// The axis direction.
        axis: [f64; 3],
        /// The angle in degrees.
        degrees: f64,
    },
    /// Scaling about a centre applied before the current transformation.
    PreScale {
        /// The fixed point.
        center: [f64; 3],
        /// The scale factor.
        factor: f64,
    },
    /// Mirror in a plane, applied after the current transformation.
    Mirror {
        /// A point of the plane.
        origin: [f64; 3],
        /// The plane normal.
        normal: [f64; 3],
    },
    /// The local transformation of another named object.
    CopyFrom(&'a str),
    /// The identity.
    Reset,
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn direction([x, y, z]: [f64; 3]) -> Result<Direction3, Failure> {
    Direction3::new(x, y, z)
}

impl LocationChange<'_> {
    /// The new local transformation from the `current` one.
    fn applied(&self, current: Transform3, session: &ViewerSession) -> Result<Transform3, Failure> {
        let mut trsf = Transform3::default();
        match *self {
            Self::Translate([x, y, z]) => trsf.set_translation_part(Vector3::new(x, y, z)),
            Self::PreRotate {
                origin,
                axis,
                degrees,
            } => {
                let axis = Axis1::new(point(origin), direction(axis)?);
                trsf.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))?;
                return Ok(current.multiplied(trsf));
            }
            Self::Rotate {
                origin,
                axis,
                degrees,
            } => {
                let axis = Axis1::new(point(origin), direction(axis)?);
                trsf.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))?;
            }
            Self::PreScale { center, factor } => {
                trsf.set_scale(point(center), factor)?;
                return Ok(current.multiplied(trsf));
            }
            Self::Mirror { origin, normal } => {
                trsf.set_mirror_plane(Axis2::new(point(origin), direction(normal)?)?)
            }
            Self::CopyFrom(name) => return Ok(session.located(name)?.local_transformation()),
            Self::Reset => return Ok(trsf),
        }
        Ok(trsf.multiplied(current))
    }
}

impl ViewerSession {
    /// Change the local transformation of the object `name`.
    /// Example: `session.change_location("z1", LocationChange::Translate([10., 0., 0.]))?`.
    pub fn change_location(&self, name: &str, change: LocationChange<'_>) -> Result<(), Failure> {
        let object = self.located(name)?.clone();
        let trsf = change.applied(object.local_transformation(), self)?;
        let location = Location::new(trsf);
        self.with_update(|context| context.set_location(&object, &location))
    }

    pub(super) fn located(&self, name: &str) -> Result<&Handle<dyn AisInteractiveObject>, Failure> {
        self.object(name).ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} is not displayed; expected a displayed object name"
            ))
        })
    }
}
