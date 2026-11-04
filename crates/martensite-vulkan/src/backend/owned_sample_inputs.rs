//! The eight declared rows reuse Martensite's material/light snapshot and user-program builders.
use crate::user_program::{RenderUserProgramLibrary, UserProgramDraw, UserUniformScene};
use crate::{
    PrimitiveDraw, RenderFrame, RenderLightSet, RenderLighting, RenderMaterial,
    RenderMaterialColorSpace, RenderShading, RenderVertex, identity_clip,
};
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;
type OwnedResult<T> = Result<T, Box<dyn std::error::Error>>;
const ROWS: [&str; 8] = [
    "directional-existing",
    "directional-updated",
    "intensity7-existing",
    "intensity7-updated",
    "positional-existing",
    "positional-updated",
    "spot-existing",
    "spot-updated",
];

pub(super) fn frame(row: &str) -> OwnedResult<RenderFrame> {
    if !ROWS.contains(&row) {
        return Err(format!("row={row:?}; expected one of {ROWS:?}").into());
    }
    let draw = owned_draw(row)?;
    Ok(RenderFrame::new(64, 64, [0., 0., 0., 1.], vec![draw])?)
}
fn owned_draw(row: &str) -> OwnedResult<PrimitiveDraw> {
    let material = material();
    let lighting = owned_lighting(row, material)?;
    let vertices = positions().map(|position| RenderVertex::new(position, [0.75, 0.5, 0.25, 1.]));
    let mut draw = PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip())?
        .with_lighting(lighting)?;
    draw.user_program = Some(Arc::new(program(material)?));
    request_owned_clockwise(&mut draw);
    Ok(draw)
}
fn owned_lighting(row: &str, material: RenderMaterial) -> OwnedResult<RenderLighting> {
    let lighting = RenderLighting::new(
        normals(row),
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )?;
    Ok(lighting.with_lights(lights(row)?))
}
fn request_owned_clockwise(draw: &mut PrimitiveDraw) {
    // The existing low-level flag selects raster winding under TopLeftLogicalV2. This
    // owned request claims no mirrored geometry: every transform/vertex byte stays intact.
    draw.mirrored = true;
}

