//! Default C-locale ASCII STL insertion order and scientific precision.
use super::{
    Failure, Handle, ProgressRange, ProgressScope, RwStl, Triangulation, Write, advance_progress,
    facet_coordinates, write_block,
};
use martensite_tkernel::standard::c_string::{FormatArg, sprintf_into};
impl RwStl {
    /// Writes ASCII facets with twelve fractional scientific digits and no stream flush.
    /// `RwStl::write_ascii(&mesh, &mut Vec::new(), &ProgressRange::default())?`.
    pub fn write_ascii(
        mesh: &Handle<Triangulation>,
        stream: &mut dyn Write,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let Some(mesh) = mesh.get() else {
            return Ok(false);
        };
        if mesh.nb_triangles() <= 0 || !write_block(stream, b"solid \n") {
            return Ok(false);
        }
        let count = mesh.nb_triangles();
        let scope = ProgressScope::with_literal(progress, "Triangles", f64::from(count), false);
        write_ascii_facets(mesh, stream, count, &scope)
    }
}
use std::fmt::Write as FmtWrite;

struct StackFmt {
    buf: [u8; 40],
    len: usize,
}

impl StackFmt {
    #[inline]
    fn new() -> Self {
        Self {
            buf: [0; 40],
            len: 0,
        }
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl FmtWrite for StackFmt {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let bytes = s.as_bytes();
        let end = self.len + bytes.len();
        if end > self.buf.len() {
            return Err(std::fmt::Error);
        }
        self.buf[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }
}

fn write_ascii_facets(
    mesh: &Triangulation,
    stream: &mut dyn Write,
    count: i32,
    scope: &ProgressScope<'_>,
) -> Result<bool, Failure> {
    let mut sink = AsciiFacetSink {
        stream,
        failed: false,
        number: Vec::new(),
    };
    for index in 1..=count {
        sink.facet(facet_coordinates(mesh, index)?);
        if sink.failed || !advance_progress(scope, index) {
            return Ok(false);
        }
    }
    sink.emit(b"endsolid\n");
    Ok(!sink.failed)
}
struct AsciiFacetSink<'a> {
    stream: &'a mut dyn Write,
    failed: bool,
    number: Vec<u8>,
}
impl AsciiFacetSink<'_> {
    #[inline]
    fn emit(&mut self, bytes: &[u8]) {
        if !self.failed {
            self.failed = !write_block(self.stream, bytes);
        }
    }
    #[inline]
    fn real(&mut self, value: f64) {
        if self.failed {
            return;
        }
        if !value.is_finite() {
            let Ok(length) = sprintf_into(&mut self.number, b"%.12e", &[FormatArg::Real(value)])
            else {
                self.failed = true;
                return;
            };
            self.failed = !write_block(self.stream, &self.number[..length]);
            return;
        }
        let mut out = [0u8; 40];
        let mut pos = 0usize;
        if value.is_sign_negative() {
            out[pos] = b'-';
            pos += 1;
        }
        let mut tmp = StackFmt::new();
        let _ = write!(&mut tmp, "{:.12e}", value.abs());
        let bytes = tmp.as_bytes();
        if let Some(e_pos) = bytes.iter().position(|&b| b == b'e') {
            let prefix = &bytes[..=e_pos];
            out[pos..pos + prefix.len()].copy_from_slice(prefix);
            pos += prefix.len();
            let exp = &bytes[e_pos + 1..];
            let (sign, digits) = if let Some(rest) = exp.strip_prefix(b"-") {
                (b'-', rest)
            } else {
                (b'+', exp.strip_prefix(b"+").unwrap_or(exp))
            };
            out[pos] = sign;
            pos += 1;
            if digits.len() == 1 {
                out[pos] = b'0';
                pos += 1;
            }
            out[pos..pos + digits.len()].copy_from_slice(digits);
            pos += digits.len();
        } else {
            out[pos..pos + bytes.len()].copy_from_slice(bytes);
            pos += bytes.len();
        }
        self.failed = !write_block(self.stream, &out[..pos]);
    }
    fn triplet(&mut self, coordinates: &[f64]) {
        self.real(coordinates[0]);
        self.emit(b" ");
        self.real(coordinates[1]);
        self.emit(b" ");
        self.real(coordinates[2]);
    }
    fn vertex(&mut self, coordinates: &[f64]) {
        self.emit(b"     vertex ");
        self.triplet(coordinates);
        self.emit(b"\n");
    }
    fn facet(&mut self, coordinates: [f64; 12]) {
        self.emit(b" facet normal ");
        self.triplet(&coordinates[..3]);
        self.emit(b"\n");
        self.emit(b"   outer loop\n");
        for vertex in coordinates[3..].as_chunks::<3>().0 {
            self.vertex(vertex);
        }
        self.emit(b"   endloop\n");
        self.emit(b" endfacet\n");
    }
}
