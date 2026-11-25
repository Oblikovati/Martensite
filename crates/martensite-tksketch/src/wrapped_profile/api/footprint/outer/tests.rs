use super::*;

fn polygon(points: &[[f64; 2]]) -> OuterTrim {
    let edges = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(&a, &b)| {
            let mut bounds = TrimBox::empty();
            bounds.include(a);
            bounds.include(b);
            DirectedEdge {
                chord: [a, b],
                bounds,
                straight: true,
            }
        })
        .collect();
    OuterTrim::from_edges(edges).ok().unwrap()
}

#[test]
fn sloped_trim_distinguishes_interior_boundary_and_outside_within_the_same_box() {
    let mut triangle = polygon(&[[0., 0.], [4., 0.], [0., 4.]]);
    let inside = TrimBox([[0.5, 1.], [0.5, 1.]]);
    assert_eq!(triangle.check(inside).ok().unwrap(), None);
    let touching = TrimBox([[1., 2.], [1., 2.]]);
    assert_eq!(
        triangle.check(touching).ok().unwrap(),
        Some(OuterTrimRefusal::Boundary { edge: 2 })
    );
    let outside = TrimBox([[3., 3.5], [3., 3.5]]);
    assert_eq!(
        triangle.check(outside).ok().unwrap(),
        Some(OuterTrimRefusal::Winding { actual: 0 })
    );
}

#[test]
fn concave_trim_uses_winding_after_boundary_separation() {
    let mut domain = polygon(&[[0., 0.], [4., 0.], [4., 4.], [2., 4.], [2., 2.], [0., 2.]]);
    assert_eq!(
        domain.check(TrimBox([[0.5, 1.], [0.5, 1.]])).ok().unwrap(),
        None
    );
    assert_eq!(
        domain.check(TrimBox([[3., 3.5], [3., 3.5]])).ok().unwrap(),
        None
    );
    assert_eq!(
        domain.check(TrimBox([[0.5, 1.], [3., 3.5]])).ok().unwrap(),
        Some(OuterTrimRefusal::Winding { actual: 0 })
    );
}
