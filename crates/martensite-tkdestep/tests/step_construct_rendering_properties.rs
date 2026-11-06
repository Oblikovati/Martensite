//! The behaviour of `StepConstructRenderingProperties`: entities, materials and colours, in both
//! directions, with the tolerances of the original checks.
use martensite_tkdestep::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use martensite_tkdestep::step_data::StepDataSelectType;
use martensite_tkdestep::step_visual::{
    StepVisualRenderingPropertiesSelect, StepVisualShadingSurfaceMethod,
    StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular,
    StepVisualSurfaceStyleRenderingWithProperties, StepVisualSurfaceStyleTransparent,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkxcaf::xcaf_doc::XCAFDocVisMaterialCommon;

const TRANSPARENCY: f64 = 0.25;
const AMBIENT_FACTOR: f64 = 0.3;
const DIFFUSE_FACTOR: f64 = 1.0;
const SPECULAR_FACTOR: f64 = 0.8;
const SPECULAR_EXPONENT: f64 = 45.0;

fn surface_color() -> Color {
    Color::new(0.8, 0.5, 0.2, ColorSpace::Rgb).unwrap()
}

fn specular_color() -> Color {
    Color::new(0.9, 0.9, 0.9, ColorSpace::Rgb).unwrap()
}

fn step_rendering_properties() -> Handle<StepVisualSurfaceStyleRenderingWithProperties> {
    let surface = StepConstructStyles::encode_color(&surface_color()).unwrap();
    let transparent = Handle::new(StepVisualSurfaceStyleTransparent::new());
    transparent.init(TRANSPARENCY);
    let specular = StepConstructStyles::encode_color(&specular_color()).unwrap();
    let reflectance = Handle::new(StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular::new());
    reflectance.init(
        AMBIENT_FACTOR,
        DIFFUSE_FACTOR,
        SPECULAR_FACTOR,
        SPECULAR_EXPONENT,
        &specular,
    );
    let mut first = StepVisualRenderingPropertiesSelect::default();
    first.set_value(&transparent.cast()).unwrap();
    let mut second = StepVisualRenderingPropertiesSelect::default();
    second.set_value(&reflectance.cast()).unwrap();
    let properties = Handle::new(HArray1::with_bounds(1, 2).unwrap());
    {
        let mut items = properties.get().unwrap().change_array1();
        items.set_value(1, first).unwrap();
        items.set_value(2, second).unwrap();
    }
    let result = Handle::new(StepVisualSurfaceStyleRenderingWithProperties::new());
    result.init(
        StepVisualShadingSurfaceMethod::SsmNormalShading,
        &surface,
        &properties,
    );
    result
}

fn material() -> XCAFDocVisMaterialCommon {
    let surface = surface_color();
    XCAFDocVisMaterialCommon {
        diffuse_color: surface,
        transparency: TRANSPARENCY as f32,
        ambient_color: Color::new(
            surface.red() * AMBIENT_FACTOR,
            surface.green() * AMBIENT_FACTOR,
            surface.blue() * AMBIENT_FACTOR,
            ColorSpace::Rgb,
        )
        .unwrap(),
        specular_color: specular_color(),
        shininess: (SPECULAR_EXPONENT / 128.0) as f32,
        is_defined: true,
        ..XCAFDocVisMaterialCommon::default()
    }
}

fn colors_equal_within(first: &Color, second: &Color, tolerance: f64) -> bool {
    (first.red() - second.red()).abs() <= tolerance
        && (first.green() - second.green()).abs() <= tolerance
        && (first.blue() - second.blue()).abs() <= tolerance
}

fn colors_equal(first: &Color, second: &Color) -> bool {
    colors_equal_within(first, second, 0.01)
}

fn is_near(value: f64, expected: f64, tolerance: f64) -> bool {
    (value - expected).abs() <= tolerance
}

fn colored_props() -> StepConstructRenderingProperties {
    let mut props = StepConstructRenderingProperties::new();
    props.init_color(&surface_color(), TRANSPARENCY);
    props
}

fn fully_reflecting_props() -> StepConstructRenderingProperties {
    let mut props = colored_props();
    props.set_ambient_diffuse_and_specular_reflectance(
        AMBIENT_FACTOR,
        DIFFUSE_FACTOR,
        SPECULAR_FACTOR,
        SPECULAR_EXPONENT,
        &specular_color(),
    );
    props
}

#[test]
fn default_constructor() {
    let props = StepConstructRenderingProperties::new();
    assert!(!props.is_defined());
    assert!(!props.is_ambient_reflectance_defined());
    assert!(!props.is_diffuse_reflectance_defined());
    assert!(!props.is_specular_reflectance_defined());
    assert!(!props.is_specular_exponent_defined());
    assert!(!props.is_specular_colour_defined());
    assert_eq!(props.transparency(), 0.0);
    assert_eq!(
        props.rendering_method(),
        StepVisualShadingSurfaceMethod::SsmNormalShading
    );
}

#[test]
fn rgba_constructor() {
    // An RGBA color with alpha = 0.75 (transparency = 0.25)
    let props =
        StepConstructRenderingProperties::from_rgba(&ColorRgba::from_rgb(surface_color(), 0.75));
    assert!(props.is_defined());
    assert!(!props.is_ambient_reflectance_defined());
    assert!(!props.is_diffuse_reflectance_defined());
    assert!(!props.is_specular_reflectance_defined());
    assert!(colors_equal(&props.surface_color(), &surface_color()));
    assert!(is_near(props.transparency(), 0.25, 0.001));
    assert_eq!(
        props.rendering_method(),
        StepVisualShadingSurfaceMethod::SsmNormalShading
    );
}

#[test]
fn step_rendering_properties_constructor() {
    let props =
        StepConstructRenderingProperties::from_entity(&step_rendering_properties()).unwrap();
    assert!(props.is_defined());
    assert!(props.is_ambient_reflectance_defined());
    assert!(props.is_diffuse_reflectance_defined());
    assert!(props.is_specular_reflectance_defined());
    assert!(props.is_specular_exponent_defined());
    assert!(props.is_specular_colour_defined());
    assert!(colors_equal(&props.surface_color(), &surface_color()));
    assert!(is_near(props.transparency(), TRANSPARENCY, 0.001));
    assert!(is_near(props.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(is_near(props.diffuse_reflectance(), DIFFUSE_FACTOR, 0.001));
    assert!(is_near(
        props.specular_reflectance(),
        SPECULAR_FACTOR,
        0.001
    ));
    assert!(is_near(props.specular_exponent(), SPECULAR_EXPONENT, 0.001));
    assert!(colors_equal(&props.specular_colour(), &specular_color()));
    assert_eq!(
        props.rendering_method(),
        StepVisualShadingSurfaceMethod::SsmNormalShading
    );
}

#[test]
fn material_constructor() {
    let props = StepConstructRenderingProperties::from_common_material(&material());
    assert!(props.is_defined());
    assert!(props.is_ambient_reflectance_defined());
    assert!(props.is_diffuse_reflectance_defined());
    assert!(props.is_specular_reflectance_defined());
    assert!(props.is_specular_exponent_defined());
    assert!(props.is_specular_colour_defined());
    assert!(colors_equal(&props.surface_color(), &surface_color()));
    assert!(is_near(props.transparency(), TRANSPARENCY, 0.001));
    assert!(is_near(props.ambient_reflectance(), AMBIENT_FACTOR, 0.02));
    assert!(is_near(props.diffuse_reflectance(), 1.0, 0.001));
}

#[test]
fn set_reflectance_properties() {
    let mut props = colored_props();
    assert!(props.is_defined());
    props.set_ambient_reflectance(AMBIENT_FACTOR);
    assert!(props.is_ambient_reflectance_defined());
    assert!(is_near(props.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(!props.is_diffuse_reflectance_defined());
    props.set_ambient_and_diffuse_reflectance(AMBIENT_FACTOR, DIFFUSE_FACTOR);
    assert!(props.is_ambient_reflectance_defined());
    assert!(props.is_diffuse_reflectance_defined());
    assert!(is_near(props.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(is_near(props.diffuse_reflectance(), DIFFUSE_FACTOR, 0.001));
    assert!(!props.is_specular_reflectance_defined());
    let props = fully_reflecting_props();
    assert!(props.is_ambient_reflectance_defined());
    assert!(props.is_diffuse_reflectance_defined());
    assert!(props.is_specular_reflectance_defined());
    assert!(props.is_specular_exponent_defined());
    assert!(props.is_specular_colour_defined());
    assert!(is_near(props.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(is_near(props.diffuse_reflectance(), DIFFUSE_FACTOR, 0.001));
    assert!(is_near(
        props.specular_reflectance(),
        SPECULAR_FACTOR,
        0.001
    ));
    assert!(is_near(props.specular_exponent(), SPECULAR_EXPONENT, 0.001));
    assert!(colors_equal(&props.specular_colour(), &specular_color()));
}

#[test]
fn create_rendering_properties() {
    let entity = fully_reflecting_props()
        .create_surface_rendering_properties()
        .unwrap();
    let live = entity.get().expect("the entity of defined properties");
    assert_eq!(
        live.rendering_method(),
        StepVisualShadingSurfaceMethod::SsmNormalShading
    );
    // Verify properties through re-parsing
    let parsed = StepConstructRenderingProperties::from_entity(&entity).unwrap();
    assert!(parsed.is_defined());
    assert!(colors_equal(&parsed.surface_color(), &surface_color()));
    assert!(is_near(parsed.transparency(), TRANSPARENCY, 0.001));
    assert!(is_near(parsed.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(is_near(parsed.diffuse_reflectance(), DIFFUSE_FACTOR, 0.001));
    assert!(is_near(
        parsed.specular_reflectance(),
        SPECULAR_FACTOR,
        0.001
    ));
    assert!(is_near(
        parsed.specular_exponent(),
        SPECULAR_EXPONENT,
        0.001
    ));
    assert!(colors_equal(&parsed.specular_colour(), &specular_color()));
}

#[test]
fn create_xcaf_material() {
    let material = fully_reflecting_props().create_xcaf_material().unwrap();
    assert!(material.is_defined);
    // Check basic properties
    assert!(colors_equal(&material.diffuse_color, &surface_color()));
    assert!(is_near(
        f64::from(material.transparency),
        TRANSPARENCY,
        0.001
    ));
    // Check calculated ambient color
    let surface = surface_color();
    let expected = Color::new(
        surface.red() * AMBIENT_FACTOR,
        surface.green() * AMBIENT_FACTOR,
        surface.blue() * AMBIENT_FACTOR,
        ColorSpace::Rgb,
    )
    .unwrap();
    assert!(colors_equal(&material.ambient_color, &expected));
    // Check specular properties
    assert!(colors_equal(&material.specular_color, &specular_color()));
    assert!(is_near(
        f64::from(material.shininess),
        SPECULAR_EXPONENT / 128.0,
        0.01
    ));
}

#[test]
fn bidirectional_conversion() {
    let original = material();
    let props = StepConstructRenderingProperties::from_common_material(&original);
    let converted = props.create_xcaf_material().unwrap();
    assert!(converted.is_defined);
    assert!(colors_equal(
        &converted.diffuse_color,
        &original.diffuse_color
    ));
    assert!(is_near(
        f64::from(converted.transparency),
        f64::from(original.transparency),
        0.001
    ));
    assert!(colors_equal_within(
        &converted.ambient_color,
        &original.ambient_color,
        0.05
    ));
    assert!(colors_equal_within(
        &converted.specular_color,
        &original.specular_color,
        0.05
    ));
    assert!(is_near(
        f64::from(converted.shininess),
        f64::from(original.shininess),
        0.05
    ));
}

#[test]
fn init_with_rgba_color() {
    // An RGBA color with alpha = 0.6 (transparency = 0.4)
    let expected = Color::new(0.3, 0.6, 0.9, ColorSpace::Rgb).unwrap();
    let mut props = StepConstructRenderingProperties::new();
    props.init_rgba(&ColorRgba::from_rgb(expected, 0.6));
    assert!(props.is_defined());
    assert!(!props.is_ambient_reflectance_defined());
    assert!(!props.is_diffuse_reflectance_defined());
    assert!(!props.is_specular_reflectance_defined());
    assert!(colors_equal(&props.surface_color(), &expected));
    assert!(is_near(props.transparency(), 0.4, 0.001));
}

#[test]
fn init_with_custom_rendering_method() {
    let mut props = colored_props();
    props.set_rendering_method(StepVisualShadingSurfaceMethod::SsmDotShading);
    assert!(props.is_defined());
    assert!(colors_equal(&props.surface_color(), &surface_color()));
    assert!(is_near(props.transparency(), TRANSPARENCY, 0.001));
    assert_eq!(
        props.rendering_method(),
        StepVisualShadingSurfaceMethod::SsmDotShading
    );
}

#[test]
fn material_convertible() {
    // Initially shouldn't be convertible
    assert!(!StepConstructRenderingProperties::new().is_material_convertible());
    // After setting basic properties, still shouldn't be convertible
    assert!(!colored_props().is_material_convertible());
    // After setting all required properties, should be convertible
    assert!(fully_reflecting_props().is_material_convertible());
}

#[test]
fn null_inputs() {
    let mut props = StepConstructRenderingProperties::new();
    // Creating from null STEP rendering properties
    props.init_entity(&Handle::default()).unwrap();
    assert!(!props.is_defined());
    // Creating from an undefined material
    let undefined = XCAFDocVisMaterialCommon {
        is_defined: false,
        ..XCAFDocVisMaterialCommon::default()
    };
    assert!(!StepConstructRenderingProperties::from_common_material(&undefined).is_defined());
}

#[test]
fn create_ambient_only_properties() {
    let mut props = colored_props();
    props.set_ambient_reflectance(AMBIENT_FACTOR);
    let entity = props.create_surface_rendering_properties().unwrap();
    assert!(entity.is_some());
    // Verify properties through re-parsing
    let parsed = StepConstructRenderingProperties::from_entity(&entity).unwrap();
    assert!(parsed.is_defined());
    assert!(parsed.is_ambient_reflectance_defined());
    assert!(!parsed.is_diffuse_reflectance_defined());
    assert!(!parsed.is_specular_reflectance_defined());
    assert!(colors_equal(&parsed.surface_color(), &surface_color()));
    assert!(is_near(parsed.transparency(), TRANSPARENCY, 0.001));
    assert!(is_near(parsed.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
}

#[test]
fn create_ambient_and_diffuse_properties() {
    let mut props = colored_props();
    props.set_ambient_and_diffuse_reflectance(AMBIENT_FACTOR, DIFFUSE_FACTOR);
    let entity = props.create_surface_rendering_properties().unwrap();
    assert!(entity.is_some());
    // Verify properties through re-parsing
    let parsed = StepConstructRenderingProperties::from_entity(&entity).unwrap();
    assert!(parsed.is_defined());
    assert!(parsed.is_ambient_reflectance_defined());
    assert!(parsed.is_diffuse_reflectance_defined());
    assert!(!parsed.is_specular_reflectance_defined());
    assert!(colors_equal(&parsed.surface_color(), &surface_color()));
    assert!(is_near(parsed.transparency(), TRANSPARENCY, 0.001));
    assert!(is_near(parsed.ambient_reflectance(), AMBIENT_FACTOR, 0.001));
    assert!(is_near(parsed.diffuse_reflectance(), DIFFUSE_FACTOR, 0.001));
}

#[test]
fn non_standard_specular_color() {
    // A material with a custom specular color not matching the diffuse one
    let custom = Color::new(0.1, 0.8, 0.2, ColorSpace::Rgb).unwrap();
    let material = XCAFDocVisMaterialCommon {
        specular_color: custom,
        ..material()
    };
    let props = StepConstructRenderingProperties::from_common_material(&material);
    // The specular color is preserved as actual color, not just a factor
    assert!(props.is_specular_colour_defined());
    assert!(colors_equal(&props.specular_colour(), &custom));
    // Converting back preserves the special color
    let reconverted = props.create_xcaf_material().unwrap();
    assert!(colors_equal_within(
        &reconverted.specular_color,
        &custom,
        0.05
    ));
}

#[test]
fn extreme_values() {
    let mut props = StepConstructRenderingProperties::new();
    // Test with full transparency
    props.init_color(&surface_color(), 1.0);
    assert!(is_near(props.transparency(), 1.0, 0.001));
    // Values should be clamped when creating material
    props.set_ambient_diffuse_and_specular_reflectance(0.0, 2.0, 1.0, 256.0, &specular_color());
    let material = props.create_xcaf_material().unwrap();
    // Ambient should be 0
    assert!(is_near(material.ambient_color.red(), 0.0, 0.001));
    assert!(is_near(material.ambient_color.green(), 0.0, 0.001));
    assert!(is_near(material.ambient_color.blue(), 0.0, 0.001));
    // Shininess should be reasonable value (256/128 = 2, clamped to 1.0)
    assert!(is_near(f64::from(material.shininess), 1.0, 0.1));
}
