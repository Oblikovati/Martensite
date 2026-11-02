//! Exercise existing stopping branches at their exact predecessor states.
use super::*;
use martensite_tkg3d::{
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::gp::{Axis3, Direction3};

fn bounded_plane(normal: [f64; 3]) -> tool::Adaptor3dSurfaceHandle {
    let direction = Direction3::new(normal[0], normal[1], normal[2]).unwrap();
    let axis = Axis3::from_normal(Default::default(), direction).unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(axis)).cast();
    Handle::new(GeomAdaptorSurface::with_bounds(&surface, (-1., 1., -1., 1.)).unwrap()).cast()
}
fn active_march() -> (IntWalkPWalking, MarchState) {
    let first = bounded_plane([0., 0., 1.]);
    let second = bounded_plane([1., 0., 0.]);
    let mut walking = IntWalkPWalking::new(&first, &second, 1e-7, 1e-7, 0.01, 0.01).unwrap();
    walking.done = false;
    walking.line = Handle::new(IntSurfLineOn2S::new(None));
    let parameters = one_based(4).unwrap();
    let natural = walking.natural_walking_bounds().unwrap();
    let state = walking.first_point(&parameters, natural).unwrap().unwrap();
    (walking, state)
}
#[test]
fn no_append_stop_records_the_actual_threshold_branch() {
    let (mut walking, mut state) = active_march();
    state.level_of_iter_without_append = 20;
    state.deja_reparti = true;
    assert!(matches!(
        walking.march_step(&mut state).unwrap(),
        Flow::Stop
    ));
    assert_eq!(walking.search_observation.no_append_limit, 1);
}
#[test]
fn repeated_start_stop_records_the_actual_skip_limit() {
    let (mut walking, mut state) = active_march();
    state.inc_key = 4999;
    let first_point = state.pf;
    let flow = walking.skip_first_point(&mut state, first_point).unwrap();
    assert_eq!(walking.observe_march_flow(flow), Some(false));
    assert_eq!(state.inc_key, 5000);
    assert_eq!(walking.search_observation.repeated_start_limit, 1);
}
#[test]
fn point_count_stop_records_the_actual_append_limit() {
    let (mut walking, mut state) = active_march();
    state.reject_index = 249999;
    assert!(walking.append_previous_point(&mut state).unwrap());
    assert_eq!(state.reject_index, 250000);
    assert!(state.arrive);
    assert_eq!(walking.search_observation.point_count_limit, 1);
}
