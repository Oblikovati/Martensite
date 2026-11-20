//! Public placement keeps the inward depth roles on a connected cone atlas.
use super::*;

#[test]
fn public_connected_atlas_places_an_inward_strip_on_an_oblique_cone() {
    use crate::surface_sketch::normal_extrude_cone_profile_on_chart;
    let mut request = strip_request();
    select_uv_program(&mut request);
    request.support.normal = NormalSense::Inward;
    request.support.apex = [8., -12., 2.];
    request.support.axis = [0., 3., 4.];
    request.support.radial = [2., 0., 0.];
    request.support.chart_origin = 0.2;
    let placed = normal_extrude_cone_profile_on_chart(&request, atlas_selection()).unwrap();
    assert_eq!(placed.support(), request.support);
    assert_eq!(placed.canonical().depths(), request.depths);
    assert!(
        placed.placement().members[&SurfaceRegionId(1)]
            .validation
            .is_valid()
    );
    assert!(placed.placement().achieved_spatial_error.upper <= request.options.spatial_error);
    assert_eq!(
        placed.export_native().unwrap().entities.len(),
        placed.canonical().history().len()
    );
    let cap_height = caller_cap_heights(placed.canonical());
    let expected =
        (request.depths[1] - request.depths[0]) * martensite_cmath::sin(request.support.semi_angle);
    assert!((cap_height[1] - cap_height[0] - expected).abs() < 1e-12);
}

fn caller_cap_heights(result: &WrappedConeNormalResult) -> [f64; 2] {
    use martensite_tkbrep::{brep::BRepTool, topods};
    use martensite_tkg3d::geom::{GeomConicalSurface, GeomElementarySurface};
    let native = result.export_native().unwrap();
    [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace].map(|role| {
        let (_, shape) = native
            .entities
            .iter()
            .find(|(key, _)| key.role == role)
            .unwrap();
        let face = topods::face(shape).unwrap();
        let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
        let cone = surface.cast::<GeomConicalSurface>();
        cone.get().unwrap().position().location().z()
    })
}
