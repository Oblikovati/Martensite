//! Graphic3d_MaterialAspect catalogue probe (legacy BSDF/PBR material probes retired by #623).
use super::common::{Reader, end, int, text};
use super::print::Output;
use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dNameOfMaterial};

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
