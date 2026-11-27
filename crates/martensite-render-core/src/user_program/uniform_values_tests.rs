//! Tests of the std140 default-block filler (uniform_values.rs).
use super::spirv_reflect::{UniformMember, UniformScalar, UniformShape};
use super::uniform_values::{
    UserUniformScene, UserUniformValues, fill_uniform_block, uniform_block_size,
};

fn member(
    name: &str,
    offset: u32,
    shape: (UniformScalar, u32, u32),
    array: Option<(u32, u32)>,
) -> UniformMember {
    let (scalar, rows, columns) = shape;
    UniformMember {
        name: name.into(),
        offset,
        shape: UniformShape {
            scalar,
            rows,
            columns,
            matrix_stride: if columns > 1 { 16 } else { 0 },
        },
        array,
    }
}

fn floats(bytes: &[u8], at: usize, count: usize) -> Vec<f32> {
    let words = bytes[at..at + 4 * count].as_chunks::<4>().0;
    words.iter().map(|word| f32::from_ne_bytes(*word)).collect()
}

fn ints(bytes: &[u8], at: usize, count: usize) -> Vec<i32> {
    let words = bytes[at..at + 4 * count].as_chunks::<4>().0;
    words.iter().map(|word| i32::from_ne_bytes(*word)).collect()
}

fn translation(x: f32) -> [f32; 16] {
    let mut matrix = [0.; 16];
    for i in 0..4 {
        matrix[5 * i] = 1.;
    }
    matrix[12] = x;
    matrix
}

#[test]
fn matrices_and_their_float_inverse_transpose_fill_by_name() {
    let scene = UserUniformScene {
        model_world: translation(2.),
        ..Default::default()
    };
    let mat4 = (UniformScalar::Float, 4, 4);
    let members = [
        member("occModelWorldMatrix", 0, mat4, None),
        member("occModelWorldMatrixInverse", 64, mat4, None),
        member("occModelWorldMatrixInverseTranspose", 128, mat4, None),
    ];
    let bytes = fill_uniform_block(&members, uniform_block_size(&members), &scene);
    assert_eq!(bytes.len(), 192);
    assert_eq!(floats(&bytes, 48, 4), [2., 0., 0., 1.]);
    assert_eq!(floats(&bytes, 64 + 48, 4), [-2., 0., 0., 1.]);
    // The transpose moves the translation into the last row.
    assert_eq!(floats(&bytes, 128 + 12, 1), [-2.]);
}

#[test]
fn light_types_pad_to_the_program_limit_with_minus_one() {
    let scene = UserUniformScene {
        lights: vec![([[1.; 4]; 4], 1), ([[2.; 4]; 4], 3)],
        max_lights: 4,
        ..Default::default()
    };
    let members = [
        member("occLightSourcesCount", 0, (UniformScalar::Int, 1, 1), None),
        member(
            "occLightSources",
            16,
            (UniformScalar::Float, 4, 1),
            Some((16, 16)),
        ),
        member(
            "occLightSourcesTypes",
            272,
            (UniformScalar::Int, 1, 1),
            Some((4, 16)),
        ),
    ];
    let bytes = fill_uniform_block(&members, uniform_block_size(&members), &scene);
    assert_eq!(ints(&bytes, 0, 1), [2]);
    assert_eq!(floats(&bytes, 16 + 4 * 16, 4), [2.; 4]);
    let types: Vec<i32> = (0..4).map(|i| ints(&bytes, 272 + 16 * i, 1)[0]).collect();
    assert_eq!(types, [1, 3, -1, -1]);
}

#[test]
fn a_mat3_variable_uses_the_std140_column_stride_and_unknown_members_stay_zero() {
    let identity = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
    let scene = UserUniformScene {
        variables: vec![("uNormTrsf".into(), UserUniformValues::Float(identity))],
        ..Default::default()
    };
    let members = [
        member("uNormTrsf", 0, (UniformScalar::Float, 3, 3), None),
        member("uUnknown", 48, (UniformScalar::Float, 4, 1), None),
    ];
    let bytes = fill_uniform_block(&members, uniform_block_size(&members), &scene);
    assert_eq!(floats(&bytes, 16, 4), [0., 1., 0., 0.]);
    assert_eq!(floats(&bytes, 32, 3), [0., 0., 1.]);
    assert_eq!(floats(&bytes, 48, 4), [0.; 4]);
}

#[test]
fn a_program_variable_overrides_the_scene_value_of_the_same_name() {
    let scene = UserUniformScene {
        color: [1.; 4],
        variables: vec![("occColor".into(), UserUniformValues::Float(vec![0.5; 4]))],
        ..Default::default()
    };
    let members = [member("occColor", 0, (UniformScalar::Float, 4, 1), None)];
    let bytes = fill_uniform_block(&members, 16, &scene);
    assert_eq!(floats(&bytes, 0, 4), [0.5; 4]);
}

#[test]
fn texture_transform_fills_both_vec4_rows() {
    let scene = UserUniformScene {
        texture_trsf: [[-0.5, 1., 2., -1.], [0., 1., 0., 0.]],
        ..Default::default()
    };
    let rows = member(
        "occTexTrsf2d",
        0,
        (UniformScalar::Float, 4, 1),
        Some((2, 16)),
    );
    let bytes = fill_uniform_block(std::slice::from_ref(&rows), 32, &scene);
    assert_eq!(floats(&bytes, 0, 8), [-0.5, 1., 2., -1., 0., 1., 0., 0.]);
}
