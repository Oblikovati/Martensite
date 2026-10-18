//! Output mirrors validate initialized bits before any operation mutation.
use super::*;
impl AdvApp2VarIsoOperationFrame {
    pub(super) fn validate_real_output_mirrors<Role: std::fmt::Debug, const COUNT: usize>(
        &self,
        origins: Option<[usize; COUNT]>,
        roles: [Role; COUNT],
        mirrors: [&[f64]; COUNT],
    ) -> Result<(), Failure> {
        let Some(origins) = origins else {
            return Ok(());
        };
        for ((origin, role), mirror) in origins.into_iter().zip(roles).zip(mirrors) {
            validate_real_output_mirror(&self.reals, origin, &role, mirror)?;
        }
        Ok(())
    }
    pub(super) fn synchronize_real_output_mirrors<const COUNT: usize>(
        &self,
        origins: Option<[usize; COUNT]>,
        mirrors: [&mut [f64]; COUNT],
    ) -> Result<(), Failure> {
        let Some(origins) = origins else {
            return Ok(());
        };
        for (origin, mirror) in origins.into_iter().zip(mirrors) {
            for (index, value) in mirror.iter_mut().enumerate() {
                *value = self.reals.output_value(origin, index as i32)?;
            }
        }
        Ok(())
    }
}
fn validate_real_output_mirror<Role: std::fmt::Debug>(
    reals: &AdvApp2VarIsoRealFrame,
    origin: usize,
    role: &Role,
    mirror: &[f64],
) -> Result<(), Failure> {
    for (index, value) in mirror.iter().enumerate() {
        let actual = reals.output_value(origin, index as i32)?;
        if actual.to_bits() != value.to_bits() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoOperationFrame: {role:?} mirror[{index}] bits={:#x}; expected initialized caller bits={:#x}",
                    value.to_bits(),
                    actual.to_bits()
                ),
            ));
        }
    }
    Ok(())
}
