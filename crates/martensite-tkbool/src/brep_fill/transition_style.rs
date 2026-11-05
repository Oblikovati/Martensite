//! The transition modes of a sweep at the corners of its spine, and the contact
//! kinds of a pipe section with its spine.

/// `BRepFillTransitionStyle`: how the sweep treats the discontinuities of the spine;
/// `TransitionStyle::Round as i32 == 2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TransitionStyle {
    /// `BRepFillModified`: the discontinuities are treated by modification of the
    /// sweeping mode.
    #[default]
    Modified = 0,
    /// `BRepFillRight`: right corners (the surfaces are extended and intersected).
    Right = 1,
    /// `BRepFillRound`: round corners (a pipe of revolution fills the gaps).
    Round = 2,
}

/// `BRepFillTypeOfContact`: the contact of a section with its spine;
/// `TypeOfContact::ContactOnBorder as i32 == 2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TypeOfContact {
    /// `BRepFillNoContact`: the section is placed as it is.
    #[default]
    NoContact = 0,
    /// `BRepFillContact`: the section is moved to touch the spine.
    Contact = 1,
    /// `BRepFillContactOnBorder`: the section is moved to touch the spine at its border.
    ContactOnBorder = 2,
}

#[cfg(test)]
mod tests {
    use super::{TransitionStyle, TypeOfContact};

    #[test]
    fn discriminants_follow_the_native_declaration_order() {
        assert_eq!(TransitionStyle::Modified as i32, 0);
        assert_eq!(TransitionStyle::Right as i32, 1);
        assert_eq!(TransitionStyle::Round as i32, 2);
        assert_eq!(TypeOfContact::NoContact as i32, 0);
        assert_eq!(TypeOfContact::Contact as i32, 1);
        assert_eq!(TypeOfContact::ContactOnBorder as i32, 2);
    }
}
