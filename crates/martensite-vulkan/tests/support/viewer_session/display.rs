//! Displaying a named shape, either as a new interactive object or by updating the existing one.
use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::ais::{AisShapeMembers as _, AisShapeSource};
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers as _;

/// Where a transform-persistent object is anchored.
#[derive(Clone, Copy, Debug)]
pub enum PersistenceAnchor {
    /// The persistence's own origin anchor, left unchanged.
    Origin,
    /// Zoom/rotate persistence re-anchored at a 3D point.
    Point([f64; 3]),
}

/// Presentation options applied before display; `DisplayRequest::default()` is a plain display.
#[derive(Clone, Debug, Default)]
pub struct DisplayRequest {
    /// The object's own display mode (0 wireframe, 1 shaded); `None` follows the context.
    pub display_mode: Option<i32>,
    /// The object's own highlight mode; `None` keeps the default.
    pub highlight_mode: Option<i32>,
    /// Transform persistence flags and anchor.
    pub persistence: Option<(Graphic3dTransModeFlags, PersistenceAnchor)>,
    /// Mark the object mutable before its first display.
    pub mutable: bool,
}

impl DisplayRequest {
    /// Zoom persistence anchored at `point`; `DisplayRequest::zoom_persistent([0., 0., 0.])`.
    pub fn zoom_persistent(point: [f64; 3]) -> Self {
        let anchor = PersistenceAnchor::Point(point);
        Self {
            persistence: Some((Graphic3dTransModeFlags::ZOOM_PERS, anchor)),
            ..Self::default()
        }
    }

    /// Zoom persistence with the default origin anchor; `DisplayRequest::zoom_persistent_at_origin()`.
    pub fn zoom_persistent_at_origin() -> Self {
        let anchor = PersistenceAnchor::Origin;
        Self {
            persistence: Some((Graphic3dTransModeFlags::ZOOM_PERS, anchor)),
            ..Self::default()
        }
    }

    /// The same request with an own display mode; `request.with_display_mode(1)`.
    pub fn with_display_mode(self, mode: i32) -> Self {
        Self {
            display_mode: Some(mode),
            ..self
        }
    }

    /// The same request displaying a mutable object; `request.mutable()`.
    pub fn mutable(self) -> Self {
        Self {
            mutable: true,
            ..self
        }
    }

    /// The same request with an own highlight mode; `request.with_highlight_mode(1)`.
    pub fn with_highlight_mode(self, mode: i32) -> Self {
        Self {
            highlight_mode: Some(mode),
            ..self
        }
    }

    fn transform_persistence(&self) -> Result<Option<Handle<Graphic3dTransformPers>>, Failure> {
        let Some((flags, anchor)) = self.persistence else {
            return Ok(None);
        };
        // The persistence is created at the origin and then re-anchored, as the viewer does.
        let persistence = Graphic3dTransformPers::with_point(flags, &Point3::new(0.0, 0.0, 0.0))?;
        if let PersistenceAnchor::Point([x, y, z]) = anchor {
            persistence.set_anchor_point(&Point3::new(x, y, z))?;
        }
        Ok(Some(Handle::new(persistence)))
    }
}

