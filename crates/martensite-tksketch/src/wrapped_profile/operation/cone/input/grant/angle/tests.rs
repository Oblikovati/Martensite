use super::*;

#[test]
fn pole_sectors_use_the_short_arc_across_the_negative_axis() {
    let right = width(&[[1., -0.25], [1., 0.25]]);
    let left = width(&[[-1., -0.25], [-1., 0.25]]);
    let exact = 2. * martensite_cmath::atan(0.25);
    for upper in [right, left] {
        assert!((exact..exact + 1e-13).contains(&upper), "{upper}");
    }
    assert_eq!(width(&[[1., 0.], [-1., 0.]]), TAU.next_up());
    assert_eq!(width(&[[1., 0.], [0., 0.]]), TAU.next_up());
}
