//! Proof that a caller is inside this crate, for the retained `protected` members
//! that abstract classes share with their subclasses (`GeomVector::gpVec`,
//! `GeomConic::pos`, `GeomElementarySurface::pos`...). Other crates cannot
//! build a value of it.

/// The access token of the protected trait members; only this crate builds one.
pub struct ProtectedAccess {
    _private: (),
}
pub(crate) const PROTECTED: ProtectedAccess = ProtectedAccess { _private: () };
