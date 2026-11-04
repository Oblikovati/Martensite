//! Live returned fields remain distinct from explicitly copied selecting-volume values.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis1, Direction3, Point2, Point3};
use martensite_tkservice::graphic3d::{Graphic3dCamera, Graphic3dCameraProjection};
use martensite_tkv3d::select_mgr::*;

fn initialized_axis() -> SelectMgrAxisIntersector {
    let axis = SelectMgrAxisIntersector::new();
    axis.init(Axis1::new(Point3::new(1., 2., 3.), Direction3::X));
    axis
}

fn orthographic_camera() -> Handle<Graphic3dCamera> {
    let camera = Handle::new(Graphic3dCamera::default());
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera
        .set_eye_and_center(&Point3::new(0., 0., 10.), &Point3::default())
        .unwrap();
    camera.set_z_range(0.1, 100.).unwrap();
    camera.set_scale(20.);
    camera
}

fn configure_camera(volume: &SelectMgrRectangularFrustum) -> Handle<Graphic3dCamera> {
    let camera = orthographic_camera();
    volume.set_camera(&camera);
    volume.set_window_size(800, 600);
    camera
}

fn assert_ray_values(volume: &SelectMgrRectangularFrustum, values: (Point3, Point3, Direction3)) {
    assert_eq!(values.0, volume.get_near_pnt().unwrap());
    assert_eq!(values.1, volume.get_far_pnt().unwrap());
    assert_eq!(values.2, volume.get_view_ray_direction().unwrap());
}

fn reorient_and_build(volume: &SelectMgrRectangularFrustum, camera: &Handle<Graphic3dCamera>) {
    camera
        .set_eye_and_center(&Point3::new(10., 0., 0.), &Point3::default())
        .unwrap();
    volume.build().unwrap();
}

fn assert_uninitialized_failure(failure: Failure) {
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(failure.message().contains("expected Point(0) after Init"));
    assert!(failure.message().contains("selection_type="));
}

#[test]
fn base_defaults_bind_shared_static_slots_across_actual_receivers() {
    let left = SelectMgrTriangularFrustum::new();
    let right = SelectMgrTriangularFrustum::new();
    let near = left.get_near_pnt_ref().unwrap();
    let far = left.get_far_pnt_ref().unwrap();
    assert_eq!(near.value(), Point3::default());
    assert_eq!(far.value(), Point3::new(f64::MAX, f64::MAX, f64::MAX));
    assert!(near.is_same_reference(&right.get_near_pnt_ref().unwrap()));
    assert!(far.is_same_reference(&right.get_far_pnt_ref().unwrap()));
    assert!(!near.is_same_reference(&far));
    assert_base_direction_and_mouse(&left, &right);
}

fn assert_base_direction_and_mouse(
    left: &SelectMgrTriangularFrustum,
    right: &SelectMgrTriangularFrustum,
) {
    let left_direction = left.get_view_ray_direction_ref().unwrap();
    let right_direction = right.get_view_ray_direction_ref().unwrap();
    let left_mouse = left.get_mouse_position_ref();
    let right_mouse = right.get_mouse_position_ref();
    assert!(left_direction.is_same_reference(&right_direction));
    assert!(left_mouse.is_same_reference(&right_mouse));
    assert_eq!(left_direction.value(), Direction3::X);
    assert_eq!(left_mouse.value(), Point2::new(f64::MAX, f64::MAX));
}

#[test]
fn uninitialized_axis_ref_getters_preserve_actual_typed_failures() {
    let axis = SelectMgrAxisIntersector::new();
    let errors = [
        axis.get_near_pnt_ref()
            .err()
            .expect("uninitialized near must fail"),
        axis.get_far_pnt_ref()
            .err()
            .expect("uninitialized far must fail"),
        axis.get_view_ray_direction_ref()
            .err()
            .expect("uninitialized direction must fail"),
    ];
    for error in errors {
        assert_uninitialized_failure(error);
    }
}

#[test]
fn axis_captured_fields_observe_init_while_copied_values_remain_snapshots() {
    let axis = initialized_axis();
    let near = axis.get_near_pnt_ref().unwrap();
    let direction = axis.get_view_ray_direction_ref().unwrap();
    let copied_near = axis.get_near_pnt().unwrap();
    let copied_direction = axis.get_view_ray_direction().unwrap();
    axis.init(Axis1::new(Point3::new(-4., 5., 6.), Direction3::Y));
    assert_eq!(near.value(), Point3::new(-4., 5., 6.));
    assert_eq!(direction.value(), Direction3::Y);
    assert_eq!(copied_near, Point3::new(1., 2., 3.));
    assert_eq!(copied_direction, Direction3::X);
    assert!(near.is_same_reference(&axis.get_near_pnt_ref().unwrap()));
    assert!(direction.is_same_reference(&axis.get_view_ray_direction_ref().unwrap()));
}

