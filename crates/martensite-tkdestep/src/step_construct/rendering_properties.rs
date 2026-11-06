//! `StepConstructRenderingProperties`: the rendering properties specifying visual appearance in STEP:
//! parses the STEP entities and creates new ones.
use super::StepConstructStyles;
use crate::step_data::StepDataSelectType;
use crate::step_entity::{HArray1, step_array1_item, step_array1_set, step_new_array1};
use crate::step_visual::{
    StepVisualColour, StepVisualRenderingPropertiesSelect, StepVisualShadingSurfaceMethod,
    StepVisualSurfaceStyleReflectanceAmbient, StepVisualSurfaceStyleReflectanceAmbientDiffuse,
    StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular,
    StepVisualSurfaceStyleRenderingWithProperties, StepVisualSurfaceStyleTransparent,
};
use martensite_tkernel::precision;
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxcaf::xcaf_doc::{XCAFDocVisMaterial, XCAFDocVisMaterialCommon};

mod conversion;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use conversion::*;

const WHERE: &str = "StepConstructRenderingProperties";
/// Maximal allowed deviation between the RGB ratios of a colour and the diffuse colour.
const MAX_RATIO_DEVIATION: f64 = 0.2;
/// The scale from shininess to the STEP specular exponent.
const SHININESS_SCALE: f64 = 128.0;

/// A value that may be defined, keeping its value when not.
#[derive(Clone, Copy)]
struct Defined<T> {
    value: T,
    defined: bool,
}

