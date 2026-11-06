//! The manipulator command: create, set up, attach, place, drag and detach an interactive
//! manipulator by name, applying its options in the command's order.
use super::*;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3};
use martensite_tkv3d::ais::{
    AisManipulator, AisManipulatorMode, AisManipulatorOptionsForAttach, AisManipulatorSkin,
    AisShape, AisShapeParameters as _,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

/// Where an attached manipulator is placed.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ManipulatorPlacement {
    /// No placement.
    #[default]
    Off,
    /// The centre of the attached objects' boxes (`center`, the attach default).
    Center,
    /// The attached object's local transformation (`location`).
    Location,
    /// The local transformation times the shape's own location (`shapeLocation`).
    ShapeLocation,
}

/// One part setting: axis, mode and on/off.
#[derive(Clone, Copy, Debug)]
pub struct ManipulatorPart {
    /// The axis index 0..=2, or None for every axis.
    pub axis: Option<i32>,
    /// The manipulation mode of the part.
    pub mode: AisManipulatorMode,
    /// Whether the part is shown and active.
    pub enabled: bool,
}

/// The location with optional main and X directions.
#[derive(Clone, Copy, Debug)]
pub struct ManipulatorFrame {
    /// The new origin.
    pub location: [f64; 3],
    /// The new main (Z) direction, else the current one.
    pub direction: Option<[f64; 3]>,
    /// The new X direction, else the current one.
    pub x_direction: Option<[f64; 3]>,
}

/// The steps of a drag: its start, its target and its stop.
#[derive(Clone, Copy, Debug, Default)]
pub struct ManipulatorDrag {
    /// The pixel a drag starts at.
    pub start: Option<(i32, i32)>,
    /// The pixel a drag moves to.
    pub to: Option<(i32, i32)>,
    /// Stop the drag, applying it (true) or aborting (false).
    pub stop: Option<bool>,
}

/// One manipulator command; unset options keep the manipulator's state.
/// Example: `ManipulatorCommand { attach: Some("b"), ..Default::default() }`.
#[derive(Clone, Debug, Default)]
pub struct ManipulatorCommand<'a> {
    /// Activate the detected mode on detection.
    pub auto_activate: Option<bool>,
    /// Follow translation.
    pub follow_translation: Option<bool>,
    /// Follow rotation.
    pub follow_rotation: Option<bool>,
    /// Follow dragging.
    pub follow_dragging: Option<bool>,
    /// The gap between parts.
    pub gap: Option<f32>,
    /// The size.
    pub size: Option<f32>,
    /// Part settings, in order.
    pub parts: Vec<ManipulatorPart>,
    /// The location frame.
    pub frame: Option<ManipulatorFrame>,
    /// Keep the manipulator zoomable or not.
    pub zoomable: Option<bool>,
    /// Draw the manipulator flat or not.
    pub flat: Option<bool>,
    /// The placement of an attached manipulator.
    pub placement: Option<ManipulatorPlacement>,
    /// Adjust the size to the attached object.
    pub adjust_size: Option<bool>,
    /// Enable the manipulation modes.
    pub enable_modes: Option<bool>,
    /// The drag steps.
    pub drag: ManipulatorDrag,
    /// A move, scale or rotation.
    pub transform: Option<Transform3>,
    /// Attach in addition to the current objects.
    pub add_object: bool,
    /// Detach the manipulator.
    pub detach: bool,
    /// The object to attach.
    pub attach: Option<&'a str>,
    /// Shown only in the view of this opening-order index.
    pub view: Option<usize>,
}

