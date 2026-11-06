//! Highlight styles: display mode, colour, transparency and material
//! of the context's dynamic or selection drawer, changed in that order.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectFillArea3d, Graphic3dAspectsMembers as _, Graphic3dMaterialAspect,
    Graphic3dNameOfMaterial, Graphic3dPresentationAttributesMembers as _,
};
use martensite_tkv3d::prs3d::{Prs3dTypeOfHighlight, prs3d_copy_aspects};

/// Changes of one highlight style; `None` leaves a property as it is.
/// Example: `HighlightStyle { display_mode: Some(1), ..HighlightStyle::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HighlightStyle {
    /// Display mode of the highlight presentation (-1: the object's own mode).
    pub display_mode: Option<i32>,
    /// Highlight colour.
    pub color: Option<Color>,
    /// Highlight transparency.
    pub transparency: Option<f32>,
    /// Material of a fill-area aspect copied from the default shading aspect.
    pub material: Option<Graphic3dNameOfMaterial>,
}

impl ViewerSession {
    /// Change the `kind` highlight style of the context; selected objects are highlighted
    /// again, as the selection-properties command does. Example:
    /// `session.set_highlight_style(Prs3dTypeOfHighlight::Dynamic, &style)?`.
    pub fn set_highlight_style(
        &self,
        kind: Prs3dTypeOfHighlight,
        change: &HighlightStyle,
    ) -> Result<(), Failure> {
        let style = (*self.context.highlight_style(kind)?).clone();
        let drawer = style.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "highlight style; expected a drawer",
            )
        })?;
        if let Some(mode) = change.display_mode {
            drawer.set_display_mode(mode);
        }
        if let Some(color) = change.color {
            drawer.set_color(color);
        }
        if let Some(transparency) = change.transparency {
            drawer.set_transparency(transparency);
        }
        if let Some(material) = change.material {
            drawer.set_basic_fill_area_aspect(self.highlight_fill(drawer, material)?);
        }
        if self.context.nb_selected() != 0 {
            self.context.hilight_selected(true)?;
        }
        Ok(())
    }

    /// A fill-area aspect copied from the default shading aspect with the named material in
    /// the style's colour and transparency.
    fn highlight_fill(
        &self,
        style: &martensite_tkv3d::prs3d::Prs3dDrawer,
        name: Graphic3dNameOfMaterial,
    ) -> Result<Handle<Graphic3dAspectFillArea3d>, Failure> {
        let fill = Handle::new(Graphic3dAspectFillArea3d::default());
        let defaults = self.context.default_drawer();
        let shading = defaults
            .get()
            .map(|d| d.shading_aspect())
            .unwrap_or_default();
        if let (Some(to), Some(from)) = (fill.get(), shading.get()) {
            prs3d_copy_aspects(to, &*from.aspect());
        }
        let mut material = Graphic3dMaterialAspect::new(name);
        material.set_color(style.color())?;
        material.set_transparency(style.transparency())?;
        if let Some(aspect) = fill.get() {
            aspect.set_front_material(material);
            aspect.set_interior_color(style.color());
        }
        Ok(fill)
    }
}
