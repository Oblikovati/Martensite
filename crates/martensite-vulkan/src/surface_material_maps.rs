//! Texture sets beyond the base colour: the emissive, occlusion, normal and metallic-roughness
//! maps that `Graphic3dShaderManager::getStdProgramPhong` samples in textured lit
//! programs (`Graphic3dTextureUnit` 1-4). The first texture of the set stays the base colour.
use crate::{RenderError, error::rejected, surface_texture::*, text_submission::TextSceneContext};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// The map units in descriptor order (set 5, bindings 1-4).
pub(crate) const MATERIAL_MAP_UNITS: [Graphic3dTextureUnit; 4] = [
    Graphic3dTextureUnit::EMISSIVE,
    Graphic3dTextureUnit::OCCLUSION,
    Graphic3dTextureUnit::NORMAL,
    Graphic3dTextureUnit::METALLIC_ROUGHNESS,
];

/// One map snapshot with its own sampler parameters.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceMaterialMap {
    pub image: Arc<SurfaceTextureImage>,
    pub sampler: SurfaceSamplerPolicy,
}

/// The present maps in `MATERIAL_MAP_UNITS` order; a missing unit binds the native mock texture
/// (white, black for the normal map). Example: `maps.slots[2].is_some()` has a normal map.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceMaterialMaps {
    pub slots: [Option<SurfaceMaterialMap>; 4],
}

/// Accept one base-colour texture first, then 2D maps of distinct material units; the base
/// colour may be a 3D texture only for a user program (`programmable`), whose `sampler3D`
/// reads it. Example: `validate_texture_set(&*aspects.texture_set(), false)?`.
pub(crate) fn validate_texture_set(
    textures: &[Handle<dyn Graphic3dTextureMap>],
    programmable: bool,
) -> Result<(), RenderError> {
    let mut units = Vec::new();
    for (index, texture) in textures.iter().enumerate() {
        let Some(texture) = texture.get() else {
            continue;
        };
        let unit = texture.get_params().texture_unit();
        let expected = index == 0 && unit == Graphic3dTextureUnit::BASE_COLOR
            || index > 0 && MATERIAL_MAP_UNITS.contains(&unit) && !units.contains(&unit);
        let kind = texture.texture_type();
        let volume = programmable && index == 0 && kind == Graphic3dTypeOfTexture::Texture3d;
        if !(kind == Graphic3dTypeOfTexture::TOT_2D || volume) || !expected {
            return Err(rejected(
                (index, kind, unit, programmable),
                "a 2D base-colour texture (3D with a user program) first, then 2D emissive/occlusion/normal/metallic-roughness maps of distinct units",
            ));
        }
        units.push(unit);
    }
    Ok(())
}

/// Snapshot the maps after the base colour, or `None` for a single-texture set;
/// `snapshot_material_maps(&aspects, context)?`.
pub(crate) fn snapshot_material_maps(
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &TextSceneContext<'_>,
) -> Result<Option<Arc<SurfaceMaterialMaps>>, RenderError> {
    let set = aspects.texture_set();
    let Some(set) = set.get() else {
        return Ok(None);
    };
    let mut maps = SurfaceMaterialMaps::default();
    for texture in set.textures().iter().skip(1).filter(|t| !t.is_null()) {
        let unit = texture.get_params().texture_unit();
        let slot = MATERIAL_MAP_UNITS.iter().position(|u| *u == unit);
        let slot = slot.ok_or_else(|| rejected(unit, "a material map texture unit"))?;
        maps.slots[slot] = Some(snapshot_map(texture, context)?);
    }
    Ok(maps
        .slots
        .iter()
        .any(Option::is_some)
        .then(|| Arc::new(maps)))
}

fn snapshot_map(
    texture: &Handle<dyn Graphic3dTextureMap>,
    context: &TextSceneContext<'_>,
) -> Result<SurfaceMaterialMap, RenderError> {
    let images = &context.group.texture_images;
    let image = images
        .snapshot(texture)?
        .ok_or_else(|| rejected(texture.get_id(), "a loadable material map image"))?;
    let sampler = images.samplers.policy(&image, &texture.get_params())?;
    Ok(SurfaceMaterialMap { image, sampler })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn texture(unit: Graphic3dTextureUnit) -> Handle<dyn Graphic3dTextureMap> {
        let texture = Handle::new(Graphic3dTexture2D::from_pix_map(Handle::default()));
        texture.get_params().set_texture_unit(unit);
        texture.cast()
    }
    #[test]
    fn texture_sets_put_the_base_colour_first_and_maps_on_distinct_units() {
        use Graphic3dTextureUnit as Unit;
        let all = [
            Unit::BASE_COLOR,
            Unit::EMISSIVE,
            Unit::OCCLUSION,
            Unit::NORMAL,
            Unit::METALLIC_ROUGHNESS,
        ];
        assert!(validate_texture_set(&all.map(texture), false).is_ok());
        assert!(
            validate_texture_set(&[texture(Unit::BASE_COLOR), Handle::default()], false).is_ok()
        );
        assert!(validate_texture_set(&[texture(Unit::NORMAL)], false).is_err());
        let repeated = [Unit::BASE_COLOR, Unit::NORMAL, Unit::NORMAL];
        assert!(validate_texture_set(&repeated.map(texture), false).is_err());
        let foreign = [Unit::BASE_COLOR, Graphic3dTextureUnit::Unit5];
        assert!(validate_texture_set(&foreign.map(texture), false).is_err());
    }
    #[test]
    fn a_3d_base_colour_needs_a_user_program_and_maps_stay_2d() {
        use Graphic3dTextureUnit as Unit;
        let volume = Handle::new(Graphic3dTexture3D::from_pix_map(Handle::default()));
        let volume: Handle<dyn Graphic3dTextureMap> = volume.cast();
        assert!(validate_texture_set(std::slice::from_ref(&volume), true).is_ok());
        assert!(validate_texture_set(std::slice::from_ref(&volume), false).is_err());
        volume.get_params().set_texture_unit(Unit::NORMAL);
        let maps = [texture(Unit::BASE_COLOR), volume];
        assert!(validate_texture_set(&maps, true).is_err());
    }
}
