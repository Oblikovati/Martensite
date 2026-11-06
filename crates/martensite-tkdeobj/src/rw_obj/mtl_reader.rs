//! `RwObjMtlReader`: reading an MTL material file.
use super::{RwObjMaterial, RwObjTools};
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::{DataMap, Vec3};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::{character::is_space, strtod_bytes};
use martensite_tkernel::tcollection::AsciiString;
use std::path::Path;

/// Lines longer than this arrive in pieces, each read as a line of its own.
const LINE_LIMIT: usize = 254;

fn text_of(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn joined(first: &AsciiString, second: &AsciiString) -> AsciiString {
    AsciiString::from_raw(&[first.as_bytes(), second.as_bytes()].concat())
}

/// A location of the same file relative to a folder, from the tail of an absolute path.
fn find_relative_path(absolute: &AsciiString, folder: &AsciiString) -> Option<AsciiString> {
    let base = if folder.ends_with("\\") || folder.ends_with("/") {
        folder.clone()
    } else {
        joined(folder, &AsciiString::new("/"))
    };
    let mut relative = AsciiString::default();
    let mut path = absolute.clone();
    loop {
        let (parent, file) = CadPath::folder_and_file_from_path(path.as_bytes());
        if parent.is_empty() || file.is_empty() {
            return None;
        }
        relative = if relative.is_empty() {
            file
        } else {
            AsciiString::from_raw(&[file.as_bytes(), b"/", relative.as_bytes()].concat())
        };
        if Path::new(&text_of(&joined(&base, &relative))).exists() {
            return Some(relative);
        }
        path = without_trailing_separators(&parent);
    }
}

fn without_trailing_separators(path: &AsciiString) -> AsciiString {
    let mut bytes = path.as_bytes();
    while bytes.len() >= 2 && matches!(bytes[bytes.len() - 1], b'/' | b'\\') {
        bytes = &bytes[..bytes.len() - 1];
    }
    AsciiString::from_raw(bytes)
}

/// Reads materials into a map; `RwObjMtlReader::new(&mut materials).read(&folder, &file)`.
pub struct RwObjMtlReader<'m> {
    path: AsciiString,
    materials: &'m mut DataMap<AsciiString, RwObjMaterial>,
    nb_lines: i32,
}

/// The material being read.
struct PendingMaterial {
    name: AsciiString,
    material: RwObjMaterial,
    has_aspect: bool,
}

