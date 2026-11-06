//! What the offset and sweep commands keep across steps, as the reference command files' statics do.
use martensite_tkernel::precision;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::{
    brep_offset::BRepOffsetMakeOffset, brep_offset_api::BRepOffsetAPIMakePipeShell,
};

/// The state of `offsetload` and `mksweep`.
pub struct CommandState {
    pub offset: BRepOffsetMakeOffset,
    pub has_caps: bool,
    pub tolerance: f64,
    pub complete: bool,
    pub join: JoinType,
    pub remove_internal_edges: bool,
    pub sweep: Option<BRepOffsetAPIMakePipeShell>,
}

impl Default for CommandState {
    fn default() -> Self {
        Self {
            offset: BRepOffsetMakeOffset::new(),
            has_caps: false,
            tolerance: precision::confusion(),
            complete: false,
            join: JoinType::Arc,
            remove_internal_edges: false,
            sweep: None,
        }
    }
}
