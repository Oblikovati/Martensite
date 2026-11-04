//! Closed-form boxes independently pin strict boundaries in the existing released culling tool.
use martensite_tkernel::{
    ncollection::{Mat4, Vec3},
    standard::Handle,
};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::*;

fn boundary_tool(perspective: bool) -> Graphic3dCullingTool {
    let camera = Handle::new(Graphic3dCamera::default());
    camera.set_projection_type(if perspective {
        Graphic3dCameraProjection::Perspective
    } else {
        Graphic3dCameraProjection::Orthographic
    });
    camera.set_scale(64.);
    camera.set_eye(&Point3::new(0., 0., 20.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_aspect(1.);
    camera.set_z_range(1., 100.).unwrap();
    let mut tool = Graphic3dCullingTool::default();
    tool.set_view_volume(&camera, &Mat4::identity());
    tool.set_viewport_size(64, 64, 1.);
    tool.cache_clip_pts_projections();
    tool
}

fn predicate_record(tool: &Graphic3dCullingTool, perspective: bool, distance: f64, size: f64) {
    let mut context = Graphic3dCullingContext::default();
    tool.set_culling_distance(&mut context, distance);
    tool.set_culling_size(&mut context, size);
    let too_distant = tool.is_too_distant(
        &context,
        &Vec3::new(-1., -1., -1.),
        &Vec3::new(1., 1., 1.),
        None,
    );
    let too_small = tool.is_too_small(&context, &Vec3::new(0., 0., 0.), &Vec3::new(3., 4., 0.));
    assert_eq!(too_distant, perspective && distance == 18.);
    assert_eq!(too_small, !perspective && size > 5.);
    println!(
        "{{\"perspective\":{perspective},\"distance\":{distance},\"size\":{size},\"effective_distance\":{},\"effective_size_squared\":{},\"too_distant\":{too_distant},\"too_small\":{too_small}}}",
        context.dist_cull, context.size_cull2
    );
}

#[test]
fn actual_distance_and_size_predicates_preserve_strict_boundary_direction() {
    for perspective in [false, true] {
        let tool = boundary_tool(perspective);
        for distance in [18., 19., 20., 0., -1.] {
            predicate_record(&tool, perspective, distance, 0.);
        }
        for size in [4., 5., 5.0_f64.next_up(), 0., -1.] {
            predicate_record(&tool, perspective, 0., size);
        }
    }
}

#[test]
fn nonpositive_and_infinite_limits_remain_disabled() {
    let tool = boundary_tool(true);
    for limit in [0., -1., f64::INFINITY, f64::NAN] {
        let mut context = Graphic3dCullingContext::default();
        tool.set_culling_distance(&mut context, limit);
        tool.set_culling_size(&mut context, limit);
        assert_eq!(context.dist_cull, -1.);
        assert_eq!(context.size_cull2, -1.);
    }
}
