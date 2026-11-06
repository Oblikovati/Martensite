//! Static shader selection admits only the owned text interfaces.
use crate::RenderTextInterpolationBehavior as Behavior;

pub(super) fn selected(behavior: Behavior, clipped: bool) -> Option<[&'static [u8]; 2]> {
    match behavior {
        Behavior::HardwareV1 => None,
        Behavior::ProjectedGrid12V2 => Some(shaders(clipped)),
        Behavior::ProjectedGrid12FlatCornersV4 => Some(flat_shaders(clipped)),
    }
}

pub(super) fn selected_pipeline(
    behavior: Behavior,
    is_text: bool,
    clipped: bool,
) -> Option<[&'static [u8]; 2]> {
    if !is_text {
        return None;
    }
    selected(behavior, clipped)
}

pub(super) fn shaders(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return [
            include_bytes!("../../shaders/text.grid12.clipped.vert.spv"),
            include_bytes!("../../shaders/text.grid12.clipped.frag.spv"),
        ];
    }
    [
        include_bytes!("../../shaders/text.grid12.vert.spv"),
        include_bytes!("../../shaders/text.grid12.frag.spv"),
    ]
}

fn flat_shaders(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return [
            include_bytes!("../../shaders/text.grid12.flat-v4.clipped.vert.spv"),
            include_bytes!("../../shaders/text.grid12.flat-v4.clipped.frag.spv"),
        ];
    }
    [
        include_bytes!("../../shaders/text.grid12.flat-v4.vert.spv"),
        include_bytes!("../../shaders/text.grid12.flat-v4.frag.spv"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_shader_variants_are_distinct_and_text_specific() {
        let ordinary = shaders(false);
        let clipped = shaders(true);
        assert_ne!(ordinary[0], include_bytes!("../../shaders/text.vert.spv"));
        assert_ne!(ordinary[1], include_bytes!("../../shaders/text.frag.spv"));
        assert_ne!(ordinary[0], clipped[0]);
        assert_ne!(ordinary[1], clipped[1]);
    }

    #[test]
    fn hardware_keeps_stock_selection_and_grid_is_explicit() {
        assert!(selected(Behavior::HardwareV1, false).is_none());
        assert_eq!(
            selected(Behavior::ProjectedGrid12V2, false),
            Some(shaders(false))
        );
        assert_eq!(
            selected(Behavior::ProjectedGrid12V2, true),
            Some(shaders(true))
        );
    }

    #[test]
    fn renderer_pipeline_routes_only_text_through_the_selected_grid_shader() {
        assert!(selected_pipeline(Behavior::ProjectedGrid12V2, false, false).is_none());
        assert_eq!(
            selected_pipeline(Behavior::ProjectedGrid12V2, true, false),
            Some(shaders(false))
        );
        assert!(selected_pipeline(Behavior::HardwareV1, true, false).is_none());
    }
    #[test]
    fn core_flat_transport_has_its_own_pair_without_changing_v2_selection() {
        let pair = selected(Behavior::ProjectedGrid12FlatCornersV4, false).unwrap();
        assert_ne!(pair, shaders(false));
        assert_ne!(
            pair,
            selected(Behavior::ProjectedGrid12FlatCornersV4, true).unwrap()
        );
        assert_eq!(
            selected(Behavior::ProjectedGrid12V2, false),
            Some(shaders(false))
        );
        assert!(selected_pipeline(Behavior::ProjectedGrid12FlatCornersV4, false, false).is_none());
    }
}
