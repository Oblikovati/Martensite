//! The conversions of `StepConstructRenderingProperties` to and from STEP and XCAF.
use super::*;

impl StepConstructRenderingProperties {
    /// `init(XCAFDocVisMaterialCommon)`: use diffuse color as the base surface color. An undefined
    /// material only resets the color, the transparency and the flag.
    pub fn init_common_material(&mut self, material: &XCAFDocVisMaterialCommon) {
        if !material.is_defined {
            self.my_surface_color = Color::named(NamedColor::WHITE);
            self.my_transparency = 0.0;
            self.my_is_defined = false;
            return;
        }
        *self = Self::default();
        self.my_surface_color = material.diffuse_color;
        self.my_transparency = f64::from(material.transparency);
        self.my_is_defined = true;
        let diffuse = material.diffuse_color;
        // Find maximum diffuse component to avoid division by zero for dark colors
        let diffuse_is_lit =
            diffuse.red().max(diffuse.green().max(diffuse.blue())) > precision::confusion();
        // Check if ambient color is non-default and diffuse color has non-zero components
        if diffuse_is_lit
            && let Some(factor) = close_ratio_average(material.ambient_color, diffuse)
            // Check if factor is significantly different from default (0.1)
            && (factor - 0.1).abs() > 0.01
        {
            self.my_ambient_reflectance.set(factor.clamp(0.0, 1.0));
        }
        // Diffuse reflectance is always 1.0 for the main color
        self.my_diffuse_reflectance.set(1.0);
        if diffuse_is_lit {
            self.read_specular(material);
        }
        // Convert shininess to specular exponent using fixed scale factor
        if material.shininess >= 0.0 && (material.shininess - 1.0).abs() > 0.01 {
            self.my_specular_exponent
                .set(f64::from(material.shininess) * SHININESS_SCALE);
        }
    }
    /// Specular reflectance factor relative to the diffuse color, or the actual specular color when
    /// the ratios differ significantly (with an average for formats that don't support color).
    fn read_specular(&mut self, material: &XCAFDocVisMaterialCommon) {
        let specular = material.specular_color;
        match close_ratio_average(specular, material.diffuse_color) {
            Some(factor) => {
                // Check if factor is significantly different from default (0.2)
                if (factor - 0.2).abs() > 0.01 {
                    self.my_specular_reflectance.set(factor.clamp(0.0, 1.0));
                }
            }
            None => {
                self.my_specular_colour.set(specular);
                self.my_specular_reflectance
                    .set((specular.red() + specular.green() + specular.blue()) / 3.0);
            }
        }
    }
    /// `init(XCAFDocVisMaterial)`: nothing for a null material.
    pub fn init_material(&mut self, material: &Handle<XCAFDocVisMaterial>) -> Result<(), Failure> {
        if let Some(live) = material.get() {
            self.init_common_material(&live.convert_to_common_material()?);
        }
        Ok(())
    }
    /// `init(surface_color, transparency = 0)`.
    pub fn init_color(&mut self, surface_color: &Color, transparency: f64) {
        *self = Self::default();
        self.my_surface_color = *surface_color;
        self.my_transparency = transparency;
        self.my_is_defined = true;
    }
    /// `set_ambient_reflectance`.
    pub fn set_ambient_reflectance(&mut self, ambient: f64) {
        self.my_ambient_reflectance.set(ambient);
    }
    /// `set_ambient_and_diffuse_reflectance`.
    pub fn set_ambient_and_diffuse_reflectance(&mut self, ambient: f64, diffuse: f64) {
        self.my_ambient_reflectance.set(ambient);
        self.my_diffuse_reflectance.set(diffuse);
    }
    /// `set_ambient_diffuse_and_specular_reflectance`.
    pub fn set_ambient_diffuse_and_specular_reflectance(
        &mut self,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        specular_exponent: f64,
        specular_colour: &Color,
    ) {
        self.set_ambient_and_diffuse_reflectance(ambient, diffuse);
        self.my_specular_reflectance.set(specular);
        self.my_specular_exponent.set(specular_exponent);
        self.my_specular_colour.set(*specular_colour);
    }
    /// The reflectance item of the properties defined here, null without a usable model.
    fn reflectance_item(&self) -> Result<Handle<StandardTransient>, Failure> {
        let ambient = self.my_ambient_reflectance;
        let diffuse = self.my_diffuse_reflectance;
        let specular = self.my_specular_reflectance;
        if ambient.defined && !diffuse.defined && !specular.defined {
            // Add ambient reflectance only
            let item = Handle::new(StepVisualSurfaceStyleReflectanceAmbient::new());
            item.init(ambient.value);
            return Ok(item.cast());
        }
        if ambient.defined && diffuse.defined && !specular.defined {
            // Add ambient and diffuse reflectance
            let item = Handle::new(StepVisualSurfaceStyleReflectanceAmbientDiffuse::new());
            item.init(ambient.value, diffuse.value);
            return Ok(item.cast());
        }
        if self.my_specular_colour.defined
            && self.my_specular_exponent.defined
            && specular.defined
            && diffuse.defined
            && ambient.defined
        {
            // Add full PBR reflectance model (ambient, diffuse, specular)
            let item = Handle::new(StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular::new());
            let colour = StepConstructStyles::encode_color(&self.my_specular_colour.value)?;
            item.init(
                ambient.value,
                diffuse.value,
                specular.value,
                self.my_specular_exponent.value,
                &colour,
            );
            return Ok(item.cast());
        }
        Ok(Handle::default())
    }
    /// `create_rendering_properties()`: the entity with the surface color.
    pub fn create_surface_rendering_properties(
        &self,
    ) -> Result<Handle<StepVisualSurfaceStyleRenderingWithProperties>, Failure> {
        self.create_rendering_properties(&StepConstructStyles::encode_color(
            &self.my_surface_color,
        )?)
    }
    /// `create_rendering_properties(render_colour)`: the entity with the given color, null when not
    /// defined; a null colour takes the surface color.
    pub fn create_rendering_properties(
        &self,
        render_colour: &Handle<StepVisualColour>,
    ) -> Result<Handle<StepVisualSurfaceStyleRenderingWithProperties>, Failure> {
        if !self.my_is_defined {
            return Ok(Handle::default());
        }
        let colour = if render_colour.is_some() {
            render_colour.clone()
        } else {
            StepConstructStyles::encode_color(&self.my_surface_color)?
        };
        // Transparency is always included, and the reflectance property if we have any defined
        let reflectance = self.reflectance_item()?;
        let properties: Handle<HArray1<StepVisualRenderingPropertiesSelect>> =
            step_new_array1(1, 1 + i32::from(reflectance.is_some()))?;
        let transparent = Handle::new(StepVisualSurfaceStyleTransparent::new());
        transparent.init(self.my_transparency);
        let mut select = StepVisualRenderingPropertiesSelect::default();
        select.set_value(&transparent.cast())?;
        step_array1_set(&properties, 1, select, WHERE)?;
        if reflectance.is_some() {
            let mut select = StepVisualRenderingPropertiesSelect::default();
            select.set_value(&reflectance)?;
            step_array1_set(&properties, 2, select, WHERE)?;
        }
        // Create STEP surface style rendering with properties
        let entity = Handle::new(StepVisualSurfaceStyleRenderingWithProperties::new());
        entity.init(self.my_rendering_method, &colour, &properties);
        Ok(entity)
    }
    /// `create_xcaf_material`: the material of these properties, not defined when they are not.
    pub fn create_xcaf_material(&self) -> Result<XCAFDocVisMaterialCommon, Failure> {
        let mut material = XCAFDocVisMaterialCommon::default();
        if !self.my_is_defined {
            material.is_defined = false;
            return Ok(material);
        }
        // Use surface color as the base diffuse color
        material.diffuse_color = self.my_surface_color;
        material.transparency = self.my_transparency as f32;
        material.is_defined = true;
        // Set default values for other properties
        material.ambient_color = Color::new(0.1, 0.1, 0.1, ColorSpace::Rgb)?;
        material.specular_color = Color::new(0.2, 0.2, 0.2, ColorSpace::Rgb)?;
        material.emissive_color = Color::new(0.0, 0.0, 0.0, ColorSpace::Rgb)?;
        material.shininess = 1.0;
        // Handle ambient reflectance - apply factor to diffuse color
        if self.my_ambient_reflectance.defined {
            material.ambient_color =
                scaled_color(self.my_surface_color, self.my_ambient_reflectance.value)?;
        }
        // Handle specular properties - apply factor to diffuse color or use explicit color
        if self.my_specular_colour.defined {
            material.specular_color = self.my_specular_colour.value;
        } else if self.my_specular_reflectance.defined {
            material.specular_color =
                scaled_color(self.my_surface_color, self.my_specular_reflectance.value)?;
        }
        // Handle shininess (specular exponent)
        if self.my_specular_exponent.defined {
            material.shininess =
                (self.my_specular_exponent.value / SHININESS_SCALE).min(1.0) as f32;
        }
        Ok(material)
    }
    /// `get_rgba_color`.
    pub fn get_rgba_color(&self) -> ColorRgba {
        ColorRgba::from_rgb(self.my_surface_color, (1.0 - self.my_transparency) as f32)
    }
    /// `surface_color`.
    pub fn surface_color(&self) -> Color {
        self.my_surface_color
    }
    /// `transparency`: 0 is opaque, 1 fully transparent.
    pub fn transparency(&self) -> f64 {
        self.my_transparency
    }
    /// `rendering_method`.
    pub fn rendering_method(&self) -> StepVisualShadingSurfaceMethod {
        self.my_rendering_method
    }
    /// `set_rendering_method`.
    pub fn set_rendering_method(&mut self, method: StepVisualShadingSurfaceMethod) {
        self.my_rendering_method = method;
    }
    /// `is_defined`.
    pub fn is_defined(&self) -> bool {
        self.my_is_defined
    }
    /// `is_material_convertible`: fully defined for conversion to STEP.
    pub fn is_material_convertible(&self) -> bool {
        self.my_is_defined
            && self.my_ambient_reflectance.defined
            && self.my_diffuse_reflectance.defined
            && self.my_specular_reflectance.defined
            && self.my_specular_exponent.defined
            && self.my_specular_colour.defined
    }
    /// `ambient_reflectance`.
    pub fn ambient_reflectance(&self) -> f64 {
        self.my_ambient_reflectance.value
    }
    /// `is_ambient_reflectance_defined`.
    pub fn is_ambient_reflectance_defined(&self) -> bool {
        self.my_ambient_reflectance.defined
    }
    /// `diffuse_reflectance`.
    pub fn diffuse_reflectance(&self) -> f64 {
        self.my_diffuse_reflectance.value
    }
    /// `is_diffuse_reflectance_defined`.
    pub fn is_diffuse_reflectance_defined(&self) -> bool {
        self.my_diffuse_reflectance.defined
    }
    /// `specular_reflectance`.
    pub fn specular_reflectance(&self) -> f64 {
        self.my_specular_reflectance.value
    }
    /// `is_specular_reflectance_defined`.
    pub fn is_specular_reflectance_defined(&self) -> bool {
        self.my_specular_reflectance.defined
    }
    /// `specular_exponent`.
    pub fn specular_exponent(&self) -> f64 {
        self.my_specular_exponent.value
    }
    /// `is_specular_exponent_defined`.
    pub fn is_specular_exponent_defined(&self) -> bool {
        self.my_specular_exponent.defined
    }
    /// `specular_colour`.
    pub fn specular_colour(&self) -> Color {
        self.my_specular_colour.value
    }
    /// `is_specular_colour_defined`.
    pub fn is_specular_colour_defined(&self) -> bool {
        self.my_specular_colour.defined
    }
}
