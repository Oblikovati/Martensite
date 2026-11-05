//! The enumerations of the topological data structure: the kinds of its entities, the
//! orientation of same domain shapes and the status of a check.

/// The kind of an entity of the data structure (`TopOpeBRepDSKind`): a geometry
/// (point, curve, surface) or a shape type; `TopOpeBRepDSKind::Edge as i32 == 4`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum TopOpeBRepDSKind {
    /// `TopOpeBRepDSPOINT` (also the value of the zeroed storage natively).
    #[default]
    Point,
    /// `TopOpeBRepDSCURVE`.
    Curve,
    /// `TopOpeBRepDSSURFACE`.
    Surface,
    /// `TopOpeBRepDSVERTEX`.
    Vertex,
    /// `TopOpeBRepDSEDGE`.
    Edge,
    /// `TopOpeBRepDSWIRE`.
    Wire,
    /// `TopOpeBRepDSFACE`.
    Face,
    /// `TopOpeBRepDSSHELL`.
    Shell,
    /// `TopOpeBRepDSSOLID`.
    Solid,
    /// `TopOpeBRepDSCOMPSOLID`.
    CompSolid,
    /// `TopOpeBRepDSCOMPOUND`.
    Compound,
    /// `TopOpeBRepDSUNKNOWN`.
    Unknown,
}

/// The orientation of a shape relative to its same domain reference
/// (`TopOpeBRepDSConfig`); `TopOpeBRepDSConfig::SameOriented as i32 == 1`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TopOpeBRepDSConfig {
    /// `TopOpeBRepDSUNSHGEOMETRY`: no shared geometry.
    #[default]
    UnshGeometry,
    /// `TopOpeBRepDSSAMEORIENTED`.
    SameOriented,
    /// `TopOpeBRepDSDIFFORIENTED`.
    DiffOriented,
}

/// The status of a data structure check (`TopOpeBRepDSCheckStatus`);
/// `TopOpeBRepDSCheckStatus::Ok as i32 == 0`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TopOpeBRepDSCheckStatus {
    /// `TopOpeBRepDSOK`.
    #[default]
    Ok,
    /// `TopOpeBRepDSNOK`.
    Nok,
}
