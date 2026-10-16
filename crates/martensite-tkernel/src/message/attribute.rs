//! Named data attached to an extended alert.
use super::json;
use crate::standard::dump::DumpStream;
use crate::standard::{RuntimeType, RuntimeTyped, StandardTransient};
use crate::tcollection::AsciiString;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// Shared attribute handle.
pub type AttributeHandle = Arc<dyn Attribute>;

/// State every attribute embeds: the custom name.
#[derive(Debug, Default)]
pub struct AttributeBase {
    name: Mutex<AsciiString>,
}
impl AttributeBase {
    /// Creates the state with a name; `AttributeBase::new(&AsciiString::new("step"))`.
    pub fn new(name: &AsciiString) -> Self {
        Self {
            name: Mutex::new(name.clone()),
        }
    }
    /// The custom name; `base.name()`.
    pub fn name(&self) -> AsciiString {
        self.name
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
    /// Replaces the custom name; `base.set_name(&AsciiString::new("step"))`.
    pub fn set_name(&self, name: &AsciiString) {
        *self
            .name
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = name.clone();
    }
}

/// Attribute behavior; the provided methods are the base attribute implementations.
///
/// ```
/// use martensite_tkernel::message::{Attribute, BaseAttribute};
/// use martensite_tkernel::tcollection::AsciiString;
/// let attribute = BaseAttribute::new(&AsciiString::new("step"));
/// assert_eq!(attribute.get_message_key().as_bytes(), b"step");
/// ```
pub trait Attribute: Any + Send + Sync {
    /// The embedded name state; `attribute.attribute_base()`.
    fn attribute_base(&self) -> &AttributeBase;
    /// The most-derived runtime descriptor; `attribute.dynamic_type().name()`.
    fn dynamic_type(&self) -> &'static RuntimeType;
    /// Message key: the custom name, or an empty key; `attribute.get_message_key()`.
    fn get_message_key(&self) -> AsciiString {
        self.get_name()
    }
    /// Example: `attribute.get_name()`.
    fn get_name(&self) -> AsciiString {
        self.attribute_base().name()
    }
    /// Example: `attribute.set_name(&AsciiString::new("step"))`.
    fn set_name(&self, name: &AsciiString) {
        self.attribute_base().set_name(name);
    }
    /// Dumps as JSON: `"className": "Message_Attribute", "Name": "<name>"`.
    fn dump_json(&self, output: &mut DumpStream, depth: i32) {
        dump_attribute(self.attribute_base(), output, depth);
    }
}

/// The base attribute JSON dump, which derived dumps write as their base class.
pub(crate) fn dump_attribute(base: &AttributeBase, output: &mut DumpStream, _depth: i32) {
    json::class_begin(output, BaseAttribute::static_type().name());
    json::field_string(output, "Name", base.name().as_bytes());
}

/// A plain attribute; `BaseAttribute::new(&AsciiString::new("step"))`.
#[derive(Debug, Default)]
pub struct BaseAttribute {
    base: AttributeBase,
}
impl BaseAttribute {
    /// Creates a named attribute; `BaseAttribute::new(&AsciiString::default())`.
    pub fn new(name: &AsciiString) -> Self {
        Self {
            base: AttributeBase::new(name),
        }
    }
}
message_runtime_type!(BaseAttribute, "Message_Attribute", StandardTransient);
impl Attribute for BaseAttribute {
    fn attribute_base(&self) -> &AttributeBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
}

/// Downcasts a shared attribute to a concrete type; `downcast_attribute::<AttributeMeter>(&a)`.
pub fn downcast_attribute<T: Attribute>(attribute: &AttributeHandle) -> Option<Arc<T>> {
    let any: Arc<dyn Any + Send + Sync> = attribute.clone();
    any.downcast::<T>().ok()
}
