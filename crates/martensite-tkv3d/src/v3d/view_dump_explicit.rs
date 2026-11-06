//! Full explicit image-dump arguments reuse the established options consumer.
use super::{V3dImageDumpOptions, V3dLightName, V3dStereoDumpOptions, V3dView};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::{
    graphic3d::{Graphic3dBufferType, Graphic3dZLayerId},
    image::ImagePixMap,
};

type ExplicitDumpFrame = (i32, i32, Graphic3dBufferType, bool);
type ExplicitDumpSelection<'anchor> = (
    Graphic3dZLayerId,
    i32,
    V3dStereoDumpOptions,
    V3dLightName<'anchor>,
);

impl V3dView {
    /// Forward all explicit dump arguments, preserving the borrowed light-name pointer.
    /// Example: `view.to_pix_map_full(&image, 64, 32, Graphic3dBufferType::Rgb, true, -5, 0, V3dStereoDumpOptions::Mono, V3dLightName::default())?`.
    // The native overload keeps each argument independently selectable; the older convenience stays available.
    #[allow(clippy::too_many_arguments)]
    pub fn to_pix_map_full(
        &self,
        image: &ImagePixMap,
        width: i32,
        height: i32,
        buffer_type: Graphic3dBufferType,
        to_adjust_aspect: bool,
        target_z_layer_id: Graphic3dZLayerId,
        single_layer: i32,
        stereo_options: V3dStereoDumpOptions,
        light_name: V3dLightName<'_>,
    ) -> Result<bool, Failure> {
        let frame = (width, height, buffer_type, to_adjust_aspect);
        let selection = (target_z_layer_id, single_layer, stereo_options, light_name);
        self.to_pix_map(image, &explicit_dump_parameters(frame, selection))
    }
}

fn explicit_dump_parameters<'anchor>(
    frame: ExplicitDumpFrame,
    selection: ExplicitDumpSelection<'anchor>,
) -> V3dImageDumpOptions<'anchor> {
    let (width, height, buffer_type, to_adjust_aspect) = frame;
    let (target_z_layer_id, single_layer, stereo_options, light_name) = selection;
    V3dImageDumpOptions {
        width,
        height,
        buffer_type,
        to_adjust_aspect,
        target_z_layer_id,
        is_single_layer: single_layer != 0,
        stereo_options,
        light_name,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_dump_integer_truth_preserves_every_other_option() {
        let name = V3dLightName::from_c_str(c"selected-light");
        for flag in [i32::MIN, -1, 0, 1, i32::MAX] {
            let options = explicit_dump_parameters(
                (64, 32, Graphic3dBufferType::ShadowMap, false),
                (42, flag, V3dStereoDumpOptions::Mono, name),
            );
            assert_eq!((options.width, options.height), (64, 32));
            assert_eq!(options.buffer_type, Graphic3dBufferType::ShadowMap);
            assert!(!options.to_adjust_aspect);
            assert_eq!(options.target_z_layer_id, 42);
            assert_eq!(options.is_single_layer, flag != 0);
            assert_eq!(options.stereo_options, V3dStereoDumpOptions::Mono);
            assert_eq!(options.light_name.as_ptr(), name.as_ptr());
            assert_eq!(options.tile_size, V3dImageDumpOptions::default().tile_size);
        }
    }

    #[test]
    fn explicit_dump_keeps_all_stereo_variants_and_borrowed_non_utf8_bytes() {
        use V3dStereoDumpOptions::{Blended, LeftEye, Mono, RightEye};
        let bytes = [0xff, 0x80, 0];
        let text = std::ffi::CStr::from_bytes_with_nul(&bytes).unwrap();
        let name = V3dLightName::from_c_str(text);
        for stereo in [Mono, LeftEye, RightEye, Blended] {
            let options = explicit_dump_parameters(
                (8, 8, Graphic3dBufferType::ShadowMap, true),
                (42, 1, stereo, name),
            );
            assert_eq!(options.stereo_options, stereo);
            assert_eq!(options.light_name.as_ptr(), text.as_ptr());
            let consumed = options.light_name.to_ascii_string().unwrap();
            assert_eq!(consumed.as_bytes(), &bytes[..2]);
        }
    }

    #[test]
    fn explicit_dump_preserves_null_and_empty_as_distinct_pointers() {
        let empty = V3dLightName::from_c_str(c"");
        for name in [V3dLightName::null(), empty] {
            let options = explicit_dump_parameters(
                (8, 8, Graphic3dBufferType::Rgb, true),
                (-5, 0, V3dStereoDumpOptions::Mono, name),
            );
            assert_eq!(options.light_name.as_ptr(), name.as_ptr());
            assert_eq!(
                options.light_name.as_ptr().is_null(),
                name.as_ptr().is_null()
            );
        }
    }
}
