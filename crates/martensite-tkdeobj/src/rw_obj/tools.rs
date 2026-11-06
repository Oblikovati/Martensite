//! `RwObjTools`: the small parsers of the OBJ and MTL formats.
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::standard::{character::is_space, strtod_bytes};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Xyz;

/// The parsers of OBJ text; `RwObjTools::read_name(b" group\n", &mut name)`.
pub struct RwObjTools;

impl RwObjTools {
    /// Reads three values as floats, returning the bytes used and whether the last one
    /// converted; `RwObjTools::read_vec3_f32(b"1 2 3", &mut vector)`.
    pub fn read_vec3_f32(text: &[u8], vector: &mut Vec3<f32>) -> (usize, bool) {
        let mut values = Xyz::new(0.0, 0.0, 0.0);
        let result = Self::read_vec3(text, &mut values);
        *vector = Vec3::new(values.x() as f32, values.y() as f32, values.z() as f32);
        result
    }
    /// Reads three values, returning the bytes used and whether the last one converted;
    /// `RwObjTools::read_vec3(b"1 2 3", &mut point)`.
    pub fn read_vec3(text: &[u8], vector: &mut Xyz) -> (usize, bool) {
        let (x, first) = strtod_bytes(text);
        let (y, second) = strtod_bytes(&text[first..]);
        let (z, third) = strtod_bytes(&text[first + second..]);
        *vector = Xyz::new(x, y, z);
        (first + second + third, third != 0)
    }
    /// Reads a name without surrounding space; false, with an empty name, when there is none;
    /// `RwObjTools::read_name(b" group\n", &mut name)`.
    pub fn read_name(text: &[u8], name: &mut AsciiString) -> bool {
        let text = c_string(text);
        let mut tail = text.len() as isize - 1;
        if tail >= 0 && text[tail as usize] == b'\n' {
            tail -= 1;
        }
        if tail >= 0 && text[tail as usize] == b'\r' {
            tail -= 1;
        }
        while tail >= 0 && is_space(text[tail as usize]) {
            tail -= 1;
        }
        let mut from = 0_isize;
        while from < tail && is_space(text[from as usize]) {
            from += 1;
        }
        if from > tail {
            *name = AsciiString::default();
            return false;
        }
        *name = AsciiString::from_raw(&text[from as usize..=tail as usize]);
        true
    }
    /// True for a space or a tab; `RwObjTools::is_space_char(b'\t')`.
    pub fn is_space_char(byte: u8) -> bool {
        byte == b' ' || byte == b'\t'
    }
}

/// The bytes before the first NUL.
pub(super) fn c_string(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(text.len());
    &text[..end]
}

/// A C `strtol` in base 10: the value, wrapped to 32 bits as the reader's cast does, and the
/// bytes used, zero for no conversion.
pub(super) fn strtol(text: &[u8]) -> (i32, usize) {
    let mut pos = 0;
    while text.get(pos).copied().is_some_and(is_space) {
        pos += 1;
    }
    let negative = text.get(pos) == Some(&b'-');
    if negative || text.get(pos) == Some(&b'+') {
        pos += 1;
    }
    let digits_start = pos;
    let mut value = 0_i64;
    while let Some(digit) = text.get(pos).copied().filter(u8::is_ascii_digit) {
        value = value
            .saturating_mul(10)
            .saturating_add(i64::from(digit - b'0'));
        pos += 1;
    }
    if pos == digits_start {
        return (0, 0);
    }
    ((if negative { -value } else { value }) as i32, pos)
}