impl ViewerSession {
    /// Run one manipulator command on the manipulator `name`, creating it on first use.
    /// Example: `session.manipulate("m", &ManipulatorCommand { attach: Some("b"), ..Default::default() })?`.
    pub fn manipulate(
        &mut self,
        name: &str,
        command: &ManipulatorCommand<'_>,
    ) -> Result<(), Failure> {
        let manipulator = self.named_manipulator(name)?;
        let target = command
            .attach
            .map(|object| self.named(object))
            .transpose()?;
        let value = manipulator.get().ok_or_else(null_manipulator)?;
        set_behaviour(value, command)?;
        self.set_skin(&manipulator, command)?;
        if command.detach {
            value.detach()?;
            self.remove(&[name])?;
        }
        if let Some(object) = &target {
            attach(value, object, command)?;
        }
        self.limit_to_view(&manipulator, command)?;
        place(value, command, target.is_some())?;
        drag(value, command, self.view())?;
        self.redisplay_manipulator(&manipulator)
    }

    fn named_manipulator(&mut self, name: &str) -> Result<Handle<AisManipulator>, Failure> {
        if let Some(object) = self.object(name) {
            let manipulator = object.cast::<AisManipulator>();
            return (!manipulator.is_null())
                .then_some(manipulator)
                .ok_or_else(|| {
                    program_error(&format!(
                        "{name:?} is not a manipulator; expected a manipulator name"
                    ))
                });
        }
        let manipulator = AisManipulator::new()?;
        manipulator
            .get()
            .ok_or_else(null_manipulator)?
            .set_mode_activation_on_detection(true);
        self.bind_object(name, manipulator.clone().cast())?;
        Ok(manipulator)
    }

    /// Zoomable and flat changes redisplay a displayed manipulator from scratch.
    fn set_skin(
        &self,
        manipulator: &Handle<AisManipulator>,
        command: &ManipulatorCommand<'_>,
    ) -> Result<(), Failure> {
        let value = manipulator.get().ok_or_else(null_manipulator)?;
        let object: Handle<dyn AisInteractiveObject> = manipulator.clone().cast();
        for (option, apply) in [(command.zoomable, 0), (command.flat, 1)] {
            let Some(enabled) = option else { continue };
            if apply == 0 {
                value.set_zoom_persistence(!enabled)?;
            } else {
                value.set_skin_mode(if enabled {
                    AisManipulatorSkin::Flat
                } else {
                    AisManipulatorSkin::Shaded
                });
            }
            if self.context.is_displayed(&object) {
                self.context.remove(&object, false)?;
                self.context.display(&object, false)?;
            }
        }
        Ok(())
    }

    fn limit_to_view(
        &self,
        manipulator: &Handle<AisManipulator>,
        command: &ManipulatorCommand<'_>,
    ) -> Result<(), Failure> {
        let Some(index) = command.view else {
            return Ok(());
        };
        let shown = self
            .views
            .get(index)
            .map(|(_, view)| view.clone())
            .ok_or_else(|| {
                program_error(&format!(
                    "view index {index}; expected one of the {} open views",
                    self.views.len()
                ))
            })?;
        let object: Handle<dyn AisInteractiveObject> = manipulator.clone().cast();
        for (_, view) in &self.views {
            self.context.set_view_affinity(&object, view, false)?;
        }
        self.context.set_view_affinity(&object, &shown, true)
    }

    fn redisplay_manipulator(&self, manipulator: &Handle<AisManipulator>) -> Result<(), Failure> {
        let object: Handle<dyn AisInteractiveObject> = manipulator.clone().cast();
        if self.context.is_displayed(&object) {
            return self.with_update(|context| context.redisplay(&object, false, false));
        }
        self.with_update(|_| Ok(()))
    }
}

/// Follow flags, gap, parts and size, in the command's order of application.
fn set_behaviour(value: &AisManipulator, command: &ManipulatorCommand<'_>) -> Result<(), Failure> {
    if let Some(enabled) = command.auto_activate {
        value.set_mode_activation_on_detection(enabled);
    }
    value.change_transform_behavior(|behaviour| {
        if let Some(on) = command.follow_translation {
            behaviour.set_follow_translation(on);
        }
        if let Some(on) = command.follow_rotation {
            behaviour.set_follow_rotation(on);
        }
        if let Some(on) = command.follow_dragging {
            behaviour.set_follow_dragging(on);
        }
    });
    command
        .gap
        .filter(|gap| *gap >= 0.)
        .inspect(|gap| value.set_gap(*gap));
    for part in &command.parts {
        match part.axis {
            Some(axis) => value.set_part(axis, part.mode, part.enabled)?,
            None => value.set_part_all(part.mode, part.enabled),
        }
    }
    command
        .size
        .filter(|size| *size > 0.)
        .inspect(|size| value.set_size(*size));
    Ok(())
}

