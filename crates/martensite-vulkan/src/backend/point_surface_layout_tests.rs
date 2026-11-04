//! Selected point/surface program layouts never append the unrelated sprite ABI.
use super::*;
use crate::point_surface_texture_tests::fixture::point_draw;
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel as Model;

#[test]
fn mapped_point_push_constants_preserve_cutoff_flags_and_arbitrary_size() {
    for size in [1., 3., 17.5] {
        let draw = point_draw(Model::Unlit, size);
        let bytes = encode_transform(
            &draw,
            vk::Extent2D {
                width: 64,
                height: 64,
            },
        );
        assert_eq!(bytes.len(), 80);
        assert_eq!(
            f32::from_ne_bytes(bytes[64..68].try_into().unwrap()),
            0.1_f32.next_up()
        );
        assert_eq!(
            u32::from_ne_bytes(bytes[68..72].try_into().unwrap()) & 10,
            10
        );
        assert_eq!(f32::from_ne_bytes(bytes[76..80].try_into().unwrap()), size);
    }
}
#[test]
fn mapped_points_use_uv_vertex_layout_and_point_assembly_without_sprite_descriptor() {
    let draw = point_draw(Model::Phong, 7.);
    let kind = super::super::pipeline_variants::special_variant(&draw, false);
    let state = super::super::pipeline_state::PipelineState::new(kind);
    assert!(kind.point && kind.surface_texture && kind.lighting);
    assert!(!kind.uses_marker_sampler());
    assert_eq!(state.assembly_topology(), vk::PrimitiveTopology::POINT_LIST);
    let pair = super::super::surface_texture_pipeline::surface_shaders(kind);
    assert_eq!(
        pair[0],
        include_bytes!("../../shaders/surface.lit.image.point.vert.spv")
    );
    assert_eq!(
        pair[1],
        include_bytes!("../../shaders/surface.lit.frag.spv")
    );
}
#[test]
fn ordinary_point_sprite_constants_and_descriptor_selection_remain_unchanged() {
    let mut draw = point_draw(Model::Unlit, 3.);
    draw.surface_texture = None;
    draw.surface_cutoff = None;
    let bytes = encode_transform(
        &draw,
        vk::Extent2D {
            width: 64,
            height: 64,
        },
    );
    assert_eq!(bytes.len(), 80);
    assert_eq!(f32::from_ne_bytes(bytes[64..68].try_into().unwrap()), 3.);
    assert_eq!(&bytes[68..76], &[1, 0, 0, 0, 1, 0, 0, 0]);
    assert!(super::super::pipeline_variants::special_variant(&draw, false).uses_marker_sampler());
}
#[test]
fn both_numeric_light_behaviors_have_actual_mapped_point_vertex_modules() {
    let draw = point_draw(Model::Phong, 3.);
    let kind = super::super::pipeline_variants::special_variant(&draw, false);
    let pair = super::super::surface_texture_pipeline::surface_shaders(kind);
    for behavior in [
        crate::RenderLightDotBehavior::DriverNativeV1,
        crate::RenderLightDotBehavior::NumericMaxZeroV2,
    ] {
        assert!(super::super::light_dot_shader::selected_vertex(behavior, true, pair[0]).is_ok());
    }
}

#[test]
fn missing_image_point_constants_preserve_emission_without_changing_face_masking() {
    let fixture =
        crate::point_surface_texture_tests::fixture::PointTextureFixture::unavailable_base_image(
            false,
        );
    let point = fixture.draws().unwrap().remove(0);
    let extent = vk::Extent2D {
        width: 64,
        height: 64,
    };
    let point_bytes = encode_transform(&point, extent);
    let point_flags = u32::from_ne_bytes(point_bytes[68..72].try_into().unwrap());
    assert_eq!(point_flags & 18, 2);
    let mut face = crate::PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
    face.surface_texture_unavailable = point.surface_texture_unavailable;
    let face_bytes = encode_transform(&face, extent);
    let face_flags = u32::from_ne_bytes(face_bytes[68..72].try_into().unwrap());
    assert_eq!(face_flags & 16, 16);
}
