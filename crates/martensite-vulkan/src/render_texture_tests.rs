use super::*;

#[test]
fn a_framebuffer_image_answers_the_native_texture_queries() {
    let texture = RenderTexture::new(RenderTextureAttachment::DepthStencil, [64, 48]);
    assert_eq!(texture.size(), [64, 48, 1]);
    assert_eq!(
        (texture.size_x(), texture.size_y(), texture.size_z()),
        (64, 48, 1)
    );
    assert_eq!((texture.nb_samples(), texture.max_mipmap_level()), (1, 0));
    assert!(texture.is_valid() && texture.is_top_down());
    assert!(!texture.has_mipmaps() && !texture.is_alpha() && !texture.is_point_sprite());
    assert_eq!(texture.attachment(), RenderTextureAttachment::DepthStencil);
}
