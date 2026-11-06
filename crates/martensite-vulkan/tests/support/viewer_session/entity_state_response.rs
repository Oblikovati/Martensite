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
mod controls {
    use super::*;
    use martensite_tkg3d::geom::GeomCartesianPoint;
    use martensite_tkservice::null_driver::NullGraphicDriver;
    use martensite_tkv3d::ais::AisPoint;

    fn picked_point_session() -> ViewerSession {
        let mut session =
            ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
        let geometry = Handle::new(GeomCartesianPoint::new(0., 0., 0.)).cast();
        session
            .display_object("actual_point", AisPoint::new(&geometry).unwrap().cast())
            .unwrap();
        session
            .axis_intersections([0., 0., 10.], [0., 0., -1.])
            .unwrap();
        session
    }

    #[test]
    fn actual_null_selector_response_preserves_named_hit_without_repicking() {
        let session = picked_point_session();
        // The sensitive point deliberately sets center distance to the actual ray depth.
        let before = session.picked_depths().unwrap();
        let response = session.detected_entities_response().unwrap();
        assert_eq!(
            response,
            "Detected entities:\n  actual_point         Depth: 10 Distance: 10 Point: 0 0 0 (Select3DSensitivePoint)\n"
        );
        assert_eq!(session.picked_depths().unwrap(), before);
    }
}