impl ViewerSession {
    /// Replace a named AIS shape's topology during an identity-bound control.
    /// Example: `session.replace_shape_for_control("b", &edge_only)?`.
    pub(crate) fn replace_shape_for_control(
        &self,
        name: &str,
        shape: &Shape,
    ) -> Result<(), Failure> {
        let object = self.named_objects(&[name])?.remove(0);
        let shape_source = object.cast::<dyn AisShapeSource>();
        let presentable = shape_source.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?}; expected a named AisShape for topology control"
            ))
        })?;
        self.with_update(|context| {
            context.clear_selected(false)?;
            presentable.set_shape(shape);
            context.redisplay(&object, false, false)
        })
    }

    /// Display `shape` under `name`, creating or updating its object; `session.display("b", &b, &request)?`.
    pub fn display(
        &mut self,
        name: &str,
        shape: &Shape,
        request: &DisplayRequest,
    ) -> Result<(), Failure> {
        let existing = self.object(name).cloned();
        let created = existing.is_none();
        let object = existing.unwrap_or_else(|| AisShape::new(shape).cast());
        if created {
            self.objects.push(name, object.clone());
        }
        let context = self.context.clone();
        self.with_update(|_| {
            apply_request(&context, &object, request)?;
            if created {
                return display_new(&context, &object, request);
            }
            display_existing(&context, &object, shape, request)
        })
    }

    /// Display every shape under its new name with `request`, then
    /// one viewer update; `session.display_many(&[("b0", &b0), ("b1", &b1)], &request)?`.
    pub fn display_many(
        &mut self,
        shapes: &[(&str, &Shape)],
        request: &DisplayRequest,
    ) -> Result<(), Failure> {
        let objects: Vec<Handle<dyn AisInteractiveObject>> = shapes
            .iter()
            .map(|(_, shape)| AisShape::new(shape).cast())
            .collect();
        for ((name, _), object) in shapes.iter().zip(&objects) {
            self.objects.push(name, object.clone());
        }
        let context = self.context.clone();
        self.with_update(|_| {
            for object in &objects {
                apply_request(&context, object, request)?;
                display_new(&context, object, request)?;
            }
            Ok(())
        })
    }

    /// The object displayed under `name`; `session.object("b")`.
    pub fn object(&self, name: &str) -> Option<&Handle<dyn AisInteractiveObject>> {
        self.objects.get(name)
    }
}

fn apply_request(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    request: &DisplayRequest,
) -> Result<(), Failure> {
    if let Some(persistence) = request.transform_persistence()? {
        context.set_transform_persistence(object, &persistence.cast())?;
    }
    Ok(())
}

pub(super) fn display_new(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    request: &DisplayRequest,
) -> Result<(), Failure> {
    let presentable = object.get().expect("a new object is never null");
    if let Some(mode) = request.display_mode {
        if !presentable.accept_display_mode(mode) {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!(
                    "display mode {mode} rejected by {}; expected an accepted mode",
                    presentable.dynamic_type().name()
                ),
            ));
        }
        presentable.set_display_mode(mode);
    }
    if let Some(mode) = request.highlight_mode {
        presentable.set_hilight_mode(mode);
    }
    if request.mutable {
        presentable.set_mutable(true);
    }
    let (mode, selection) = display_modes(context, presentable);
    context.display_with_modes(object, mode, selection, false, PrsMgrDisplayStatus::None)
}

fn display_existing(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    shape: &Shape,
    request: &DisplayRequest,
) -> Result<(), Failure> {
    let presentable = object.get().expect("a bound object is never null");
    if let Some(mode) = request.display_mode {
        presentable.set_display_mode(mode);
    }
    if let Some(mode) = request.highlight_mode {
        presentable.set_hilight_mode(mode);
    }
    if request.mutable {
        presentable.set_mutable(true);
    }
    let (mode, selection) = display_modes(context, presentable);
    if let Some(prs) = object.cast::<AisShape>().get() {
        let changed = !prs.shape().is_equal(shape);
        prs.set(shape);
        if changed {
            context.redisplay(object, false, false)?;
        }
    }
    if selection == -1 {
        context.erase(object, false)?;
    }
    context.display_with_modes(object, mode, selection, false, PrsMgrDisplayStatus::None)
}

fn display_modes(context: &AisInteractiveContext, object: &dyn AisInteractiveObject) -> (i32, i32) {
    let mode = if object.has_display_mode() {
        object.display_mode()
    } else if object.accept_display_mode(context.display_mode()) {
        context.display_mode()
    } else {
        0
    };
    let selection = if context.get_auto_activate_selection() {
        object.global_selection_mode()
    } else {
        -1
    };
    (mode, selection)
}
