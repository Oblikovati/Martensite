//! Reserved axis tags must not alter any valid legacy factor or the eighty-byte layout.
use crate::{LineStyle, PrimitiveDraw, RenderVertex, identity_clip};
fn line(factor: u16) -> PrimitiveDraw {
    PrimitiveDraw::segments(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 2],
        vec![0, 1],
        identity_clip(),
        LineStyle::new(1., 0x8421, factor).unwrap(),
    )
    .unwrap()
}
#[test]
fn every_legacy_factor_retains_exact_original_eighty_bytes() {
    for factor in 1..=256 {
        let draw = line(factor);
        let bytes = encoded(&draw);
        let expected = legacy_bytes(factor);
        assert_eq!(bytes, expected);
        assert_eq!(bytes.len(), 80);
    }
}
#[test]
fn owned_axis_tags_preserve_factor_domain_and_push_range() {
    for factor in 1..=256 {
        for axis in 1..=2 {
            let mut draw = line(factor);
            draw.projected_dash = Some(crate::projected_segments::PreparedAxis {
                extent: [97, 61],
                axis,
            });
            let bytes = encoded(&draw);
            let word = u32::from_ne_bytes(bytes[76..80].try_into().unwrap());
            assert_eq!(word & 65535, u32::from(factor));
            assert_eq!(word >> 16, axis);
            assert_eq!(bytes.len(), 80);
        }
    }
}

fn encoded(draw: &PrimitiveDraw) -> Vec<u8> {
    let extent = ash::vk::Extent2D {
        width: 97,
        height: 61,
    };
    super::transform_constants::encode_transform(draw, extent)
}
fn legacy_bytes(factor: u16) -> Vec<u8> {
    let mut expected: Vec<u8> = identity_clip()
        .iter()
        .flat_map(|v| v.to_ne_bytes())
        .collect();
    expected.extend([97_f32, 61_f32].iter().flat_map(|v| v.to_ne_bytes()));
    expected.extend(0x8421_u32.to_ne_bytes());
    expected.extend(u32::from(factor).to_ne_bytes());
    expected
}
