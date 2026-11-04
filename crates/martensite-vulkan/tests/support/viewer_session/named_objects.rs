//! The session's name-to-object bindings, kept in binding order with constant-time lookup,
//! as the interactive viewer's double map is: regressions that bind tens of thousands of
//! objects must not pay a list search per command.
use super::*;
use std::collections::{HashMap, VecDeque};

/// Ordered bindings; a name bound twice resolves to its earliest live binding.
/// Example: `let mut bound = NamedObjects::default(); bound.push("b", object);`.
#[derive(Default)]
pub(super) struct NamedObjects {
    slots: Vec<Option<(String, Handle<dyn AisInteractiveObject>)>>,
    by_name: HashMap<String, VecDeque<usize>>,
    by_object: HashMap<usize, Vec<usize>>,
}

fn address(object: &Handle<dyn AisInteractiveObject>) -> usize {
    object.object_address() as usize
}

impl NamedObjects {
    /// Append the binding `name` -> `object`.
    pub(super) fn push(&mut self, name: &str, object: Handle<dyn AisInteractiveObject>) {
        let index = self.slots.len();
        self.by_name
            .entry(name.to_owned())
            .or_default()
            .push_back(index);
        self.by_object
            .entry(address(&object))
            .or_default()
            .push(index);
        self.slots.push(Some((name.to_owned(), object)));
    }

    /// The object of the earliest live binding of `name`.
    pub(super) fn get(&self, name: &str) -> Option<&Handle<dyn AisInteractiveObject>> {
        let index = *self.by_name.get(name)?.front()?;
        self.slots[index].as_ref().map(|(_, object)| object)
    }

    /// Remove the earliest live binding of `name`, returning its object.
    pub(super) fn remove(&mut self, name: &str) -> Option<Handle<dyn AisInteractiveObject>> {
        let index = *self.by_name.get(name)?.front()?;
        self.take(index)
    }

    /// Remove every binding of `object`.
    pub(super) fn remove_object(&mut self, object: &Handle<dyn AisInteractiveObject>) {
        for index in self.by_object.remove(&address(object)).unwrap_or_default() {
            self.take(index);
        }
    }

    /// Remove and return every binding in binding order.
    pub(super) fn drain(&mut self) -> Vec<(String, Handle<dyn AisInteractiveObject>)> {
        self.by_name.clear();
        self.by_object.clear();
        self.slots.drain(..).flatten().collect()
    }

    /// The live bindings in binding order.
    pub(super) fn iter(&self) -> impl Iterator<Item = &(String, Handle<dyn AisInteractiveObject>)> {
        self.slots.iter().flatten()
    }

    fn take(&mut self, index: usize) -> Option<Handle<dyn AisInteractiveObject>> {
        let (name, object) = self.slots.get_mut(index)?.take()?;
        if let Some(indices) = self.by_name.get_mut(&name) {
            indices.retain(|&i| i != index);
            if indices.is_empty() {
                self.by_name.remove(&name);
            }
        }
        if let Some(indices) = self.by_object.get_mut(&address(&object)) {
            indices.retain(|&i| i != index);
        }
        Some(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object() -> Handle<dyn AisInteractiveObject> {
        AisConnectedInteractive::new(martensite_tkv3d::prs_mgr::PrsMgrTypeOfPresentation3d::AllView)
            .cast()
    }

    #[test]
    fn earliest_binding_resolves_and_order_is_kept() {
        let (a, b) = (object(), object());
        let mut bound = NamedObjects::default();
        bound.push("x", a.clone());
        bound.push("x", b.clone());
        assert_eq!(
            bound.get("x").map(|o| o.object_address()),
            Some(a.object_address())
        );
        bound.remove("x");
        assert_eq!(
            bound.get("x").map(|o| o.object_address()),
            Some(b.object_address())
        );
        bound.remove_object(&b);
        assert!(bound.get("x").is_none() && bound.iter().next().is_none());
    }
}
