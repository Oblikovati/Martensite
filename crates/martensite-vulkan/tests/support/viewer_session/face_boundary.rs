//! Face boundaries of a shaded shape: drawn with an
//! own line aspect copied from the context defaults, then the object is recomputed.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkv3d::prs3d::Prs3dDrawer;

/// The face boundary style; `None` leaves the own aspect's colour, width or type unchanged.
/// Example: `FaceBoundary { color: Some(Color::named(NamedColor::RED)), ..FaceBoundary::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct FaceBoundary {
    /// Boundary colour.
    pub color: Option<Color>,
    /// Boundary line width.
    pub width: Option<f64>,
    /// Boundary line type.
    pub line_type: Option<AspectTypeOfLine>,
}

impl ViewerSession {
    /// Draw the face boundaries of object `name` in `style`, recompute it and update the viewer,
    /// as the aspects command does; `session.draw_face_boundaries("b", &style)?`.
    pub fn draw_face_boundaries(&self, name: &str, style: &FaceBoundary) -> Result<(), Failure> {
        let object = self.named(name)?;
        let defaults = self.context.default_drawer();
        self.with_update(|context| {
            if let Some(drawer) = object.get().map(|o| o.attributes()) {
                drawer.set_face_boundary_draw(true);
                apply_face_boundary(&drawer, &defaults, style)?;
            }
            context.redisplay(&object, false, false)
        })
    }

    /// The face boundaries of object `name` keep
    /// only edges up to continuity `most`, then the object is recomputed and the viewer
    /// updated; `session.set_face_boundary_continuity("s", Shape::C2)?`.
    pub fn set_face_boundary_continuity(&self, name: &str, most: Shape) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            if let Some(drawer) = object.get().map(|o| o.attributes()) {
                drawer.set_face_boundary_upper_continuity(most);
            }
            context.redisplay(&object, false, false)
        })
    }
}

/// Copy an own boundary aspect from the defaults when needed, then set the given values in
/// the order the aspects command applies them (colour, width, type).
fn apply_face_boundary(
    drawer: &Handle<Prs3dDrawer>,
    defaults: &Handle<Prs3dDrawer>,
    style: &FaceBoundary,
) -> Result<(), Failure> {
    if style.color.is_none() && style.width.is_none() && style.line_type.is_none() {
        return Ok(());
    }
    drawer.setup_own_face_boundary_aspect(defaults)?;
    let aspect = drawer.face_boundary_aspect();
    let Some(aspect) = aspect.get() else {
        return Ok(());
    };
    if let Some(color) = style.color {
        aspect.set_color(color);
    }
    if let Some(width) = style.width {
        aspect.set_width(width)?;
    }
    if let Some(kind) = style.line_type {
        aspect.set_type_of_line(kind);
    }
    Ok(())
}
