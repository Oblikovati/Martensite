//! Shared owned mesh fixtures and named stream/progress fakes.
use martensite_tkernel::{
    message::{ProgressIndicatorHooks, ProgressScope},
    standard::Handle,
};
use martensite_tkmath::{
    gp::Point3,
    poly::{Triangle, Triangulation},
};
use std::io::{self, Write};
pub(crate) fn planar_mesh(count: i32) -> Handle<Triangulation> {
    let mesh = Triangulation::with_sizes(3, count, false, false).unwrap();
    for (index, point) in [
        (1, Point3::new(0., 0., 0.)),
        (2, Point3::new(2., 0., 0.)),
        (3, Point3::new(0., 3., 0.)),
    ] {
        mesh.set_node(index, point).unwrap();
    }
    for index in 1..=count {
        mesh.set_triangle(index, Triangle::new(1, 2, 3)).unwrap();
    }
    Handle::new(mesh)
}
#[derive(Default)]
pub(crate) struct RecordingStream {
    pub(crate) bytes: Vec<u8>,
    pub(crate) lengths: Vec<usize>,
    pub(crate) fail_call: Option<usize>,
    pub(crate) short_call: Option<usize>,
}
impl Write for RecordingStream {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.lengths.push(bytes.len());
        if self.fail_call == Some(self.lengths.len()) {
            return Err(io::Error::other("named stream failure"));
        }
        let length = if self.short_call == Some(self.lengths.len()) {
            bytes.len().saturating_sub(1)
        } else {
            bytes.len()
        };
        self.bytes.extend_from_slice(&bytes[..length]);
        Ok(length)
    }
    fn flush(&mut self) -> io::Result<()> {
        panic!("STL serialization must not flush its borrowed stream")
    }
}
pub(crate) fn expected_facet(normal: [f32; 3], vertices: [f32; 9]) -> Vec<u8> {
    let mut bytes: Vec<u8> = normal
        .into_iter()
        .chain(vertices)
        .flat_map(f32::to_le_bytes)
        .collect();
    bytes.extend_from_slice(&[0, 0]);
    bytes
}
pub(crate) struct CancelledProgress;
impl ProgressIndicatorHooks for CancelledProgress {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        true
    }
}
/// Named stream modeling caller mutation during the header's synchronous write.
pub(crate) struct HeaderCountMutation {
    pub(crate) bytes: Vec<u8>,
    mesh: Handle<Triangulation>,
    count: i32,
    changed: bool,
}
impl HeaderCountMutation {
    pub(crate) fn new(mesh: Handle<Triangulation>, count: i32) -> Self {
        Self {
            bytes: Vec::new(),
            mesh,
            count,
            changed: false,
        }
    }
}
impl Write for HeaderCountMutation {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(bytes);
        if !self.changed {
            let mesh = self.mesh.get().unwrap();
            mesh.resize_triangles(self.count, true).unwrap();
            for index in 1..=self.count {
                mesh.set_triangle(index, Triangle::new(1, 2, 3)).unwrap();
            }
            self.changed = true;
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        panic!("writer must not flush a borrowed stream")
    }
}
