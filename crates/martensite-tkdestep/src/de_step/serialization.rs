use super::DeStepParameters;
use martensite_tkde::de::render_configuration;
use martensite_tkernel::tcollection::AsciiString;
pub(super) fn save(p: &DeStepParameters, healing: AsciiString) -> AsciiString {
    let substitutions = [
        read_resources(p),
        write_resources(p),
        vec![("Healing", healing)],
    ]
    .concat();
    render_configuration(include_str!("configuration.template"), &substitutions)
        .expect("fixed STEP template has one marker per setting")
}
fn read_resources(p: &DeStepParameters) -> Vec<(&'static str, AsciiString)> {
    [
        read_b_spline_continuity_resources(p).as_slice(),
        read_max_precision_mode_resources(p).as_slice(),
        read_surface_curve_mode_resources(p).as_slice(),
        read_product_mode_resources(p).as_slice(),
        read_tessellated_resources(p).as_slice(),
        read_shape_aspect_resources(p).as_slice(),
        read_code_page_resources(p).as_slice(),
        read_all_shapes_resources(p).as_slice(),
        read_name_resources(p).as_slice(),
    ]
    .concat()
}

fn write_resources(p: &DeStepParameters) -> Vec<(&'static str, AsciiString)> {
    [
        read_metadata_resources(p).as_slice(),
        write_precision_val_resources(p).as_slice(),
        write_tessellated_resources(p).as_slice(),
        write_unit_resources(p).as_slice(),
        write_color_resources(p).as_slice(),
        write_layer_resources(p).as_slice(),
        write_material_resources(p).as_slice(),
        clean_duplicates_resources(p).as_slice(),
    ]
    .concat()
}

fn read_b_spline_continuity_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadBSplineContinuity",
            AsciiString::from_integer(p.read_b_spline_continuity.as_raw()),
        ),
        (
            "ReadPrecisionMode",
            AsciiString::from_integer(p.read_precision_mode.as_raw()),
        ),
        (
            "ReadPrecisionVal",
            AsciiString::from_real(p.read_precision_val),
        ),
    ]
}
fn read_max_precision_mode_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadMaxPrecisionMode",
            AsciiString::from_integer(p.read_max_precision_mode.as_raw()),
        ),
        (
            "ReadMaxPrecisionVal",
            AsciiString::from_real(p.read_max_precision_val),
        ),
        (
            "ReadSameParamMode",
            AsciiString::from_integer(p.read_same_param_mode as i32),
        ),
    ]
}
fn read_surface_curve_mode_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadSurfaceCurveMode",
            AsciiString::from_integer(p.read_surface_curve_mode.as_raw()),
        ),
        ("EncodeRegAngle", AsciiString::from_real(p.encode_reg_angle)),
        (
            "AngleUnit",
            AsciiString::from_integer(p.angle_unit.as_raw()),
        ),
    ]
}
fn read_product_mode_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadProductMode",
            AsciiString::from_integer(p.read_product_mode as i32),
        ),
        (
            "ReadProductContext",
            AsciiString::from_integer(p.read_product_context.as_raw()),
        ),
        (
            "ReadShapeRepr",
            AsciiString::from_integer(p.read_shape_repr.as_raw()),
        ),
    ]
}
fn read_tessellated_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadTessellated",
            AsciiString::from_integer(p.read_tessellated.as_raw()),
        ),
        (
            "ReadAssemblyLevel",
            AsciiString::from_integer(p.read_assembly_level.as_raw()),
        ),
        (
            "ReadRelationship",
            AsciiString::from_integer(p.read_relationship as i32),
        ),
    ]
}
fn read_shape_aspect_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadShapeAspect",
            AsciiString::from_integer(p.read_shape_aspect as i32),
        ),
        (
            "ReadConstrRelation",
            AsciiString::from_integer(p.read_constr_relation as i32),
        ),
        (
            "ReadSubshapeNames",
            AsciiString::from_integer(p.read_subshape_names as i32),
        ),
    ]
}
fn read_code_page_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        ("ReadCodePage", AsciiString::from_integer(p.read_code_page)),
        (
            "ReadNonmanifold",
            AsciiString::from_integer(p.read_nonmanifold as i32),
        ),
        ("ReadIdeas", AsciiString::from_integer(p.read_ideas as i32)),
    ]
}
fn read_all_shapes_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadAllShapes",
            AsciiString::from_integer(p.read_all_shapes as i32),
        ),
        (
            "ReadRootTransformation",
            AsciiString::from_integer(p.read_root_transformation as i32),
        ),
        ("ReadColor", AsciiString::from_integer(p.read_color as i32)),
    ]
}
fn read_name_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        ("ReadName", AsciiString::from_integer(p.read_name as i32)),
        ("ReadLayer", AsciiString::from_integer(p.read_layer as i32)),
        ("ReadProps", AsciiString::from_integer(p.read_props as i32)),
    ]
}
fn read_metadata_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadMetadata",
            AsciiString::from_integer(p.read_metadata as i32),
        ),
        (
            "ReadProductMetadata",
            AsciiString::from_integer(p.read_product_metadata as i32),
        ),
        (
            "WritePrecisionMode",
            AsciiString::from_integer(p.write_precision_mode.as_raw()),
        ),
    ]
}
fn write_precision_val_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WritePrecisionVal",
            AsciiString::from_real(p.write_precision_val),
        ),
        (
            "WriteAssembly",
            AsciiString::from_integer(p.write_assembly.as_raw()),
        ),
        (
            "WriteSchema",
            AsciiString::from_integer(p.write_schema.as_raw()),
        ),
    ]
}
fn write_tessellated_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteTessellated",
            AsciiString::from_integer(p.write_tessellated.as_raw()),
        ),
        ("WriteProductName", p.write_product_name.clone()),
        (
            "WriteSurfaceCurMode",
            AsciiString::from_integer(p.write_surface_cur_mode as i32),
        ),
    ]
}
fn write_unit_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        ("WriteUnit", AsciiString::from_integer(p.write_unit)),
        (
            "WriteVertexMode",
            AsciiString::from_integer(p.write_vertex_mode.as_raw()),
        ),
        (
            "WriteSubshapeNames",
            AsciiString::from_integer(p.write_subshape_names as i32),
        ),
    ]
}
fn write_color_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteColor",
            AsciiString::from_integer(p.write_color as i32),
        ),
        (
            "WriteNonmanifold",
            AsciiString::from_integer(p.write_nonmanifold as i32),
        ),
        ("WriteName", AsciiString::from_integer(p.write_name as i32)),
    ]
}
fn write_layer_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteLayer",
            AsciiString::from_integer(p.write_layer as i32),
        ),
        (
            "WriteProps",
            AsciiString::from_integer(p.write_props as i32),
        ),
        (
            "WriteMetadata",
            AsciiString::from_integer(p.write_metadata as i32),
        ),
    ]
}
fn write_material_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteMaterial",
            AsciiString::from_integer(p.write_material as i32),
        ),
        (
            "WriteVisMaterial",
            AsciiString::from_integer(p.write_vis_material as i32),
        ),
        (
            "WriteModelType",
            AsciiString::from_integer(p.write_model_type.as_raw()),
        ),
    ]
}
fn clean_duplicates_resources(p: &DeStepParameters) -> [(&'static str, AsciiString); 2] {
    [
        (
            "CleanDuplicates",
            AsciiString::from_integer(p.clean_duplicates as i32),
        ),
        (
            "WriteScalingTrsf",
            AsciiString::from_integer(p.write_scaling_trsf as i32),
        ),
    ]
}
