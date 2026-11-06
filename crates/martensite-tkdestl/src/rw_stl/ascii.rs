//! Default C-locale ASCII STL insertion order and scientific precision.
use super::*;
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
    fn emit(&mut self, bytes: &[u8]) {
        if !self.failed {
            self.failed = !write_block(self.stream, bytes);
        }
    }
    fn real(&mut self, value: f64) {
        if self.failed {
            return;
        }
        let Ok(length) = sprintf_into(&mut self.number, b"%.12e", &[FormatArg::Real(value)]) else {
            self.failed = true;
            return;
        };
        self.failed = !write_block(self.stream, &self.number[..length]);
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
