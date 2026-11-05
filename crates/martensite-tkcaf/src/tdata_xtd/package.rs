//! `TDataXtd`: the package functions and the enumerations of the extended attributes.
use super::{
    TDataXtdAxis, TDataXtdConstraint, TDataXtdGeometry, TDataXtdPatternStd, TDataXtdPlacement,
    TDataXtdPlane, TDataXtdPoint, TDataXtdPosition, TDataXtdShape,
};
use martensite_tkernel::standard::{Guid, Handle};
use martensite_tklcaf::tdf::{TdfAttribute, TdfDerivedAttribute};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

/// `TDataXtdGeometryEnum`, in native order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum TDataXtdGeometryEnum {
    /// `TDataXtdANY_GEOM`.
    #[default]
    AnyGeom,
    /// `TDataXtdPOINT`.
    Point,
    /// `TDataXtdLINE`.
    Line,
    /// `TDataXtdCIRCLE`.
    Circle,
    /// `TDataXtdELLIPSE`.
    Ellipse,
    /// `TDataXtdSPLINE`.
    Spline,
    /// `TDataXtdPLANE`.
    Plane,
    /// `TDataXtdCYLINDER`.
    Cylinder,
}

/// `TDataXtdConstraintEnum`, in native order (2d planar constraints, then placement
/// constraints).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum TDataXtdConstraintEnum {
    /// `TDataXtdRADIUS`.
    #[default]
    Radius,
    /// `TDataXtdDIAMETER`.
    Diameter,
    /// `TDataXtdMINOR_RADIUS`.
    MinorRadius,
    /// `TDataXtdMAJOR_RADIUS`.
    MajorRadius,
    /// `TDataXtdTANGENT`.
    Tangent,
    /// `TDataXtdPARALLEL`.
    Parallel,
    /// `TDataXtdPERPENDICULAR`.
    Perpendicular,
    /// `TDataXtdCONCENTRIC`.
    Concentric,
    /// `TDataXtdCOINCIDENT`.
    Coincident,
    /// `TDataXtdDISTANCE`.
    Distance,
    /// `TDataXtdANGLE`.
    Angle,
    /// `TDataXtdEQUAL_RADIUS`.
    EqualRadius,
    /// `TDataXtdSYMMETRY`.
    Symmetry,
    /// `TDataXtdMIDPOINT`.
    Midpoint,
    /// `TDataXtdEQUAL_DISTANCE`.
    EqualDistance,
    /// `TDataXtdFIX`.
    Fix,
    /// `TDataXtdRIGID`.
    Rigid,
    /// `TDataXtdFROM`.
    From,
    /// `TDataXtdAXIS`.
    Axis,
    /// `TDataXtdMATE`.
    Mate,
    /// `TDataXtdALIGN_FACES`.
    AlignFaces,
    /// `TDataXtdALIGN_AXES`.
    AlignAxes,
    /// `TDataXtdAXES_ANGLE`.
    AxesAngle,
    /// `TDataXtdFACES_ANGLE`.
    FacesAngle,
    /// `TDataXtdROUND`.
    Round,
    /// `TDataXtdOFFSET`.
    Offset,
}

/// The constraint names of `TDataXtd::Print`, in enumeration order.
const CONSTRAINT_NAMES: [&str; 26] = [
    "RADIUS",
    "DIAMETER",
    "MINOR_RADIUS",
    "MAJOR_RADIUS",
    "TANGENT",
    "PARALLEL",
    "PERPENDICULAR",
    "CONCENTRIC",
    "COINCIDENT",
    "DISTANCE",
    "ANGLE",
    "EQUAL_RADIUS",
    "SYMMETRY",
    "MIDPOINT",
    "EQUAL_DISTANCE",
    "FIX",
    "RIGID",
    "FROM",
    "AXIS",
    "MATE",
    "ALIGN_FACES",
    "ALIGN_AXES",
    "AXES_ANGLE",
    "FACES_ANGLE",
    "ROUND",
    "OFFSET",
];

/// The geometry names of `TDataXtd::Print`, in enumeration order.
const GEOMETRY_NAMES: [&str; 8] = [
    "ANY_GEOM", "POINT", "LINE", "CIRCLE", "ELLIPSE", "SPLINE", "PLANE", "CYLINDER",
];

/// `TDataXtd`: package functions.
///
/// ```
/// use martensite_tkcaf::tdata_xtd::{TDataXtd, TDataXtdConstraintEnum};
/// let mut text = Vec::new();
/// TDataXtd::print_constraint(TDataXtdConstraintEnum::AlignFaces, &mut text).unwrap();
/// assert_eq!(text, b"ALIGN_FACES");
/// ```
pub struct TDataXtd;

impl TDataXtd {
    /// `IDList(anIDList)`: the IDs of the TDataXtd attributes (not Presentation or
    /// Triangulation), in the native order.
    pub fn id_list(ids: &mut Vec<Guid>) {
        ids.extend([
            TDataXtdAxis::get_id(),
            TDataXtdConstraint::get_id(),
            TDataXtdGeometry::get_id(),
            TDataXtdPatternStd::get_id(),
            TDataXtdPlacement::get_id(),
            TDataXtdPoint::get_id(),
            TDataXtdPlane::get_id(),
            TDataXtdPosition::get_id(),
            TDataXtdShape::get_id(),
        ]);
    }
    /// `Print(GEO, S)`: the name of a geometry type.
    pub fn print_geometry(
        geometry: TDataXtdGeometryEnum,
        stream: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(stream, "{}", GEOMETRY_NAMES[geometry as usize])
    }
    /// `Print(CTR, S)`: the name of a constraint type.
    pub fn print_constraint(
        constraint: TDataXtdConstraintEnum,
        stream: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(stream, "{}", CONSTRAINT_NAMES[constraint as usize])
    }
    /// The `IMPLEMENT_DERIVED_ATTRIBUTE` registrations of martensite-tkcaf (Shape, Axis, Point, Plane,
    /// Placement, in the native link order). Natively the library load registers them; the
    /// implementation registers them once, when the first of these attributes is created.
    pub fn register_derived_attributes() {
        // A flag, not a `Once`: the registry builds its prototypes with these constructors,
        // which call back here while the registration may still be running.
        static REGISTERED: AtomicBool = AtomicBool::new(false);
        if REGISTERED.swap(true, Ordering::SeqCst) {
            return;
        }
        let creators: [fn() -> Handle<dyn TdfAttribute>; 5] = [
            || Handle::new(TDataXtdShape::new()).cast(),
            || Handle::new(TDataXtdAxis::new()).cast(),
            || Handle::new(TDataXtdPoint::new()).cast(),
            || Handle::new(TDataXtdPlane::new()).cast(),
            || Handle::new(TDataXtdPlacement::new()).cast(),
        ];
        for create in creators {
            TdfDerivedAttribute::register(create, None, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_follow_the_enumerations() {
        let mut text = Vec::new();
        TDataXtd::print_geometry(TDataXtdGeometryEnum::Cylinder, &mut text).unwrap();
        TDataXtd::print_constraint(TDataXtdConstraintEnum::Offset, &mut text).unwrap();
        TDataXtd::print_constraint(TDataXtdConstraintEnum::Radius, &mut text).unwrap();
        assert_eq!(text, b"CYLINDEROFFSETRADIUS");
        let mut ids = Vec::new();
        TDataXtd::id_list(&mut ids);
        assert_eq!(ids.len(), 9);
        assert_eq!(ids[0], TDataXtdAxis::get_id());
    }
}
