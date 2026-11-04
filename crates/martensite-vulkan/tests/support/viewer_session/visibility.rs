//! Showing and hiding named objects that stay registered in the context.
use super::*;

impl ViewerSession {
    /// Keep only the named objects displayed: show them, erase every other named object.
    /// Example: `session.display_only(&["b1"])?`.
    pub fn display_only(&self, names: &[&str]) -> Result<(), Failure> {
        let kept: Vec<_> = names
            .iter()
            .filter_map(|name| self.object(name).cloned())
            .collect();
        self.with_update(|context| {
            for object in &kept {
                context.display(object, false)?;
            }
            for (_, object) in self.objects.iter() {
                if !kept.contains(object) {
                    context.erase(object, false)?;
                }
            }
            Ok(())
        })
    }

    /// Erase the named objects, keeping them registered, then update.
    /// Example: `session.erase(&["b"])?`.
    pub fn erase(&self, names: &[&str]) -> Result<(), Failure> {
        let erased: Vec<_> = names
            .iter()
            .filter_map(|name| self.object(name).cloned())
            .collect();
        self.with_update(|context| {
            for object in &erased {
                context.erase(object, false)?;
            }
            Ok(())
        })
    }

    /// Show again objects `erase` hid, failing
    /// on an unknown name; `session.show_erased(&["b"])?`.
    pub fn show_erased(&self, names: &[&str]) -> Result<(), Failure> {
        let shown = self.named_objects(names)?;
        self.with_update(|context| {
            for object in &shown {
                context.display(object, false)?;
            }
            Ok(())
        })
    }

    /// The objects bound to `names`, failing on the first unknown one.
    /// Example: `session.named_objects(&["b"])?`.
    pub fn named_objects(
        &self,
        names: &[&str],
    ) -> Result<Vec<Handle<dyn AisInteractiveObject>>, Failure> {
        let found = names.iter().map(|name| {
            self.object(name).cloned().ok_or_else(|| {
                martensite_tkernel::standard::exceptions::program_error(&format!(
                    "object {name:?} not bound; expected a displayed object name"
                ))
            })
        });
        found.collect()
    }

    /// Remove the named objects from the context and forget their names, then update the
    /// viewer once; `session.remove(&["cyl"])?`.
    pub fn remove(&mut self, names: &[&str]) -> Result<(), Failure> {
        let mut removed = Vec::new();
        for name in names {
            if let Some(object) = self.objects.remove(name) {
                removed.push(object);
            }
        }
        self.with_update(|context| {
            for object in &removed {
                context.remove(object, false)?;
            }
            Ok(())
        })
    }
}

#[cfg(feature = "vertex-output-diagnostics")]
impl ViewerSession {
    /// Resolve an exact diagnostic actor name without alias/rank inference; `session.vertex_output_actor_binding("body")?`.
    pub fn vertex_output_actor_binding(
        &self,
        name: &str,
    ) -> Result<Option<Handle<dyn AisInteractiveObject>>, martensite_vulkan::VertexOutputFailure>
    {
        let matches = self
            .objects
            .iter()
            .filter(|(actual, _)| actual == name)
            .map(|(_, actor)| (actor.object_address() as usize, actor.clone()))
            .collect::<std::collections::BTreeMap<_, _>>();
        if matches.len() > 1 {
            return Err(martensite_vulkan::VertexOutputFailure::AmbiguousSource {
                source: name.to_owned(),
                matches: matches.len(),
            });
        }
        Ok(matches.into_values().next())
    }
}
