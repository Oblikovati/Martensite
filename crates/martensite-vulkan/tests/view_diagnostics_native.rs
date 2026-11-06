//! GlView::DiagnosticInformation (with GlContext::DiagnosticInformation) on real
//! devices: the Vulkan view fills the same categories in the same order, keeps the native keys
//! whose meaning holds under Vulkan, and reports GL/GLX strings as Vulkan strings.
//!
//! Native dictionaries come from the recorded view_diagnostics capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/view_diagnostics/native.txt run
//! with DISPLAY=:1, LIBGL_ALWAYS_SOFTWARE=1 for llvmpipe). Combined texture units are a device
//! budget that differs between the APIs (see driver_limits_native.rs).
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkservice::graphic3d::{Graphic3dDiagnosticInfo as Info, Graphic3dFrameStatsDict};
use martensite_vulkan::Graphic3dRenderDriver;

const NATIVE: &str = include_str!("fixtures/view_diagnostics/native.txt");
/// VkDriverId values of the qualified drivers.
const DRIVER_ID_MESA_RADV: u32 = 3;
const DRIVER_ID_MESA_LLVMPIPE: u32 = 13;

type Entries = Vec<(String, String)>;

/// The native `== category` block of the driver matching the selected Vulkan device.
fn native_category(driver_id: u32, category: &str) -> Entries {
    let driver = match driver_id {
        DRIVER_ID_MESA_LLVMPIPE => "# driver sw",
        DRIVER_ID_MESA_RADV => "# driver host",
        other => panic!("received driver id {other}; expected lavapipe (13) or RADV (3)"),
    };
    let header = format!("== {category}");
    NATIVE
        .lines()
        .skip_while(|line| *line != driver)
        .skip_while(|line| *line != header)
        .skip(1)
        .take_while(|line| !line.starts_with("== ") && !line.starts_with("# "))
        .map(|line| {
            let (key, value) = line.split_once(": ").expect("native `key: value` line");
            (key.to_string(), value.to_string())
        })
        .collect()
}

struct DiagnosedView {
    driver: martensite_tkernel::standard::Handle<Graphic3dRenderDriver>,
    scene: scene::ViewerScene,
}
impl DiagnosedView {
    /// A 64x64 virtual window redrawn once.
    fn new() -> Self {
        let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
        let scene = scene::ViewerScene::new(&driver);
        scene.render_view().try_redraw().unwrap();
        Self { driver, scene }
    }
    fn entries(&self, flags: Info) -> Entries {
        let mut dict = Graphic3dFrameStatsDict::default();
        let view = self.scene.cview();
        view.get().unwrap().diagnostic_information(&mut dict, flags);
        let text = |value: &martensite_tkernel::tcollection::AsciiString| {
            String::from_utf8_lossy(value.as_bytes()).into_owned()
        };
        (1..=dict.extent())
            .map(|index| {
                let key = text(dict.find_key(index).unwrap());
                (key, text(dict.find_from_index(index).unwrap()))
            })
            .filter(|(key, _)| !key.starts_with("Martensite "))
            .collect()
    }
    fn driver_id(&self) -> u32 {
        self.driver
            .driver_identity()
            .expect("a Vulkan device")
            .driver_id
    }
}

fn value<'a>(entries: &'a Entries, key: &str) -> &'a str {
    let found = entries.iter().find(|(name, _)| name == key);
    &found
        .unwrap_or_else(|| panic!("missing {key} in {entries:?}"))
        .1
}

#[test]
fn limits_equal_the_native_driver_except_the_api_budget() {
    let view = DiagnosedView::new();
    let native = native_category(view.driver_id(), "limits");
    let vulkan = view.entries(Info::LIMITS);
    let keys = |entries: &Entries| entries.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>();
    assert_eq!(keys(&vulkan), keys(&native));
    for key in ["Max texture size", "Max FBO dump size", "Max MSAA samples"] {
        assert_eq!(value(&vulkan, key), value(&native, key), "{key}");
    }
    let units = view
        .driver
        .device_limits()
        .unwrap()
        .max_combined_texture_units();
    assert_eq!(
        value(&vulkan, "Max combined texture units"),
        units.to_string()
    );
    assert!(view.driver.validation_errors().unwrap().is_empty());
}

#[test]
fn frame_buffer_entries_follow_the_native_view() {
    let view = DiagnosedView::new();
    let native = native_category(view.driver_id(), "frame_buffer");
    let vulkan = view.entries(Info::FRAME_BUFFER);
    let keys = |entries: &Entries| entries.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>();
    assert_eq!(keys(&vulkan), keys(&native));
    assert_eq!(value(&vulkan, "Viewport"), value(&native, "Viewport"));
    assert_eq!(
        value(&vulkan, "ResolutionRatio"),
        value(&native, "ResolutionRatio")
    );
    // Native GL_SRGB8_ALPHA8 colour; Vulkan keeps a stencil-capable depth attachment.
    let fbo = value(&vulkan, "FBO buffer");
    assert!(
        fbo.starts_with("VK_FORMAT_R8G8B8A8_SRGB VK_FORMAT_D"),
        "{fbo}"
    );
    assert!(view.driver.validation_errors().unwrap().is_empty());
}

/// Category of a key, so native GL/GLX and Vulkan keys compare by role.
fn category(key: &str) -> &'static str {
    match key {
        k if k.starts_with("GLX") || k.starts_with("VKloader") || k.starts_with("VKinstance") => {
            "platform"
        }
        "GLextensions" | "VKextensions" => "extensions",
        k if k.starts_with("GL") || k.starts_with("VK") => "device",
        k if k.starts_with("Max ") => "limits",
        "Viewport" | "Window buffer" => "window",
        "ResolutionRatio" | "FBO buffer" => "scene",
        _ => "memory",
    }
}

fn category_order(entries: &Entries) -> Vec<&'static str> {
    let mut order: Vec<&'static str> = entries.iter().map(|(key, _)| category(key)).collect();
    order.dedup();
    order
}

#[test]
fn complete_dictionary_orders_categories_like_native() {
    let view = DiagnosedView::new();
    let native = native_category(view.driver_id(), "complete");
    let vulkan = view.entries(Info::COMPLETE);
    assert_eq!(category_order(&vulkan), category_order(&native));
    for key in ["VKvendor", "VKdevice", "VKdriver", "VKversion", "VKdebug"] {
        assert!(!value(&vulkan, key).is_empty(), "{key}");
    }
    assert!(value(&vulkan, "VKextensions").contains("VK_KHR_"));
    assert!(view.driver.validation_errors().unwrap().is_empty());
}
