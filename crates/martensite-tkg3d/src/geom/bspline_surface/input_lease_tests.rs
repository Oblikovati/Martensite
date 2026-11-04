//! Real geometry cells exercise lease refusal and owned coefficient isolation.
use super::super::*;
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::SharedReadFailure,
};
use martensite_tkmath::gp::Point3;

fn tensor() -> GeomBSplineSurface {
    let poles = poles();
    let weights = weights();
    let u = Array1::from_shared(&[-2., 3.], 1);
    let v = Array1::from_shared(&[0., 1.], 1);
    let um = Array1::from_shared(&[2, 2], 1);
    let vm = Array1::from_shared(&[3, 3], 1);
    GeomBSplineSurface::with_weights(
        &poles,
        &weights,
        (&u, &um),
        (&v, &vm),
        (1, 2),
        (false, false),
    )
    .unwrap()
}
fn poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 3).unwrap();
    for row in 1..=2 {
        for column in 1..=3 {
            let x = if row == 1 { -0. } else { 2. };
            poles
                .set_value(
                    row,
                    column,
                    Point3::new(x, f64::from(column), f64::from(row + column)),
                )
                .unwrap();
        }
    }
    poles
}
fn weights() -> Array2<'static, f64> {
    let mut weights = Array2::with_bounds(1, 2, 1, 3).unwrap();
    for row in 1..=2 {
        for column in 1..=3 {
            weights
                .set_value(row, column, f64::from(10 * row + column))
                .unwrap();
        }
    }
    weights
}
#[test]
fn coherent_surface_copy_preserves_raw_axes_weights_and_index_layout() {
    let surface = tensor();
    let lease = surface.try_input_lease().unwrap();
    let input = lease.stored_input().unwrap();
    assert_eq!(lease.stored_sizes(), [6, 6, 2, 2, 4, 2, 2, 6]);
    assert_eq!(input.shape(), [2, 3]);
    assert_eq!(input.weights_shape(), [2, 3]);
    assert_eq!(input.lower_bounds(), [[1; 2]; 2]);
    assert_eq!(input.rational(), [true, true]);
    assert_eq!(input.poles()[0][0].to_bits(), (-0.0f64).to_bits());
    assert_eq!(input.poles()[5], [2., 3., 5.]);
    assert_eq!(input.weights(), &[11., 12., 13., 21., 22., 23.]);
    assert_axis(&input.axes()[0], 1, [-2., 3.], &[-2., -2., 3., 3.]);
    assert_axis(&input.axes()[1], 2, [0., 1.], &[0., 0., 0., 1., 1., 1.]);
    assert!(!input.evaluation_override());
}
fn assert_axis(axis: &StoredSplineSurfaceAxis, degree: i32, domain: [f64; 2], flat: &[f64]) {
    assert_eq!(axis.degree(), degree);
    assert_eq!(axis.domain(), domain);
    assert_eq!(axis.knots(), &domain);
    assert_eq!(axis.multiplicities(), &[degree + 1; 2]);
    assert_eq!(axis.flat_knots(), flat);
    assert_eq!(axis.lower_bounds(), [1; 3]);
    assert!(!axis.periodic());
}
#[test]
fn copied_surface_input_is_immutable_send_sync_and_isolated_after_lease_drop() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<StoredSplineSurfaceInput>();
    let surface = tensor();
    let lease = surface.try_input_lease().unwrap();
    let input = lease.stored_input().unwrap();
    assert!(surface.state.try_write().is_none());
    drop(lease);
    surface.set_pole(1, 1, Point3::new(9., 8., 7.)).unwrap();
    assert_eq!(input.poles()[0][0].to_bits(), (-0.0f64).to_bits());
    assert_eq!(
        surface
            .try_input_lease()
            .unwrap()
            .stored_input()
            .unwrap()
            .poles()[0],
        [9., 8., 7.]
    );
}
#[test]
fn surface_lease_refuses_real_busy_writer_without_waiting() {
    let surface = tensor();
    let writer = surface.state.write();
    assert!(matches!(
        surface.try_input_lease(),
        Err(SharedReadFailure::Busy)
    ));
    drop(writer);
    assert!(surface.try_input_lease().is_ok());
}
#[test]
fn surface_lease_preserves_actual_poison_refusal() {
    let surface = tensor();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _writer = surface.state.write();
        panic!("NamedSurfaceWriterPoison");
    }));
    assert!(panic.is_err());
    assert!(matches!(
        surface.try_input_lease(),
        Err(SharedReadFailure::Poisoned)
    ));
}
#[test]
fn stored_surface_copy_never_normalizes_raw_nonrational_weight_storage() {
    let surface = tensor();
    {
        let mut fields = surface.state.write();
        fields.u_rational = false;
        fields.v_rational = false;
        fields.weights.set_value(1, 1, f64::from_bits(1)).unwrap();
    }
    let input = surface.try_input_lease().unwrap().stored_input().unwrap();
    assert_eq!(input.rational(), [false, false]);
    assert_eq!(input.weights()[0].to_bits(), 1);
}
#[test]
fn bad_stored_domain_returns_actual_failure_instead_of_fabricated_bounds() {
    let surface = tensor();
    surface.state.write().u.flat_knots = Array1::default();
    assert!(surface.try_input_lease().unwrap().stored_input().is_err());
}
