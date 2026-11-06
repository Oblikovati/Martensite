//! `RwObjObjWriterContext`: the low-level writing of an OBJ file.
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::{IndexedDataMap, Vec2, Vec3, Vec4};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::split_text_lines;
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// The first line of a written file.
pub(super) const EXPORT_HEADER: &str = "# Exported by Martensite";

/// Writes the statements of an OBJ file, with the indices shifted past the faces already written.
///
/// ```no_run
/// use martensite_tkdeobj::rw_obj::RwObjObjWriterContext;
/// use martensite_tkernel::ncollection::{IndexedDataMap, Vec3};
/// use martensite_tkernel::tcollection::AsciiString;
/// let mut writer = RwObjObjWriterContext::new(&AsciiString::new("tri.obj"));
/// assert!(writer.is_opened());
/// writer.write_header(3, 1, &AsciiString::default(), &IndexedDataMap::default());
/// writer.write_vertex(&Vec3::new(0.0, 0.0, 0.0));
/// assert!(writer.close());
/// ```
pub struct RwObjObjWriterContext {
    /// The number of faces so far.
    pub nb_faces: i32,
    file: Option<BufWriter<File>>,
    name: AsciiString,
    active_material: AsciiString,
    elem_pos_first: i32,
    elem_norm_first: i32,
    elem_uv_first: i32,
    has_normals: bool,
    has_tex_coords: bool,
}

/// A name on one line: a line break in a statement would make the rest of it a new statement.
fn single_line(text: &AsciiString) -> Vec<u8> {
    text.as_bytes()
        .iter()
        .map(|byte| {
            if matches!(byte, b'\r' | b'\n') {
                b' '
            } else {
                *byte
            }
        })
        .collect()
}

/// `%f` of a value: six decimals, and the words the C library prints for the others.
pub(super) fn c_fixed(value: f64) -> String {
    if value.is_nan() {
        return String::from(if value.is_sign_negative() {
            "-nan"
        } else {
            "nan"
        });
    }
    if value.is_infinite() {
        return String::from(if value < 0.0 { "-inf" } else { "inf" });
    }
    format!("{value:.6}")
}

