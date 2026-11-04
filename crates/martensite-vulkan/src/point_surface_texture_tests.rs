//! UV point semantics are pinned before device rendering, without accepting sprite fallbacks.
use martensite_tkservice::{aspect::AspectTypeOfMarker, graphic3d::*};
#[path = "point_surface_texture_fixture.rs"]
pub(crate) mod fixture;
use fixture::{PointTextureFixture, point_draw};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};

#[test]
fn unlit_mapped_points_extract_actual_palette_and_size() {
    assert_model(Graphic3dTypeOfShadingModel::Unlit);
}
#[test]
fn gouraud_mapped_points_extract_actual_palette_and_size() {
    assert_model(Graphic3dTypeOfShadingModel::Gouraud);
}
#[test]
fn phong_mapped_points_extract_actual_palette_and_size() {
    assert_model(Graphic3dTypeOfShadingModel::Phong);
}
fn assert_model(model: Graphic3dTypeOfShadingModel) {
    let draw = point_draw(model, 3.);
    assert_eq!(draw.topology(), crate::PrimitiveTopology::Points);
    assert_eq!(draw.point_style.as_ref().unwrap().size, 3.);
    let texture = draw.surface_texture.as_ref().expect("actual mapped image");
    assert_eq!(texture.image.extent, [2, 2]);
    assert_eq!(texture.uv, [[0.25, 0.25, 0., 1.], [0.75, 0.25, 0., 1.]]);
    assert!(texture.modulate);
    assert_eq!(
        draw.lighting.is_some(),
        model != Graphic3dTypeOfShadingModel::Unlit
    );
    if let Some(lighting) = &draw.lighting {
        assert!(lighting.vertex_colors);
        assert_eq!(lighting.normals, [[0., 0., 1.]; 2]);
    }
}
#[test]
fn point_uvs_ignore_texture_matrix_and_both_image_row_flags() {
    for top_down in [false, true] {
        let fixture = PointTextureFixture::new(top_down, false);
        fixture
            .texture
            .get_params()
            .set_scale(martensite_tkernel::ncollection::Vec2::new(3., 4.));
        let draw = fixture.draws().unwrap().remove(0);
        let texture = draw.surface_texture.unwrap();
        assert_eq!(texture.uv, [[0.25, 0.25, 0., 1.], [0.75, 0.25, 0., 1.]]);
        assert!(texture.pending_texgen.is_none());
    }
}
#[test]
fn point_alpha_cutoff_is_unlit_only_and_exactly_inclusive() {
    for model in [
        Graphic3dTypeOfShadingModel::Unlit,
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        let draw = point_draw(model, 7.);
        assert_eq!(
            draw.surface_cutoff,
            (model == Graphic3dTypeOfShadingModel::Unlit).then_some(0.1_f32.next_up())
        );
    }
    let cutoff = point_draw(Graphic3dTypeOfShadingModel::Unlit, 1.)
        .surface_cutoff
        .unwrap();
    assert!(0.1_f32 < cutoff);
    assert!(0.1_f32.next_up() >= cutoff);
}
#[test]
fn ordinary_mapping_refuses_sprite_coexistence_without_dropping_requested_marker() {
    let fixture = PointTextureFixture::new(true, false);
    fixture.aspect.set_marker_type(AspectTypeOfMarker::Plus);
    assert!(
        fixture
            .draws()
            .unwrap_err()
            .to_string()
            .contains("coexistence")
    );
    fixture.aspect.set_texture_map_on(false);
    assert!(!fixture.draws().unwrap().is_empty());
}
#[test]
fn fixed_function_mapping_is_explicitly_incomplete_instead_of_using_wrong_shader() {
    let fixture = PointTextureFixture::new(true, false);
    fixture
        .driver
        .set_lighting_pipeline(crate::RenderLightingPipeline::FixedFunction)
        .unwrap();
    assert!(
        fixture
            .draws()
            .unwrap_err()
            .to_string()
            .contains("fixed-function")
    );
}
#[test]
fn actual_vertex_colors_keep_precedence_over_the_mapped_image() {
    let fixture = PointTextureFixture::new(true, true);
    let draw = fixture.draws().unwrap().remove(0);
    assert!(draw.source_color_array);
    assert_eq!(
        draw.vertices.iter().map(|v| v.color).collect::<Vec<_>>(),
        [[0., 1., 1., 1.], [1., 0., 1., 1.]]
    );
    assert_eq!(
        crate::point_surface_texture::point_surface_flags(&draw) & 8192,
        8192
    );
}

#[test]
fn absent_point_normals_demote_requested_lighting_before_alpha_selection() {
    let fixture = PointTextureFixture::with_normals(true, false, false);
    for model in [
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        fixture.aspect.set_shading_model(model);
        let draw = fixture.draws().unwrap().remove(0);
        assert!(draw.lighting.is_none());
        assert_eq!(draw.surface_cutoff, Some(0.1_f32.next_up()));
    }
}
#[test]
fn unavailable_point_image_does_not_demote_existing_nodal_lighting() {
    let fixture = PointTextureFixture::unavailable_base_image(false);
    fixture.texture.get_params().set_modulate(false);
    for model in [
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        fixture.aspect.set_shading_model(model);
        let draw = fixture.draws().unwrap().remove(0);
        assert!(draw.lighting.is_some());
        assert!(draw.surface_cutoff.is_none());
        assert!(draw.surface_texture_unavailable.unwrap().modulate);
    }
}

#[test]
fn fresh_unavailable_point_base_is_black_before_lighting() {
    let fixture = PointTextureFixture::unavailable_base_image(false);
    for model in [
        Graphic3dTypeOfShadingModel::Unlit,
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        fixture.aspect.set_shading_model(model);
        let draw = fixture.draws().unwrap().remove(0);
        assert!(draw.surface_texture_unavailable.is_some());
        assert_eq!(draw.vertices.len(), 2);
        for vertex in &draw.vertices {
            assert_eq!(vertex.color, [0., 0., 0., 1.]);
        }
    }
}

#[test]
fn null_and_disabled_point_maps_keep_uniform_color() {
    for null_map in [false, true] {
        let draw = unmapped_blue_point_draw(null_map);
        assert!(draw.surface_texture.is_none());
        assert!(draw.surface_texture_unavailable.is_none());
        assert_eq!(draw.vertices[0].color, [0., 0., 1., 1.]);
    }
}
fn unmapped_blue_point_draw(null_map: bool) -> crate::PrimitiveDraw {
    let fixture = PointTextureFixture::new(true, false);
    fixture
        .aspect
        .set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    fixture
        .aspect
        .set_interior_color(Color::named(NamedColor::BLUE));
    if null_map {
        fixture.aspect.set_texture_map(Handle::default());
    } else {
        fixture.aspect.set_texture_map_on(false);
    }
    fixture.draws().unwrap().remove(0)
}

#[test]
fn fresh_unavailable_point_base_preserves_real_color_provenance_before_lighting() {
    let fixture = PointTextureFixture::unavailable_base_image(true);
    for model in [
        Graphic3dTypeOfShadingModel::Unlit,
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        fixture.aspect.set_shading_model(model);
        let draw = fixture.draws().unwrap().remove(0);
        assert!(draw.source_color_array);
        assert_eq!(draw.vertices[0].color, [0., 1., 1., 1.]);
        assert_eq!(draw.vertices[1].color, [1., 0., 1., 1.]);
    }
}
