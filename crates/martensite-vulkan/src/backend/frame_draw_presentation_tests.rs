use super::from_semantic_draw;
use crate::{LineStyle, PrimitiveDraw, PrimitiveTopology, RenderPolygonOffset, RenderVertex};

#[test]
fn fill_offset_stays_disabled_when_semantic_line_uses_private_quad_geometry() {
    let semantic = line_with_fill_offset();
    let private = crate::thin_line::selected_draw(
        crate::RenderThinLineCoverage::DiamondExitV2,
        crate::RenderThinAxisCoverage::HardwareV1,
        &semantic,
        ash::vk::Extent2D {
            width: 32,
            height: 32,
        },
        true,
    )
    .unwrap()
    .unwrap();
    assert_eq!(private.topology(), PrimitiveTopology::Triangles);
    let presentation = from_semantic_draw(&semantic);
    assert_eq!(presentation.polygon_offset, semantic.polygon_offset());
    assert!(!presentation.polygon);
    assert_eq!(
        fill_bias(presentation.polygon_offset, presentation.polygon),
        [0., 0.]
    );
}

fn line_with_fill_offset() -> PrimitiveDraw {
    let vertices = vec![
        RenderVertex::new([-0.5, 0., 0.], [0., 1., 1., 1.]),
        RenderVertex::new([0.5, 0., 0.], [0., 1., 1., 1.]),
    ];
    PrimitiveDraw::segments(
        vertices,
        vec![0, 1],
        crate::identity_clip(),
        LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap()
    .with_polygon_offset(RenderPolygonOffset::new(1, 1.5, -2.).unwrap())
}

fn fill_bias(offset: Option<RenderPolygonOffset>, is_polygon: bool) -> [f32; 2] {
    let mut state = crate::polygon_offset::PolygonOffsetState::default();
    state.advance(offset, is_polygon)
}
