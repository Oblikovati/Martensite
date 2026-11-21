//! Frozen location coefficients survive both initial publication and detached decoding.
use super::*;
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
const ROWS: [[f64; 4]; 3] = [[0.6, -0.8, 0., 2.], [0.8, 0.6, 0., 3.], [0., 0., 1., 4.]];
fn placed(transform: Transform3) -> Shape {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap();
    vertex
        .into_shape()
        .located(&Location::new(transform), false)
        .unwrap()
}
fn coefficient_bits(shape: &Shape) -> [[u64; 4]; 3] {
    let transform = shape.location().transformation();
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            transform
                .value(i as i32 + 1, j as i32 + 1)
                .unwrap()
                .to_bits()
        })
    })
}
fn frozen(bytes: &[u8]) -> Checked<(Shape, Vec<(u64, Shape)>)> {
    decode_with_policies(
        bytes,
        &[7],
        SurfaceFrameReadPolicy::PreserveStoredV1,
        LocationTransformReadPolicy::PreserveStoredRigidV1,
    )
}
#[test]
fn explicit_frozen_policies_preserve_oblique_root_and_semantic_location_bits() {
    let shape = placed(Transform3::from_stored_rigid_coefficients(ROWS).unwrap());
    let bytes = encode(&shape, &[(7_u64, shape.clone())]).unwrap();
    let (root, bindings) = frozen(&bytes).unwrap();
    let expected = ROWS.map(|row| row.map(f64::to_bits));
    assert_eq!(coefficient_bits(&root), expected);
    assert_eq!(coefficient_bits(&bindings[0].1), expected);
    let replay_bytes = encode(&root, &bindings).unwrap();
    let (replay, _) = frozen(&replay_bytes).unwrap();
    assert_eq!(coefficient_bits(&replay), expected);
}
#[test]
fn rigid_frozen_decode_refuses_scaling_while_legacy_decode_keeps_supporting_it() {
    let mut scale = Transform3::default();
    scale.set_scale(Point3::new(0., 0., 0.), 2.).unwrap();
    let shape = placed(scale);
    let bytes = encode(&shape, &[(7_u64, shape.clone())]).unwrap();
    assert!(frozen(&bytes).is_err());
    assert!(decode_with_policy(&bytes, &[7_u64], SurfaceFrameReadPolicy::PreserveStoredV1).is_ok());
}
