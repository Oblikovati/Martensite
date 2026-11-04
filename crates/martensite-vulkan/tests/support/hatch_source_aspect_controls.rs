//! One toolkit registration exercises the actual released aspect handles without a GPU.
use super::scene::source_aspect::apply_source_hatch_pattern;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{
    aspect::{AspectHatchStyle, AspectInteriorStyle},
    graphic3d::*,
};

#[test]
fn actual_null_hatch_handle_gets_vertical_for_interior_only() {
    let aspect = Graphic3dAspectFillArea3d::default();
    aspect.set_hatch_style(Handle::default());
    assert!(
        aspect.hatch_style().is_null(),
        "actual_null={}; expected explicit null-handle fixture",
        aspect.hatch_style().is_null()
    );
    apply_source_hatch_pattern(&aspect, None);
    assert_eq!(aspect.interior_style(), AspectInteriorStyle::Hatch);
    assert!(
        !aspect.hatch_style().is_null(),
        "actual_null={}; expected nonnull Vertical",
        aspect.hatch_style().is_null()
    );
    assert_eq!(
        aspect.hatch_style().hatch_type(),
        AspectHatchStyle::Vertical as i32
    );
}

#[test]
fn actual_nonnull_handles_including_solid_zero_keep_identity() {
    for style in [
        AspectHatchStyle::Solid,
        AspectHatchStyle::GridWide,
        AspectHatchStyle::Horizontal,
    ] {
        let aspect = Graphic3dAspectFillArea3d::default();
        let existing = Handle::new(Graphic3dHatchStyle::predefined(style));
        aspect.set_hatch_style(existing.clone());
        apply_source_hatch_pattern(&aspect, None);
        assert_eq!(aspect.interior_style(), AspectInteriorStyle::Hatch);
        let actual = aspect.hatch_style();
        assert!(
            actual == existing,
            "style={style:?}, actual_null={}; expected same existing nonnull handle",
            actual.is_null()
        );
        assert_eq!(actual.hatch_type(), style as i32);
    }
}
