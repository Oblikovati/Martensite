//! Graphic3d_MaterialAspect.
use super::common::{Reader, end, flag, int, real, text};
use super::pbr::{print_bsdf, vec3, vec4};
use super::print::Output;
use super::tried;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkservice::graphic3d::{
    Graphic3dMaterialAspect, Graphic3dNameOfMaterial, Graphic3dPbrMaterial,
    Graphic3dTypeOfReflection,
};

/// The material of a `Graphic3d_NameOfMaterial` value from the corpus (always in range).
pub fn material_name(value: i32) -> Graphic3dNameOfMaterial {
    const NAMES: [Graphic3dNameOfMaterial; 26] = {
        use Graphic3dNameOfMaterial as N;
        [
            N::Brass,
            N::Bronze,
            N::Copper,
            N::Gold,
            N::Pewter,
            N::Plastered,
            N::Plastified,
            N::Silver,
            N::Steel,
            N::Stone,
            N::ShinyPlastified,
            N::Satin,
            N::Metalized,
            N::Ionized,
            N::Chrome,
            N::Aluminum,
            N::Obsidian,
            N::Neon,
            N::Jade,
            N::Charcoal,
            N::Water,
            N::Glass,
            N::Diamond,
            N::Transparent,
            N::Default,
            N::UserDefined,
        ]
    };
    NAMES[value as usize]
}
fn color(output: &Output, value: Color) {
    value
        .values(ColorSpace::Rgb)
        .into_iter()
        .for_each(|value| real(output, value));
}
fn colors(output: &Output, m: &Graphic3dMaterialAspect) {
    [
        m.ambient_color(),
        m.diffuse_color(),
        m.specular_color(),
        m.emissive_color(),
    ]
    .into_iter()
    .for_each(|value| color(output, value));
}
fn pbr(output: &Output, p: &Graphic3dPbrMaterial) {
    vec4(output, p.color().to_vec4());
    [p.metallic(), p.normalized_roughness(), p.ior()]
        .into_iter()
        .for_each(|c| real(output, f64::from(c)));
    vec3(output, p.emission());
}

/// `sv_material Name`.
pub fn material_case(output: &Output, values: &[f64]) {
    let m = Graphic3dMaterialAspect::new(material_name(Reader::new(values).integer()));
    colors(output, &m);
    [m.transparency(), m.refraction_index(), m.shininess()]
        .into_iter()
        .for_each(|c| real(output, f64::from(c)));
    int(output, m.material_type() as i32);
    int(output, m.name() as i32);
    text(output, m.string_name().as_bytes());
    print_bsdf(output, m.bsdf());
    pbr(output, m.pbr_material());
    let mut dump = DumpStream::default();
    m.dump_json(&mut dump, -1).expect("in-memory dump");
    text(output, dump.bytes());
    end(output);
}

/// `sv_material_rank Rank`.
pub fn rank_case(output: &Output, values: &[f64]) {
    let rank = Reader::new(values).integer();
    match Graphic3dMaterialAspect::material_name_of(rank) {
        Ok(name) => text(output, name.as_bytes()),
        Err(error) => output.write(format_args!("{} ", error.exception_type())),
    }
    match Graphic3dMaterialAspect::material_type_of(rank) {
        Ok(kind) => int(output, kind as i32),
        Err(error) => output.write(format_args!("{} ", error.exception_type())),
    }
    end(output);
}

/// `sv_material_edit Name R G B Delta Shine Transparency Index`.
pub fn edit_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut m = Graphic3dMaterialAspect::new(material_name(r.integer()));
    let rgb = [r.real(), r.real(), r.real()];
    let [delta, shine, transparency, index] = [0; 4].map(|_| r.real() as f32);
    let c =
        Color::new(rgb[0], rgb[1], rgb[2], ColorSpace::Rgb).expect("corpus colors are in range");
    tried(output, m.set_color(c));
    int(output, m.name() as i32);
    colors(output, &m);
    pbr(output, m.pbr_material());
    m.increase_shine(delta);
    real(output, f64::from(m.shininess()));
    edit_coefficients(output, &mut m, shine, transparency, index);
    edit_reflection(output, &mut m, c);
    end(output);
}

fn edit_coefficients(
    output: &Output,
    m: &mut Graphic3dMaterialAspect,
    shine: f32,
    transparency: f32,
    index: f32,
) {
    tried(output, m.set_shininess(shine));
    real(output, f64::from(m.shininess()));
    int(output, m.name() as i32);
    tried(output, m.set_transparency(transparency));
    [m.transparency(), m.alpha(), m.pbr_material().alpha()]
        .into_iter()
        .for_each(|c| real(output, f64::from(c)));
    tried(output, m.set_refraction_index(index));
    real(output, f64::from(m.refraction_index()));
}

fn edit_reflection(output: &Output, m: &mut Graphic3dMaterialAspect, c: Color) {
    use Graphic3dTypeOfReflection as K;
    let mut fresh = Graphic3dMaterialAspect::new(m.requested_name());
    fresh.set_specular_color(c);
    int(output, fresh.name() as i32);
    text(output, fresh.string_name().as_bytes());
    [K::Ambient, K::Diffuse, K::Specular, K::Emission]
        .into_iter()
        .for_each(|kind| flag(output, fresh.reflection_mode(kind)));
    flag(output, m.is_equal(&fresh));
    m.reset();
    flag(
        output,
        m.is_equal(&Graphic3dMaterialAspect::new(m.requested_name())),
    );
}
