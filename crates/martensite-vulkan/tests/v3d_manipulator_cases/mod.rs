//! Interactive manipulators: attaching, parts, placement, view affinity, zoom persistence and
//! mouse-driven translation, rotation and scaling of the attached objects.
mod attach_parts;
mod drags;
mod moved_selection;
mod object_drags;
pub(crate) use moved_selection::{moved_box_selection, persistent_pair_drag};
mod view_drags;
pub(crate) use attach_parts::{disabled_parts, held_trihedron, located_shape, view_affinity};
pub(crate) use drags::{child_drag, drag_in_2d, gesture_drags, stepped_rotation};
pub(crate) use object_drags::{dragged_object, rotated_object, scaled_object, translated_object};
pub(crate) use view_drags::{flat_skin, turned_camera_translation, zoom_persistence};

/// What every manipulator program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::{
        DisplayRequest, ManipulatorCommand, ManipulatorDrag, ManipulatorPart, ManipulatorPlacement,
    };
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkv3d::ais::AisManipulatorMode;
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// Attach the manipulator to `object` with the attach defaults.
    pub(super) fn attach(object: &str) -> ManipulatorCommand<'_> {
        ManipulatorCommand {
            attach: Some(object),
            ..Default::default()
        }
    }

    /// Detach the manipulator.
    pub(super) fn detach() -> ManipulatorCommand<'static> {
        ManipulatorCommand {
            detach: true,
            ..Default::default()
        }
    }

    /// Hover and select at `from`, then the start, transform and stop commands of a drag
    /// to `to`, each a separate manipulator command; `drag(&mut run, "m", (265, 265), (265, 200))?`.
    pub(super) fn drag(
        run: &mut SessionRun,
        name: &str,
        from: (i32, i32),
        to: (i32, i32),
    ) -> Outcome {
        run.session().move_cursor(from.0, from.1)?;
        selected_drag(run, name, from, to)
    }

    /// Select at `from`, then start, transform to `to` and stop,
    /// without a hover first; `selected_drag(&mut run, "m", (180, 215), (65, 350))?`.
    pub(super) fn selected_drag(
        run: &mut SessionRun,
        name: &str,
        from: (i32, i32),
        to: (i32, i32),
    ) -> Outcome {
        run.session().select_point(from.0, from.1)?;
        let steps = [
            ManipulatorDrag {
                start: Some(from),
                ..Default::default()
            },
            ManipulatorDrag {
                to: Some(to),
                ..Default::default()
            },
            ManipulatorDrag {
                stop: Some(true),
                ..Default::default()
            },
        ];
        for step in steps {
            run.session().manipulate(
                name,
                &ManipulatorCommand {
                    drag: step,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    /// Move to (0, 0), then to `point`: a fresh hover; `hover(&mut run, (80, 133))?`.
    pub(super) fn hover(run: &mut SessionRun, point: (i32, i32)) -> Outcome {
        run.session().move_cursor(0, 0)?;
        run.session().move_cursor(point.0, point.1)?;
        Ok(())
    }

    /// Display a point named `name`.
    pub(super) fn display_point(run: &mut SessionRun, name: &str, [x, y, z]: [f64; 3]) -> Outcome {
        use martensite_tkernel::standard::Handle;
        use martensite_tkg3d::geom::GeomCartesianPoint;
        use martensite_tkv3d::ais::AisPoint;
        let point = Handle::new(GeomCartesianPoint::new(x, y, z));
        run.session()
            .display_object(name, AisPoint::new(&point.cast())?.cast())?;
        Ok(())
    }
}

mod border_selection;
pub(crate) use border_selection::overlapped_border_selection;
