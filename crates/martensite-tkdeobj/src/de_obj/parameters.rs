use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;

/// Values controlling OBJ transfers; `RwObjInternalSection::default()`.
#[derive(Clone, Debug, PartialEq)]
pub struct RwObjInternalSection {
    /// `FileLengthUnit` transfer setting.
    pub file_length_unit: f64,
    /// `SystemCS` transfer setting.
    pub system_cs: RwMeshCoordinateSystem,
    /// `FileCS` transfer setting.
    pub file_cs: RwMeshCoordinateSystem,
    /// `ReadSinglePrecision` transfer setting.
    pub read_single_precision: bool,
    /// `ReadCreateShapes` transfer setting.
    pub read_create_shapes: bool,
    /// `ReadRootPrefix` transfer setting.
    pub read_root_prefix: AsciiString,
    /// `ReadFillDoc` transfer setting.
    pub read_fill_doc: bool,
    /// `ReadFillIncomplete` transfer setting.
    pub read_fill_incomplete: bool,
    /// `ReadMemoryLimitMiB` transfer setting.
    pub read_memory_limit_mib: i32,
    /// `WriteComment` transfer setting.
    pub write_comment: AsciiString,
    /// `WriteAuthor` transfer setting.
    pub write_author: AsciiString,
}
impl Default for RwObjInternalSection {
    fn default() -> Self {
        Self {
            file_length_unit: 1.0,
            system_cs: RwMeshCoordinateSystem::Z_UP,
            file_cs: RwMeshCoordinateSystem::Y_UP,
            read_single_precision: false,
            read_create_shapes: false,
            read_root_prefix: AsciiString::default(),
            read_fill_doc: true,
            read_fill_incomplete: true,
            read_memory_limit_mib: -1,
            write_comment: AsciiString::default(),
            write_author: AsciiString::default(),
        }
    }
}
