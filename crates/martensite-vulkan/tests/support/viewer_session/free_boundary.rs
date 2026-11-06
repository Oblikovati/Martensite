//! Free boundaries, on or off, with colour and width:
//! the visibility flag and a fresh line aspect copied from the
//! current one, applied to an object (then recomputed) or to the context defaults.
use super::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::aspect::AspectTypeOfLine;
use martensite_tkv3d::prs3d::{Prs3dDrawer, Prs3dLineAspect, prs3d_copy_aspects};

/// The free boundary changes; `None` leaves a value unchanged.
/// Example: `FreeBoundary { color: Some(Color::named(NamedColor::BLUE)), ..FreeBoundary::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct FreeBoundary {
    /// Whether free boundaries are drawn.
    pub draw: Option<bool>,
    /// Free boundary line width.
    pub width: Option<f64>,
    /// Free boundary colour.
    pub color: Option<Color>,
}

impl ViewerSession {
    /// Apply `change` to the object `name`, recompute it and update the viewer.
    /// Example: `session.set_free_boundary("c_1", &change)?`.
    pub fn set_free_boundary(&self, name: &str, change: &FreeBoundary) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            if let Some(drawer) = object.get().map(|o| o.attributes()) {
                apply_free_boundary(&drawer, change)?;
            }
            context.redisplay(&object, false, false)
        })
    }

    /// Apply `change` to the context's default drawer, then
    /// recompute every named object; `session.set_default_free_boundary(&change)?`.
    pub fn set_default_free_boundary(&self, change: &FreeBoundary) -> Result<(), Failure> {
        apply_free_boundary(&self.context.default_drawer(), change)?;
        let objects: Vec<_> = self
            .objects
            .iter()
            .map(|(_, object)| object.clone())
            .collect();
        self.with_update(|context| {
            for object in &objects {
                context.redisplay(object, false, false)?;
            }
            Ok(())
        })
    }
}

/// The aspects command's order: visibility, width (own aspect created once), colour (always a
/// new aspect copied from the current one).
fn apply_free_boundary(drawer: &Handle<Prs3dDrawer>, change: &FreeBoundary) -> Result<(), Failure> {
    let Some(value) = drawer.get() else {
        return Ok(());
    };
    if let Some(draw) = change.draw {
        value.set_free_boundary_draw(draw);
    }
    if let Some(width) = change.width {
        if !value.has_own_free_boundary_aspect() {
            value.set_free_boundary_aspect(copied_free_boundary(value)?);
        }
        if let Some(aspect) = value.free_boundary_aspect().get() {
            aspect.set_width(width)?;
        }
    }
    if let Some(color) = change.color {
        let own = copied_free_boundary(value)?;
        if let Some(aspect) = own.get() {
            aspect.set_color(color);
        }
        value.set_free_boundary_aspect(own);
    }
    Ok(())
}

/// A new red solid line aspect whose line style is copied from the current free boundary one.
fn copied_free_boundary(drawer: &Prs3dDrawer) -> Result<Handle<Prs3dLineAspect>, Failure> {
    let red = Color::named(NamedColor::RED);
    let own = Handle::new(Prs3dLineAspect::new(red, AspectTypeOfLine::Solid, 1.0)?);
    if let (Some(to), Some(from)) = (own.get(), drawer.free_boundary_aspect().get()) {
        prs3d_copy_aspects(&*to.aspect(), &*from.aspect());
    }
    Ok(own)
}
