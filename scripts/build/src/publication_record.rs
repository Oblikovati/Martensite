//! Shared catalog vocabulary keeps history validation independent of shipment selection.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PublicationReason {
    ChangedSource,
    FirstPublication,
    HistoricalReuse,
}
