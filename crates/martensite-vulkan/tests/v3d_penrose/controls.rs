use super::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn volume(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass()
}

#[test]
fn penrose_boxes_have_the_authored_repeated_scale_volumes() {
    let boxes = geometry::boxes().unwrap();
    assert_eq!(boxes.len(), 10);
    let mut scale = 1.;
    for shape in &boxes {
        scale *= 0.94;
        let expected = 1.8 * 1.8 * 1.3 * scale * scale * scale;
        assert!((volume(shape) - expected).abs() < 1e-12 * expected);
    }
}

#[test]
fn penrose_executes_both_prism_cuts_and_the_actual_hollowing_cut() {
    let mut boxes = geometry::boxes().unwrap();
    let original_first = volume(&boxes[0]);
    let original_last = volume(&boxes[9]);
    geometry::trim_last(&mut boxes).unwrap();
    assert!(volume(&boxes[9]) > 0. && volume(&boxes[9]) < original_last);
    geometry::hollow_first(&mut boxes).unwrap();
    assert!(volume(&boxes[0]) > 0. && volume(&boxes[0]) < original_first);
    for shape in &boxes {
        crate::shape_checks::check_shape_valid(shape).unwrap();
    }
}

#[test]
fn penrose_null_camera_lights_and_ray_flags_keep_literal_parameters() {
    use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 1024, 512).unwrap();
    scene::camera(&session).unwrap();
    scene::lights(&mut session).unwrap();
    assert_camera_and_rendering(session.view().view_base());
    assert_eq!(session.view().view_base().active_lights().len(), 4);
    session.close();
}

fn assert_camera_and_rendering(view: &martensite_tkv3d::v3d::V3dView) {
    assert_eq!(view.camera().fov_y(), 25.);
    // VViewParams sets relative scale first; eye/center then determine perspective extent.
    let distance = (10_f64 * 10. + 10. * 10. + 6.8 * 6.8).sqrt();
    // Camera deliberately uses this truncated degrees-to-radians constant.
    let extent = 2. * distance * martensite_cmath::tan(0.5_f64 * 0.0174532925 * 25.);
    let expected = view.default_camera().scale() / extent;
    assert!((view.scale() - expected).abs() < expected * 1e-14);
    let params = view.rendering_params();
    assert_eq!(
        params.method,
        martensite_tkservice::graphic3d::Graphic3dRenderingMode::Raytracing
    );
    assert!(params.ray_tracing.is_antialiasing_enabled);
    assert!(!params.ray_tracing.is_reflection_enabled);
    assert!(!params.ray_tracing.is_shadow_enabled);
}

#[test]
fn penrose_bounding_preview_retains_actual_extent_and_orange_drawable() {
    use martensite_raster_core::{PaletteKind, RasterViewer, drawable::RasterDrawable};
    use martensite_tkmath::{bnd::BoundingBox3, gp::Point3};
    use std::rc::Rc;
    let mut bounds = BoundingBox3::default();
    bounds.add_point(Point3::new(1., 2., 3.));
    bounds.add_point(Point3::new(4., 6., 8.));
    let preview = bounding_preview::BoundingPreview::new(&bounds).unwrap();
    assert!(preview.context.is_batch());
    let center = preview.drawable.obb().center();
    assert_eq!((center.x(), center.y(), center.z()), (2.5, 4., 5.5));
    let system = Rc::new(martensite_raster_core::MemoryWindowSystem::new());
    let mut viewer: RasterViewer = RasterViewer::with_adapter(Some(system));
    viewer.make_view(1, "+X+Y", 0, 0, 100, 100).unwrap();
    let displayed: Rc<dyn RasterDrawable> = preview.drawable.clone();
    viewer.display(&displayed).unwrap();
    assert_eq!(viewer.display_state().current_color, PaletteKind::Orange);
}
