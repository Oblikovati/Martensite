//! What the last cursor move picked: the detected owner's point and the picked entity types.
use super::*;
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorResultMembers as _;

impl ViewerSession {
    /// The picked point of the detected owner, or `f64::MAX` coordinates when nothing is
    /// detected, as the move-to command reports it; `session.detected_point()?`.
    pub fn detected_point(&self) -> Result<[f64; 3], Failure> {
        let owner = self.context.detected_owner();
        let selector = self.context.main_selector();
        for rank in 1..=selector.nb_picked() {
            if selector.picked(rank)? == *owner {
                let point = selector.picked_point(rank)?;
                return Ok([point.x(), point.y(), point.z()]);
            }
        }
        Ok([f64::MAX; 3])
    }

    /// The type name of every picked sensitive entity in rank order, such as
    /// "Select3DSensitiveCylinder"; `session.picked_entity_types?`.
    pub fn picked_entity_types(&self) -> Result<Vec<String>, Failure> {
        let selector = self.context.main_selector();
        let mut names = Vec::new();
        for rank in 1..=selector.nb_picked() {
            let entity = selector.picked_entity(rank)?;
            if let Some(entity) = entity.get() {
                names.push(entity.dynamic_type().name().to_owned());
            }
        }
        Ok(names)
    }

    /// The depth of every picked sensitive entity in rank order, read from
    /// the values after "Depth:"; `session.picked_depths()?`.
    pub fn picked_depths(&self) -> Result<Vec<f64>, Failure> {
        let selector = self.context.main_selector();
        (1..=selector.nb_picked())
            .map(|rank| Ok(selector.picked_data(rank)?.depth))
            .collect()
    }
}

/// Owned axis intersection: point, original float normal and display-normal length.
pub type AxisPickObservation = ([f64; 3], [f32; 3], f64);

impl ViewerSession {
    /// Pick on the live controller and own every ranked hit; `session.axis_intersections([0.,0.,6.], [0.,0.,-1.])?`.
    pub fn axis_intersections(
        &mut self,
        [x, y, z]: [f64; 3],
        [dx, dy, dz]: [f64; 3],
    ) -> Result<Vec<AxisPickObservation>, Failure> {
        use martensite_tkmath::gp::{Axis1, Direction3, Point3};
        let axis = Axis1::new(Point3::new(x, y, z), Direction3::new(dx, dy, dz)?);
        let view = self.view().clone();
        let mut first = Point3::new(0., 0., 0.);
        if !self
            .controller
            .pick_axis(&mut first, &self.context, view.view_base(), axis)?
        {
            return Ok(Vec::new());
        }
        let selector = self.context.main_selector();
        (1..=selector.nb_picked())
            .map(|rank| axis_observation(selector.get().unwrap(), rank))
            .collect()
    }
}

fn axis_observation(
    selector: &dyn martensite_tkv3d::select_mgr::SelectMgrViewerSelectorSource,
    rank: i32,
) -> Result<AxisPickObservation, Failure> {
    let hit = selector.picked_data(rank)?;
    let point = hit.point;
    let normal = hit.normal;
    let length = hit
        .entity
        .get()
        .map_or(1., |entity| 0.2 * entity.bounding_box().size().max_comp());
    Ok((
        [point.x(), point.y(), point.z()],
        [normal.x, normal.y, normal.z],
        length,
    ))
}

impl ViewerSession {
    /// Snapshot the native entity-type stream, including a wire's detected child.
    /// Example: `session.picked_entity_state_types()?` preserves real rank/child ordering.
    pub fn picked_entity_state_types(&self) -> Result<Vec<String>, Failure> {
        let selector = self.context.main_selector();
        let mut names = Vec::new();
        for rank in 1..=selector.nb_picked() {
            let entity = selector.picked_entity(rank)?;
            names.extend(entity_state_type_rows(&entity)?);
        }
        Ok(names)
    }
}

fn entity_state_type_rows(
    entity: &Handle<dyn martensite_tkv3d::select3d::Select3DSensitiveEntity>,
) -> Result<Vec<String>, Failure> {
    use martensite_tkv3d::select3d::Select3DSensitiveWire;
    let value = entity.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            "picked entity=null; expected real ranked entity",
        )
    })?;
    let mut names = vec![value.dynamic_type().name().to_owned()];
    if let Some(wire) = entity.cast::<Select3DSensitiveWire>().get() {
        names.push(wire_detected_type(wire)?);
    }
    Ok(names)
}

fn wire_detected_type(
    wire: &martensite_tkv3d::select3d::Select3DSensitiveWire,
) -> Result<String, Failure> {
    let child = wire.get_last_detected();
    let child = child.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            "picked wire child=null; expected actual last detected entity",
        )
    })?;
    Ok(child.dynamic_type().name().to_owned())
}

#[cfg(test)]
mod entity_type_tests {
    use super::*;
    use martensite_tkmath::gp::{Axis1, Axis2, Circle3, Direction3, Point3};
    use martensite_tkv3d::{
        select_basics::SelectBasicsPickResult,
        select_mgr::{SelectMgrEntityOwner, SelectMgrSelectingVolumeManager},
        select3d::{Select3DSensitiveCircle, Select3DSensitiveEntity, Select3DSensitiveWire},
    };

    fn circle_entity() -> Handle<dyn Select3DSensitiveEntity> {
        let owner = SelectMgrEntityOwner::new(0).cast();
        let circle = Circle3::new(Axis2::default(), 1.).unwrap();
        Handle::new(Select3DSensitiveCircle::new(&owner, &circle, true)).cast()
    }

    fn wire_entity() -> Handle<Select3DSensitiveWire> {
        let owner = SelectMgrEntityOwner::new(0).cast();
        let wire = Handle::new(Select3DSensitiveWire::new(&owner));
        wire.add(&circle_entity()).unwrap();
        wire
    }

    #[test]
    fn plain_real_entity_emits_only_its_actual_root_type() {
        let circle = circle_entity();
        assert_eq!(
            entity_state_type_rows(&circle).unwrap(),
            ["Select3DSensitiveCircle"]
        );
        assert!(entity_state_type_rows(&Handle::default()).is_err());
    }

    #[test]
    fn real_wire_match_emits_root_then_actual_detected_circle_type() {
        let wire = wire_entity();
        let mut manager = SelectMgrSelectingVolumeManager::new();
        manager.init_axis_selecting_volume(Axis1::new(Point3::new(0., 0., -2.), Direction3::Z));
        manager.build_selecting_volume().unwrap();
        let mut pick = SelectBasicsPickResult::default();
        assert!(wire.matches(&manager, &mut pick).unwrap());
        assert_eq!(wire.get_last_detected(), wire.get_edges()[0]);
        let types = entity_state_type_rows(&wire.cast()).unwrap();
        assert_eq!(types, ["Select3DSensitiveWire", "Select3DSensitiveCircle"]);
    }

    #[test]
    fn unmatched_wire_cannot_invent_a_detected_child_descriptor() {
        let wire = wire_entity();
        assert!(wire.get_last_detected().is_null());
        let failure = entity_state_type_rows(&wire.cast()).unwrap_err();
        assert!(
            failure
                .message()
                .contains("expected actual last detected entity")
        );
    }
}
