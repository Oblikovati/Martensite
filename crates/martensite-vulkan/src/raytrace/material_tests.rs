use super::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dAspects, Graphic3dMaterialAspect, Graphic3dNameOfMaterial,
};

fn plastic_green() -> Handle<Graphic3dAspects> {
    let aspects = Graphic3dAspects::default();
    aspects.set_front_material(Graphic3dMaterialAspect::new(
        Graphic3dNameOfMaterial::Plastified,
    ));
    aspects.set_interior_color(Color::named(NamedColor::GREEN));
    Handle::new(aspects)
}

#[test]
fn aspect_materials_scale_openpbr_base_color_by_interior_colour() {
    let aspects = plastic_green();
    let material = RaytraceMaterial::of_aspects(&*aspects, RenderMaterialColorSpace::Linear);
    let source = aspects.front_material();
    let green = Color::named(NamedColor::GREEN).rgb();
    let base = source.openpbr_material().base.color;
    assert_eq!(material.gpu.base_color_weight[0], base[0] * green[0]);
    assert_eq!(material.gpu.base_color_weight[1], base[1] * green[1]);
    assert_eq!(material.gpu.base_color_weight[2], base[2] * green[2]);
    assert_eq!(material.tex.slots[0], -1);
}

#[test]
fn physic_materials_preserve_physical_openpbr_base_color() {
    let aspects = Graphic3dAspects::default();
    aspects.set_front_material(Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold));
    let material = RaytraceMaterial::of_aspects(&aspects, RenderMaterialColorSpace::Linear);
    assert!((material.gpu.base_spec_params[0] - 1.0).abs() < 1e-5);
    assert!(material.gpu.base_color_weight[0] > 0.8);
}
