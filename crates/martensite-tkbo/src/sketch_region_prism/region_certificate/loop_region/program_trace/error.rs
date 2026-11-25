// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProgramFailure {
    pub role: IntegralRole,
    pub loops: Vec<Vec<AuthoredLoopUse>>,
    pub consumed: u64,
    pub cause: Box<IntegralLedgerError>,
}
impl ProgramFailure {
    pub(super) fn new(
        role: IntegralRole,
        loops: Vec<Vec<AuthoredLoopUse>>,
        consumed: u64,
        cause: IntegralLedgerError,
    ) -> Self {
        Self {
            role,
            loops,
            consumed,
            cause: Box::new(cause),
        }
    }
    pub(crate) fn offender(&self) -> Option<RegionSourceId> {
        let IntegralLedgerError::Math(error) = self.cause.as_ref() else {
            return None;
        };
        self.loops
            .iter()
            .flatten()
            .nth(error.occurrence_index?)
            .map(|item| item.source)
    }
    pub(crate) fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}
impl std::fmt::Display for ProgramFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "integral role {:?}, source {:?}, graph cells {}: {}",
            self.role,
            self.offender(),
            self.consumed,
            self.cause
        )
    }
}
impl std::error::Error for ProgramFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
