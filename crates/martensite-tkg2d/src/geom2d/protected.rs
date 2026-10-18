//! Crate-private construction authority for shared geometry state.
//!
//! Abstract geometry traits expose shared state behind
//! [`SharedState`](martensite_tkernel::standard::SharedState), while callers
//! outside this crate cannot construct the access token.
/// Proof that a caller is inside this crate, for the defined `protected` members
/// that abstract classes share with their subclasses (`Geom2dVector::gpVec2d`,
/// `Geom2dConic::pos`). Other crates cannot build a value of it.
pub struct ProtectedAccess {
    _private: (),
}
pub(crate) const PROTECTED: ProtectedAccess = ProtectedAccess { _private: () };
