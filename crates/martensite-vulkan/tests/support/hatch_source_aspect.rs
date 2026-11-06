//! Interior-only source commands use actual style ownership rather than a type-number proxy.
use martensite_tkservice::{
    aspect::{AspectHatchStyle, AspectInteriorStyle},
    graphic3d::*,
};

/// Apply the actual null-handle fallback; `apply_source_hatch_pattern(&aspect, None)`.
pub(crate) fn apply_source_hatch_pattern(
    aspect: &Graphic3dAspectFillArea3d,
    explicit: Option<AspectHatchStyle>,
) {
    aspect.set_interior_style(AspectInteriorStyle::Hatch);
    if let Some(style) = explicit {
        aspect.set_hatch_style_type(style);
    } else if aspect.hatch_style().is_null() {
        // ViewerCommands sets Vertical only when the actual handle is absent.
        aspect.set_hatch_style_type(AspectHatchStyle::Vertical);
    }
}