impl RwObjObjWriterContext {
    /// Creates the file, reporting a failure; `RwObjObjWriterContext::new(&file)`.
    pub fn new(name: &AsciiString) -> Self {
        let path = String::from_utf8_lossy(name.as_bytes()).into_owned();
        let file = File::create(&path).map(BufWriter::new).ok();
        if file.is_none() {
            Message::send_fail_text(&AsciiString::new(&format!(
                "File cannot be created\n{path}"
            )));
        }
        Self {
            nb_faces: 0,
            file,
            name: name.clone(),
            active_material: AsciiString::default(),
            elem_pos_first: 1,
            elem_norm_first: 1,
            elem_uv_first: 1,
            has_normals: false,
            has_tex_coords: false,
        }
    }
    /// True when the file was created; `writer.is_opened()`.
    pub fn is_opened(&self) -> bool {
        self.file.is_some()
    }
    /// Closes the file, true when everything reached it; `writer.close()`.
    pub fn close(&mut self) -> bool {
        self.file
            .take()
            .is_some_and(|mut file| file.flush().is_ok())
    }
    /// True when normals are defined; `writer.has_normals()`.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }
    /// Sets whether normals are defined; `writer.set_normals(true)`.
    pub fn set_normals(&mut self, has_normals: bool) {
        self.has_normals = has_normals;
    }
    /// True when texture coordinates are defined; `writer.has_tex_coords()`.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }
    /// Sets whether texture coordinates are defined; `writer.set_tex_coords(true)`.
    pub fn set_tex_coords(&mut self, has_tex_coords: bool) {
        self.has_tex_coords = has_tex_coords;
    }
    fn write_text(&mut self, text: &[u8]) -> io::Result<()> {
        match self.file.as_mut() {
            Some(file) => file.write_all(text),
            None => Err(io::Error::from(io::ErrorKind::NotConnected)),
        }
    }
    fn write_line(&mut self, text: &str) -> bool {
        self.write_text(text.as_bytes()).is_ok()
    }
    /// Writes the header: counts, file information and the material library;
    /// `writer.write_header(8, 12, &library, &information)`.
    pub fn write_header(
        &mut self,
        nb_nodes: i32,
        nb_elems: i32,
        mat_lib: &AsciiString,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
    ) -> bool {
        let head = format!("{EXPORT_HEADER}\n#  Vertices: {nb_nodes}\n#     Faces: {nb_elems}\n");
        let mut is_ok = self.write_line(&head);
        for (key, value) in file_info.items() {
            is_ok = self.write_information(key, value) && is_ok;
        }
        if !mat_lib.is_empty() {
            let line = [b"mtllib ", single_line(mat_lib).as_slice(), b"\n"].concat();
            is_ok = self.write_text(&line).is_ok() && is_ok;
        }
        is_ok
    }
    fn write_information(&mut self, key: &AsciiString, value: &AsciiString) -> bool {
        let (key_lines, value_lines) = (split_text_lines(key), split_text_lines(value));
        let mut text = Vec::new();
        for (index, line) in key_lines.iter().enumerate() {
            text.extend_from_slice(if index > 0 { b"\n# " } else { b"# " });
            text.extend_from_slice(line.as_bytes());
        }
        text.extend_from_slice(if key_lines.is_empty() { b"# " } else { b":" });
        for (index, line) in value_lines.iter().enumerate() {
            text.extend_from_slice(if index > 0 { b"\n# " } else { b" " });
            text.extend_from_slice(line.as_bytes());
        }
        text.push(b'\n');
        self.write_text(&text).is_ok()
    }
    /// The active material, empty when none; `writer.active_material()`.
    pub fn active_material(&self) -> &AsciiString {
        &self.active_material
    }
    /// Sets the active material; `writer.write_active_material(&name)`.
    pub fn write_active_material(&mut self, material: &AsciiString) -> bool {
        self.active_material = material.clone();
        let line = if material.is_empty() {
            b"usemtl\n".to_vec()
        } else {
            [b"usemtl ", single_line(material).as_slice(), b"\n"].concat()
        };
        self.write_text(&line).is_ok()
    }
    fn corner(&self, node: i32) -> String {
        let position = node + self.elem_pos_first;
        let normal = node + self.elem_norm_first;
        let uv = node + self.elem_uv_first;
        match (self.has_normals, self.has_tex_coords) {
            (true, true) => format!("{position}/{uv}/{normal}"),
            (true, false) => format!("{position}//{normal}"),
            (false, true) => format!("{position}/{uv}"),
            (false, false) => position.to_string(),
        }
    }
    fn write_corners(&mut self, nodes: &[i32]) -> bool {
        let corners: Vec<String> = nodes.iter().map(|node| self.corner(*node)).collect();
        self.write_line(&format!("f {}\n", corners.join(" ")))
    }
    /// Writes a triangle; `writer.write_triangle(&Vec3::new(0, 1, 2))`.
    pub fn write_triangle(&mut self, triangle: &Vec3<i32>) -> bool {
        self.write_corners(&[triangle.x, triangle.y, triangle.z])
    }
    /// Writes a quad; `writer.write_quad(&Vec4::new(0, 1, 2, 3))`.
    pub fn write_quad(&mut self, quad: &Vec4<i32>) -> bool {
        self.write_corners(&[quad.x, quad.y, quad.z, quad.w])
    }
    /// Writes a vertex; `writer.write_vertex(&Vec3::new(0.0, 0.0, 0.0))`.
    pub fn write_vertex(&mut self, value: &Vec3<f32>) -> bool {
        let [x, y, z] = [value.x, value.y, value.z].map(|v| c_fixed(f64::from(v)));
        self.write_line(&format!("v {x} {y} {z}\n"))
    }
    /// Writes a normal; `writer.write_normal(&Vec3::new(0.0, 0.0, 1.0))`.
    pub fn write_normal(&mut self, value: &Vec3<f32>) -> bool {
        let [x, y, z] = [value.x, value.y, value.z].map(|v| c_fixed(f64::from(v)));
        self.write_line(&format!("vn {x} {y} {z}\n"))
    }
    /// Writes texture coordinates; `writer.write_tex_coord(&Vec2::new(0.5, 0.5))`.
    pub fn write_tex_coord(&mut self, value: &Vec2<f32>) -> bool {
        let [u, v] = [value.x, value.y].map(|v| c_fixed(f64::from(v)));
        self.write_line(&format!("vt {u} {v}\n"))
    }
    /// Writes a group name; `writer.write_group(&name)`.
    pub fn write_group(&mut self, value: &AsciiString) -> bool {
        let line = if value.is_empty() {
            b"g\n".to_vec()
        } else {
            [b"g ", single_line(value).as_slice(), b"\n"].concat()
        };
        self.write_text(&line).is_ok()
    }
    /// Moves the indices past a face; `writer.flush_face(8)`.
    pub fn flush_face(&mut self, nb_nodes: i32) {
        self.elem_pos_first += nb_nodes;
        if self.has_normals {
            self.elem_norm_first += nb_nodes;
        }
        if self.has_tex_coords {
            self.elem_uv_first += nb_nodes;
        }
    }
}

impl Drop for RwObjObjWriterContext {
    /// A file not closed by [`Self::close`] was not written completely.
    fn drop(&mut self) {
        if self.file.take().is_some() {
            let name = String::from_utf8_lossy(self.name.as_bytes()).into_owned();
            Message::send_fail_text(&AsciiString::new(&format!(
                "File cannot be written\n{name}"
            )));
        }
    }
}
