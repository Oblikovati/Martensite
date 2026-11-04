//! Category order and keys of the Vulkan diagnostic dictionary against the native body.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dFrameStatsDict;

fn diagnostics() -> ViewDiagnostics {
    ViewDiagnostics {
        facts: Some(RenderDiagnosticFacts {
            instance_version: ash::vk::make_api_version(0, 1, 4, 313),
            api_version: ash::vk::make_api_version(0, 1, 3, 289),
            validation: true,
            instance_extensions: vec!["VK_KHR_surface".into(), "VK_EXT_debug_utils".into()],
            device_extensions: vec!["VK_KHR_swapchain".into()],
            scene_formats: [
                "VK_FORMAT_R8G8B8A8_SRGB".into(),
                "VK_FORMAT_D32_SFLOAT_S8_UINT".into(),
            ],
            frame_prepared: true,
        }),
        identity: None,
        limits: None,
        memory: None,
        viewport: (64, 48),
        resolution_ratio: 1.5,
    }
}

fn entries(diagnostics: &ViewDiagnostics, flags: Info) -> Vec<(String, String)> {
    let mut dict = Graphic3dFrameStatsDict::default();
    diagnostics.fill(&mut dict, flags);
    (1..=dict.extent())
        .map(|index| {
            let text = |value: &AsciiString| String::from_utf8_lossy(value.as_bytes()).into_owned();
            (
                text(dict.find_key(index).unwrap()),
                text(dict.find_from_index(index).unwrap()),
            )
        })
        .collect()
}

fn keys(diagnostics: &ViewDiagnostics, flags: Info) -> Vec<String> {
    entries(diagnostics, flags)
        .into_iter()
        .map(|(key, _)| key)
        .collect()
}

#[test]
fn complete_dictionary_follows_the_native_category_order() {
    assert_eq!(
        keys(&diagnostics(), Info::COMPLETE),
        [
            "VKloaderVersion",
            "VKinstanceExtensions",
            "VKversion",
            "VKdebug",
            "Viewport",
            "Window buffer",
            "VKextensions",
            "ResolutionRatio",
            "FBO buffer",
        ]
    );
}

#[test]
fn values_spell_versions_formats_and_extension_lists() {
    let values = entries(&diagnostics(), Info::COMPLETE);
    let value = |key: &str| values.iter().find(|(k, _)| k == key).unwrap().1.clone();
    assert_eq!(value("VKloaderVersion"), "1.4.313");
    assert_eq!(
        value("VKinstanceExtensions"),
        "VK_KHR_surface VK_EXT_debug_utils"
    );
    assert_eq!(value("VKversion"), "1.3.289");
    assert_eq!(value("Viewport"), "64x48");
    assert_eq!(value("ResolutionRatio"), "1.5");
    assert_eq!(
        value("FBO buffer"),
        "VK_FORMAT_R8G8B8A8_SRGB VK_FORMAT_D32_SFLOAT_S8_UINT"
    );
}

#[test]
fn extension_lists_need_the_extensions_flag() {
    assert_eq!(
        keys(&diagnostics(), Info::BASIC),
        [
            "VKloaderVersion",
            "VKversion",
            "VKdebug",
            "Viewport",
            "Window buffer",
            "ResolutionRatio",
            "FBO buffer",
        ]
    );
}

#[test]
fn unprepared_frames_have_no_fbo_buffer_and_injected_devices_add_nothing() {
    let mut unprepared = diagnostics();
    unprepared.facts.as_mut().unwrap().frame_prepared = false;
    assert_eq!(
        keys(&unprepared, Info::FRAME_BUFFER),
        ["Viewport", "Window buffer", "ResolutionRatio"]
    );
    let injected = ViewDiagnostics {
        facts: None,
        ..diagnostics()
    };
    assert!(keys(&injected, Info::COMPLETE).is_empty());
}

#[test]
fn memory_entries_use_truncated_mebibytes() {
    assert_eq!(mebibytes(3 * 1024 * 1024 - 1), "2 MiB");
    assert_eq!(free_bytes(std::iter::empty()), None);
}
