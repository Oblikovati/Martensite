use super::plain_body;
use crate::backend::directional_phong_fixture::lit_surface;
use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
use crate::{PrimitiveDraw, RenderBackend, RenderFrame};

fn frame(draw: PrimitiveDraw) -> RenderFrame {
    RenderFrame::new(48, 32, [0., 0., 0., 1.], vec![draw]).unwrap()
}

#[test]
fn a_kept_upload_draws_what_a_fresh_upload_draws() {
    let mut renderer = create_resource_renderer();
    let draw = lit_surface(6);
    let first = renderer.render(&frame(draw.clone())).unwrap();
    assert!(
        draw.upload_memo.is_kept(),
        "a plain surface keeps its upload"
    );
    // The camera moved: same body, new clip transform.
    let mut moved = draw.clone();
    moved.clip_transform[12] = 0.25;
    let kept = renderer.render(&frame(moved.clone())).unwrap();
    let mut fresh = moved.clone();
    fresh.body = std::sync::Arc::new((*moved.body).clone());
    assert!(
        !fresh.upload_memo.is_kept(),
        "a copied body starts without an upload"
    );
    let reference = renderer.render(&frame(fresh)).unwrap();
    assert_ne!(
        kept.rgba, first.rgba,
        "the kept upload takes the new transform"
    );
    assert_eq!(kept.rgba, reference.rgba);
    assert_eq!(kept.depth, reference.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn edged_lines_and_points_are_not_plain_surfaces() {
    let mut edged = lit_surface(6);
    edged.polygon_edge_style = Some(crate::LineStyle::new(1., 0xFFFF, 1).unwrap());
    assert!(plain_body(&lit_surface(6)));
    assert!(!plain_body(&edged));
}
