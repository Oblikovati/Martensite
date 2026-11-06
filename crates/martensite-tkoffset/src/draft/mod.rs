//! `Draft`: the draft angle modification (`DraftModification`) and its records
//! (`DraftEdgeInfo`, `DraftFaceInfo`, `DraftVertexInfo`, `DraftErrorStatus`).
mod info;
mod modification;

pub use info::{DraftEdgeInfo, DraftErrorStatus, DraftFaceInfo, DraftVertexInfo};
pub use modification::DraftModification;
