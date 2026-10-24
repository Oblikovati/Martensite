//! `ShapeExtendStatus` and `ShapeExtendParametrisation`: the status flags of the Shape
//! Healing tools and the global parametrisation of a composite surface.

/// `ShapeExtendStatus`: one flag of the status a Shape Healing tool records, in the
/// order, so that `status as i32` is the enumerator value.
///
/// A tool keeps its status as a bit field (`ShapeExtend::encode_status` gives the bits
/// of one flag); `Done` and `Fail` stand for any of the eight `DONEi` or `FAILi` flags,
/// and `Ok` for no flag at all.
///
/// ```
/// use martensite_tkshhealing::shape_extend::ShapeExtendStatus;
/// assert_eq!(ShapeExtendStatus::Fail as i32, 18);
/// assert_eq!(ShapeExtendStatus::from_index(9), Some(ShapeExtendStatus::Done));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeExtendStatus {
    /// `ShapeExtendOK`: nothing done, everything OK.
    Ok,
    /// `ShapeExtendDONE1`: something was done, case 1.
    Done1,
    /// `ShapeExtendDONE2`: something was done, case 2.
    Done2,
    /// `ShapeExtendDONE3`: something was done, case 3.
    Done3,
    /// `ShapeExtendDONE4`: something was done, case 4.
    Done4,
    /// `ShapeExtendDONE5`: something was done, case 5.
    Done5,
    /// `ShapeExtendDONE6`: something was done, case 6.
    Done6,
    /// `ShapeExtendDONE7`: something was done, case 7.
    Done7,
    /// `ShapeExtendDONE8`: something was done, case 8.
    Done8,
    /// `ShapeExtendDONE`: something was done (any of the `DONEi`).
    Done,
    /// `ShapeExtendFAIL1`: the method failed, case 1.
    Fail1,
    /// `ShapeExtendFAIL2`: the method failed, case 2.
    Fail2,
    /// `ShapeExtendFAIL3`: the method failed, case 3.
    Fail3,
    /// `ShapeExtendFAIL4`: the method failed, case 4.
    Fail4,
    /// `ShapeExtendFAIL5`: the method failed, case 5.
    Fail5,
    /// `ShapeExtendFAIL6`: the method failed, case 6.
    Fail6,
    /// `ShapeExtendFAIL7`: the method failed, case 7.
    Fail7,
    /// `ShapeExtendFAIL8`: the method failed, case 8.
    Fail8,
    /// `ShapeExtendFAIL`: the method failed (any of the `FAILi`).
    Fail,
}

impl ShapeExtendStatus {
    /// Every enumerator, in order.
    pub const ALL: [Self; 19] = [
        Self::Ok,
        Self::Done1,
        Self::Done2,
        Self::Done3,
        Self::Done4,
        Self::Done5,
        Self::Done6,
        Self::Done7,
        Self::Done8,
        Self::Done,
        Self::Fail1,
        Self::Fail2,
        Self::Fail3,
        Self::Fail4,
        Self::Fail5,
        Self::Fail6,
        Self::Fail7,
        Self::Fail8,
        Self::Fail,
    ];

    /// The enumerator of value `index`, `None` outside `[0, 18]`;
    /// `ShapeExtendStatus::from_index(0) == Some(ShapeExtendStatus::Ok)`.
    pub fn from_index(index: i32) -> Option<Self> {
        usize::try_from(index)
            .ok()
            .and_then(|i| Self::ALL.get(i).copied())
    }
}

/// `ShapeExtendParametrisation`: how a composite surface builds its global
/// parametrisation from its patches, in the order.
///
/// ```
/// use martensite_tkshhealing::shape_extend::ShapeExtendParametrisation;
/// assert_eq!(ShapeExtendParametrisation::Unitary as i32, 2);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeExtendParametrisation {
    /// `ShapeExtendNatural`: each patch of the first row and column adds its range,
    /// `U(i+1) = U(i) + URange(i, 1)`.
    Natural,
    /// `ShapeExtendUniform`: each patch gives the range 1, `U(i) = i - 1`.
    Uniform,
    /// `ShapeExtendUnitary`: a uniform parametrisation of the global range `[0, 1]`.
    Unitary,
}
