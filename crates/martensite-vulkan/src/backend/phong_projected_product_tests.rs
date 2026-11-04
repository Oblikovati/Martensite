//! Integer shader model checked against independent exact binary64 products.
fn significand_exponent(word: u32) -> (u32, u32) {
    let exponent = (word >> 23) & 255;
    let fraction = word & 0x7fffff;
    if exponent == 0 {
        return (fraction, (-149_i32) as u32);
    }
    (fraction | 0x800000, (exponent as i32 - 150) as u32)
}

fn shift_product(low: u32, high: u32, shift: u32) -> [u32; 2] {
    if shift == 0 {
        return [low, high];
    }
    if shift >= 32 {
        return [high >> (shift - 32), 0];
    }
    [(low >> shift) | (high << (32 - shift)), high >> shift]
}

fn shader_product(left: f32, right: f32) -> [u32; 4] {
    let (a, ae) = significand_exponent(left.to_bits());
    let (b, be) = significand_exponent(right.to_bits());
    if a == 0 || b == 0 {
        return [0; 4];
    }
    let wide = u64::from(a) * u64::from(b);
    let (low, high) = (wide as u32, (wide >> 32) as u32);
    let shift = if low == 0 {
        32 + high.trailing_zeros()
    } else {
        low.trailing_zeros()
    };
    let [low, high] = shift_product(low, high, shift);
    [
        low,
        high,
        ae.wrapping_add(be).wrapping_add(shift),
        (left.to_bits() ^ right.to_bits()) & 0x80000000,
    ]
}

fn assert_product_identity(left: [f32; 2], right: [f32; 2]) {
    let expected =
        f64::from(left[0]) * f64::from(left[1]) == f64::from(right[0]) * f64::from(right[1]);
    assert_eq!(
        shader_product(left[0], left[1]) == shader_product(right[0], right[1]),
        expected,
        "products {left:?} versus {right:?}"
    );
}

#[test]
fn normalized_products_cover_zero_subnormal_and_exponent_wrap() {
    let words = [
        0, 0x80000000, 1, 2, 0x7fffff, 0x800000, 0x800001, 0x3f000000, 0x3f800000, 0x40000000,
        0x7f000000, 0x7f7fffff,
    ];
    for a in words.map(f32::from_bits) {
        for b in words.map(f32::from_bits) {
            assert_product_identity([a, b], [b, a]);
            assert_product_identity([a, b], [-a, -b]);
            assert_product_identity([a, b], [-a, b]);
            assert_product_identity([a, b], [a, 1.]);
        }
    }
    assert_eq!(
        shader_product(f32::from_bits(1), 1.),
        [1, 0, (-149_i32) as u32, 0]
    );
    assert_eq!(
        shader_product(f32::MIN_POSITIVE, f32::MIN_POSITIVE),
        [1, 0, (-252_i32) as u32, 0]
    );
    assert_eq!(shader_product(1., 1.), [1, 0, 0, 0]);
}

#[test]
fn generated_product_identities_match_exact_float64_multiplication() {
    let mut seed = 0xb14c913d_u32;
    for _ in 0..10000 {
        let values: [f32; 4] = std::array::from_fn(|_| {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            f32::from_bits(seed & 0xfeffffff)
        });
        let [a, b, c, d] = values;
        assert_product_identity([a, b], [c, d]);
        assert_product_identity([a, b], [b, a]);
        if (a * 2.).is_finite() && b * 0.5 != 0. && f64::from(b * 0.5) * 2. == f64::from(b) {
            assert_product_identity([a, b], [a * 2., b * 0.5]);
        }
    }
}

#[test]
fn shader_guard_rejects_invalid_inputs_before_exact_products() {
    let shader = include_str!("../../shaders/phong_projected_axis_v3.glsl");
    let guard = shader.find("if (!phongInsideClip(phongClip[0])").unwrap();
    let compare = shader.find("bool x = phongSameRatio").unwrap();
    assert!(guard < compare);
    for requirement in [
        "any(isnan(clip))",
        "any(isinf(clip))",
        "clip.w <= 0.0",
        "lessThanEqual(abs(clip.xyz), vec3(clip.w))",
    ] {
        assert!(
            shader.contains(requirement),
            "missing shader validity predicate {requirement}"
        );
    }
    let fragment = include_str!("../../shaders/lighting.frag");
    assert!(fragment.contains("lighting.settings.x == 2u && phongExactProjectedAxis()"));
}
