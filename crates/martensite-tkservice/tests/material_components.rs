//! The material side of the twenty-material sample scene: the material aliases and the four
//! color columns, checked without a renderer. It does not check rendered images.
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::{
    Graphic3dMaterialAspect, Graphic3dNameOfMaterial as Material,
};

const SAMPLE_MATERIALS: [(&str, Material); 20] = [
    ("brass", Material::Brass),
    ("bronze", Material::Bronze),
    ("copper", Material::Copper),
    ("gold", Material::Gold),
    ("jade", Material::Jade),
    ("neon_phc", Material::Neon),
    ("pewter", Material::Pewter),
    ("obsidian", Material::Obsidian),
    ("plaster", Material::Plastered),
    ("plastic", Material::Plastified),
    ("satin", Material::Satin),
    ("silver", Material::Silver),
    ("steel", Material::Steel),
    ("stone", Material::Stone),
    ("chrome", Material::Chrome),
    ("aluminium", Material::Aluminum),
    ("water", Material::Water),
    ("glass", Material::Glass),
    ("diamond", Material::Diamond),
    ("charcoal", Material::Charcoal),
];

fn sample_color_columns() -> [Option<Color>; 4] {
    [
        None,
        Some(Color::named(NamedColor::RED)),
        Some(Color::named(NamedColor::GREEN)),
        Some(Color::named(NamedColor::BLUE1)),
    ]
}

#[test]
fn sample_material_aliases_resolve_to_the_expected_twenty_materials() {
    for (name, expected) in SAMPLE_MATERIALS {
        assert_eq!(
            Graphic3dMaterialAspect::material_from_name(name),
            Some(expected)
        );
    }
}

#[test]
fn sample_color_columns_preserve_material_emission_on_both_objects() {
    for (_, name) in SAMPLE_MATERIALS {
        let original = Graphic3dMaterialAspect::new(name);
        for color in sample_color_columns() {
            // Each sample cell assigns the material independently to its sphere and box.
            check_sample_material_color(&original, color);
        }
    }
}

fn check_sample_material_color(original: &Graphic3dMaterialAspect, color: Option<Color>) {
    for mut material in [original.clone(), original.clone()] {
        if let Some(color) = color {
            material.set_color(color).unwrap();
        }
        assert_eq!(material.requested_name(), original.requested_name());
        assert_eq!(material.emissive_color(), original.emissive_color());
        assert_eq!(material.specular_color(), original.specular_color());
        assert_eq!(material.transparency(), original.transparency());
    }
}

#[test]
fn sample_neon_keeps_nonzero_emission_after_each_color_override() {
    for color in sample_color_columns() {
        let mut neon = Graphic3dMaterialAspect::new(Material::Neon);
        if let Some(color) = color {
            neon.set_color(color).unwrap();
        }
        let emission = neon.emissive_color().rgb_vec3();
        assert_eq!(
            [emission.x, emission.y, emission.z],
            [0.0, 0.787_412, 0.142_892]
        );
    }
}
