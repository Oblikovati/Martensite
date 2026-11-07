use super::RwObjInternalSection;
use martensite_tkde::de::render_configuration;
use martensite_tkernel::tcollection::AsciiString;

pub(super) fn save(parameters: &RwObjInternalSection) -> AsciiString {
    let substitutions = [
        common(parameters).as_slice(),
        read_shapes(parameters).as_slice(),
        read_document(parameters).as_slice(),
        write(parameters).as_slice(),
    ]
    .concat();
    render_configuration(include_str!("configuration.template"), &substitutions)
        .expect("fixed OBJ template has exactly one placeholder per setting")
}
fn common(parameters: &RwObjInternalSection) -> [(&'static str, AsciiString); 3] {
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
fn read_shapes(parameters: &RwObjInternalSection) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadSinglePrecision",
            AsciiString::from_integer(parameters.read_single_precision as i32),
        ),
        (
            "ReadCreateShapes",
            AsciiString::from_integer(parameters.read_create_shapes as i32),
        ),
        ("ReadRootPrefix", parameters.read_root_prefix.clone()),
    ]
}
fn read_document(parameters: &RwObjInternalSection) -> [(&'static str, AsciiString); 3] {
    [
        (
            "ReadFillDoc",
            AsciiString::from_integer(parameters.read_fill_doc as i32),
        ),
        (
            "ReadFillIncomplete",
            AsciiString::from_integer(parameters.read_fill_incomplete as i32),
        ),
        (
            "ReadMemoryLimitMiB",
            AsciiString::from_integer(parameters.read_memory_limit_mib),
        ),
    ]
}
fn write(parameters: &RwObjInternalSection) -> [(&'static str, AsciiString); 2] {
    [
        ("WriteComment", parameters.write_comment.clone()),
        ("WriteAuthor", parameters.write_author.clone()),
    ]
}
