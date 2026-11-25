// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::tests::rectangle_support;
use super::*;

#[test]
fn cap_datum_bound_applies_to_both_depth_orders_and_both_normal_senses() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    for outward in [false, true] {
        let mut cap = rectangle_support(2., outward);
        cap.carriers = nominal_carriers(outward);
        let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
        let first = cap_error(&cap, 0, &meridian, &mut arithmetic).unwrap();
        assert!(arithmetic.upper_f64(first.upper).unwrap() < 1e-12);
        shift_last(&mut cap, [1. / 32., -1. / 16.]);
        let last = cap_error(&cap, 1, &meridian, &mut arithmetic).unwrap();
        let last = arithmetic.upper_f64(last.upper).unwrap();
        assert!((last - 3. / 32.).abs() < 1e-12);
    }
}

#[test]
fn changing_the_decoded_semi_angle_refuses_constant_offset_correspondence() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let mut cap = rectangle_support(2., true);
    cap.carriers = nominal_carriers(true);
    cap.carriers[1][2] = 0.5_f64.next_up().to_bits();
    let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
    assert!(matches!(
        cap_error(&cap, 1, &meridian, &mut arithmetic),
        Err(ConicalReferenceFailure::Binding {
            field: "decoded cap semi-angle"
        })
    ));
}

#[test]
fn ruled_bound_adds_the_worst_trim_error_to_the_cap_datum_error() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let caps = arithmetic.scalar(1. / 32.).unwrap();
    let error = side_error(1. / 128., caps.upper, &mut arithmetic).unwrap();
    assert_eq!(arithmetic.upper_f64(error.upper).unwrap(), 5. / 128.);
}

#[test]
fn individually_admitted_errors_can_refuse_their_composed_spatial_limit() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let mut cap = rectangle_support(2., true);
    cap.carriers = nominal_carriers(true);
    shift_last(&mut cap, [1. / 32., 0.]);
    let measures = NormalMeasures {
        volume: arithmetic.zero(),
        displacement: arithmetic.scalar(1. / 32.).unwrap(),
        radii: arithmetic.scalar(2.).unwrap(),
        meridian: Meridian::capture(&cap, &mut arithmetic).unwrap(),
    };
    assert!(matches!(
        calculate(&cap, &measures, 0.05, &mut arithmetic),
        Err(ConicalReferenceFailure::Bound {
            field: "composed decoded cap spatial error",
            ..
        })
    ));
}

fn nominal_carriers(outward: bool) -> [[u64; 3]; 2] {
    let [sin, cos] = [martensite_cmath::sin(0.5), martensite_cmath::cos(0.5)];
    let last = if outward { 2. } else { -2. };
    [0., last]
        .map(|depth| [10. * sin + depth * cos, 10. * cos - depth * sin, 0.5].map(f64::to_bits))
}

fn shift_last(cap: &mut CapBinding, [radial, axial]: [f64; 2]) {
    for (value, shift) in cap.carriers[1].iter_mut().zip([radial, axial]) {
        *value = (f64::from_bits(*value) + shift).to_bits();
    }
}
