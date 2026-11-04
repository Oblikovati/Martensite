//! Named, deterministic extension catalog shared by CPU-only admission tests.
use ash::vk;
use std::ffi::CStr;

pub(in crate::backend) struct PortabilityCatalogFake {
    pub(in crate::backend) extensions: Vec<vk::ExtensionProperties>,
}
impl PortabilityCatalogFake {
    /// Build a deterministic native-name catalog; `PortabilityCatalogFake::named(&[name])`.
    pub(in crate::backend) fn named(names: &[&CStr]) -> Self {
        let extensions = names
            .iter()
            .map(|name| {
                let mut extension = vk::ExtensionProperties::default();
                for (target, byte) in extension
                    .extension_name
                    .iter_mut()
                    .zip(name.to_bytes_with_nul())
                {
                    *target = *byte as std::ffi::c_char;
                }
                extension
            })
            .collect();
        Self { extensions }
    }
}
