use super::RwPlyInternalSection;
use martensite_tkde::de::render_configuration;
use martensite_tkernel::tcollection::AsciiString;

pub(super) fn save(parameters: &RwPlyInternalSection) -> AsciiString {
    let substitutions = [
        common(parameters).as_slice(),
        flags(parameters).as_slice(),
        identifiers(parameters).as_slice(),
        text(parameters).as_slice(),
    ]
    .concat();
    render_configuration(include_str!("configuration.template"), &substitutions)
        .expect("fixed PLY template has one placeholder per setting")
}
fn common(parameters: &RwPlyInternalSection) -> [(&'static str, AsciiString); 3] {
    [
        (
            "FileLengthUnit",
            AsciiString::from_real(parameters.file_length_unit),
        ),
        (
            "SystemCS",
            AsciiString::from_integer(parameters.system_cs.as_raw()),
        ),
        (
            "FileCS",
            AsciiString::from_integer(parameters.file_cs.as_raw()),
        ),
    ]
}
fn flags(parameters: &RwPlyInternalSection) -> [(&'static str, AsciiString); 3] {
    [
        (
            "WriteNormals",
            AsciiString::from_integer(parameters.write_normals as i32),
        ),
        (
            "WriteColors",
            AsciiString::from_integer(parameters.write_colors as i32),
        ),
        (
            "WriteTexCoords",
            AsciiString::from_integer(parameters.write_tex_coords as i32),
        ),
    ]
}
fn identifiers(parameters: &RwPlyInternalSection) -> [(&'static str, AsciiString); 2] {
    [
        (
            "WritePartId",
            AsciiString::from_integer(parameters.write_part_id as i32),
        ),
        (
            "WriteFaceId",
            AsciiString::from_integer(parameters.write_face_id as i32),
        ),
    ]
}
fn text(parameters: &RwPlyInternalSection) -> [(&'static str, AsciiString); 2] {
    [
        ("WriteComment", parameters.write_comment.clone()),
        ("WriteAuthor", parameters.write_author.clone()),
    ]
}
