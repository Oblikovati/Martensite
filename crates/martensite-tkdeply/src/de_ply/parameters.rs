use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;

/// Values controlling PLY transfers; `RwPlyInternalSection::default()`.
#[derive(Clone, Debug, PartialEq)]
pub struct RwPlyInternalSection {
    /// `FileLengthUnit` transfer setting.
    pub file_length_unit: f64,
    /// `SystemCS` transfer setting.
    pub system_cs: RwMeshCoordinateSystem,
    /// `FileCS` transfer setting.
    pub file_cs: RwMeshCoordinateSystem,
    /// `WriteNormals` transfer setting.
    pub write_normals: bool,
    /// `WriteColors` transfer setting.
    pub write_colors: bool,
    /// `WriteTexCoords` transfer setting.
    pub write_tex_coords: bool,
    /// `WritePartId` transfer setting.
    pub write_part_id: bool,
    /// `WriteFaceId` transfer setting.
    pub write_face_id: bool,
    /// `WriteComment` transfer setting.
    pub write_comment: AsciiString,
    /// `WriteAuthor` transfer setting.
    pub write_author: AsciiString,
}
impl Default for RwPlyInternalSection {
    fn default() -> Self {
        Self {
            file_length_unit: 1.0,
            system_cs: RwMeshCoordinateSystem::Z_UP,
            file_cs: RwMeshCoordinateSystem::Y_UP,
            write_normals: true,
            write_colors: true,
            write_tex_coords: false,
            write_part_id: true,
            write_face_id: false,
            write_comment: AsciiString::default(),
            write_author: AsciiString::default(),
        }
    }
}
