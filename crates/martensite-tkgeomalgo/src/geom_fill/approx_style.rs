//! `GeomFill_ApproxStyle`: which part of a sweep drives its approximation.

/// The approximation style (`GeomFill_ApproxStyle`), with the native values;
/// `GeomFillApproxStyle::Location as i32 == 1`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GeomFillApproxStyle {
    /// `GeomFill_Section`.
    #[default]
    Section = 0,
    /// `GeomFill_Location`.
    Location = 1,
}