fn attach(
    value: &AisManipulator,
    object: &Handle<dyn AisInteractiveObject>,
    command: &ManipulatorCommand<'_>,
) -> Result<(), Failure> {
    let mut options = AisManipulatorOptionsForAttach::default();
    if let Some(placement) = command.placement {
        options.adjust_position = placement == ManipulatorPlacement::Center;
    }
    command.adjust_size.inspect(|on| options.adjust_size = *on);
    command
        .enable_modes
        .inspect(|on| options.enable_modes = *on);
    if !(command.add_object && value.is_attached()) {
        return value.attach(object, &options);
    }
    let objects = value.objects();
    objects
        .get()
        .ok_or_else(null_manipulator)?
        .append(object.clone());
    value.attach_objects(&objects, &options)
}

/// Adjust the placement (unless attaching at the centre, which attach did), then set the frame.
fn place(
    value: &AisManipulator,
    command: &ManipulatorCommand<'_>,
    attached_now: bool,
) -> Result<(), Failure> {
    let placement = command.placement.unwrap_or_default();
    let skip = placement == ManipulatorPlacement::Off
        || (attached_now && placement == ManipulatorPlacement::Center);
    if !skip && value.is_attached() {
        value.set_position(&adjusted_position(value, placement)?)?;
    }
    let Some(frame) = command.frame else {
        return Ok(());
    };
    let current = value.position();
    let direction = frame
        .direction
        .map(direction3)
        .transpose()?
        .unwrap_or(current.direction());
    let x = frame
        .x_direction
        .map(direction3)
        .transpose()?
        .unwrap_or(current.x_direction());
    let [px, py, pz] = frame.location;
    value.set_position(&Axis2::with_x_direction(
        Point3::new(px, py, pz),
        direction,
        x,
    )?)
}

fn adjusted_position(
    value: &AisManipulator,
    placement: ManipulatorPlacement,
) -> Result<Axis2, Failure> {
    let object = value.object()?;
    let base = object
        .get()
        .ok_or_else(null_manipulator)?
        .local_transformation();
    match placement {
        ManipulatorPlacement::Location => martensite_tkmath::gp::xoy().transformed(base),
        ManipulatorPlacement::ShapeLocation => {
            let shape = object.cast::<AisShape>();
            let shape = shape
                .get()
                .ok_or_else(|| program_error("manipulator is not attached to shape"))?;
            martensite_tkmath::gp::xoy()
                .transformed(base * shape.shape().location().transformation())
        }
        _ => Err(program_error(&format!(
            "placement {placement:?} without an object; expected location or shapeLocation"
        ))),
    }
}

fn drag(
    value: &AisManipulator,
    command: &ManipulatorCommand<'_>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    if let Some((x, y)) = command.drag.start {
        value.start_transform(x, y, view)?;
    }
    if let Some((x, y)) = command.drag.to {
        value.transform_at(x, y, view)?;
    }
    if let Some(apply) = command.drag.stop {
        value.stop_transform(apply)?;
    }
    match &command.transform {
        Some(trsf) => value.transform(trsf),
        None => Ok(()),
    }
}

fn direction3([x, y, z]: [f64; 3]) -> Result<Direction3, Failure> {
    Direction3::new(x, y, z)
}

fn program_error(message: &str) -> Failure {
    martensite_tkernel::standard::exceptions::program_error(message)
}

fn null_manipulator() -> Failure {
    program_error("a null manipulator or attached object; expected a live one")
}
