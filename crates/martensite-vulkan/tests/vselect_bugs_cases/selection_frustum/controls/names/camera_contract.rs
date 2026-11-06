//! Independent pose contracts use literal inputs, never measured final values as expectations.
use super::*;
use martensite_tkmath::gp::{Direction3, Point3};

const EYE: [f64; 3] = [16.40, 10.98, 24.59];
const CENTER: [f64; 3] = [3.03, 2.70, 5.10];
const UP: [f64; 3] = [-0.09, 0.94, -0.33];
// Roundoff budgets cover chained setters/trigonometry, not original selection tolerances.
const UNIT_ROUNDOFF: f64 = 256.0 * f64::EPSILON;

fn frustum_literal_direction_distance() -> ([f64; 3], f64) {
    let delta = std::array::from_fn::<_, 3, _>(|i| CENTER[i] - EYE[i]);
    let distance = delta.iter().map(|value| value * value).sum::<f64>().sqrt();
    (delta.map(|value| value / distance), distance)
}

fn frustum_literal_up(direction: [f64; 3]) -> [f64; 3] {
    let dot = (0..3).map(|i| UP[i] * direction[i]).sum::<f64>();
    let projected = std::array::from_fn::<_, 3, _>(|i| UP[i] - dot * direction[i]);
    let length = projected
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    projected.map(|value| value / length)
}

fn frustum_close(actual: f64, expected: f64, budget: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= budget,
        "actual={actual:?}; expected={expected:?} within roundoff budget={budget:?}"
    );
}

fn frustum_components(actual: [f64; 3], expected: [f64; 3], budget: f64) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        frustum_close(actual, expected, budget);
    }
}

fn frustum_point_components(point: Point3) -> [f64; 3] {
    // Read coordinates directly; a transform here would contaminate the independent pose check.
    [point.x(), point.y(), point.z()]
}

fn frustum_direction_components(direction: Direction3) -> [f64; 3] {
    // Re-normalizing here would hide an error in the published direction.
    [direction.x(), direction.y(), direction.z()]
}

fn frustum_final_position(session: &ViewerSession) {
    let camera = session.view().camera();
    let (_, distance) = frustum_literal_direction_distance();
    // World-unit budget scales with literal pose magnitude; vectors use a separate unit budget.
    let budget = UNIT_ROUNDOFF * distance.max(1.0);
    let eye = frustum_point_components(camera.eye());
    let center = frustum_point_components(camera.center());
    frustum_components(eye, EYE, budget);
    frustum_components(center, CENTER, budget);
    frustum_close(camera.distance(), distance, budget);
}

/// Check the literal final pose independently; `frustum_final_pose(session)`.
pub(super) fn frustum_final_pose(session: &ViewerSession) {
    let camera = session.view().camera();
    let (direction, _) = frustum_literal_direction_distance();
    assert_eq!(
        camera.projection_type(),
        Graphic3dCameraProjection::Perspective
    );
    frustum_final_position(session);
    let actual_direction = frustum_direction_components(camera.direction());
    let up = frustum_direction_components(camera.up());
    frustum_components(actual_direction, direction, UNIT_ROUNDOFF);
    frustum_components(up, frustum_literal_up(direction), UNIT_ROUNDOFF);
}

/// Bind independent mapping inputs before setters; `let before = frustum_mapping_before(session)`.
pub(super) fn frustum_mapping_before(session: &ViewerSession) -> (f64, f64, f64) {
    let default = session.view().view_base().default_camera();
    let inputs = (
        default.scale(),
        session.view().camera().fov_y(),
        default.fov_y(),
    );
    assert_eq!(inputs.1, 45.0);
    assert_eq!(inputs.2, 45.0);
    inputs
}

fn frustum_relative_close(actual: f64, expected: f64, factor: f64) {
    let budget = factor * UNIT_ROUNDOFF * expected.abs().max(1.0);
    frustum_close(actual, expected, budget);
}

/// Check mapping against pre-command inputs; `frustum_final_mapping(session, before)`.
pub(super) fn frustum_final_mapping(session: &ViewerSession, before: (f64, f64, f64)) {
    let view = session.view();
    let default = view.view_base().default_camera();
    let camera = view.camera();
    let (_, distance) = frustum_literal_direction_distance();
    // The reviewed conversion policy uses this decimal factor, not standard-PI degrees.
    // Graphic3dCamera pin that behavior.
    let half_angle = 0.5 * 0.0174532925 * before.1;
    let raw_scale = 2.0 * distance * martensite_cmath::tan(half_angle);
    let ratio = before.0 / raw_scale;
    assert_eq!(default.scale(), before.0);
    assert_eq!(camera.fov_y(), before.1);
    assert_eq!(default.fov_y(), before.2);
    frustum_relative_close(camera.scale(), raw_scale, 1.0);
    // Dimensionless ratio additionally covers division of independently scaled values.
    frustum_relative_close(view.view_base().scale(), ratio, 2.0);
}