impl<'m> RwObjMtlReader<'m> {
    /// A reader into a map; `RwObjMtlReader::new(&mut materials)`.
    pub fn new(materials: &'m mut DataMap<AsciiString, RwObjMaterial>) -> Self {
        Self {
            path: AsciiString::default(),
            materials,
            nb_lines: 0,
        }
    }
    /// Reads a file of a folder; true when the map gained materials;
    /// `reader.read(&folder, &file)`.
    pub fn read(&mut self, folder: &AsciiString, file: &AsciiString) -> bool {
        self.path = joined(folder, file);
        let Ok(content) = std::fs::read(text_of(&self.path)) else {
            let text = format!("OBJ material file '{}' is not found!", text_of(&self.path));
            Message::send_warning_text(&AsciiString::new(&text));
            return false;
        };
        let count = self.materials.len();
        let mut pending = PendingMaterial {
            name: AsciiString::default(),
            material: RwObjMaterial::default(),
            has_aspect: false,
        };
        for line in limited_lines(&content) {
            self.nb_lines += 1;
            self.read_line(line, folder, &mut pending);
        }
        self.flush_material(&mut pending);
        self.materials.len() != count
    }
    fn flush_material(&mut self, pending: &mut PendingMaterial) {
        if pending.name.is_empty() {
            return;
        }
        if pending.has_aspect {
            pending.material.name = pending.name.clone();
        } else {
            // An incomplete definition is reset.
            pending.material = RwObjMaterial::default();
        }
        self.materials
            .bind(pending.name.clone(), pending.material.clone());
        pending.has_aspect = false;
    }
    fn read_line(&mut self, line: &[u8], folder: &AsciiString, pending: &mut PendingMaterial) {
        let start = line
            .iter()
            .position(|byte| !is_space(*byte))
            .unwrap_or(line.len());
        let text = &line[start..];
        if matches!(text.first(), None | Some(b'#' | b'\n' | 0)) {
            return;
        }
        if text.starts_with(b"newmtl") {
            self.flush_material(pending);
            pending.material = RwObjMaterial::default();
            if !RwObjTools::read_name(tail(text, 7), &mut pending.name) {
                let warning = format!(
                    "Empty OBJ material at line {} in file {}",
                    self.nb_lines,
                    text_of(&self.path)
                );
                Message::send_warning_text(&AsciiString::new(&warning));
            }
            return;
        }
        if !self.read_color_or_scalar(text, pending) {
            read_texture(text, folder, pending);
        }
    }
    fn read_color_or_scalar(&mut self, text: &[u8], pending: &mut PendingMaterial) -> bool {
        if has_key(text, b"Ka") {
            self.read_color(text, pending, |material, color| {
                material.ambient_color = color
            });
        } else if has_key(text, b"Kd") {
            self.read_color(text, pending, |material, color| {
                material.diffuse_color = color
            });
        } else if has_key(text, b"Ks") {
            self.read_color(text, pending, |material, color| {
                material.specular_color = color
            });
        } else if has_key(text, b"Ns") {
            let specular = strtod_bytes(tail(text, 3)).0;
            if specular >= 0.0 {
                pending.material.shininess = (specular / 1000.0).min(1.0) as f32;
                pending.has_aspect = true;
            }
        } else if has_key(text, b"Tr") {
            let transparency = strtod_bytes(tail(text, 3)).0;
            if self.validate_scalar(transparency) && transparency <= 0.99 {
                pending.material.transparency = transparency as f32;
                pending.has_aspect = true;
            }
        } else if has_key(text, b"d") {
            let alpha = strtod_bytes(tail(text, 2)).0;
            if self.validate_scalar(alpha) && alpha >= 0.01 {
                pending.material.transparency = (1.0 - alpha) as f32;
                pending.has_aspect = true;
            }
        } else {
            return false;
        }
        true
    }
    fn read_color(
        &mut self,
        text: &[u8],
        pending: &mut PendingMaterial,
        set: impl FnOnce(&mut RwObjMaterial, Color),
    ) {
        let mut color = Vec3::new(0.0_f32, 0.0, 0.0);
        RwObjTools::read_vec3_f32(tail(text, 3), &mut color);
        if !self.validate_color(&color) {
            return;
        }
        let values = [color.x, color.y, color.z].map(f64::from);
        if let Ok(parsed) = Color::new(values[0], values[1], values[2], ColorSpace::Srgb) {
            set(&mut pending.material, parsed);
            pending.has_aspect = true;
        }
    }
    fn validate_scalar(&self, value: f64) -> bool {
        if (0.0..=1.0).contains(&value) {
            return true;
        }
        self.warn_invalid("scalar");
        false
    }
    fn validate_color(&self, color: &Vec3<f32>) -> bool {
        if [color.x, color.y, color.z]
            .iter()
            .all(|channel| (0.0..=1.0).contains(channel))
        {
            return true;
        }
        self.warn_invalid("color");
        false
    }
    fn warn_invalid(&self, what: &str) {
        let text = format!(
            "Invalid {what} in OBJ material at line {} in file {}",
            self.nb_lines,
            text_of(&self.path)
        );
        Message::send_warning_text(&AsciiString::new(&text));
    }
}

fn read_texture(text: &[u8], folder: &AsciiString, pending: &mut PendingMaterial) {
    let (key_length, slot) = if has_key(text, b"map_Kd") {
        (7, &mut pending.material.diffuse_texture)
    } else if has_key(text, b"map_Ks") {
        (7, &mut pending.material.specular_texture)
    } else if has_key(text, b"map_Bump") {
        (9, &mut pending.material.bump_texture)
    } else {
        return;
    };
    if RwObjTools::read_name(tail(text, key_length), slot) {
        process_texture_path(slot, folder);
        pending.has_aspect = true;
    }
}

/// Makes a texture path usable from the folder; an absolute path that is missing is looked for
/// beside the file.
fn process_texture_path(path: &mut AsciiString, folder: &AsciiString) {
    if !CadPath::is_absolute_path(path.as_bytes()) {
        *path = joined(folder, path);
        return;
    }
    let warning = format!(
        "OBJ file specifies absolute path to the texture image file which may be inaccessible on another device\n{}",
        text_of(path)
    );
    Message::send_warning_text(&AsciiString::new(&warning));
    if Path::new(&text_of(path)).exists() {
        return;
    }
    if let Some(relative) = find_relative_path(path, folder) {
        *path = joined(folder, &relative);
    }
}

fn has_key(text: &[u8], key: &[u8]) -> bool {
    text.starts_with(key) && text.get(key.len()).copied().is_some_and(is_space)
}

fn tail(text: &[u8], from: usize) -> &[u8] {
    text.get(from..).unwrap_or(&[])
}

/// Splits as a 255 byte `fgets` does: at each newline, kept, and at the limit.
fn limited_lines(content: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = content;
    std::iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let newline = rest.iter().take(LINE_LIMIT).position(|byte| *byte == b'\n');
        let length = newline.map_or(LINE_LIMIT.min(rest.len()), |index| index + 1);
        let (line, remaining) = rest.split_at(length);
        rest = remaining;
        Some(line)
    })
}
