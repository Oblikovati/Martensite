//! Ordinary errors still restore and attempt the restored image before returning a refusal.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
    Unlit,
    Restored,
}
impl Phase {
    /// Select the fixed exclusive capture destination; `Phase::Unlit.basename()`.
    pub(super) fn basename(self) -> &'static str {
        match self {
            Self::Unlit => "bug27700_unlit_diagnostic.png",
            Self::Restored => "bug27700_restored_diagnostic.png",
        }
    }
}
/// Preserve ordinary errors while restoring and observing; `paired(&mut scope, capture)?`.
pub(super) fn paired(
    scope: &mut scope::PointLightingScope,
    mut capture: impl FnMut(Phase) -> Outcome,
) -> Outcome {
    let controlled = capture(Phase::Unlit);
    let restoration = scope.restore();
    let restored = if restoration.is_ok() {
        capture(Phase::Restored)
    } else {
        Ok(())
    };
    combine(controlled, restoration, restored)
}
fn combine(controlled: Outcome, restoration: io::Result<()>, restored: Outcome) -> Outcome {
    let failures: Vec<_> = [controlled, restoration.map_err(Into::into), restored]
        .into_iter()
        .filter_map(Result::err)
        .map(|error| error.to_string())
        .collect();
    if failures.is_empty() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "point shading diagnostic refused: {}",
        failures.join("; ")
    ))
    .into())
}
