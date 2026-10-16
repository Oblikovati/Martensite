//! Hash and equality specializations compare full string contents.
use super::HAsciiString;
use crate::{ncollection::HashPolicy, standard::Handle};

/// Content policy for shared ASCII string handle keys; `HAsciiString::hash_policy()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HAsciiStringHasher;
impl HashPolicy<Handle<HAsciiString>> for HAsciiStringHasher {
    fn hash(&self, key: &Handle<HAsciiString>) -> usize {
        key.get().map_or(0, |text| text.string().hash_code())
    }
    fn equal(&self, first: &Handle<HAsciiString>, second: &Handle<HAsciiString>) -> bool {
        if first == second {
            return true;
        }
        match (first.get(), second.get()) {
            (Some(first), Some(second)) => first.string().as_bytes() == second.string().as_bytes(),
            _ => false,
        }
    }
}
impl HAsciiString {
    /// Selects content hashing rather than generic handle identity; `let policy = HAsciiString::hash_policy()`.
    pub fn hash_policy() -> HAsciiStringHasher {
        HAsciiStringHasher
    }
}
