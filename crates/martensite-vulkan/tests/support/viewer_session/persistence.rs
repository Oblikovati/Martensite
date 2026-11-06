//! Objects kept at a window corner or anchor by a transform persistence, and trihedrons.
use super::display::display_new;
use super::*;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkmath::gp::{Direction3, Point3};

/// A trihedron or 2D persistence at `corner` with a pixel `offset`, created at the lower-left
/// corner and moved as the interactive viewer does.
/// Example: `corner_persistence(Graphic3dTransModeFlags::TRIEDRON_PERS, corner, [40, 40])?`.
pub fn corner_persistence(
    flags: Graphic3dTransModeFlags,
    corner: AspectTypeOfTriedronPosition,
    [x, y]: [i32; 2],
) -> Result<Handle<dyn Graphic3dTransformPersSource>, Failure> {
    let persistence = Graphic3dTransformPers::with_corner(
        flags,
        AspectTypeOfTriedronPosition::LEFT_LOWER,
        Vec2::new(0, 0),
    )?;
    persistence.set_corner_2d(corner)?;
    persistence.set_offset_2d(Vec2::new(x, y))?;
    Ok(Handle::new(persistence).cast())
}

impl ViewerSession {
    /// Display a new presentation of `shape` as `name` with `persistence` set before its
    /// first display; `session.display_persistent("b", &b, pers, &request)?`.
    pub fn display_persistent(
        &mut self,
        name: &str,
        shape: &Shape,
        persistence: Handle<dyn Graphic3dTransformPersSource>,
        request: &DisplayRequest,
    ) -> Result<(), Failure> {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        self.objects.push(name, object.clone());
        self.with_update(|context| {
            context.set_transform_persistence(&object, &persistence)?;
            display_new(context, &object, request)
        })
    }

    /// Give the displayed non-shape object `name` a persistence and display it again with
    /// the context defaults; `session.redisplay_persistent("t", pers)?`.
    pub fn redisplay_persistent(
        &self,
        name: &str,
        persistence: Handle<dyn Graphic3dTransformPersSource>,
    ) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} not displayed; expected a displayed object name"
            ))
        })?;
        self.with_update(|context| {
            context.set_transform_persistence(&object, &persistence)?;
            context.display(&object, false)
        })
    }

    /// Display as `name` a trihedron on the world axes; `session.display_trihedron("tri")?`.
    pub fn display_trihedron(&mut self, name: &str) -> Result<(), Failure> {
        let placement = martensite_tkg3d::geom::GeomAxis2Placement::from_directions(
            Point3::new(0., 0., 0.),
            Direction3::new(0., 0., 1.)?,
            Direction3::new(1., 0., 0.)?,
        )?;
        let trihedron = AisTrihedron::new(&Handle::new(placement))?;
        self.display_object(name, trihedron.cast())
    }

    /// Select the object `name` when it is not selected and deselect it otherwise.
    /// Example: `session.toggle_selected("b")?`.
    pub fn toggle_selected(&self, name: &str) -> Result<(), Failure> {
        let object = self.object(name).cloned().unwrap_or_default();
        self.context.add_or_remove_selected_object(&object, true)
    }

    /// Whether the object `name` is selected; `session.is_selected("bc")`.
    pub fn is_selected(&self, name: &str) -> bool {
        self.object(name)
            .is_some_and(|object| self.context.is_selected_object(object))
    }
}
