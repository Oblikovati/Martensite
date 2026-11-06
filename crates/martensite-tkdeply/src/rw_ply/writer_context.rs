//! `RwPlyPlyWriterContext`: the low-level writing of an ASCII PLY file.
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::{IndexedDataMap, Vec2, Vec3, Vec4};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::format_general;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point3;
use std::fs::File;
use std::io::{BufWriter, Write};

/// The comment naming the writer, the first after the format line.
const EXPORT_COMMENT: &str = "comment Exported by Martensite\n";
/// Significant digits that bring any single-precision value back exactly, positions, normals and
/// texture coordinates alike: with fewer, large coordinates of a model round to a coarser grid
/// than the `float` the header declares can hold.
const SINGLE_DIGITS: usize = 9;
/// Significant digits that bring any double back exactly: with fewer, a position the header declares
/// as `double` would hold no more than a single-precision one.
const DOUBLE_DIGITS: usize = 17;

/// Where a context writes when it does not create a file itself.
pub type RwPlyOutput = Box<dyn Write + Send>;

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

fn path_text(path: &AsciiString) -> String {
    String::from_utf8_lossy(path.as_bytes()).into_owned()
}

/// Splits text into right-trimmed lines; an empty line between two breaks is no line. Repeated
/// lines and a last line of one character are kept: they are the text the caller asked to write.
fn split_lines(text: &AsciiString) -> Vec<AsciiString> {
    let mut lines = Vec::new();
    let bytes = text.as_bytes();
    let (mut from, mut index) = (0, 0);
    while index < bytes.len() {
        if matches!(bytes[index], b'\r' | b'\n') {
            add_line(&mut lines, &bytes[from..index]);
            if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
                index += 1;
            }
            from = index + 1;
        }
        index += 1;
    }
    add_line(&mut lines, &bytes[from..]);
    lines
}

fn add_line(lines: &mut Vec<AsciiString>, line: &[u8]) {
    if !line.is_empty() {
        let mut line = AsciiString::from_raw(line);
        line.right_adjust();
        lines.push(line);
    }
}

/// One entry of the file information as comment lines: the first line of the value follows the key.
fn information_text(key: &AsciiString, value: &AsciiString) -> Vec<u8> {
    let (key_lines, value_lines) = (split_lines(key), split_lines(value));
    let mut text = Vec::new();
    append_comments(&mut text, &key_lines);
    text.extend_from_slice(if key_lines.is_empty() {
        b"comment "
    } else {
        b":"
    });
    append_comments(&mut text, &value_lines);
    text.push(b'\n');
    text
}

fn append_comments(text: &mut Vec<u8>, lines: &[AsciiString]) {
    for (index, line) in lines.iter().enumerate() {
        text.extend_from_slice(if index > 0 {
            b"\ncomment "
        } else {
            b"comment "
        });
        text.extend_from_slice(line.as_bytes());
    }
}

/// Writes the header and records of an ASCII PLY file; vertices come before elements, and the header
/// states how many of each follow.
///
/// ```no_run
/// use martensite_tkdeply::rw_ply::RwPlyPlyWriterContext;
/// use martensite_tkernel::ncollection::{IndexedDataMap, Vec2, Vec3, Vec4};
/// use martensite_tkernel::tcollection::AsciiString;
/// use martensite_tkmath::gp::Point3;
/// let mut writer = RwPlyPlyWriterContext::new();
/// assert!(writer.open(&AsciiString::new("point.ply"), None));
/// assert!(writer.write_header(1, 0, &IndexedDataMap::default()));
/// let (normal, uv) = (Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0));
/// let color = Vec4::new(255, 255, 255, 255);
/// assert!(writer.write_vertex(&Point3::new(1.0, 2.0, 3.0), &normal, &uv, &color)?);
/// assert!(writer.close(false));
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct RwPlyPlyWriterContext {
    output: Option<RwPlyOutput>,
    is_good: bool,
    name: AsciiString,
    nb_header_verts: i32,
    nb_header_elems: i32,
    nb_verts: i32,
    nb_elems: i32,
    surf_id: i32,
    vert_offset: i32,
    is_double_prec: bool,
    has_normals: bool,
    has_colors: bool,
    has_tex_coords: bool,
    has_surf_id: bool,
}

