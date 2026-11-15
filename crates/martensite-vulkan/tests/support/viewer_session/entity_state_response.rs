//! The response uses actual selector ranks and object bindings without another pick.
use super::*;
#[path = "../../viewer_commands/display_commands/detected_entity_response.rs"]
mod response;

impl ViewerSession {
    /// Read the full response for the current detection; `session.detected_entities_response()?`.
    pub fn detected_entities_response(&self) -> Result<String, Failure> {
        let selector = self.context().main_selector();
        let name = |object: &Handle<dyn AisInteractiveObject>| {
            self.objects
                .iter()
                .find(|(_, bound)| bound == object)
                .map(|(name, _)| name.clone())
                .unwrap_or_default()
        };
        response::detected_entity_response(selector.get(), &name)
    }
}

#[cfg(test)]
#[path = "entity_state_response_controls.rs"]
mod controls;
