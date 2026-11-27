use super::*;
use crate::RenderVertex;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;

fn triangle(culling: RenderFaceCulling, clip: [f32; 16]) -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([1., 0., 0.], [1.; 4]); 3];
    PrimitiveDraw::new(vertices, vec![0, 1, 2], clip)
        .unwrap()
        .with_face_culling(culling)
}
fn silhouette_aspects(enabled: bool) -> Handle<Graphic3dAspects> {
    let aspects = Handle::new(Graphic3dAspects::default());
    aspects.set_draw_silhouette(enabled);
    aspects.set_edge_color(Color::named(NamedColor::RED));
    aspects
}

#[test]
fn the_outline_is_front_culled_in_the_edge_color_and_needs_back_culling() {
    let params = SilhouetteParams {
        thickness: 0.5,
        ortho_scale: 2.,
    };
    let aspects = silhouette_aspects(true);
    let back = triangle(RenderFaceCulling::Back, crate::identity_clip());
    let outline = outline_draw(&back, &*aspects, params).unwrap().unwrap();
    assert_eq!(outline.face_culling, RenderFaceCulling::Front);
    assert_eq!(outline.vertices[0].color, [1., 0., 0., 1.]);
    // Without normals (no lighting snapshot) nothing is displaced.
    assert_eq!(outline.vertices[0].position, [1., 0., 0.]);
    let none = triangle(RenderFaceCulling::None, crate::identity_clip());
    assert!(outline_draw(&none, &*aspects, params).unwrap().is_none());
    let disabled = silhouette_aspects(false);
    assert!(outline_draw(&back, &*disabled, params).unwrap().is_none());
}

#[test]
fn displacement_uses_the_ortho_scale_or_the_clip_w() {
    let mut clip = crate::identity_clip();
    clip[3] = 0.5; // w = 0.5 * x + 1
    let vertex = RenderVertex::new([2., 0., 0.], [1.; 4]);
    let ortho = SilhouetteParams {
        thickness: 0.25,
        ortho_scale: 4.,
    };
    assert_eq!(displaced(&clip, &vertex, [0., 1., 0.], ortho), [2., 1., 0.]);
    let perspective = SilhouetteParams {
        thickness: 0.25,
        ortho_scale: -1.,
    };
    assert_eq!(
        displaced(&clip, &vertex, [0., 0., 2.], perspective),
        [2., 0., 1.]
    );
}
