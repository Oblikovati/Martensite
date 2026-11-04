//! Capture all aspect fields without handle addresses; material floats retain stored f32 bits.
use martensite_tkernel::ncollection::{Vec3, Vec4};
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsFields, Graphic3dAspectsSource, Graphic3dBsdf, Graphic3dMaterialAspect,
};
use martensite_tkv3d::{ais::AisTriangulation, prs_mgr::PrsMgrPresentableObjectMembers as _};

pub(super) fn appearance(bytes: &mut Vec<u8>, object: &AisTriangulation) {
    let shading = object.attributes().shading_aspect();
    let aspect = shading.aspect();
    let fields = aspect.aspects_base().fields.read();
    aspect_flags(bytes, &fields);
    aspect_floats(bytes, &fields);
    for color in [
        fields.interior_color,
        fields.back_interior_color,
        fields.edge_color,
    ] {
        vector4(bytes, color.to_vec4());
    }
    material(bytes, &fields.front_material);
    material(bytes, &fields.back_material);
}

fn aspect_flags(bytes: &mut Vec<u8>, fields: &Graphic3dAspectsFields) {
    let flags = [
        fields.program.is_null(),
        fields.texture_set.is_null(),
        fields.marker_image.is_null(),
        fields.hatch_style.is_null(),
        fields.text_font.is_null(),
        fields.to_skip_first_edge,
        fields.to_distinguish_materials,
        fields.to_use_vertex_color_for_back_faces,
        fields.to_draw_edges,
        fields.to_draw_silhouette,
        fields.to_map_texture,
        fields.is_text_zoomable,
    ];
    bytes.extend(flags.map(u8::from));
    aspect_enums(bytes, fields);
}

fn aspect_enums(bytes: &mut Vec<u8>, fields: &Graphic3dAspectsFields) {
    let enums = format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        fields.interior_style,
        fields.shading_model,
        fields.face_culling,
        fields.alpha_mode,
        fields.line_type,
        fields.marker_type,
        fields.text_style,
        fields.text_display_type,
        fields.text_font_aspect
    );
    bytes.extend((enums.len() as u64).to_le_bytes());
    bytes.extend(enums.as_bytes());
    bytes.extend(fields.line_factor.to_le_bytes());
    bytes.extend(fields.line_pattern.to_le_bytes());
}

fn aspect_floats(bytes: &mut Vec<u8>, fields: &Graphic3dAspectsFields) {
    for value in [
        fields.alpha_cutoff,
        fields.line_width,
        fields.marker_scale,
        fields.text_angle,
    ] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
    let offset = &fields.polygon_offset;
    bytes.extend(format!("{:?}", offset.mode).as_bytes());
    bytes.extend(offset.factor.to_bits().to_le_bytes());
    bytes.extend(offset.units.to_bits().to_le_bytes());
}

fn material(bytes: &mut Vec<u8>, material: &Graphic3dMaterialAspect) {
    bytes.extend(format!("{material:?}").as_bytes());
    material_colors(bytes, material);
    for value in [
        material.transparency(),
        material.refraction_index(),
        material.shininess(),
    ] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
    bsdf(bytes, material.bsdf());
    pbr(bytes, material);
}

fn material_colors(bytes: &mut Vec<u8>, material: &Graphic3dMaterialAspect) {
    for color in [
        material.ambient_color(),
        material.diffuse_color(),
        material.specular_color(),
        material.emissive_color(),
    ] {
        for value in color.rgb() {
            bytes.extend(value.to_bits().to_le_bytes());
        }
    }
}

fn bsdf(bytes: &mut Vec<u8>, bsdf: &Graphic3dBsdf) {
    for vector in [
        bsdf.kc,
        bsdf.ks,
        bsdf.absorption,
        bsdf.fresnel_coat.serialize(),
        bsdf.fresnel_base.serialize(),
    ] {
        vector4(bytes, vector);
    }
    for vector in [bsdf.kd, bsdf.kt, bsdf.le] {
        vector3(bytes, vector);
    }
}

fn pbr(bytes: &mut Vec<u8>, material: &Graphic3dMaterialAspect) {
    let pbr = material.pbr_material();
    for value in [pbr.metallic(), pbr.roughness(), pbr.ior(), pbr.alpha()] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
    vector4(bytes, pbr.color().to_vec4());
    vector3(bytes, pbr.emission());
}

fn vector4(bytes: &mut Vec<u8>, vector: Vec4<f32>) {
    for value in [vector.x(), vector.y(), vector.z(), vector.w()] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
}

fn vector3(bytes: &mut Vec<u8>, vector: Vec3<f32>) {
    for value in [vector.x(), vector.y(), vector.z()] {
        bytes.extend(value.to_bits().to_le_bytes());
    }
}
