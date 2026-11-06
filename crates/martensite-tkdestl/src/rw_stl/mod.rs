//! Triangulation reading and writing, with caller-owned streams.
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::{Failure, Handle},
};
use martensite_tkmath::{
    gp::{Point3, Vector3},
    poly::Triangulation,
};
use std::io::Write;
mod ascii;
mod collector;
mod merge_tool;
mod read;
mod reader;
mod reader_ascii;
mod reader_binary;
mod reader_domains;
mod write_path;
use read::read_binary_mode;
pub use reader::{DEFAULT_MERGE_ANGLE, RwStlReader, RwStlReaderBase};
pub use write_path::OStreamWriter;
pub(crate) use write_path::open_output_file;
const CHUNK_FACETS: usize = 4096;
const FACET_BYTES: usize = 50;
// TODO(target_endian = "big"): characterize the host-byte conversion helpers; Linux writes little endian.

/// STL mesh operations; `RwStl::write_binary(&mesh, &mut bytes, &ProgressRange::default())?`.
pub struct RwStl;
impl RwStl {
    /// Writes binary facets and polls cancellation every thousand triangles.
    /// Null/empty meshes and stream errors return false; invalid indices retain their geometry failure.
    /// `RwStl::write_binary(&mesh, &mut Vec::new(), &ProgressRange::default())?`.
    pub fn write_binary(
        mesh: &Handle<Triangulation>,
        stream: &mut dyn Write,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(mesh) = mesh.get() else {
            return Ok(false);
        };
        if mesh.nb_triangles() <= 0 {
            return Ok(false);
        }
        if !write_header(stream) {
            return Ok(false);
        }
        // Obtains its loop/count snapshot after the synchronous header write.
        let count = mesh.nb_triangles();
        if !write_block(stream, &count.to_le_bytes()) {
            return Ok(false);
        }
        let scope = ProgressScope::with_literal(progress, "Triangles", f64::from(count), false);
        write_facets(mesh, stream, count, &scope)
    }
}
fn write_header(stream: &mut dyn Write) -> bool {
    let mut header = [0; 80];
    let label = b"STL Exported by Martensite";
    header[..label.len()].copy_from_slice(label);
    write_block(stream, &header)
}
fn write_block(stream: &mut dyn Write, bytes: &[u8]) -> bool {
    // An ostream short write sets failure; retrying here would change that observable contract.
    stream
        .write(bytes)
        .is_ok_and(|written| written == bytes.len())
}
fn write_facets(
    mesh: &Triangulation,
    stream: &mut dyn Write,
    count: i32,
    scope: &ProgressScope<'_>,
) -> Result<bool, Failure> {
    let mut chunk = Vec::with_capacity(CHUNK_FACETS * FACET_BYTES);
    for index in 1..=count {
        chunk.extend_from_slice(&facet_bytes(mesh, index)?);
        if chunk.len() == CHUNK_FACETS * FACET_BYTES {
            if !write_block(stream, &chunk) {
                return Ok(false);
            }
            chunk.clear();
        }
        if !advance_progress(scope, index) {
            return Ok(false);
        }
    }
    // Code attempts the final zero-length write even at an exact chunk boundary.
    Ok(write_block(stream, &chunk))
}
fn advance_progress(scope: &ProgressScope<'_>, triangle: i32) -> bool {
    if triangle % 1000 != 0 {
        return true;
    }
    if !scope.more() {
        return false;
    }
    drop(scope.next(1000.));
    true
}
fn facet_points(mesh: &Triangulation, index: i32) -> Result<[Point3; 3], Failure> {
    let (first, second, third) = mesh.triangle(index)?.get();
    Ok([mesh.node(first)?, mesh.node(second)?, mesh.node(third)?])
}
fn facet_normal(points: [Point3; 3]) -> Result<Vector3, Failure> {
    let mut normal =
        Vector3::between(points[0], points[1]).crossed(Vector3::between(points[0], points[2]));
    if normal.square_magnitude() > f64::MIN_POSITIVE {
        normal.normalize()?;
    } else {
        normal.set_coordinates(0., 0., 0.);
    }
    Ok(normal)
}
fn facet_coordinates(mesh: &Triangulation, index: i32) -> Result<[f64; 12], Failure> {
    let points = facet_points(mesh, index)?;
    let normal = facet_normal(points)?;
    Ok([
        normal.x(),
        normal.y(),
        normal.z(),
        points[0].x(),
        points[0].y(),
        points[0].z(),
        points[1].x(),
        points[1].y(),
        points[1].z(),
        points[2].x(),
        points[2].y(),
        points[2].z(),
    ])
}
#[allow(clippy::cast_possible_truncation)] // Binary STL stores each coordinate as float32.
fn facet_bytes(mesh: &Triangulation, index: i32) -> Result<[u8; FACET_BYTES], Failure> {
    let coordinates = facet_coordinates(mesh, index)?;
    let mut facet = [0; FACET_BYTES];
    for (index, value) in coordinates.into_iter().enumerate() {
        facet[index * 4..index * 4 + 4].copy_from_slice(&(value as f32).to_le_bytes());
    }
    Ok(facet)
}
