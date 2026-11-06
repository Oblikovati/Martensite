use super::DeIgesParameters;
use martensite_tkde::de::render_configuration;
use martensite_tkernel::tcollection::AsciiString;
pub(super) fn save(p: &DeIgesParameters, healing: AsciiString) -> AsciiString {
    let substitutions = [
        read_precision(p).as_slice(),
        read_limits(p).as_slice(),
        read_surface(p).as_slice(),
        read_visibility(p).as_slice(),
        read_names_and_brep(p).as_slice(),
        write_surface_and_headers(p).as_slice(),
        write_headers_and_precision(p).as_slice(),
        write_precision_and_plane(p).as_slice(),
        write_attributes(p).as_slice(),
        &[("Healing", healing)],
    ]
    .concat();
    render_configuration(include_str!("configuration.template"), &substitutions)
        .expect("fixed IGES template has one marker per setting")
}
fn read_precision(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
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
fn read_limits(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
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
fn read_surface(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadSurfaceCurveMode",
            AsciiString::from_integer(p.read_surface_curve_mode.as_raw()),
        ),
        ("EncodeRegAngle", AsciiString::from_real(p.encode_reg_angle)),
        (
            "ReadApproxd1",
            AsciiString::from_integer(p.read_approxd1 as i32),
        ),
    ]
}
fn read_visibility(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadFaultyEntities",
            AsciiString::from_integer(p.read_faulty_entities as i32),
        ),
        (
            "ReadOnlyVisible",
            AsciiString::from_integer(p.read_only_visible as i32),
        ),
        ("ReadColor", AsciiString::from_integer(p.read_color as i32)),
    ]
}
fn read_names_and_brep(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        ("ReadName", AsciiString::from_integer(p.read_name as i32)),
        ("ReadLayer", AsciiString::from_integer(p.read_layer as i32)),
        (
            "WriteBRepMode",
            AsciiString::from_integer(p.write_b_rep_mode.as_raw()),
        ),
    ]
}
fn write_surface_and_headers(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteConvertSurfaceMode",
            AsciiString::from_integer(p.write_convert_surface_mode.as_raw()),
        ),
        ("WriteHeaderAuthor", p.write_header_author.clone()),
        ("WriteHeaderCompany", p.write_header_company.clone()),
    ]
}
fn write_headers_and_precision(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        ("WriteHeaderProduct", p.write_header_product.clone()),
        ("WriteHeaderReciever", p.write_header_reciever.clone()),
        (
            "WritePrecisionMode",
            AsciiString::from_integer(p.write_precision_mode.as_raw()),
        ),
    ]
}
fn write_precision_and_plane(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WritePrecisionVal",
            AsciiString::from_real(p.write_precision_val),
        ),
        (
            "WritePlaneMode",
            AsciiString::from_integer(p.write_plane_mode.as_raw()),
        ),
        (
            "WriteOffsetMode",
            AsciiString::from_integer(p.write_offset_mode as i32),
        ),
    ]
}
fn write_attributes(p: &DeIgesParameters) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteColor",
            AsciiString::from_integer(p.write_color as i32),
        ),
        ("WriteName", AsciiString::from_integer(p.write_name as i32)),
        (
            "WriteLayer",
            AsciiString::from_integer(p.write_layer as i32),
        ),
    ]
}
