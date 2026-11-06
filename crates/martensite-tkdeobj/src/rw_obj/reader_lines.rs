//! The line loop of `RwObjReader` and its statements.
use super::polygon::triangulate_polygon;
use super::reader_statements::{
    check_memory, push_group, push_material, push_object, push_smooth_group, read_material_lib,
};
use super::tools::{c_string, strtol};
use super::{RwObjReader, RwObjSubMesh, RwObjSubMeshReason, RwObjTools};
use martensite_tkernel::message::{Message, ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{Vec2, Vec3};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::character::is_space;
use martensite_tkernel::standard::{Failure, ReadLineBuffer, strtod_bytes};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkrwmesh::rw_mesh::RwMeshInputStream;
use std::io::SeekFrom;
use std::mem::size_of;
use std::time::Instant;

const BUFFER_SIZE: usize = 4 * 1024;
const MIB: i64 = 1024 * 1024;
const INITIAL_ELEMENT_SIZE: usize = 1024;

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

fn send_warning(text: &str) {
    Message::send_warning_text(&AsciiString::new(text));
}

fn file_text(file: &AsciiString) -> String {
    String::from_utf8_lossy(file.as_bytes()).into_owned()
}

fn reset<R: RwObjReader + ?Sized>(reader: &mut R, file: &AsciiString) {
    let base = reader.obj_base_mut();
    base.mem_estim = 0;
    base.nb_lines = 0;
    base.nb_probe_nodes = 0;
    base.nb_probe_elems = 0;
    base.nb_elems_big = 0;
    base.to_abort = false;
    base.obj_verts.reset();
    base.obj_verts_uv.clear();
    base.obj_norms.clear();
    base.packed_indices.clear();
    base.materials.clear(false);
    base.file_comments = AsciiString::default();
    base.external_files.clear(false);
    base.active_sub_mesh = RwObjSubMesh::default();
    base.folder = CadPath::folder_and_file_from_path(file.as_bytes()).0;
    base.curr_elem = vec![-1; INITIAL_ELEMENT_SIZE];
}

/// The length of a stream, zero for a stream that cannot seek.
fn stream_length(stream: &mut dyn RwMeshInputStream) -> i64 {
    let length = stream.seek(SeekFrom::End(0)).unwrap_or(0);
    if stream.seek(SeekFrom::Start(0)).is_err() {
        return 0;
    }
    length as i64
}

/// Reads or probes a stream; see [`RwObjReader::read_stream`].
pub(super) fn read_all<R: RwObjReader + ?Sized>(
    reader: &mut R,
    stream: &mut dyn RwMeshInputStream,
    file: &AsciiString,
    progress: &ProgressRange<'_>,
    to_probe: bool,
) -> Result<bool, Failure> {
    reset(reader, file);
    let file_length = stream_length(stream);
    if file_length <= 0 {
        send_fail(&format!("Error: file '{}' is empty", file_text(file)));
        return Ok(false);
    }
    let mut buffer = ReadLineBuffer::new(BUFFER_SIZE);
    buffer.set_multiline_mode(true, true);
    let scope = ProgressScope::with_literal(
        progress,
        "Reading text OBJ file",
        (file_length / MIB) as f64,
        false,
    );
    let mut lines = LineLoop {
        is_start: true,
        position: 0,
        passed_mib: 0,
        timer: Instant::now(),
    };
    loop {
        let mut read_bytes = 0;
        let Some(line) = buffer.read_line_counted(&mut &mut *stream, &mut read_bytes) else {
            break;
        };
        let line = c_string(line);
        reader.obj_base_mut().nb_lines += 1;
        if !lines.advance(read_bytes, &scope) {
            return Ok(false);
        }
        if !lines.read_comment(reader, line) && !to_probe {
            read_statement(reader, line)?;
            if !check_memory(reader) {
                let active = reader.obj_base().active_sub_mesh.clone();
                reader.add_mesh(&active, RwObjSubMeshReason::NewObject)?;
                return Ok(false);
            }
        } else if to_probe && !line.is_empty() && line[0] != b'#' {
            probe_statement(reader, line);
        }
    }
    finish(reader, to_probe)?;
    Ok(true)
}

/// What the line loop keeps between lines.
struct LineLoop {
    is_start: bool,
    position: i64,
    passed_mib: i64,
    timer: Instant,
}

impl LineLoop {
    /// Counts bytes and reports progress once a second; false on a user break.
    fn advance(&mut self, read_bytes: i64, scope: &ProgressScope<'_>) -> bool {
        self.position += read_bytes;
        if self.timer.elapsed().as_secs_f64() <= 1.0 {
            return true;
        }
        if !scope.more() {
            return false;
        }
        let read_mib = self.position / MIB;
        scope.next((read_mib - self.passed_mib) as f64);
        self.passed_mib = read_mib;
        self.timer = Instant::now();
        true
    }
    /// Consumes a comment or an empty line, keeping the head comments of the file.
    fn read_comment<R: RwObjReader + ?Sized>(&mut self, reader: &mut R, line: &[u8]) -> bool {
        match line.first() {
            Some(b'#') => {
                if self.is_start {
                    append_comment(reader, &line[1..]);
                }
                true
            }
            None | Some(b'\n') => true,
            _ => {
                self.is_start = false;
                false
            }
        }
    }
}

fn append_comment<R: RwObjReader + ?Sized>(reader: &mut R, text: &[u8]) {
    let mut comment = AsciiString::from_raw(text);
    comment.left_adjust();
    comment.right_adjust();
    if comment.is_empty() {
        return;
    }
    let base = reader.obj_base_mut();
    if !base.file_comments.is_empty() {
        base.file_comments.append(&AsciiString::new("\n"));
    }
    base.file_comments.append(&comment);
}

/// The byte after a keyword, a NUL at the end of the line.
fn at(line: &[u8], index: usize) -> u8 {
    line.get(index).copied().unwrap_or(0)
}

fn rest(line: &[u8], index: usize) -> &[u8] {
    line.get(index..).unwrap_or(&[])
}

fn probe_statement<R: RwObjReader + ?Sized>(reader: &mut R, line: &[u8]) {
    if line.starts_with(b"mtllib") {
        read_material_lib(reader, material_argument(line));
    } else if at(line, 0) == b'v' && RwObjTools::is_space_char(at(line, 1)) {
        reader.obj_base_mut().nb_probe_nodes += 1;
    } else if at(line, 0) == b'f' && RwObjTools::is_space_char(at(line, 1)) {
        reader.obj_base_mut().nb_probe_elems += 1;
    }
}

fn material_argument(line: &[u8]) -> &[u8] {
    if is_space(at(line, 6)) {
        rest(line, 7)
    } else {
        &[]
    }
}

fn read_statement<R: RwObjReader + ?Sized>(reader: &mut R, line: &[u8]) -> Result<(), Failure> {
    let (first, second) = (at(line, 0), at(line, 1));
    if first == b'v' && RwObjTools::is_space_char(second) {
        reader.obj_base_mut().nb_probe_nodes += 1;
        push_vertex(reader, rest(line, 2));
    } else if first == b'v' && second == b'n' && RwObjTools::is_space_char(at(line, 2)) {
        push_normal(reader, rest(line, 3));
    } else if first == b'v' && second == b't' && RwObjTools::is_space_char(at(line, 2)) {
        push_texel(reader, rest(line, 3));
    } else if first == b'f' && RwObjTools::is_space_char(second) {
        reader.obj_base_mut().nb_probe_elems += 1;
        push_indices(reader, rest(line, 2))?;
    } else if first == b'g' && is_space(second) {
        push_group(reader, rest(line, 2))?;
    } else if first == b's' && is_space(second) {
        push_smooth_group(reader, rest(line, 2))?;
    } else if first == b'o' && is_space(second) {
        push_object(reader, rest(line, 2))?;
    } else if line.starts_with(b"mtllib") {
        read_material_lib(reader, material_argument(line));
    } else if line.starts_with(b"usemtl") {
        push_material(reader, material_argument(line))?;
    }
    Ok(())
}

fn finish<R: RwObjReader + ?Sized>(reader: &mut R, to_probe: bool) -> Result<(), Failure> {
    collect_external_files(reader);
    if !to_probe {
        let active = reader.obj_base().active_sub_mesh.clone();
        reader.add_mesh(&active, RwObjSubMeshReason::NewObject)?;
    }
    let big = reader.obj_base().nb_elems_big;
    if big != 0 {
        send_warning(&format!(
            "Warning: OBJ reader, {big} polygon(s) have been split into triangles"
        ));
    }
    Ok(())
}

fn collect_external_files<R: RwObjReader + ?Sized>(reader: &mut R) {
    let base = reader.obj_base_mut();
    let textures: Vec<AsciiString> = base
        .materials
        .values()
        .flat_map(|m| [&m.diffuse_texture, &m.specular_texture, &m.bump_texture])
        .filter(|path| !path.is_empty())
        .cloned()
        .collect();
    for texture in textures {
        base.external_files.add(texture);
    }
}

fn push_vertex<R: RwObjReader + ?Sized>(reader: &mut R, text: &[u8]) {
    let mut xyz = Xyz::new(0.0, 0.0, 0.0);
    RwObjTools::read_vec3(text, &mut xyz);
    let base = reader.obj_base_mut();
    base.cs_trsf.transform_position(&mut xyz);
    base.mem_estim += if base.obj_verts.is_single_precision() {
        size_of::<Vec3<f32>>()
    } else {
        size_of::<Point3>()
    };
    base.obj_verts
        .append(Point3::new(xyz.x(), xyz.y(), xyz.z()));
}

fn push_normal<R: RwObjReader + ?Sized>(reader: &mut R, text: &[u8]) {
    let mut normal = Vec3::new(0.0_f32, 0.0, 0.0);
    RwObjTools::read_vec3_f32(text, &mut normal);
    let base = reader.obj_base_mut();
    base.cs_trsf.transform_normal(&mut normal);
    base.mem_estim += size_of::<Vec3<f32>>();
    base.obj_norms.push(normal);
}

fn push_texel<R: RwObjReader + ?Sized>(reader: &mut R, text: &[u8]) {
    let (u, used) = strtod_bytes(text);
    let (v, _) = strtod_bytes(&text[used..]);
    let base = reader.obj_base_mut();
    base.mem_estim += size_of::<Vec2<f32>>();
    base.obj_verts_uv.push(Vec2::new(u as f32, v as f32));
}

/// Parses `position[/uv[/normal]]`, the indices zero based; `None` when no number starts here.
fn parse_corner(line: &[u8], pos: &mut usize) -> Option<[i32; 3]> {
    let mut indices = [-1; 3];
    let (position, used) = strtol(rest(line, *pos));
    if used == 0 {
        return None;
    }
    indices[0] = position.wrapping_sub(1);
    *pos += used;
    if at(line, *pos) != b'/' {
        return Some(indices);
    }
    *pos += 1;
    if at(line, *pos) != b'/' {
        let (uv, used) = strtol(rest(line, *pos));
        indices[1] = uv.wrapping_sub(1);
        *pos += used;
    }
    if at(line, *pos) == b'/' {
        *pos += 1;
        if !is_space(at(line, *pos)) {
            let (normal, used) = strtol(rest(line, *pos));
            indices[2] = normal.wrapping_sub(1);
            *pos += used;
        }
    }
    Some(indices)
}

/// Negative indices count back from the last item read.
fn resolve_negative<R: RwObjReader + ?Sized>(reader: &R, indices: &mut [i32; 3]) {
    let base = reader.obj_base();
    let uppers = [
        base.obj_verts.upper(),
        base.obj_verts_uv.len() as i32 - 1,
        base.obj_norms.len() as i32 - 1,
    ];
    for (index, upper) in indices.iter_mut().zip(uppers) {
        if *index < -1 {
            *index += upper + 2;
        }
    }
}

/// The node of a corner: a known set of indices, or a new node. `None` aborts the element.
fn corner_node<R: RwObjReader + ?Sized>(reader: &mut R, indices: [i32; 3]) -> Option<i32> {
    if let Some(node) = reader.obj_base().packed_indices.get(&indices) {
        return Some(*node);
    }
    let base = reader.obj_base_mut();
    base.mem_estim += usize::from(indices[0] >= 0) * size_of::<Vec3<f32>>()
        + usize::from(indices[1] >= 0) * size_of::<Vec2<f32>>()
        + usize::from(indices[2] >= 0) * size_of::<Vec3<f32>>()
        + 4 * size_of::<i32>()
        + size_of::<i32>();
    if indices[0] < base.obj_verts.lower() || indices[0] > base.obj_verts.upper() {
        base.to_abort = true;
        let lines = base.nb_lines;
        send_fail(&format!(
            "Error: invalid OBJ syntax at line {lines}: vertex index is out of range"
        ));
        return None;
    }
    let point = base.obj_verts.value(indices[0]);
    let node = reader.add_node(point);
    reader.obj_base_mut().packed_indices.insert(indices, node);
    set_corner_uv(reader, node, indices[1]);
    set_corner_normal(reader, node, indices[2]);
    Some(node)
}

fn set_corner_uv<R: RwObjReader + ?Sized>(reader: &mut R, node: i32, index: i32) {
    if index < 0 {
        return;
    }
    let base = reader.obj_base();
    let lines = base.nb_lines;
    if base.obj_verts_uv.is_empty() {
        send_warning(&format!(
            "Warning: invalid OBJ syntax at line {lines}: UV index is specified but no UV nodes are defined"
        ));
        return;
    }
    let Some(uv) = base.obj_verts_uv.get(index as usize).copied() else {
        send_warning(&format!(
            "Warning: invalid OBJ syntax at line {lines}: UV index is out of range"
        ));
        reader.set_node_uv(node, Vec2::new(0.0, 0.0));
        return;
    };
    reader.set_node_uv(node, uv);
}

fn set_corner_normal<R: RwObjReader + ?Sized>(reader: &mut R, node: i32, index: i32) {
    if index < 0 {
        return;
    }
    let base = reader.obj_base();
    let lines = base.nb_lines;
    if base.obj_norms.is_empty() {
        send_warning(&format!(
            "Warning: invalid OBJ syntax at line {lines}: Normal index is specified but no Normals nodes are defined"
        ));
        return;
    }
    let Some(normal) = base.obj_norms.get(index as usize).copied() else {
        send_warning(&format!(
            "Warning: invalid OBJ syntax at line {lines}: Normal index is out of range"
        ));
        reader.set_node_normal(node, Vec3::new(0.0, 0.0, 1.0));
        return;
    };
    reader.set_node_normal(node, normal);
}

/// Handles `f indices`: a triangle, a quad, or a polygon split into triangles.
fn push_indices<R: RwObjReader + ?Sized>(reader: &mut R, line: &[u8]) -> Result<(), Failure> {
    let mut pos = 0;
    let mut count = 0;
    while let Some(mut indices) = parse_corner(line, &mut pos) {
        resolve_negative(reader, &mut indices);
        let Some(node) = corner_node(reader, indices) else {
            return Ok(());
        };
        let current = &mut reader.obj_base_mut().curr_elem;
        if current.len() <= count {
            current.resize(count * 2, -1);
        }
        current[count] = node;
        count += 1;
        if matches!(at(line, pos), b'\n' | 0) {
            break;
        }
        if at(line, pos) != b' ' {
            pos += 1;
        }
    }
    add_element_of(reader, count)
}

fn add_element_of<R: RwObjReader + ?Sized>(reader: &mut R, count: usize) -> Result<(), Failure> {
    let nodes: Vec<i32> = reader.obj_base().curr_elem[..count.max(3)].to_vec();
    if nodes[..3].iter().any(|node| *node < 0) || count < 3 {
        return Ok(());
    }
    let element_size = 4 * size_of::<i32>();
    if count <= 4 {
        reader.obj_base_mut().mem_estim += element_size;
        let last = if count == 4 { nodes[3] } else { -1 };
        reader.add_element(nodes[0], nodes[1], nodes[2], last);
        return Ok(());
    }
    let added = triangulate_polygon(reader, &nodes)?;
    if added >= 1 {
        let base = reader.obj_base_mut();
        base.nb_elems_big += 1;
        base.mem_estim += element_size * added as usize;
    }
    Ok(())
}
