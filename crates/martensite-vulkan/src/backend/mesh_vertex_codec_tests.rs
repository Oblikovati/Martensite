use super::*;

#[test]
fn packed_vertices_keep_color_words_and_use_part_local_position_codes() {
    let color = [0.25, -0., 1., 0.75];
    let vertices = [[2., -3., 7.], [5., 9., 7.], [3.5, 3., 7.]]
        .map(|position| RenderVertex::new(position, color));
    let (packed, decode) = pack_mesh_vertices(&vertices, &[[0., 0., 1.]; 3]).unwrap();
    let [low, high, middle] = [0, 1, 2].map(|index| words_for_vertex(&packed, index));

    assert_eq!(low[..3], [0, 0, 0]);
    assert_eq!(high[..3], [0xffff_ffff, 0, 0]);
    assert_eq!(middle[..3], [0x8000_8000, 0, 0]);
    assert!(words_for_vertex(&packed, 2)[3..].eq(&color.map(f32::to_bits)));
    assert_eq!(decode.origin, vertices[0].position);
    assert_eq!(decode.step[2], 0.);
}

#[test]
fn generated_positions_stay_in_the_quantization_and_float_rounding_bound() {
    let low = [-4_100., 60., -0.25];
    let high = [2_800., 85.5, 0.25];
    let vertices = generated_positions(low, high);
    let normals = vec![[1., 0., 0.]; vertices.len()];
    let (packed, decode) = pack_mesh_vertices(&vertices, &normals).unwrap();
    for (index, vertex) in vertices.iter().enumerate() {
        let [xy, z, ..] = words_for_vertex(&packed, index);
        let codes = [xy as u16, (xy >> 16) as u16, z as u16];
        for axis in 0..3 {
            let code = f32::from(codes[axis]);
            let decoded = decode.origin[axis] + code * decode.step[axis];
            let bound = position_bound(low[axis], high[axis], decode.step[axis]);
            assert!((f64::from(decoded) - f64::from(vertex.position[axis])).abs() <= bound);
        }
    }
}

fn generated_positions(low: [f32; 3], high: [f32; 3]) -> Vec<RenderVertex> {
    let mut points = vec![low, high];
    for sample in 1..2048 {
        let weights = [37, 107, 769].map(|step| ((sample * step) % 2047) as f32 / 2047.);
        points.push(std::array::from_fn(|i| {
            ((high[i] - low[i]) * weights[i] + low[i]).clamp(low[i], high[i])
        }));
    }
    points
        .into_iter()
        .map(|position| RenderVertex::new(position, [1.; 4]))
        .collect()
}

fn position_bound(low: f32, high: f32, step: f32) -> f64 {
    let span = f64::from(high) - f64::from(low);
    let levels = 65_535.;
    let packed_span = f64::from(step) * levels;
    // Closest integer: half a bin. The binary32 step differs from the exact step and
    // the shader rounds its multiply and add independently.
    span / (2. * levels)
        + (span - packed_span).abs()
        + f64::from(f32::EPSILON) * (f64::from(low).abs() + 2. * packed_span)
}

fn words_for_vertex(packed: &[u8], index: usize) -> [u32; 7] {
    let offset = index * PACKED_STRIDE as usize;
    std::array::from_fn(|word| {
        let start = offset + 4 * word;
        u32::from_ne_bytes(packed[start..start + 4].try_into().unwrap())
    })
}

#[test]
fn oct_direction_error_is_bounded_across_seams_hemispheres_and_axes() {
    for x in -11..=11 {
        for y in -11..=11 {
            for z in -11..=11 {
                if (x, y, z) == (0, 0, 0) {
                    continue;
                }
                let normal = [x as f32, y as f32, z as f32];
                let source = unit(normal.map(f64::from));
                let decoded = unproject_oct(encode_octahedral(normal).unwrap());
                let error = (0..3)
                    .map(|i| source[i] - decoded[i])
                    .map(|delta| delta * delta)
                    .sum::<f64>()
                    .sqrt();
                // A half-bin in each chart axis moves the unfolded point by at most
                // sqrt(3) / 32767. Unit normalization costs at most 2 * sqrt(3).
                assert!(error <= 6. / 32767., "{normal:?}: {error}");
            }
        }
    }
}

fn unproject_oct(encoded: [i16; 2]) -> [f64; 3] {
    let [u, v] = encoded.map(|value| f64::from(value) / 32767.);
    let face_height = 1. - u.abs() - v.abs();
    let [mut x, mut y] = [u, v];
    if face_height < 0. {
        x = if u < 0. {
            -(1. - v.abs())
        } else {
            1. - v.abs()
        };
        y = if v < 0. {
            -(1. - u.abs())
        } else {
            1. - u.abs()
        };
    }
    unit([x, y, face_height])
}

fn unit(vector: [f64; 3]) -> [f64; 3] {
    let length = vector.iter().map(|value| value * value).sum::<f64>().sqrt();
    vector.map(|value| value / length)
}

#[test]
fn inappropriate_vertices_keep_the_exact_path() {
    assert!(pack_mesh_vertices(&[], &[]).is_none());
    let ordinary = RenderVertex::new([0.; 3], [1.; 4]);
    for invalid in [
        RenderVertex::homogeneous([1., 2., 3., 0.5], [1.; 4]),
        RenderVertex::new([f32::NAN, 0., 0.], [1.; 4]),
        RenderVertex::new([f32::from_bits(1), 0., 0.], [1.; 4]),
    ] {
        assert!(pack_mesh_vertices(&[ordinary, invalid], &[[0., 0., 1.]; 2]).is_none());
    }
    let extremes = [-f32::MAX, f32::MAX].map(|x| RenderVertex::new([x, 0., 0.], [1.; 4]));
    assert!(pack_mesh_vertices(&extremes, &[[0., 0., 1.]; 2]).is_none());
    assert!(pack_mesh_vertices(&[ordinary], &[[0.; 3]]).is_none());
    assert!(pack_mesh_vertices(&[ordinary], &[]).is_none());
}

#[test]
fn decode_push_values_match_two_aligned_shader_vectors() {
    let decode = MeshPositionDecode {
        origin: [1., 2., 3.],
        step: [4., 5., 6.],
    };
    let mut output = vec![0xff; 64];
    decode.append_push_bytes(&mut output);
    assert_eq!(output.len(), 64 + DECODE_PUSH_BYTES as usize);
    assert_eq!(&output[..64], &[0xff; 64]);
    assert_eq!(
        &output[64..],
        [1_f32, 2., 3., 0., 4., 5., 6., 0.]
            .map(f32::to_ne_bytes)
            .as_flattened()
    );
}
