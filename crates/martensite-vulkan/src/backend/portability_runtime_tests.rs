//! Named query fakes establish selected-receiver and pNext composition, not driver support.
use super::super::portability_admission::fixture::PortabilityCatalogFake;
use super::*;
use ash::vk::Handle;
use std::sync::Mutex;

struct SelectedPortabilityFake {
    catalog: PortabilityCatalogFake,
    expected: vk::PhysicalDevice,
    alignment: u32,
    point_polygons: bool,
    queries: Mutex<Vec<&'static str>>,
}
impl SelectedPortabilityFake {
    fn device(advertised: bool, alignment: u32) -> Self {
        let names = if advertised {
            vec![ash::khr::portability_subset::NAME]
        } else {
            vec![]
        };
        Self {
            catalog: PortabilityCatalogFake::named(&names),
            expected: vk::PhysicalDevice::from_raw(73),
            alignment,
            point_polygons: false,
            queries: Mutex::new(Vec::new()),
        }
    }
}
impl DevicePortabilitySource for SelectedPortabilityFake {
    fn extensions(
        &self,
        physical: vk::PhysicalDevice,
    ) -> Result<Vec<vk::ExtensionProperties>, RenderError> {
        assert_eq!(physical, self.expected);
        self.queries.lock().unwrap().push("extensions");
        Ok(self.catalog.extensions.clone())
    }
    fn stride_alignment(&self, physical: vk::PhysicalDevice) -> u32 {
        assert_eq!(physical, self.expected);
        self.queries.lock().unwrap().push("subset-properties");
        self.alignment
    }
    fn point_polygons(&self, physical: vk::PhysicalDevice) -> bool {
        assert_eq!(physical, self.expected);
        self.queries.lock().unwrap().push("subset-features");
        self.point_polygons
    }
}

#[test]
fn native_device_never_queries_unadvertised_subset_structures() {
    let source = SelectedPortabilityFake::device(false, 0);
    let admission = admit_device(&source, source.expected).unwrap();
    assert_eq!(admission.extension, None);
    assert!(admission.requested_features().is_none());
    assert_eq!(*source.queries.lock().unwrap(), ["extensions"]);
}

#[test]
fn subset_query_and_admission_use_the_same_selected_physical_receiver() {
    let source = SelectedPortabilityFake::device(true, 4);
    let admission = admit_device(&source, source.expected).unwrap();
    assert_eq!(
        admission.extension,
        Some(ash::khr::portability_subset::NAME)
    );
    assert!(admission.requested_features().is_some());
    assert_eq!(
        *source.queries.lock().unwrap(),
        ["extensions", "subset-properties", "subset-features"]
    );
}

#[test]
fn queried_incompatible_selected_device_preserves_typed_cause() {
    let source = SelectedPortabilityFake::device(true, 16);
    let error = admit_device(&source, source.expected).unwrap_err();
    assert_eq!(
        error.portability_failure(),
        Some(
            &crate::RenderPortabilityFailure::IncompatibleVertexStrideAlignment {
                observed: 16,
                required_divisor: 4,
            }
        )
    );
    assert_eq!(
        *source.queries.lock().unwrap(),
        ["extensions", "subset-properties"]
    );
}

#[test]
fn offscreen_enumeration_is_independent_of_window_and_validation_extensions() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_enumeration::NAME]);
    let (names, flags) = compose_instance_extensions(Vec::new(), &catalog.extensions, false);
    assert_eq!(names, [ash::khr::portability_enumeration::NAME.as_ptr()]);
    assert_eq!(flags, vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);
    let (names, _) = compose_instance_extensions(
        vec![ash::khr::surface::NAME.as_ptr()],
        &catalog.extensions,
        true,
    );
    assert_eq!(
        names,
        [
            ash::khr::surface::NAME.as_ptr(),
            ash::khr::portability_enumeration::NAME.as_ptr(),
            ash::ext::debug_utils::NAME.as_ptr(),
            crate::backend::validation_settings::LAYER_SETTINGS_EXTENSION.as_ptr()
        ]
    );
}

#[test]
fn absent_subset_preserves_original_device_chain_exactly() {
    let mut core = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true);
    let before = vk::DeviceCreateInfo::default().push_next(&mut core);
    let original = before.p_next;
    let mut subset = None;
    let after = link_subset(before, &mut subset);
    assert_eq!(after.p_next, original);
    assert_eq!(after.s_type, before.s_type);
    assert_eq!(
        after.enabled_extension_count,
        before.enabled_extension_count
    );
}

#[test]
fn subset_prepends_a_live_owner_without_replacing_existing_feature_nodes() {
    let mut core = vk::PhysicalDeviceVulkan13Features::default()
        .dynamic_rendering(true)
        .synchronization2(true);
    let original = vk::DeviceCreateInfo::default().push_next(&mut core);
    let old_head = original.p_next;
    let mut subset = Some(vk::PhysicalDevicePortabilitySubsetFeaturesKHR::default());
    let after = link_subset(original, &mut subset);
    inspect_subset_chain(&after, old_head);
}

fn inspect_subset_chain(after: &vk::DeviceCreateInfo<'_>, old_head: *const std::ffi::c_void) {
    // Both lexical owners remain alive while this description is consumed.
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let head = unsafe {
        &*after
            .p_next
            .cast::<vk::PhysicalDevicePortabilitySubsetFeaturesKHR<'_>>()
    };
    assert_eq!(
        head.s_type,
        vk::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR
    );
    assert_eq!(head.p_next.cast_const(), old_head);
    assert_eq!(head.triangle_fans, vk::FALSE);
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let tail = unsafe { &*head.p_next.cast::<vk::PhysicalDeviceVulkan13Features<'_>>() };
    assert_eq!(
        (tail.dynamic_rendering, tail.synchronization2),
        (vk::TRUE, vk::TRUE)
    );
}

#[test]
fn supported_subset_polygon_points_are_the_only_new_requested_feature() {
    let mut source = SelectedPortabilityFake::device(true, 4);
    source.point_polygons = true;
    let plan = admit_device(&source, source.expected).unwrap();
    let requested = plan.requested_features().unwrap();
    assert_eq!(requested.point_polygons, vk::TRUE);
    assert_eq!(requested.triangle_fans, vk::FALSE);
    assert_eq!(requested.events, vk::FALSE);
}
#[test]
fn unsupported_subset_polygon_points_remain_disabled() {
    let source = SelectedPortabilityFake::device(true, 4);
    let plan = admit_device(&source, source.expected).unwrap();
    assert_eq!(plan.requested_features().unwrap().point_polygons, vk::FALSE);
}
