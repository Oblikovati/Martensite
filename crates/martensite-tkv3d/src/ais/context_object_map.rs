//! Context keys retain their actual handle while live walks release the context guard.
use super::{AisGlobalStatus, AisInteractiveObject};
use martensite_tkernel::{
    ncollection::{DataMap, HashBucketTable},
    standard::{Failure, Handle, HandleSlot},
};

type ContextObject = Handle<dyn AisInteractiveObject>;
type ObjectStatus = Handle<AisGlobalStatus>;

pub(super) struct ContextObjectEntry {
    pub(super) object: HandleSlot<dyn AisInteractiveObject>,
    status: ObjectStatus,
}

/// The interactive object's handle and status, indexed by native object identity.
/// Example: `context.fields.read().objects.find(&object)?`.
#[derive(Default)]
pub struct AisContextObjectMap {
    entries: DataMap<usize, ContextObjectEntry>,
}

impl AisContextObjectMap {
    /// Return the number of registered objects; `objects.len()`.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check for an empty context table; `objects.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Reserve the native bucket table; `objects.resize(1000)`.
    pub fn resize(&mut self, size: usize) {
        self.entries.resize(size);
    }

    /// Read the registered object's status; `objects.find(&object)?`.
    pub fn find(&self, object: &ContextObject) -> Result<&ObjectStatus, Failure> {
        self.entries
            .find(&object.hash_code())
            .map(|entry| &entry.status)
    }

    /// Read the status of a registered object; `objects.seek(&object).is_some()`.
    pub fn seek(&self, object: &ContextObject) -> Option<&ObjectStatus> {
        self.entries
            .seek(&object.hash_code())
            .map(|entry| &entry.status)
    }

    /// Check object identity membership; `objects.is_bound(&object)`.
    pub fn is_bound(&self, object: &ContextObject) -> bool {
        self.entries.is_bound(&object.hash_code())
    }

    /// Add an object or replace its status; `objects.bind(object, status)`.
    pub fn bind(&mut self, object: ContextObject, status: ObjectStatus) -> bool {
        let key = object.hash_code();
        let mut inserted = false;
        let entry = self.entries.try_emplaced(key, || {
            inserted = true;
            ContextObjectEntry {
                object: HandleSlot::new(object),
                status: Handle::default(),
            }
        });
        entry.status = status;
        inserted
    }

    /// Remove the object and its status; `objects.unbind(&object)`.
    pub fn unbind(&mut self, object: &ContextObject) -> bool {
        self.entries.unbind(&object.hash_code())
    }

    /// Walk borrowed keys and values through the original bucket chains; `objects.items()`.
    pub fn items(&self) -> impl Iterator<Item = (&ContextObject, &ObjectStatus)> {
        self.entries
            .items()
            .map(|(_, entry)| (&*entry.object, &entry.status))
    }

    pub(super) fn buckets(&self) -> Option<HashBucketTable<'_, usize, ContextObjectEntry>> {
        self.entries.buckets()
    }
}
