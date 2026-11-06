use super::*;
use crate::RenderCoordinateConvention::{LegacyBottomUpV1, TopLeftLogicalV2};
use vk::SampleCountFlags as Count;

/// The standard positions of the Vulkan specification ("Standard sample locations").
fn standard(samples: Count) -> Vec<[f32; 2]> {
    match samples {
        Count::TYPE_2 => vec![[0.75, 0.75], [0.25, 0.25]],
        Count::TYPE_4 => vec![
            [0.375, 0.125],
            [0.875, 0.375],
            [0.125, 0.625],
            [0.625, 0.875],
        ],
        Count::TYPE_8 => vec![
            [0.5625, 0.3125],
            [0.4375, 0.6875],
            [0.8125, 0.5625],
            [0.3125, 0.1875],
            [0.1875, 0.8125],
            [0.0625, 0.4375],
            [0.6875, 0.9375],
            [0.9375, 0.0625],
        ],
        other => panic!("no standard pattern for {other:?}"),
    }
}

fn properties(bits: u32, counts: Count) -> vk::PhysicalDeviceSampleLocationsPropertiesEXT<'static> {
    vk::PhysicalDeviceSampleLocationsPropertiesEXT::default()
        .sample_location_sub_pixel_bits(bits)
        .sample_location_sample_counts(counts)
}

fn one_pixel(_: Count) -> vk::Extent2D {
    vk::Extent2D {
        width: 1,
        height: 1,
    }
}

#[test]
fn each_mirrored_position_is_the_standard_one_with_y_flipped_on_the_sixteenth_grid() {
    for samples in [Count::TYPE_2, Count::TYPE_4, Count::TYPE_8] {
        let mirrored = mirrored_positions(samples).unwrap();
        let expected: Vec<_> = standard(samples)
            .into_iter()
            .map(|[x, y]| [x, 1. - y])
            .collect();
        let actual: Vec<_> = mirrored.iter().map(|at| [at.x, at.y]).collect();
        assert_eq!(actual, expected, "{samples:?}");
        let on_grid = |v: f32| (v * 16.).fract() == 0.;
        assert!(mirrored.iter().all(|at| on_grid(at.x) && on_grid(at.y)));
        let info = locations_info(mirrored);
        assert_eq!(info.sample_locations_per_pixel, samples);
        assert_eq!(info.sample_locations_count as usize, mirrored.len());
    }
    // The user decision names the 4-sample result explicitly.
    let four: Vec<_> = mirrored_positions(Count::TYPE_4)
        .unwrap()
        .iter()
        .map(|at| (at.x, at.y))
        .collect();
    assert_eq!(
        four,
        [
            (0.375, 0.875),
            (0.875, 0.625),
            (0.125, 0.375),
            (0.625, 0.125)
        ]
    );
}

#[test]
fn single_sample_sixteen_samples_and_unoffered_counts_keep_the_standard_positions() {
    assert!(mirrored_positions(Count::TYPE_1).is_none());
    assert!(mirrored_positions(Count::TYPE_16).is_none());
    let lavapipe = Count::TYPE_4 | Count::TYPE_8;
    assert!(places_mirrored(lavapipe, Count::TYPE_4));
    assert!(!places_mirrored(lavapipe, Count::TYPE_2));
    assert!(!places_mirrored(lavapipe | Count::TYPE_1, Count::TYPE_1));
    assert!(!places_mirrored(Count::TYPE_16, Count::TYPE_16));
    assert!(!places_mirrored(Count::empty(), Count::TYPE_4));
}

#[test]
fn coarse_subpixel_grids_and_empty_location_grids_are_refused() {
    let offered = Count::TYPE_1 | Count::TYPE_2 | Count::TYPE_4 | Count::TYPE_8 | Count::TYPE_16;
    assert_eq!(
        admitted_counts(&properties(4, offered), one_pixel),
        Count::TYPE_2 | Count::TYPE_4 | Count::TYPE_8
    );
    // Three bits cannot hold the 1/16 positions.
    assert_eq!(
        admitted_counts(&properties(3, offered), one_pixel),
        Count::empty()
    );
    let no_eight = |samples| {
        if samples == Count::TYPE_8 {
            vk::Extent2D::default()
        } else {
            one_pixel(samples)
        }
    };
    assert_eq!(
        admitted_counts(&properties(4, offered), no_eight),
        Count::TYPE_2 | Count::TYPE_4
    );
}

#[test]
fn only_the_top_left_frame_mirrors_and_only_multisampled_depth_is_compatible() {
    let offered = Count::TYPE_4;
    assert_eq!(frame_counts(TopLeftLogicalV2, offered), offered);
    assert_eq!(frame_counts(LegacyBottomUpV1, offered), Count::empty());
    let depth = vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL;
    let compatible = vk::ImageCreateFlags::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT;
    assert_eq!(depth_flags(offered, Count::TYPE_4, depth), compatible);
    let none = vk::ImageCreateFlags::empty();
    assert_eq!(depth_flags(offered, Count::TYPE_1, depth), none);
    assert_eq!(depth_flags(offered, Count::TYPE_8, depth), none);
    let color = vk::ImageAspectFlags::COLOR;
    assert_eq!(depth_flags(offered, Count::TYPE_4, color), none);
}