impl<T> Defined<T> {
    const fn unset(value: T) -> Self {
        Self {
            value,
            defined: false,
        }
    }
    fn set(&mut self, value: T) {
        self.value = value;
        self.defined = true;
    }
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructRenderingProperties;
/// use martensite_tkernel::quantity::{Color, NamedColor};
/// let mut props = StepConstructRenderingProperties::new();
/// assert!(!props.is_defined() && props.create_rendering_properties(&Default::default()).unwrap().is_null());
/// props.init_color(&Color::named(NamedColor::RED), 0.25);
/// assert!(props.is_defined() && props.transparency() == 0.25);
/// props.set_ambient_reflectance(0.5);
/// assert!(props.is_ambient_reflectance_defined() && !props.is_material_convertible());
/// ```
#[derive(Clone, Copy)]
pub struct StepConstructRenderingProperties {
    my_surface_color: Color,
    my_transparency: f64,
    my_rendering_method: StepVisualShadingSurfaceMethod,
    my_is_defined: bool,
    my_ambient_reflectance: Defined<f64>,
    my_diffuse_reflectance: Defined<f64>,
    my_specular_reflectance: Defined<f64>,
    my_specular_exponent: Defined<f64>,
    my_specular_colour: Defined<Color>,
}

impl Default for StepConstructRenderingProperties {
    fn default() -> Self {
        Self {
            my_surface_color: Color::named(NamedColor::WHITE),
            my_transparency: 0.0,
            my_rendering_method: StepVisualShadingSurfaceMethod::SsmNormalShading,
            my_is_defined: false,
            my_ambient_reflectance: Defined::unset(0.0),
            my_diffuse_reflectance: Defined::unset(0.0),
            my_specular_reflectance: Defined::unset(0.0),
            my_specular_exponent: Defined::unset(0.0),
            my_specular_colour: Defined::unset(Color::named(NamedColor::WHITE)),
        }
    }
}

/// The average ratio of the channels of `color` to those of the diffuse colour, when these ratios are
/// reasonably close; otherwise `color` isn't a simple multiplier of the diffuse colour.
fn close_ratio_average(color: Color, diffuse: Color) -> Option<f64> {
    let ratio = |channel: f64, base: f64| {
        if base > precision::confusion() {
            channel / base
        } else {
            0.0
        }
    };
    let red = ratio(color.red(), diffuse.red());
    let green = ratio(color.green(), diffuse.green());
    let blue = ratio(color.blue(), diffuse.blue());
    let min = red.min(green.min(blue));
    let max = red.max(green.max(blue));
    ((max - min) < MAX_RATIO_DEVIATION).then_some((red + green + blue) / 3.0)
}

/// `color` multiplied by a factor clamped to [0, 1].
fn scaled_color(color: Color, factor: f64) -> Result<Color, Failure> {
    let clamped = factor.clamp(0.0, 1.0);
    Color::new(
        color.red() * clamped,
        color.green() * clamped,
        color.blue() * clamped,
        ColorSpace::Rgb,
    )
}

impl StepConstructRenderingProperties {
    /// `StepConstructRenderingProperties()`: empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructRenderingProperties(theRenderingProperties)`.
    pub fn from_entity(
        properties: &Handle<StepVisualSurfaceStyleRenderingWithProperties>,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        made.init_entity(properties)?;
        Ok(made)
    }
    /// `StepConstructRenderingProperties(theRGBAColor)`.
    pub fn from_rgba(color: &ColorRgba) -> Self {
        let mut made = Self::default();
        made.init_rgba(color);
        made
    }
    /// `StepConstructRenderingProperties(theColor, theTransparency)`.
    pub fn from_step_color(
        color: &Handle<StepVisualColour>,
        transparency: f64,
    ) -> Result<Self, Failure> {
        let mut made = Self::default();
        made.init_step_color(color, transparency)?;
        Ok(made)
    }
    /// `StepConstructRenderingProperties(XCAFDocVisMaterialCommon)`.
    pub fn from_common_material(material: &XCAFDocVisMaterialCommon) -> Self {
        let mut made = Self::default();
        made.init_common_material(material);
        made
    }
    /// `StepConstructRenderingProperties(XCAFDocVisMaterial)`.
    pub fn from_material(material: &Handle<XCAFDocVisMaterial>) -> Result<Self, Failure> {
        let mut made = Self::default();
        made.init_material(material)?;
        Ok(made)
    }
    /// `StepConstructRenderingProperties(theSurfaceColor, theTransparency = 0)`.
    pub fn from_color(surface_color: &Color, transparency: f64) -> Self {
        let mut made = Self::default();
        made.init_color(surface_color, transparency);
        made
    }
    /// One property of `init(entity)`: transparency or a reflectance model.
    fn read_property(
        &mut self,
        select: &StepVisualRenderingPropertiesSelect,
    ) -> Result<(), Failure> {
        // Check for transparency
        if let Some(transparent) = select.surface_style_transparent().get() {
            self.my_transparency = transparent.transparency();
            self.my_is_defined = true;
        }
        // Check for ambient reflectance
        let ambient = select.surface_style_reflectance_ambient();
        if let Some(full) = ambient
            .cast::<StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular>()
            .get()
        {
            self.my_is_defined = true;
            self.my_ambient_reflectance.set(full.ambient_reflectance());
            self.my_diffuse_reflectance.set(full.diffuse_reflectance());
            self.my_specular_reflectance
                .set(full.specular_reflectance());
            self.my_specular_exponent.set(full.specular_exponent());
            self.my_specular_colour.defined = StepConstructStyles::decode_color(
                &full.specular_colour(),
                &mut self.my_specular_colour.value,
            )?;
        } else if let Some(both) = ambient
            .cast::<StepVisualSurfaceStyleReflectanceAmbientDiffuse>()
            .get()
        {
            self.my_ambient_reflectance.set(both.ambient_reflectance());
            self.my_diffuse_reflectance.set(both.diffuse_reflectance());
            self.my_is_defined = true;
        } else if let Some(only) = ambient.get() {
            self.my_ambient_reflectance.set(only.ambient_reflectance());
            self.my_is_defined = true;
        }
        Ok(())
    }
    /// `init(rendering_properties)`: color, transparency and the other properties of the entity.
    pub fn init_entity(
        &mut self,
        properties: &Handle<StepVisualSurfaceStyleRenderingWithProperties>,
    ) -> Result<(), Failure> {
        *self = Self::default();
        let Some(entity) = properties.get() else {
            return Ok(());
        };
        self.my_rendering_method = entity.rendering_method();
        // Decode surface color using StepConstructStyles utility
        let color = entity.surface_colour();
        if color.is_some() {
            let mut decoded = Color::default();
            if StepConstructStyles::decode_color(&color, &mut decoded)? {
                self.my_surface_color = decoded;
                self.my_is_defined = true;
            }
        }
        // Process rendering properties
        let selects = entity.properties();
        if let Some(live) = selects.get() {
            for i in 1..=live.array1().length() {
                self.read_property(&step_array1_item(&selects, i, WHERE)?)?;
            }
        }
        Ok(())
    }
    /// `init(rgba_color)`.
    pub fn init_rgba(&mut self, color: &ColorRgba) {
        *self = Self::default();
        self.my_surface_color = *color.rgb();
        self.my_transparency = 1.0 - f64::from(color.alpha());
        self.my_is_defined = true;
    }
    /// `init(color, transparency)`.
    pub fn init_step_color(
        &mut self,
        color: &Handle<StepVisualColour>,
        transparency: f64,
    ) -> Result<(), Failure> {
        *self = Self::default();
        self.my_transparency = transparency;
        if color.is_null() {
            return Ok(());
        }
        // Decode color using StepConstructStyles utility
        let mut decoded = Color::default();
        if StepConstructStyles::decode_color(color, &mut decoded)? {
            self.my_surface_color = decoded;
            self.my_is_defined = true;
        }
        Ok(())
    }
}
