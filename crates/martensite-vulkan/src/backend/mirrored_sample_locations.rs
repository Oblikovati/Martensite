//! Mirrored standard multisample positions (user decision 2026-10-28).
//!
//! OpenGL measures sample positions from the pixel's lower-left corner in the y-up window and
//! Vulkan from the upper-left corner in the y-down framebuffer; both drivers place the same
//! standard pattern in their own frame, so the native multisampled edges are the vertical mirror
//! of the Vulkan ones (states-msaa of tests/user_program_draws_native.rs: 386 lavapipe edge
//! pixels, all equal to the bottom-up control). With `VK_EXT_sample_locations` every multisampled
//! pipeline places the standard positions with y mirrored to `1 - y`, and multisampled depth
//! attachments are created compatible with them. A device without the extension, a sample count
//! outside its `sampleLocationSampleCounts`, and the legacy bottom-up frame (already y-up) keep
//! the standard positions.
use ash::{Entry, Instance, vk};

const fn at(x: f32, y: f32) -> vk::SampleLocationEXT {
    vk::SampleLocationEXT { x, y }
}

/// Standard 2-sample positions (0.75,0.75) (0.25,0.25), y mirrored.
const MIRRORED_2: [vk::SampleLocationEXT; 2] = [at(0.75, 0.25), at(0.25, 0.75)];
/// Standard 4-sample positions (0.375,0.125) (0.875,0.375) (0.125,0.625) (0.625,0.875), y mirrored.
const MIRRORED_4: [vk::SampleLocationEXT; 4] = [
    at(0.375, 0.875),
    at(0.875, 0.625),
    at(0.125, 0.375),
    at(0.625, 0.125),
];
/// Standard 8-sample positions (0.5625,0.3125) (0.4375,0.6875) (0.8125,0.5625) (0.3125,0.1875)
/// (0.1875,0.8125) (0.0625,0.4375) (0.6875,0.9375) (0.9375,0.0625), y mirrored.
const MIRRORED_8: [vk::SampleLocationEXT; 8] = [
    at(0.5625, 0.6875),
    at(0.4375, 0.3125),
    at(0.8125, 0.4375),
    at(0.3125, 0.8125),
    at(0.1875, 0.1875),
    at(0.0625, 0.5625),
    at(0.6875, 0.0625),
    at(0.9375, 0.9375),
];
/// The mirrored positions lie on a 1/16 pixel grid.
const SUBPIXEL_BITS: u32 = 4;
const MIRRORED_COUNTS: vk::SampleCountFlags = vk::SampleCountFlags::from_raw(
    vk::SampleCountFlags::TYPE_2.as_raw()
        | vk::SampleCountFlags::TYPE_4.as_raw()
        | vk::SampleCountFlags::TYPE_8.as_raw(),
);

/// The mirrored standard positions of `samples`, or `None` where no standard pattern is mirrored.
/// Example: `mirrored_positions(vk::SampleCountFlags::TYPE_4).unwrap()[0]` is (0.375, 0.875).
pub(super) fn mirrored_positions(
    samples: vk::SampleCountFlags,
) -> Option<&'static [vk::SampleLocationEXT]> {
    match samples {
        vk::SampleCountFlags::TYPE_2 => Some(&MIRRORED_2),
        vk::SampleCountFlags::TYPE_4 => Some(&MIRRORED_4),
        vk::SampleCountFlags::TYPE_8 => Some(&MIRRORED_8),
        _ => None,
    }
}

/// Whether a pipeline or depth attachment of `samples` places mirrored positions on a device
/// mirroring `counts`. Example: `places_mirrored(runtime.mirrored_samples, samples)`.
pub(super) fn places_mirrored(counts: vk::SampleCountFlags, samples: vk::SampleCountFlags) -> bool {
    samples != vk::SampleCountFlags::TYPE_1
        && counts.contains(samples)
        && mirrored_positions(samples).is_some()
}