fn positions() -> [[f32; 3]; 3] {
    // The public V2 renderer uses a negative-height viewport: input +Y maps toward the top.
    // These dyadic object values derive exactly the declared (8,8),(56,8),(8,54) windows.
    [[-0.75, 0.75, 0.], [0.75, 0.75, 0.], [-0.75, -0.6875, 0.]]
}
fn normals(row: &str) -> Vec<[f32; 3]> {
    let mut normals = vec![[0., 0., 1.], [0.6, 0., 0.8], [0., 0.6, 0.8]];
    if row.ends_with("-updated") {
        normals[1] = [1., 0., 0.];
    }
    normals
}
fn color(values: [f32; 3]) -> Color {
    Color::new(
        values[0].into(),
        values[1].into(),
        values[2].into(),
        ColorSpace::Rgb,
    )
    .expect("the fixed dyadic owned material colors lie in [0,1]")
}
fn material_aspect() -> Graphic3dMaterialAspect {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_material_type(Graphic3dTypeOfMaterial::Aspect);
    material.set_ambient_color(color([0.125, 0.25, 0.375]));
    material.set_diffuse_color(color([0.5, 0.25, 0.125]));
    material.set_specular_color(color([0.25, 0.125, 0.0625]));
    material.set_emissive_color(color([0.015625, 0.03125, 0.0625]));
    material.set_shininess(0.25).expect("fixed owned shininess");
    material
}
fn material() -> RenderMaterial {
    let source = material_aspect();
    let mut material = RenderMaterial::default();
    material.init(
        RenderMaterialColorSpace::Linear,
        &source,
        color([0.75, 0.5, 0.25]),
        &source,
        color([0.75, 0.5, 0.25]),
    );
    material
}
fn lights(row: &str) -> OwnedResult<RenderLightSet> {
    let ambient = Handle::new(Graphic3dCLight::new(Graphic3dTypeOfLightSource::Ambient));
    ambient.set_color(color([0.125, 0.25, 0.5]));
    ambient.set_intensity(2.)?;
    let set = Handle::new(Graphic3dLightSet::default());
    set.add(&ambient)?;
    set.add(&source_light(row)?)?;
    Ok(RenderLightSet::snapshot(&set, identity_clip())?)
}
fn source_light(row: &str) -> OwnedResult<Handle<Graphic3dCLight>> {
    let kind = match row.split('-').next() {
        Some("positional") => Graphic3dTypeOfLightSource::Positional,
        Some("spot") => Graphic3dTypeOfLightSource::Spot,
        _ => Graphic3dTypeOfLightSource::Directional,
    };
    let light = Handle::new(Graphic3dCLight::new(kind));
    light.set_color(color([0.75, 0.5, 0.25]));
    light.set_intensity(if row.starts_with("intensity7-") {
        7.
    } else {
        1.
    })?;
    configure_source(&light, kind)?;
    Ok(light)
}
fn configure_source(light: &Graphic3dCLight, kind: Graphic3dTypeOfLightSource) -> OwnedResult<()> {
    if kind != Graphic3dTypeOfLightSource::Positional {
        light.set_direction_coords(0., 0., -1.)?;
    }
    if kind != Graphic3dTypeOfLightSource::Directional {
        light.set_position_coords(0., 0., 12.)?;
        light.set_attenuation(0.5, 0.125)?;
    }
    if kind == Graphic3dTypeOfLightSource::Spot {
        light.set_angle(0.75)?;
        light.set_concentration(0.125)?;
    }
    Ok(())
}
fn program(material: RenderMaterial) -> OwnedResult<UserProgramDraw> {
    let library = RenderUserProgramLibrary::default();
    crate::user_program::register_samples(&library)?;
    let modules = library.modules(&crate::user_program::phong_shading_program())?;
    let scene = UserUniformScene {
        model_world: identity_clip(),
        world_view: identity_clip(),
        projection: identity_clip(),
        max_lights: 8,
        material: material.packed_common(),
        color: [0.75, 0.5, 0.25, 1.],
        ..Default::default()
    };
    Ok(UserProgramDraw::new(modules, scene))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clockwise_request_changes_only_the_explicit_raster_selector() {
        let mut draw = owned_draw("directional-existing").unwrap();
        draw.mirrored = false;
        let before = draw.clone();
        request_owned_clockwise(&mut draw);
        assert!(draw.mirrored);
        assert_eq!(draw.face_culling, before.face_culling);
        draw.mirrored = false;
        assert_eq!(draw, before);
    }
    #[test]
    fn clockwise_request_preserves_the_complete_packed_uniform_block() {
        let mut draw = owned_draw("spot-updated").unwrap();
        let program = draw.user_program.as_ref().unwrap().clone();
        let before = program.uniform_bytes(draw.lighting.as_ref().map(|lighting| &lighting.lights));
        draw.mirrored = false;
        request_owned_clockwise(&mut draw);
        let after = program.uniform_bytes(draw.lighting.as_ref().map(|lighting| &lighting.lights));
        assert_eq!(before, after);
    }
    #[test]
    fn production_identity_inverse_transpose_retains_signed_zero_words() {
        let draw = owned_draw("directional-existing").unwrap();
        let program = draw.user_program.as_ref().unwrap();
        let block = program.uniform_bytes(draw.lighting.as_ref().map(|lighting| &lighting.lights));
        // Cofactor inversion preserves IEEE signed zeros; captured blocks must not rewrite them.
        let words: Vec<u32> = block[576..640]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| u32::from_ne_bytes(*word))
            .collect();
        assert!(words.contains(&0x8000_0000));
        for (index, word) in words.iter().enumerate() {
            let expected = if index % 5 == 0 { 1.0_f32.to_bits() } else { 0 };
            assert_eq!(word & 0x7fff_ffff, expected);
        }
    }
    #[test]
    fn all_eight_rows_keep_one_draw_and_expected_actual_uniform_abi() {
        for row in ROWS {
            let frame = frame(row).unwrap();
            let draw = &frame.draws()[0];
            let program = draw.user_program.as_ref().unwrap();
            let bytes =
                program.uniform_bytes(draw.lighting.as_ref().map(|lighting| &lighting.lights));
            assert_eq!(bytes.len(), 1920);
            assert_eq!(draw.indices, [0, 1, 2]);
            assert_eq!(frame.layers().len(), 1);
            assert_eq!(i32::from_ne_bytes(bytes[1904..1908].try_into().unwrap()), 0);
        }
    }
    #[test]
    fn geometry_and_live_normal_change_are_exact_and_do_not_modify_positions() {
        let before = frame("spot-existing").unwrap();
        let after = frame("spot-updated").unwrap();
        assert_eq!(before.draws()[0].vertices, after.draws()[0].vertices);
        assert_eq!(
            after.draws()[0].lighting.as_ref().unwrap().normals()[1],
            [1., 0., 0.]
        );
        assert_eq!(positions()[2], [-0.75, -0.6875, 0.]);
    }
    #[test]
    fn unknown_lighting_row_is_refused_before_any_device_operation() {
        let error = frame("directional-unknown").unwrap_err().to_string();
        assert!(error.contains("directional-unknown"));
        assert!(error.contains("expected one of"));
    }
    #[test]
    fn light_builders_preserve_intensity_and_material_packing() {
        // packed_light keeps RGB chromaticity and stores intensity in alpha; the known
        // Phong sample reads only .rgb (light_snapshot.rs and user/phong_shading.frag.glsl).
        assert_eq!(
            lights("intensity7-existing").unwrap().sources[0].color,
            [0.75, 0.5, 0.25, 7.]
        );
        assert_eq!(
            lights("directional-existing").unwrap().ambient,
            [0.25, 0.5, 1., 1.]
        );
        assert_eq!(material().packed_common().len(), 8);
    }
}
