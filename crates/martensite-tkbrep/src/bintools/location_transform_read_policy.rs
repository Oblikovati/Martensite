//! Explicit interpretation of stored location transforms (#451).
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Transform3;

/// Selects location reconstruction independently of format and analytic surface policy.
/// `LocationTransformReadPolicy::PreserveStoredRigidV1` retains admitted rigid coefficients.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LocationTransformReadPolicy {
    /// Reconstructs similarities with determinant scaling and orthogonalization.
    #[default]
    LegacyReplay,
    /// Retains checked rigid coefficients; refuses scaled, sheared or inconclusive inputs.
    PreserveStoredRigidV1,
}
impl LocationTransformReadPolicy {
    pub(super) fn decode(self, rows: [[f64; 4]; 3]) -> Result<Transform3, Failure> {
        if self == Self::PreserveStoredRigidV1 {
            return Transform3::from_stored_rigid_coefficients(rows);
        }
        let mut transform = Transform3::default();
        transform.set_values(rows)?;
        Ok(transform)
    }
}
