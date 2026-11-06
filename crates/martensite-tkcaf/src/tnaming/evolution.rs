//! The two enumerations of TNaming: how a named shape was produced and how a name
//! re-finds a selected shape.

/// `TNamingEvolution`: the kind of a `TNamingNamedShape`, in native order.
///
/// ```
/// use martensite_tkcaf::tnaming::TNamingEvolution;
/// assert_eq!(TNamingEvolution::Modify as i32, 2);
/// assert_eq!(TNamingEvolution::default(), TNamingEvolution::Primitive);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum TNamingEvolution {
    /// `TNamingPRIMITIVE`: new shapes without an origin.
    #[default]
    Primitive,
    /// `TNamingGENERATED`: new shapes generated from old ones.
    Generated,
    /// `TNamingMODIFY`: old shapes modified into new ones.
    Modify,
    /// `TNamingDELETE`: old shapes deleted.
    Delete,
    /// `TNamingREPLACE`: kept for compatibility (loaded as `GENERATED`).
    Replace,
    /// `TNamingSELECTED`: a selection of a shape in a context.
    Selected,
}

impl TNamingEvolution {
    /// The enumerator of a native integer value (`(TNamingEvolution)i`), `None` out of range;
    /// `TNamingEvolution::from_index(3) == Some(TNamingEvolution::Delete)`.
    pub fn from_index(index: i32) -> Option<Self> {
        const ALL: [TNamingEvolution; 6] = [
            TNamingEvolution::Primitive,
            TNamingEvolution::Generated,
            TNamingEvolution::Modify,
            TNamingEvolution::Delete,
            TNamingEvolution::Replace,
            TNamingEvolution::Selected,
        ];
        usize::try_from(index)
            .ok()
            .and_then(|i| ALL.get(i).copied())
    }
}

/// `TNamingNameType`: the algorithm of a `TNamingName`, in native order.
///
/// ```
/// use martensite_tkcaf::tnaming::TNamingNameType;
/// assert_eq!(TNamingNameType::Orientation as i32, 9);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum TNamingNameType {
    /// `TNamingUNKNOWN`.
    #[default]
    Unknown,
    /// `TNamingIDENTITY`.
    Identity,
    /// `TNamingMODIFUNTIL`.
    ModifUntil,
    /// `TNamingGENERATION`.
    Generation,
    /// `TNamingINTERSECTION`.
    Intersection,
    /// `TNamingUNION`.
    Union,
    /// `TNamingSUBSTRACTION`.
    Substraction,
    /// `TNamingCONSTSHAPE`.
    ConstShape,
    /// `TNamingFILTERBYNEIGHBOURGS`.
    FilterByNeighbourgs,
    /// `TNamingORIENTATION`.
    Orientation,
    /// `TNamingWIREIN`.
    WireIn,
    /// `TNamingSHELLIN`.
    ShellIn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolution_indexes_follow_the_native_order() {
        for index in 0..6 {
            let evolution = TNamingEvolution::from_index(index).unwrap();
            assert_eq!(evolution as i32, index);
        }
        assert_eq!(TNamingEvolution::from_index(6), None);
        assert_eq!(TNamingEvolution::from_index(-1), None);
    }

    #[test]
    fn name_types_follow_the_native_order() {
        assert_eq!(TNamingNameType::Unknown as i32, 0);
        assert_eq!(TNamingNameType::ShellIn as i32, 11);
    }
}
