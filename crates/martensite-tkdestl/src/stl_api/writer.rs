//! `StlApiWriter`.
use crate::rw_stl::{OStreamWriter, RwStl, open_output_file};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    message::{Message, ProgressRange},
    osd::IosOpenMode,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::poly::{Triangle, Triangulation, mesh_purpose};
use std::io::Write;

/// Writes the triangulations of a shape's faces; nothing is meshed here.
#[derive(Clone, Copy, Debug)]
pub struct StlApiWriter {
    ascii_mode: bool,
}
impl Default for StlApiWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl StlApiWriter {
    /// Default parameters: text mode; `StlApiWriter::new()`.
    pub const fn new() -> Self {
        Self { ascii_mode: true }
    }
    /// `ASCIIMode`: the flag to read or change; true writes text, false binary;
    /// `*writer.ascii_mode() = false`.
    pub const fn ascii_mode(&mut self) -> &mut bool {
        &mut self.ascii_mode
    }
    /// False when the file cannot be opened or the
    /// shape has no triangulation;
    /// `writer.write_file(&shape, &"part.stl".into(), &ProgressRange::default())?`.
    pub fn write_file(
        &self,
        shape: &Shape,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let mode = if self.ascii_mode {
            IosOpenMode::NONE
        } else {
            IosOpenMode::BINARY
        };
        let Some(mut stream) = open_output_file(file, mode) else {
            return Ok(false);
        };
        self.write_stream(shape, &mut OStreamWriter(&mut stream), progress)
    }
    /// Writes one mesh made of every face's triangles;
    /// `writer.write_stream(&shape, &mut bytes, &ProgressRange::default())?`.
    pub fn write_stream(
        &self,
        shape: &Shape,
        stream: &mut dyn Write,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let faces = FaceMeshes::of_shape(shape)?;
        if faces.nb_triangles == 0 {
            return Ok(false);
        }
        let mesh = Handle::new(faces.merged_mesh()?);
        let is_done = if self.ascii_mode {
            RwStl::write_ascii(&mesh, stream, progress)?
        } else {
            RwStl::write_binary(&mesh, stream, progress)?
        };
        if is_done && faces.nb_faces_without_mesh > 0 {
            send_skipped_faces_warning(faces.nb_faces_without_mesh);
        }
        Ok(is_done)
    }
}

/// One face to copy: its mesh, location transform and orientation.
struct FaceMesh {
    mesh: Handle<Triangulation>,
    transform: martensite_tkmath::gp::Transform3,
    is_reversed: bool,
}

/// The triangulated faces of a shape in explorer order, with the totals to size the mesh.
struct FaceMeshes {
    faces: Vec<FaceMesh>,
    nb_nodes: i32,
    nb_triangles: i32,
    nb_faces_without_mesh: i32,
}
impl FaceMeshes {
    fn of_shape(shape: &Shape) -> Result<Self, Failure> {
        let mut result = Self {
            faces: Vec::new(),
            nb_nodes: 0,
            nb_triangles: 0,
            nb_faces_without_mesh: 0,
        };
        let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
        while explorer.more() {
            result.add_face(explorer.current())?;
            explorer.next()?;
        }
        Ok(result)
    }
    fn add_face(&mut self, shape: &Shape) -> Result<(), Failure> {
        let (mesh, location) = BRepTool::triangulation(&topods::face(shape)?, mesh_purpose::NONE)?;
        let Some(data) = mesh.get() else {
            self.nb_faces_without_mesh += 1;
            return Ok(());
        };
        self.nb_nodes += data.nb_nodes();
        self.nb_triangles += data.nb_triangles();
        self.faces.push(FaceMesh {
            mesh: mesh.clone(),
            transform: location.transformation(),
            is_reversed: shape.orientation() == Orientation::Reversed,
        });
        Ok(())
    }
    fn merged_mesh(&self) -> Result<Triangulation, Failure> {
        let merged = Triangulation::with_sizes(self.nb_nodes, self.nb_triangles, false, false)?;
        let (mut node_offset, mut triangle_offset) = (0, 0);
        for face in &self.faces {
            let Some(mesh) = face.mesh.get() else {
                continue;
            };
            copy_face_nodes(&merged, mesh, face, node_offset)?;
            copy_face_triangles(&merged, mesh, face, node_offset, triangle_offset)?;
            node_offset += mesh.nb_nodes();
            triangle_offset += mesh.nb_triangles();
        }
        Ok(merged)
    }
}

fn copy_face_nodes(
    merged: &Triangulation,
    mesh: &Triangulation,
    face: &FaceMesh,
    node_offset: i32,
) -> Result<(), Failure> {
    for index in 1..=mesh.nb_nodes() {
        merged.set_node(
            index + node_offset,
            mesh.node(index)?.transformed(face.transform),
        )?;
    }
    Ok(())
}

fn copy_face_triangles(
    merged: &Triangulation,
    mesh: &Triangulation,
    face: &FaceMesh,
    node_offset: i32,
    triangle_offset: i32,
) -> Result<(), Failure> {
    for index in 1..=mesh.nb_triangles() {
        let (first, mut second, mut third) = mesh.triangle(index)?.get();
        if face.is_reversed {
            std::mem::swap(&mut second, &mut third);
        }
        let shifted = Triangle::new(
            first + node_offset,
            second + node_offset,
            third + node_offset,
        );
        merged.set_triangle(index + triangle_offset, shifted)?;
    }
    Ok(())
}

fn send_skipped_faces_warning(nb_faces: i32) {
    let noun = if nb_faces == 1 {
        " face has"
    } else {
        " faces have"
    };
    Message::send_warning_text(&AsciiString::new(&format!(
        "Warning: {nb_faces}{noun} been skipped due to null triangulation"
    )));
}
