//! Preserve every diagnostic cause while cleanup and the authored source continue.
use super::*;
#[derive(Debug)]
struct PairedSphereFailure {
    first: Box<dyn std::error::Error>,
    second: Box<dyn std::error::Error>,
}
impl std::fmt::Display for PairedSphereFailure {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "sphere diagnostic first=[{}]; additional=[{}]",
            self.first, self.second
        )
    }
}
impl std::error::Error for PairedSphereFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.first.as_ref())
    }
}
pub(super) fn join_outcomes(first: Outcome, second: Outcome) -> Outcome {
    match (first, second) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(first), Err(second)) => Err(Box::new(PairedSphereFailure { first, second })),
    }
}
