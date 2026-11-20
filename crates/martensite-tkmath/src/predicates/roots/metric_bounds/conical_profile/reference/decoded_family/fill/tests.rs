// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn unwrapped_containment_refuses_with_source_and_target() {
    let outer = [diamond([0., 0.], 4.)];
    let nested = [diamond([0.25, 0.25], 0.5)];
    let members = [
        Member::capture(&outer).unwrap(),
        Member::capture(&nested).unwrap(),
    ];
    let error = verify_unwrapped(&members, &mut work(&mut || false)).unwrap_err();
    assert!(matches!(
        error,
        ConicalDecodedFillFailure::Overlap {
            source: 1,
            target: 0,
            turn: 0
        }
    ));
}

#[test]
fn unwrapped_member_inside_a_hole_is_disjoint() {
    let outer_with_hole = [diamond([0., 0.], 4.), diamond([0., 0.], 2.)];
    let nested = [diamond([0.25, 0.25], 0.5)];
    let members = [
        Member::capture(&outer_with_hole).unwrap(),
        Member::capture(&nested).unwrap(),
    ];
    let mut never = || false;
    let mut work = work(&mut never);
    verify_unwrapped(&members, &mut work).unwrap();
    assert_eq!(work.locations, 2);
}

#[test]
fn bounding_box_rejection_uses_no_ray_budget_but_polls_cancellation() {
    let controls = [diamond([0., 0.], 1.)];
    let member = Member::capture(&controls).unwrap();
    let mut never = || false;
    let mut allowance = work(&mut never);
    assert!(!member.contains([-2., 0.], &mut allowance).unwrap());
    assert_eq!(allowance.locations, 0);
    let error = member
        .contains([-2., 0.], &mut work(&mut || true))
        .unwrap_err();
    assert!(error.cancelled());
}

fn diamond([x, y]: [f64; 2], radius: f64) -> Contour {
    let points = [
        [x - radius, y],
        [x, y + radius],
        [x + radius, y],
        [x, y - radius],
    ];
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(&a, &b)| vec![a, b])
        .collect()
}

fn work(cancelled: &mut (dyn FnMut() -> bool + Send)) -> Work<'_> {
    Work {
        remaining: RootIsolationOptions::new(1e-6),
        locations: 0,
        cancelled,
    }
}
