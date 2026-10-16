//! A situation met by an algorithm, identified by a message key.
use super::json;
use crate::standard::dump::DumpStream;
use crate::standard::{RuntimeType, RuntimeTyped, StandardTransient};
use crate::tcollection::AsciiString;
use std::any::Any;
use std::sync::Arc;

/// Shared alert handle; alerts are never null.
pub type AlertHandle = Arc<dyn Alert>;

/// Alert behavior; the provided methods are the base alert implementations.
///
/// ```
/// use martensite_tkernel::message::{Alert, BaseAlert};
/// use martensite_tkernel::standard::RuntimeTyped;
/// let alert = BaseAlert::default();
/// assert_eq!(alert.get_message_key().as_bytes(), b"Message_Alert");
/// ```
pub trait Alert: Any + Send + Sync {
    /// The most-derived runtime descriptor; `alert.dynamic_type().name()`.
    fn dynamic_type(&self) -> &'static RuntimeType;
    /// Key of the user message describing the alert; the type name by default.
    fn get_message_key(&self) -> AsciiString {
        AsciiString::new(self.dynamic_type().name())
    }
    /// Whether alerts of this type merge with an earlier one; true by default.
    fn supports_merge(&self) -> bool {
        true
    }
    /// Merges this alert's data into `target`; the base merges trivially and returns true.
    fn merge(&self, _target: &AlertHandle) -> bool {
        true
    }
    /// Dumps the alert as JSON; the base writes `"className": "Message_Alert"` (the static class name).
    fn dump_json(&self, output: &mut DumpStream, _depth: i32) {
        json::class_begin(output, BaseAlert::static_type().name());
    }
}

/// A plain alert instance; `Arc::new(BaseAlert::default())`.
#[derive(Debug, Default)]
pub struct BaseAlert;
message_runtime_type!(BaseAlert, "Message_Alert", StandardTransient);
impl Alert for BaseAlert {
    fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
}

/// Declares a data-less alert type whose message key is its type name.
///
/// ```
/// martensite_tkernel::define_simple_alert!(MyAlert, "MyAlert");
/// use martensite_tkernel::message::Alert;
/// assert_eq!(MyAlert.get_message_key().as_bytes(), b"MyAlert");
/// ```
#[macro_export]
macro_rules! define_simple_alert {
    ($alert:ident, $name:literal) => {
        #[doc = concat!("Simple alert `", $name, "`.")]
        #[derive(Debug, Default)]
        pub struct $alert;
        $crate::message_runtime_type!($alert, $name, $crate::message::BaseAlert);
        impl $crate::message::Alert for $alert {
            fn dynamic_type(&self) -> &'static $crate::standard::RuntimeType {
                <Self as $crate::standard::RuntimeTyped>::static_type()
            }
        }
    };
}

/// Downcasts a shared alert to a concrete type `downcast_alert::<AlertExtended>(&a)`.
pub fn downcast_alert<T: Alert>(alert: &AlertHandle) -> Option<Arc<T>> {
    let any: Arc<dyn Any + Send + Sync> = alert.clone();
    any.downcast::<T>().ok()
}
