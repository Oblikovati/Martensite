//! `StlApiReader`.
use crate::rw_stl::{DEFAULT_MERGE_ANGLE, RwStl};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::ProgressRange,
    osd::IStream,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkmath::poly::Triangulation;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeOnMesh,
};

/// `StlApiReader`: reads STL into a shape with one face per triangle.
#[derive(Clone, Copy, Debug, Default)]
pub struct StlApiReader;
impl StlApiReader {
    /// True when `shape` received the result;
    /// `StlApiReader.read_file(&mut shape, &"part.stl".into())?`.
    pub fn read_file(&self, shape: &mut Shape, file: &AsciiString) -> Result<bool, Failure> {
        let mesh = RwStl::read_file(file, &ProgressRange::default())?;
        build_faces(&mesh, shape)
    }
    /// `StlApiReader.read_stream(&mut shape, &mut stream)?`.
    pub fn read_stream(
        &self,
        shape: &mut Shape,
        stream: &mut dyn IStream,
    ) -> Result<bool, Failure> {
        let mesh = RwStl::read_stream(stream, DEFAULT_MERGE_ANGLE, &ProgressRange::default())?;
        build_faces(&mesh, shape)
    }
}

fn build_faces(mesh: &Handle<Triangulation>, shape: &mut Shape) -> Result<bool, Failure> {
    if mesh.is_null() {
        return Ok(false);
    }
    let mut converter = BRepBuilderAPIMakeShapeOnMesh::new(mesh);
    converter.build(&ProgressRange::default())?;
    if !converter.is_done() {
        return Ok(false);
    }
    let result = converter.shape()?;
    if result.is_null() {
        return Ok(false);
    }
    *shape = result.clone();
    Ok(true)
}