impl RwPlyPlyWriterContext {
    /// A context with no file and no vertex or element attribute; `RwPlyPlyWriterContext::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// True when positions are written as doubles; `writer.is_double_precision()`.
    pub fn is_double_precision(&self) -> bool {
        self.is_double_prec
    }
    /// Sets whether positions are written as doubles; `writer.set_double_precision(true)`.
    pub fn set_double_precision(&mut self, is_double: bool) {
        self.is_double_prec = is_double;
    }
    /// True when normals are written; `writer.has_normals()`.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }
    /// Sets whether normals are written; `writer.set_normals(true)`.
    pub fn set_normals(&mut self, has_normals: bool) {
        self.has_normals = has_normals;
    }
    /// True when texture coordinates are written; `writer.has_tex_coords()`.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }
    /// Sets whether texture coordinates are written; `writer.set_tex_coords(true)`.
    pub fn set_tex_coords(&mut self, has_tex_coords: bool) {
        self.has_tex_coords = has_tex_coords;
    }
    /// True when point colors are written; `writer.has_colors()`.
    pub fn has_colors(&self) -> bool {
        self.has_colors
    }
    /// Sets whether point colors are written; `writer.set_colors(true)`.
    pub fn set_colors(&mut self, has_colors: bool) {
        self.has_colors = has_colors;
    }
    /// True when each element carries a surface id; `writer.has_surface_id()`.
    pub fn has_surface_id(&self) -> bool {
        self.has_surf_id
    }
    /// Sets whether each element carries a surface id; `writer.set_has_surface_id(true)`.
    pub fn set_has_surface_id(&mut self, has_surface_id: bool) {
        self.has_surf_id = has_surface_id;
    }
    /// True when a file is open; `writer.is_opened()`.
    pub fn is_opened(&self) -> bool {
        self.output.is_some()
    }
    /// Opens the output and forgets earlier counts: the given stream, else a new file of that name;
    /// false with a message when the file cannot be created; `writer.open(&name, None)`.
    pub fn open(&mut self, name: &AsciiString, stream: Option<RwPlyOutput>) -> bool {
        self.name = name.clone();
        (
            self.nb_header_verts,
            self.nb_header_elems,
            self.nb_verts,
            self.nb_elems,
        ) = (0, 0, 0, 0);
        self.output = stream.or_else(|| Self::created_file(name));
        self.is_good = self.output.is_some();
        if !self.is_good {
            send_fail(&format!(
                "Error: file cannot be created\n{}",
                path_text(name)
            ));
        }
        self.is_good
    }
    fn created_file(name: &AsciiString) -> Option<RwPlyOutput> {
        let file = File::create(path_text(name)).ok()?;
        Some(Box::new(BufWriter::new(file)))
    }
    /// Writes bytes unless the stream already failed, and keeps its state, as a stream's fail bit does.
    fn write_text(&mut self, text: &[u8]) -> bool {
        let Some(output) = self.output.as_mut() else {
            return false;
        };
        self.is_good = self.is_good && output.write_all(text).is_ok();
        self.is_good
    }
    /// Writes the header; `writer.write_header(8, 12, &information)`.
    pub fn write_header(
        &mut self,
        nb_nodes: i32,
        nb_elems: i32,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
    ) -> bool {
        if self.output.is_none() {
            return false;
        }
        self.nb_header_verts = nb_nodes;
        self.nb_header_elems = nb_elems;
        let mut text = format!("ply\nformat ascii 1.0\n{EXPORT_COMMENT}").into_bytes();
        for (key, value) in file_info.items() {
            text.extend_from_slice(&information_text(key, value));
        }
        text.extend_from_slice(self.properties_text(nb_nodes, nb_elems).as_bytes());
        self.write_text(&text)
    }
    fn properties_text(&self, nb_nodes: i32, nb_elems: i32) -> String {
        let kind = if self.is_double_prec {
            "double"
        } else {
            "float"
        };
        let mut text = format!(
            "element vertex {nb_nodes}\nproperty {kind} x\nproperty {kind} y\nproperty {kind} z\n"
        );
        if self.has_normals {
            text.push_str("property float nx\nproperty float ny\nproperty float nz\n");
        }
        if self.has_tex_coords {
            text.push_str("property float s\nproperty float t\n");
        }
        if self.has_colors {
            text.push_str("property uchar red\nproperty uchar green\nproperty uchar blue\n");
        }
        if nb_elems > 0 {
            text.push_str(&format!(
                "element face {nb_elems}\nproperty list uchar uint vertex_indices\n"
            ));
            if self.has_surf_id {
                text.push_str("property uint SurfaceID\n");
            }
        }
        text.push_str("end_header\n");
        text
    }
    fn position_text(&self, point: &Point3) -> String {
        let [x, y, z] = [point.x(), point.y(), point.z()].map(|value| match self.is_double_prec {
            true => format_general(value, DOUBLE_DIGITS),
            false => single_text(value as f32),
        });
        format!("{x} {y} {z}")
    }
    /// Writes a point with the attributes the header declares; a failure past the declared count;
    /// `writer.write_vertex(&point, &normal, &uv, &color)?`.
    pub fn write_vertex(
        &mut self,
        point: &Point3,
        normal: &Vec3<f32>,
        uv: &Vec2<f32>,
        color: &Vec4<u8>,
    ) -> Result<bool, Failure> {
        if self.output.is_none() {
            return Ok(false);
        }
        let mut line = self.position_text(point);
        if self.has_normals {
            let [x, y, z] = [normal.x, normal.y, normal.z].map(single_text);
            line.push_str(&format!(" {x} {y} {z}"));
        }
        if self.has_tex_coords {
            let [u, v] = [uv.x, uv.y].map(single_text);
            line.push_str(&format!(" {u} {v}"));
        }
        if self.has_colors {
            line.push_str(&format!(" {} {} {}", color.x, color.y, color.z));
        }
        line.push('\n');
        let is_written = self.write_text(line.as_bytes());
        self.nb_verts += 1;
        check_count("vertices", self.nb_verts, self.nb_header_verts)?;
        Ok(is_written)
    }
    /// The number of vertices written; `writer.nb_written_vertices()`.
    pub fn nb_written_vertices(&self) -> i32 {
        self.nb_verts
    }
    /// The offset added to element indices; `writer.vertex_offset()`.
    pub fn vertex_offset(&self) -> i32 {
        self.vert_offset
    }
    /// Sets the offset added to element indices; `writer.set_vertex_offset(8)`.
    pub fn set_vertex_offset(&mut self, offset: i32) {
        self.vert_offset = offset;
    }
    /// The surface id written with elements; `writer.surface_id()`.
    pub fn surface_id(&self) -> i32 {
        self.surf_id
    }
    /// Sets the surface id written with elements; `writer.set_surface_id(2)`.
    pub fn set_surface_id(&mut self, surface_id: i32) {
        self.surf_id = surface_id;
    }
    fn write_element(&mut self, indices: &[i32]) -> Result<bool, Failure> {
        if self.output.is_none() {
            return Ok(false);
        }
        let mut line = indices.len().to_string();
        for index in indices {
            line.push_str(&format!(" {}", self.vert_offset + index));
        }
        if self.has_surf_id {
            line.push_str(&format!(" {}", self.surf_id));
        }
        line.push('\n');
        let is_written = self.write_text(line.as_bytes());
        self.nb_elems += 1;
        check_count("elements", self.nb_elems, self.nb_header_elems)?;
        Ok(is_written)
    }
    /// Writes a triangle; a failure past the declared count; `writer.write_triangle(&Vec3::new(0, 1, 2))?`.
    pub fn write_triangle(&mut self, triangle: &Vec3<i32>) -> Result<bool, Failure> {
        self.write_element(&[triangle.x, triangle.y, triangle.z])
    }
    /// Writes a quad; a failure past the declared count; `writer.write_quad(&Vec4::new(0, 1, 2, 3))?`.
    pub fn write_quad(&mut self, quad: &Vec4<i32>) -> Result<bool, Failure> {
        self.write_element(&[quad.x, quad.y, quad.z, quad.w])
    }
    /// The number of elements written; `writer.nb_written_elements()`.
    pub fn nb_written_elements(&self) -> i32 {
        self.nb_elems
    }
    /// Closes the output, true when the file is complete: everything reached it and, unless the
    /// export was aborted, it holds as many records as its header states; `writer.close(false)`.
    pub fn close(&mut self, is_aborted: bool) -> bool {
        let Some(mut output) = self.output.take() else {
            return false;
        };
        if !(self.is_good && output.flush().is_ok()) {
            send_fail(&format!(
                "Error: file cannot be written\n{}",
                path_text(&self.name)
            ));
            return false;
        }
        is_aborted || self.has_declared_counts()
    }
    fn has_declared_counts(&self) -> bool {
        for (kind, written, declared) in [
            ("vertices", self.nb_verts, self.nb_header_verts),
            ("elements", self.nb_elems, self.nb_header_elems),
        ] {
            if written != declared {
                send_fail(&format!(
                    "Error: written less number of {kind} ({written}) than specified in PLY header ({declared})"
                ));
                return false;
            }
        }
        true
    }
}

/// `%.9g` of a single-precision value.
fn single_text(value: f32) -> String {
    format_general(f64::from(value), SINGLE_DIGITS)
}

fn check_count(kind: &str, written: i32, declared: i32) -> Result<(), Failure> {
    if written > declared {
        return Err(Failure::out_of_range(&format!(
            "RwPlyPlyWriterContext: {written} {kind} written, expected at most the {declared} of the header"
        )));
    }
    Ok(())
}

impl Drop for RwPlyPlyWriterContext {
    /// A file not closed explicitly is closed here, with the messages of [`Self::close`].
    fn drop(&mut self) {
        self.close(false);
    }
}
