//! Per-sub-shape colour and width:
//! a plain shape object is first replaced under its name by a coloured-shape copy, which is
//! displayed, then recomputed with the custom aspects.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkv3d::ais::{
    AisColoredShape, AisColoredShapeCustomAppearance as _, AisShapeSource,
};

/// The custom aspects of one sub-shape; `None` leaves a value unset.
/// Example: `SubShapeStyle { color: Some(Color::named(NamedColor::RED)), width: None }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SubShapeStyle {
    /// Custom colour of the sub-shape.
    pub color: Option<Color>,
    /// Custom line width of the sub-shape.
    pub width: Option<f64>,
}

impl ViewerSession {
    /// Give `sub_shape` of the shape object `name` the custom `style`, then update the viewer.
    /// Example: `session.style_sub_shape("b", &face, &style)?`.
    pub fn style_sub_shape(
        &mut self,
        name: &str,
        sub_shape: &Shape,
        style: &SubShapeStyle,
    ) -> Result<(), Failure> {
        let (colored, converted) = self.colored_object(name)?;
        let object: Handle<dyn AisInteractiveObject> = colored.clone().cast();
        self.with_update(|context| {
            let value = colored.get().expect("a coloured shape is never null");
            if let Some(color) = style.color {
                value.set_custom_color(sub_shape, color)?;
            }
            if let Some(width) = style.width {
                value.set_custom_width(sub_shape, width)?;
            }
            if converted {
                context.display(&object, false)?;
            }
            context.redisplay(&object, false, false)
        })
    }

    /// Hide `sub_shape` of the shape object
    /// `name`, then update the viewer; `session.hide_sub_shape("b", &face)?`.
    pub fn hide_sub_shape(&mut self, name: &str, sub_shape: &Shape) -> Result<(), Failure> {
        let (colored, converted) = self.colored_object(name)?;
        let object: Handle<dyn AisInteractiveObject> = colored.clone().cast();
        self.with_update(|context| {
            let value = colored.get().expect("a coloured shape is never null");
            let aspects = value.custom_aspects(sub_shape)?;
            if let Some(aspects) = aspects.get() {
                aspects.set_hidden(true);
            }
            if converted {
                context.display(&object, false)?;
            }
            context.redisplay(&object, false, false)
        })
    }

    /// The coloured shape bound to `name`, converting and rebinding a plain shape object;
    /// true when it was converted (and removed from the context).
    fn colored_object(&mut self, name: &str) -> Result<(Handle<AisColoredShape>, bool), Failure> {
        let object = self.named(name)?;
        let colored = object.cast::<AisColoredShape>();
        if !colored.is_null() {
            return Ok((colored, false));
        }
        let shape = object.cast::<dyn AisShapeSource>();
        let source = shape.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} is not an AisShape presentation; expected a shape object"
            ))
        })?;
        let colored = AisColoredShape::from_shape(&shape)?;
        let value = colored.get().expect("a new coloured shape is never null");
        if source.has_display_mode() {
            value.set_display_mode(source.display_mode());
        }
        value.set_local_transformation(&source.local_transformation())?;
        self.context.remove(&object, false)?;
        self.objects.remove(name);
        self.objects.push(name, colored.clone().cast());
        Ok((colored, true))
    }
}
