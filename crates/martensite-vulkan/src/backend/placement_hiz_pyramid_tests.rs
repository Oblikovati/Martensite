use super::*;

#[test]
fn pyramid_has_disjoint_levels_with_ceil_dimensions() {
    let (levels, bytes) = levels_for(vk::Extent2D {
        width: 3,
        height: 5,
    })
    .unwrap();
    let shape: Vec<_> = levels
        .iter()
        .map(|level| (level.extent.width, level.extent.height, level.offset))
        .collect();
    assert_eq!(shape, [(2, 3, 31), (1, 2, 37), (1, 1, 39)]);
    assert_eq!(bytes, 40 * 4);
    assert!(
        levels_for(vk::Extent2D {
            width: 0,
            height: 1
        })
        .is_none()
    );
}
