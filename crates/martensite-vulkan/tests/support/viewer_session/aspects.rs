//! Per-object presentation aspects: line width and the surface's interior and edge styles.
//!
//! Surface changes go to an own shading aspect of the object (copied from the context
//! defaults first); the object is recomputed when that aspect is new, and only its aspects are
//! synchronized otherwise, followed by one viewer update.
use super::*;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::aspect::AspectTypeOfFacingModel;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsEdgeMembers as _, Graphic3dAspectsLineMembers as _,
};
use martensite_tkv3d::prs3d::Prs3dDrawer;

/// Surface style changes; `None` leaves a property as it is.
/// Example: `SurfaceAspects { draw_edges: Some(true), ..SurfaceAspects::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SurfaceAspects {
    /// Interior fill style.
    pub interior_style: Option<AspectInteriorStyle>,
    /// Whether face edges are drawn.
    pub draw_edges: Option<bool>,
    /// Edge line type; also turns edge drawing on unless the type is empty and the interior
    /// style is not set in the same change.
    pub edge_type: Option<AspectTypeOfLine>,
    /// Edge colour.
    pub edge_color: Option<ColorRgba>,
    /// Which faces are culled (the shading aspect's back-facing model).
    pub face_culling: Option<Graphic3dTypeOfBackfacingModel>,
    /// The colour of back faces.
    pub back_face_color: Option<martensite_tkernel::quantity::Color>,
}

impl ViewerSession {
    /// Set the line width of the object named `name`; `session.set_line_width("b1", 4.)?`.
    pub fn set_line_width(&self, name: &str, width: f64) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            context.set_width(&object, width, false)?;
            synchronize(&object);
            Ok(())
        })
    }

    /// Apply surface style changes to the object named `name`; `session.set_surface_aspects("b", &aspects)?`.
    pub fn set_surface_aspects(&self, name: &str, aspects: &SurfaceAspects) -> Result<(), Failure> {
        let object = self.named(name)?;
        let defaults = self.context.default_drawer();
        self.with_update(|context| {
            let drawer = object.get().map(|o| o.attributes()).unwrap_or_default();
            let recompute = match drawer.get() {
                Some(drawer) => apply_surface(drawer, &defaults, aspects)?,
                None => false,
            };
            if recompute {
                return context.redisplay(&object, false, false);
            }
            synchronize(&object);
            Ok(())
        })
    }

    /// Isolines of object `name` built on its
    /// triangulation, then the object is recomputed; `session.set_iso_on_triangulation("c", true)?`.
    pub fn set_iso_on_triangulation(&self, name: &str, enabled: bool) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            context.set_object_iso_on_triangulation(enabled, &object);
            let drawer = object.get().map(|o| o.attributes()).unwrap_or_default();
            if let Some(drawer) = drawer.get() {
                drawer.set_iso_on_triangulation(enabled);
            }
            context.redisplay(&object, false, false)
        })
    }

    pub(super) fn named(&self, name: &str) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
        self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?}; expected a displayed object name"
            ))
        })
    }
}

pub(super) fn synchronize(object: &Handle<dyn AisInteractiveObject>) {
    if let Some(object) = object.get() {
        object.synchronize_aspects();
    }
}

fn apply_surface(
    drawer: &Prs3dDrawer,
    defaults: &Handle<Prs3dDrawer>,
    aspects: &SurfaceAspects,
) -> Result<bool, Failure> {
    let recompute = drawer.setup_own_shading_aspect(defaults)?;
    if let (Some(color), Some(shading)) = (aspects.back_face_color, drawer.shading_aspect().get()) {
        shading.set_color(color, AspectTypeOfFacingModel::BackSide)?;
    }
    let Some(fill) = drawer.shading_aspect().get().map(|s| s.aspect()) else {
        return Ok(recompute);
    };
    let Some(fill) = fill.get() else {
        return Ok(recompute);
    };
    if let Some(style) = aspects.interior_style {
        fill.set_interior_style(style);
    }
    if let Some(draw) = aspects.draw_edges {
        fill.set_draw_edges(draw);
    }
    if let Some(line_type) = aspects.edge_type {
        fill.set_line_type(line_type);
        if aspects.interior_style.is_none() {
            fill.set_draw_edges(line_type != AspectTypeOfLine::Empty);
        }
    }
    if let Some(color) = aspects.edge_color {
        fill.set_edge_color_rgba(color);
    }
    if let Some(culling) = aspects.face_culling {
        fill.set_face_culling(culling);
    }
    Ok(recompute)
}
