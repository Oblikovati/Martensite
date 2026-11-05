//! FilletSurf: the fillet surfaces of a contour without building the solid
//! ([`FilletSurfBuilder`]), computed by a ChFi3d fillet builder that only walks the
//! contour ([`FilletSurfInternalBuilder`]).
mod builder;
mod internal_add;
mod internal_builder;
mod internal_ops;

pub use builder::FilletSurfBuilder;
pub use internal_builder::FilletSurfInternalBuilder;

/// Where the ends of a fillet surface lie (`FilletSurfStatusType`);
/// `StatusType::NoExtremityOnEdge as i32 == 2`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum StatusType {
    /// `FilletSurfTwoExtremityOnEdge`: both ends on edges of the faces.
    TwoExtremityOnEdge = 0,
    /// `FilletSurfOneExtremityOnEdge`: one end on an edge.
    OneExtremityOnEdge = 1,
    /// `FilletSurfNoExtremityOnEdge`: no end on an edge.
    NoExtremityOnEdge = 2,
}

/// The result of the computation (`FilletSurfStatusDone`);
/// `StatusDone::IsPartial as i32 == 2`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum StatusDone {
    /// `FilletSurfIsOk`.
    IsOk = 0,
    /// `FilletSurfIsNotOk`.
    IsNotOk = 1,
    /// `FilletSurfIsPartial`: some fillets are computed.
    IsPartial = 2,
}

/// Why the computation failed (`FilletSurfErrorTypeStatus`);
/// `ErrorTypeStatus::PbFilletCompute as i32 == 5`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorTypeStatus {
    /// `FilletSurfEmptyList`: no edge given.
    EmptyList = 0,
    /// `FilletSurfEdgeNotG1`: the edges are not G1.
    EdgeNotG1 = 1,
    /// `FilletSurfFacesNotG1`: the faces are not G1.
    FacesNotG1 = 2,
    /// `FilletSurfEdgeNotOnShape`: an edge is not on the shape.
    EdgeNotOnShape = 3,
    /// `FilletSurfNotSharpEdge`: an edge is not sharp.
    NotSharpEdge = 4,
    /// `FilletSurfPbFilletCompute`: the fillet computation failed.
    PbFilletCompute = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_follow_the_native_order() {
        assert_eq!(StatusType::NoExtremityOnEdge as i32, 2);
        assert_eq!(StatusDone::IsNotOk as i32, 1);
        assert_eq!(ErrorTypeStatus::NotSharpEdge as i32, 4);
    }
}
