use super::*;
use crate::{LineStyle, RenderVertex, identity_clip};
use martensite_tkservice::graphic3d::{Graphic3dAspectFillArea3d, Graphic3dAspectsMembers};

fn triangle() -> PrimitiveDraw {
    let vertices = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]));
    PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip()).unwrap()
}
#[test]
fn polygon_points_preserve_face_geometry_and_separate_lines() {
    let source = triangle();
    let line = PrimitiveDraw::segments(
        source.vertices()[..2].to_vec(),
        vec![],
        identity_clip(),
        LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap();
    let aspect = Graphic3dAspectFillArea3d::default();
    aspect.set_interior_style(AspectInteriorStyle::Point);
    let mut draws = vec![source.clone(), line];
    apply_polygon_points(&mut draws, &aspect).unwrap();
    assert!(draws[0].rasterizes_polygon_points());
    assert!(!draws[1].rasterizes_polygon_points());
    assert_eq!(draws[0].topology(), PrimitiveTopology::Triangles);
    assert_eq!(draws[0].vertices(), source.vertices());
    assert_eq!(draws[0].indices(), source.indices());
}
#[test]
fn ordinary_aspects_do_not_change_raster_state() {
    let mut draws = vec![triangle()];
    apply_polygon_points(&mut draws, &Graphic3dAspectFillArea3d::default()).unwrap();
    assert!(!draws[0].rasterizes_polygon_points());
    let changed = draws[0].with_replacement_geometry(vec![], vec![]);
    assert!(!changed.rasterizes_polygon_points());
}
#[test]
fn point_raster_metadata_survives_geometry_replacement() {
    let mut source = triangle();
    source.polygon_points = true;
    assert!(
        source
            .with_replacement_geometry(vec![], vec![])
            .rasterizes_polygon_points()
    );
}
#[test]
fn unsupported_polygon_features_produce_typed_refusal() {
    for (fill, subset) in [(false, None), (true, Some(false)), (false, Some(true))] {
        let error = require_polygon_points(true, fill, subset).unwrap_err();
        assert_eq!(
            error.polygon_point_failure(),
            Some(&RenderPolygonPointFailure::DeviceCapabilities {
                fill_mode_non_solid: fill,
                subset_point_polygons: subset,
            })
        );
        assert!(error.to_string().contains("fillModeNonSolid="));
    }
    assert!(require_polygon_points(true, true, None).is_ok());
    assert!(require_polygon_points(true, true, Some(true)).is_ok());
    assert!(require_polygon_points(false, false, Some(false)).is_ok());
}
#[test]
fn polygon_point_failure_is_send_sync_and_has_an_error_source() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RenderPolygonPointFailure>();
    let error = require_polygon_points(true, false, None).unwrap_err();
    assert!(std::error::Error::source(&error).is_some());
}

fn registered_program_draw() -> PrimitiveDraw {
    use crate::user_program::*;
    let library = RenderUserProgramLibrary::default();
    register_samples(&library).unwrap();
    let modules = library.modules(&phong_shading_program()).unwrap();
    let mut draw = triangle();
    draw.user_program = Some(std::sync::Arc::new(UserProgramDraw::new(
        modules,
        Default::default(),
    )));
    draw
}
#[test]
fn custom_fill_programs_remain_allowed_but_unknown_point_size_is_typed() {
    let draw = registered_program_draw();
    assert!(require_polygon_point_program(&draw, false).is_ok());
    let error = require_polygon_point_program(&draw, true).unwrap_err();
    let key = draw.user_program.as_ref().unwrap().modules.key;
    assert_eq!(
        error.polygon_point_failure(),
        Some(&RenderPolygonPointFailure::CustomProgramPointSizeUndefined { key })
    );
    assert!(error.to_string().contains("PointSize contract"));
}
#[test]
fn actual_registered_program_is_refused_before_point_raster_publication() {
    let mut draws = vec![registered_program_draw()];
    let aspect = Graphic3dAspectFillArea3d::default();
    aspect.set_interior_style(AspectInteriorStyle::Point);
    let error = apply_polygon_points(&mut draws, &aspect).unwrap_err();
    assert!(matches!(
        error.polygon_point_failure(),
        Some(RenderPolygonPointFailure::CustomProgramPointSizeUndefined { .. })
    ));
    assert!(!draws[0].rasterizes_polygon_points());
    assert!(draws[0].user_program.is_some());
}
