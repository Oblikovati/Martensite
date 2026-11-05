//! The probe registry of the native reference: eleven nodes over nine formats, its order
//! and its provider lookups.
use super::probe_node::probe_node;
use super::reference_lines::{b, esc, s};
use martensite_tkde::de::{DeConfigurationNode, DeProvider, DeWrapper};
use martensite_tkernel::standard::Handle;

pub fn order(wrapper: &DeWrapper) -> String {
    let mut text = String::new();
    for (format, vendors) in wrapper.nodes().items() {
        text += &format!("{}:", esc(format));
        for (vendor, node) in vendors.items() {
            let enabled = node.get().is_some_and(|object| object.is_enabled());
            text += &format!("{}{},", esc(vendor), if enabled { "+" } else { "-" });
        }
        text += ";";
    }
    text
}

pub fn provider(wrapper: &DeWrapper, path: &str, to_import: bool) -> String {
    let mut found: Handle<dyn DeProvider> = Handle::default();
    if !wrapper
        .find_provider(&s(path), to_import, &mut found)
        .expect("a lookup")
    {
        return "none".to_owned();
    }
    let object = found.get().expect("a provider");
    format!(
        "{}/{}",
        esc(&object.get_format()),
        esc(&object.get_vendor())
    )
}

pub fn registry() -> Handle<DeWrapper> {
    let wrapper = Handle::new(DeWrapper::new());
    let object = wrapper.get().expect("a wrapper");
    let nodes = [
        probe_node("VRML", "Martensite", "vrml wrl", true, true, ""),
        probe_node("IGES", "Martensite", "igs iges", true, true, ""),
        probe_node("OBJ", "Martensite", "obj", true, false, ""),
        probe_node("GLTF", "Martensite", "gltf glb", true, true, ""),
        probe_node("PLY", "Martensite", "ply", false, true, ""),
        probe_node("XCAF", "Martensite", "xbf", true, true, ""),
        probe_node("STEP", "Martensite", "step stp", true, true, "ISO-10303"),
        probe_node("STL", "Martensite", "stl", true, true, ""),
        probe_node("BREP", "Martensite", "brep rle", true, true, ""),
        probe_node("STEP", "ACME", "step stp p21", true, false, "ISO-10303"),
        probe_node("STEP", "ZETA", "step", true, true, ""),
    ];
    for node in nodes {
        object.bind(&node.cast());
    }
    wrapper
}
pub fn unbind(wrapper: &DeWrapper, format: &'static str, vendor: &'static str) -> u8 {
    b(wrapper.unbind(&probe_node(format, vendor, "", false, false, "").cast()))
}
pub fn probe_value(
    wrapper: &DeWrapper,
    format: &str,
    vendor: &str,
) -> Handle<dyn DeConfigurationNode> {
    let mut node: Handle<dyn DeConfigurationNode> = Handle::default();
    wrapper.find(&s(format), &s(vendor), &mut node);
    node
}
