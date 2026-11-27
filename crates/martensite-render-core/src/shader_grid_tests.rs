//! GlShaderGrid CPU behaviour with analytic expectations: frame, domain, arc wrap,
//! snapping, echo, Z-fit bounds, background world-view and the shader block layout.
use super::*;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Direction3, Point3, Xyz};
use martensite_tkservice::aspect::AspectGridDrawMode;
use martensite_tkservice::graphic3d::{Graphic3dCamera, Graphic3dCameraProjection};

/// An orthographic camera looking down -Z at the XY plane, 10 units tall, square.
fn top_camera() -> Graphic3dCamera {
    let camera = Graphic3dCamera::default();
    camera.set_eye(&Point3::new(0., 0., 50.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_scale(10.);
    camera.set_aspect(1.);
    camera.set_z_range(1., 100.).unwrap();
    camera
}

fn shown(params: AspectGridParams) -> RenderShaderGrid {
    let mut grid = RenderShaderGrid::default();
    grid.display(&params, &Axis3::default(), &top_camera());
    grid
}

fn step_params(step: f64) -> AspectGridParams {
    let mut params = AspectGridParams::default();
    params.set_scale(1.0 / step).unwrap();
    params
}

fn close(a: Xyz, b: [f64; 3]) -> bool {
    (a - Xyz::new(b[0], b[1], b[2])).modulus() < 1e-12
}

#[test]
fn frame_rotates_axes_and_offsets_the_origin_along_the_normal() {
    let mut params = AspectGridParams::default();
    params.set_rotation_angle(std::f64::consts::FRAC_PI_2);
    params.set_origin(Point3::new(1., 2., 0.));
    params.set_z_offset(0.5);
    let frame = shown(params).frame();
    assert!(close(frame.origin, [1., 2., 0.5]));
    assert!(close(frame.x, [0., -1., 0.]));
    assert!(close(frame.y, [1., 0., 0.]));
}

#[test]
fn rectangular_domain_uses_half_sizes_and_circular_domain_radius_and_arc() {
    let mut params = AspectGridParams::default();
    params.set_size_x(4.).unwrap();
    params.set_size_y(2.).unwrap();
    let grid = shown(params);
    assert!(grid.is_point_in_bounds(2., 1.));
    assert!(!grid.is_point_in_bounds(2., 1.01));
    let mut circle = AspectGridParams::default();
    circle.set_angular_divisions(8);
    circle.set_radius(2.).unwrap();
    circle.set_arc_range(0., std::f64::consts::FRAC_PI_2);
    let grid = shown(circle);
    assert!(grid.is_point_in_bounds(1., 1.));
    assert!(!grid.is_point_in_bounds(-1., 1.));
    assert!(!grid.is_point_in_bounds(1.9, 1.9));
}

#[test]
fn arcs_wrap_through_the_negative_axis() {
    assert!(angle_in_arc(2.356, -2.356, std::f64::consts::PI));
    assert!(!angle_in_arc(2.356, -2.356, 0.0));
    assert!(angle_in_arc(0.0, TWO_PI, 1.0));
    assert_eq!(positive_angle_span(0.0, TWO_PI), TWO_PI);
}

#[test]
fn snapping_rounds_to_rectangular_and_polar_nodes() {
    let grid = shown(step_params(0.5));
    let snapped = grid
        .snap_point(&top_camera(), Xyz::new(0.74, -0.26, 0.))
        .unwrap();
    assert!(close(snapped.xyz(), [0.5, -0.5, 0.]));
    let mut polar = step_params(1.0);
    polar.set_angular_divisions(2);
    polar.set_radius(10.).unwrap();
    let grid = shown(polar);
    let snapped = grid
        .snap_point(&top_camera(), Xyz::new(0.1, 1.9, 0.))
        .unwrap();
    assert!(close(
        snapped.xyz(),
        [
            2.0f64 * martensite_cmath::cos(std::f64::consts::FRAC_PI_2),
            2.,
            0.
        ]
    ));
}

#[test]
fn background_and_hidden_grids_do_not_snap() {
    let mut params = step_params(0.5);
    params.set_is_background(true);
    assert!(
        shown(params)
            .snap_point(&top_camera(), Xyz::new(0., 0., 0.))
            .is_none()
    );
    let mut none = step_params(0.5);
    none.set_draw_mode(AspectGridDrawMode::None);
    let grid = shown(none);
    assert!(!grid.is_drawn());
}

#[test]
fn echo_snaps_the_pixel_under_the_cursor() {
    let grid = shown(step_params(1.0));
    let query = RenderGridPixel {
        size: [100, 100],
        pixel: [74, 24],
    };
    let echo = grid.echo(&top_camera(), query).unwrap();
    // Pixel (74, 24) maps to about (2.45, 2.55) on a 10-unit view; the node is (2, 3).
    assert!(close(echo.point.xyz(), [2., 3., 0.]));
}

#[test]
fn bounded_grids_add_their_corners_to_the_z_fit_box() {
    let mut params = AspectGridParams::default();
    params.set_size_x(4.).unwrap();
    params.set_size_y(2.).unwrap();
    let grid = shown(params);
    let mut bounds = BoundingBox3::default();
    grid.add_z_fit_bounds(&mut bounds, &top_camera());
    assert!(bounds.contains(Point3::new(2., 1., 0.)));
    assert!(bounds.contains(Point3::new(-2., -1., 0.)));
    assert!(!bounds.contains(Point3::new(2.5, 0., 0.)));
}

#[test]
fn background_grids_cancel_the_view_change_since_display() {
    let mut grid = RenderShaderGrid::default();
    let mut reference = Mat4::<f32>::identity();
    reference.translate(martensite_tkernel::ncollection::Vec3::new(1., 0., 0.));
    grid.observe_world_view(reference);
    let mut params = AspectGridParams::default();
    params.set_is_background(true);
    grid.display(&params, &Axis3::default(), &top_camera());
    assert_eq!(grid.draw_world_view(&reference), Mat4::identity());
}

#[test]
fn shader_block_is_two_matrices_ten_rows_and_two_flag_rows() {
    let grid = shown(step_params(0.5));
    let block = grid.uniforms(&top_camera(), &Mat4::identity());
    assert_eq!(block.bytes().len(), 2 * 64 + 10 * 16 + 2 * 16);
    assert_eq!(block.rows[2][3], 2.0, "uScaleX is the step frequency");
    assert_eq!(block.flags[0][1], 0, "rectangular grid type");
    assert_eq!(block.flags[0][3], 0, "orthographic camera");
}
