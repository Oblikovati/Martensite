//! Named fake targets observe every public request bit without constructing a device.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkv3d::v3d::V3dLightName;
use std::cell::{Cell, RefCell};

#[derive(Debug, PartialEq)]
struct CapturedOptions {
    size: [i32; 2],
    buffer: Graphic3dBufferType,
    stereo: V3dStereoDumpOptions,
    tile: i32,
    aspect: bool,
    layer: i32,
    single: bool,
    light_pointer: usize,
    light_bytes: Vec<u8>,
}

impl CapturedOptions {
    fn read(options: &V3dImageDumpOptions) -> Self {
        Self {
            size: [options.width, options.height],
            buffer: options.buffer_type,
            stereo: options.stereo_options,
            tile: options.tile_size,
            aspect: options.to_adjust_aspect,
            layer: options.target_z_layer_id,
            single: options.is_single_layer,
            light_pointer: options.light_name.as_ptr() as usize,
            light_bytes: options
                .light_name
                .to_ascii_string()
                .unwrap()
                .as_bytes()
                .to_vec(),
        }
    }
}

struct FakeOptionsDumpTarget {
    immediate: Cell<bool>,
    outcome: Option<bool>,
    requests: RefCell<Vec<CapturedOptions>>,
}

impl FakeOptionsDumpTarget {
    fn new(immediate: bool, outcome: Option<bool>) -> Self {
        Self {
            immediate: Cell::new(immediate),
            outcome,
            requests: RefCell::default(),
        }
    }
}

impl SourceDumpTarget for FakeOptionsDumpTarget {
    fn set_immediate_update(&self, enabled: bool) -> bool {
        self.immediate.replace(enabled)
    }
    fn dump(&self, _: &ImagePixMap, options: &V3dImageDumpOptions) -> Result<bool, Failure> {
        assert!(!self.immediate.get());
        self.requests
            .borrow_mut()
            .push(CapturedOptions::read(options));
        self.outcome.ok_or_else(|| {
            Failure::with_message(
                "Standard_Failure",
                "options target error; expected this named failure",
            )
        })
    }
}

fn explicit_options(buffer: Graphic3dBufferType, single: bool) -> V3dImageDumpOptions<'static> {
    V3dImageDumpOptions {
        width: 37,
        height: 19,
        buffer_type: buffer,
        stereo_options: V3dStereoDumpOptions::RightEye,
        tile_size: 7,
        to_adjust_aspect: false,
        target_z_layer_id: 42,
        is_single_layer: single,
        light_name: V3dLightName::from_c_str(c"actual-source-light"),
    }
}

#[test]
fn explicit_options_preserve_every_buffer_layer_stereo_and_borrowed_name_field() {
    for buffer in [
        Graphic3dBufferType::Rgb,
        Graphic3dBufferType::Depth,
        Graphic3dBufferType::ShadowMap,
    ] {
        for single in [false, true] {
            let target = FakeOptionsDumpTarget::new(true, Some(true));
            let options = explicit_options(buffer, single);
            assert!(
                perform_source_dump_options(&target, &options, &ImagePixMap::default()).unwrap()
            );
            assert_eq!(*target.requests.borrow(), [CapturedOptions::read(&options)]);
            assert!(target.immediate.get());
        }
    }
}

#[test]
fn explicit_false_and_typed_error_restore_the_actual_previous_update_state() {
    for initial in [false, true] {
        for outcome in [Some(false), None] {
            let target = FakeOptionsDumpTarget::new(initial, outcome);
            let options = explicit_options(Graphic3dBufferType::Depth, true);
            let result = perform_source_dump_options(&target, &options, &ImagePixMap::default());
            assert_eq!(result.ok(), outcome);
            assert_eq!(target.immediate.get(), initial);
            assert_eq!(target.requests.borrow().len(), 1);
        }
    }
}

#[test]
fn unchanged_default_dump_uses_the_same_request_bits_as_before_the_overload() {
    let target = FakeOptionsDumpTarget::new(true, Some(true));
    perform_source_dump(
        &target,
        (37, 19),
        V3dStereoDumpOptions::Mono,
        &ImagePixMap::default(),
    )
    .unwrap();
    let expected = V3dImageDumpOptions {
        width: 37,
        height: 19,
        ..Default::default()
    };
    assert_eq!(
        *target.requests.borrow(),
        [CapturedOptions::read(&expected)]
    );
}