/// The sample counts whose mirrored positions `physical` can place; empty without the extension.
/// Example: `let counts = probe(&entry, &instance, physical);`.
pub(super) fn probe(
    entry: &Entry,
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> vk::SampleCountFlags {
    if !super::depth_equivalence::device_lists_extension(
        instance,
        physical,
        ash::ext::sample_locations::NAME,
    ) {
        return vk::SampleCountFlags::empty();
    }
    let mut properties = vk::PhysicalDeviceSampleLocationsPropertiesEXT::default();
    let mut query = vk::PhysicalDeviceProperties2::default().push_next(&mut properties);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_properties2(physical, &mut query) };
    let loader = ash::ext::sample_locations::Instance::new(entry, instance);
    let grid = |samples| {
        let mut grid = vk::MultisamplePropertiesEXT::default();
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe { loader.get_physical_device_multisample_properties(physical, samples, &mut grid) };
        grid.max_sample_location_grid_size
    };
    admitted_counts(&properties, grid)
}

/// The counts to mirror in frames of `coordinates`: the legacy bottom-up frame already stores
/// OpenGL's y-up rows, where the standard positions are OpenGL's own; only the top-left frame
/// mirrors. Example: `frame_counts(runtime.attachment_coordinates, probe(..))`.
pub(super) fn frame_counts(
    coordinates: crate::RenderCoordinateConvention,
    offered: vk::SampleCountFlags,
) -> vk::SampleCountFlags {
    match coordinates {
        crate::RenderCoordinateConvention::TopLeftLogicalV2 => offered,
        crate::RenderCoordinateConvention::LegacyBottomUpV1 => vk::SampleCountFlags::empty(),
    }
}

/// The counts of `properties` that hold the 1/16 grid and a one-pixel location grid.
fn admitted_counts(
    properties: &vk::PhysicalDeviceSampleLocationsPropertiesEXT<'_>,
    grid: impl Fn(vk::SampleCountFlags) -> vk::Extent2D,
) -> vk::SampleCountFlags {
    if properties.sample_location_sub_pixel_bits < SUBPIXEL_BITS {
        return vk::SampleCountFlags::empty();
    }
    let offered = properties.sample_location_sample_counts & MIRRORED_COUNTS;
    [
        vk::SampleCountFlags::TYPE_2,
        vk::SampleCountFlags::TYPE_4,
        vk::SampleCountFlags::TYPE_8,
    ]
    .into_iter()
    .filter(|samples| offered.contains(*samples))
    .filter(|samples| {
        let size = grid(*samples);
        size.width >= 1 && size.height >= 1
    })
    .fold(vk::SampleCountFlags::empty(), |counts, samples| {
        counts | samples
    })
}

/// The depth-attachment creation flag that admits the mirrored positions.
/// Example: `info.flags(info.flags | depth_flags(counts, samples, aspect))`.
pub(super) fn depth_flags(
    counts: vk::SampleCountFlags,
    samples: vk::SampleCountFlags,
    aspect: vk::ImageAspectFlags,
) -> vk::ImageCreateFlags {
    if aspect.contains(vk::ImageAspectFlags::DEPTH) && places_mirrored(counts, samples) {
        return vk::ImageCreateFlags::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT;
    }
    vk::ImageCreateFlags::empty()
}

/// The locations that place `positions` in every pixel (a one-pixel grid).
/// Example: `locations_info(mirrored_positions(vk::SampleCountFlags::TYPE_4).unwrap())`.
pub(super) fn locations_info(
    positions: &'static [vk::SampleLocationEXT],
) -> vk::SampleLocationsInfoEXT<'static> {
    let samples = vk::SampleCountFlags::from_raw(positions.len() as u32);
    vk::SampleLocationsInfoEXT::default()
        .sample_locations_per_pixel(samples)
        .sample_location_grid_size(vk::Extent2D {
            width: 1,
            height: 1,
        })
        .sample_locations(positions)
}

#[cfg(test)]
#[path = "mirrored_sample_locations_tests.rs"]
mod tests;