#[test]
fn axis_far_is_shared_across_axes_but_distinct_from_base_far() {
    let left = initialized_axis();
    let right = initialized_axis();
    let base = SelectMgrTriangularFrustum::new();
    let far = left.get_far_pnt_ref().unwrap();
    assert_eq!(far.value(), base.get_far_pnt_ref().unwrap().value());
    assert!(far.is_same_reference(&right.get_far_pnt_ref().unwrap()));
    assert!(!far.is_same_reference(&base.get_far_pnt_ref().unwrap()));
    left.init(Axis1::new(Point3::new(7., 8., 9.), Direction3::Z));
    assert_eq!(far.value(), Point3::new(f64::MAX, f64::MAX, f64::MAX));
}

#[test]
fn unbuilt_rectangular_fields_have_distinct_identity_and_native_default_values() {
    let volume = SelectMgrRectangularFrustum::new();
    let other = SelectMgrRectangularFrustum::new();
    let near = volume.get_near_pnt_ref().unwrap();
    let far = volume.get_far_pnt_ref().unwrap();
    assert_eq!(near.value(), Point3::default());
    assert_eq!(far.value(), Point3::default());
    assert!(!near.is_same_reference(&far));
    assert!(!near.is_same_reference(&other.get_near_pnt_ref().unwrap()));
    assert_eq!(
        volume.get_view_ray_direction_ref().unwrap().value(),
        Direction3::X
    );
    assert_eq!(
        volume.get_mouse_position_ref().value(),
        Point2::new(f64::MAX, f64::MAX)
    );
}

#[test]
fn rectangular_ray_views_survive_init_and_observe_actual_build_changes() {
    let volume = SelectMgrRectangularFrustum::new();
    let camera = configure_camera(&volume);
    let near = volume.get_near_pnt_ref().unwrap();
    let far = volume.get_far_pnt_ref().unwrap();
    let direction = volume.get_view_ray_direction_ref().unwrap();
    volume.init(Point2::new(400., 300.));
    assert_eq!(near.value(), Point3::default());
    volume.build().unwrap();
    let first = (near.value(), far.value(), direction.value());
    assert_ray_values(&volume, first);
    reorient_and_build(&volume, &camera);
    let second = (near.value(), far.value(), direction.value());
    assert_ray_values(&volume, second);
    assert_ne!(second, first);
    assert!(near.is_same_reference(&volume.get_near_pnt_ref().unwrap()));
    assert!(far.is_same_reference(&volume.get_far_pnt_ref().unwrap()));
    assert!(direction.is_same_reference(&volume.get_view_ray_direction_ref().unwrap()));
}

#[test]
fn captured_mouse_source_does_not_reselect_after_point_box_point_transitions() {
    let volume = SelectMgrRectangularFrustum::new();
    let fallback = volume.get_mouse_position_ref();
    volume.init(Point2::new(20., 30.));
    let point = volume.get_mouse_position_ref();
    volume.init_box(Point2::new(70., 80.), Point2::new(90., 100.));
    let box_fallback = volume.get_mouse_position_ref();
    assert_eq!(point.value(), Point2::new(70., 80.));
    assert!(fallback.is_same_reference(&box_fallback));
    assert!(!point.is_same_reference(&box_fallback));
    volume.init(Point2::new(13., 17.));
    assert_eq!(point.value(), Point2::new(13., 17.));
    assert_eq!(fallback.value(), Point2::new(f64::MAX, f64::MAX));
    assert_eq!(box_fallback.value(), fallback.value());
    assert!(point.is_same_reference(&volume.get_mouse_position_ref()));
}

#[test]
fn base_trait_object_keeps_the_rectangular_override_source() {
    let rectangular = SelectMgrRectangularFrustum::new();
    rectangular.init(Point2::new(3., 4.));
    let volume: &dyn SelectMgrBaseIntersector = &rectangular;
    let mouse = volume.get_mouse_position_ref();
    let near = volume.get_near_pnt_ref().unwrap();
    rectangular.init_box(Point2::new(5., 6.), Point2::new(7., 8.));
    assert_eq!(mouse.value(), Point2::new(5., 6.));
    assert!(near.is_same_reference(&rectangular.get_near_pnt_ref().unwrap()));
    assert!(!mouse.is_same_reference(&volume.get_mouse_position_ref()));
}

fn send_sync_reference<T: Send + Sync>(reference: T) -> T {
    // Keeping the actual returned type tests its auto traits without a synthetic replacement.
    reference
}

#[test]
fn concrete_views_dispatch_through_the_base_trait_and_cross_scoped_threads() {
    let axis = initialized_axis();
    let volume: &dyn SelectMgrBaseIntersector = &axis;
    let near = send_sync_reference(volume.get_near_pnt_ref().unwrap());
    let direction = send_sync_reference(volume.get_view_ray_direction_ref().unwrap());
    let mouse = send_sync_reference(volume.get_mouse_position_ref());
    axis.init(Axis1::new(Point3::new(7., 8., 9.), Direction3::Z));
    let observed = std::thread::scope(|scope| {
        scope
            .spawn(move || (near.value(), direction.value(), mouse.value()))
            .join()
            .unwrap()
    });
    assert_eq!(observed.0, Point3::new(7., 8., 9.));
    assert_eq!(observed.1, Direction3::Z);
    assert_eq!(observed.2, Point2::new(f64::MAX, f64::MAX));
}
