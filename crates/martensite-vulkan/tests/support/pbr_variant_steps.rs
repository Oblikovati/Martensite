//! Steps shared by the PBR variants (fixtures/pbr_variants, fixtures/pbr_facet_variants):
//! clip planes, mesh edges and interior style, and the OIT method on the
//! plastic unit sphere of `PbrScene`.
use super::pbr_sphere_scene::PbrScene;
use martensite_tkernel::ncollection::Vec4;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::prs_mgr::*;

/// Clip the sphere `s` with a plane of `equation`.
pub fn add_clip_plane(scene: &PbrScene, equation: [f64; 4]) -> Handle<Graphic3dClipPlane> {
    let [a, b, c, d] = equation;
    let plane = Graphic3dClipPlane::from_equation(Vec4::new(a, b, c, d)).unwrap();
    let plane = Handle::new(plane);
    scene.sphere.add_clip_plane(&plane);
    plane
}

/// A clip chain on `s` with equations (1, 0, 0, 0.3) and (0, 1, 0, 0.2): the second plane is a
/// copy of the first with its own equation, chained after it.
pub fn add_clip_chain(scene: &PbrScene) -> Handle<Graphic3dClipPlane> {
    let first = Handle::new(Graphic3dClipPlane::from_equation(Vec4::new(1., 0., 0., 0.3)).unwrap());
    let second = Handle::new(Graphic3dClipPlane::copy_of(&first));
    second.set_equation(Vec4::new(0., 1., 0., 0.2)).unwrap();
    Graphic3dClipPlane::set_chain_next_plane(&first, second).unwrap();
    scene.sphere.add_clip_plane(&first);
    first
}

/// The four clip dumps of both variants: planes pln1, pln2 and pln3 added one by one (one, two
/// and N planes), then deleted, then the two-plane chain; `clip_plane_dumps(&scene)`.
pub fn clip_plane_dumps(scene: &PbrScene) -> Vec<Vec<u8>> {
    let mut dumps = Vec::new();
    let mut planes = Vec::new();
    for equation in [[1., 0., 0., 0.3], [0., 1., 0., 0.2], [0., 0., 1., 0.4]] {
        planes.push(add_clip_plane(scene, equation));
        dumps.push(scene.dump());
    }
    planes
        .iter()
        .for_each(|plane| scene.sphere.remove_clip_plane(plane));
    let chain = add_clip_chain(scene);
    dumps.push(scene.dump());
    scene.sphere.remove_clip_plane(&chain);
    dumps
}

/// Draw `s` with red edges or none, and set its interior style (HOLLOW is AspectIS_EMPTY).
pub fn set_edges(scene: &PbrScene, edges: bool, style: AspectInteriorStyle) {
    let drawer = scene.sphere.attributes();
    drawer
        .setup_own_shading_aspect(&scene.context.default_drawer())
        .unwrap();
    let aspect = drawer.shading_aspect().aspect();
    aspect.set_draw_edges(edges);
    aspect.set_edge_color(Color::named(NamedColor::RED));
    aspect.set_interior_style(style);
    scene.sphere.synchronize_aspects();
}

/// Set the OIT method: weighted with factor 0, peeling with 4 layers, or off.
pub fn set_oit(scene: &PbrScene, method: Graphic3dRenderTransparentMethod) {
    scene.view().change_rendering_params(&mut |params| {
        params.transparency_method = method;
        params.oit_depth_factor = 0.;
        params.nb_oit_depth_peeling_layers = 4;
    });
}

/// Make `s` half transparent, then dump per OIT method: weighted, then depth peeling;
/// `oit_dumps(&scene)`.
pub fn oit_dumps(scene: &PbrScene) -> Vec<Vec<u8>> {
    scene.set_transparency(0.5);
    [
        Graphic3dRenderTransparentMethod::BlendOit,
        Graphic3dRenderTransparentMethod::DepthPeelingOit,
    ]
    .map(|method| {
        set_oit(scene, method);
        scene.dump()
    })
    .to_vec()
}
