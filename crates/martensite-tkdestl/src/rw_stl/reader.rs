//! `RwStlReader`: the callback interface of the STL readers.
use super::{reader_ascii, reader_binary, reader_domains};
use martensite_tkernel::{
    impl_transient_interface,
    message::ProgressRange,
    osd::IStream,
    standard::{Failure, ReadLineBuffer, SharedState, StandardTransient, Transient},
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Xyz;

/// The default merge angle, `M_PI / 2`: nodes merge whatever the angle between their triangles.
pub const DEFAULT_MERGE_ANGLE: f64 = std::f64::consts::FRAC_PI_2;

/// The protected `merge_angle` and `merge_tolearance` of `RwStlReader`;
/// `RwStlReaderBase::default()` is its default constructor.
pub struct RwStlReaderBase {
    merge_angle: SharedState<f64>,
    merge_tolerance: SharedState<f64>,
}
impl Default for RwStlReaderBase {
    fn default() -> Self {
        Self {
            merge_angle: SharedState::new(DEFAULT_MERGE_ANGLE),
            merge_tolerance: SharedState::new(0.0),
        }
    }
}

/// `RwStlReader`: reads STL data into a model the implementer owns.
///
/// Implement the three callbacks; the reading members are provided. Callbacks take `&self` so
/// a reader can be shared through `Handle<dyn RwStlReader>`, as every transient here is.
pub trait RwStlReader: Transient {
    /// The embedded base with the protected fields.
    fn reader_base(&self) -> &RwStlReaderBase;
    /// `add_node`: creates a node in the target model and returns its identifier.
    fn add_node(&self, point: &Xyz) -> i32;
    /// `add_triangle`: creates a triangle on nodes returned by [`RwStlReader::add_node`].
    fn add_triangle(&self, node1: i32, node2: i32, node3: i32);
    /// `add_solid`: ends one domain of a multi-domain file; nothing by default. It returns a
    /// `result` because a model may build a triangulation here, which can raise.
    fn add_solid(&self) -> Result<(), Failure> {
        Ok(())
    }
    /// `merge_angle`: the maximum angle in radians between triangles merging a node;
    /// `reader.merge_angle()`.
    fn merge_angle(&self) -> f64 {
        *self.reader_base().merge_angle.read()
    }
    /// `set_merge_angle`: `reader.set_merge_angle(std::f64::consts::FRAC_PI_4)` keeps nodes split
    /// at corners sharper than 45 degrees.
    fn set_merge_angle(&self, angle: f64) {
        *self.reader_base().merge_angle.write() = angle;
    }
    /// `merge_tolerance`: the linear merge distance; 0 merges only equal points;
    /// `reader.merge_tolerance()`.
    fn merge_tolerance(&self) -> f64 {
        *self.reader_base().merge_tolerance.read()
    }
    /// `set_merge_tolerance`: `reader.set_merge_tolerance(1.0e-6)`.
    fn set_merge_tolerance(&self, tolerance: f64) {
        *self.reader_base().merge_tolerance.write() = tolerance;
    }
    /// `Read`: reads a file of one or more concatenated domains, detecting the format; false on
    /// an error or user break. Nodes do not merge across domains. A text file that ends in a
    /// partial block also returns false, because the block read fails the stream;
    /// `reader.read(&"part.stl".into(), &ProgressRange::default())?`.
    fn read(&self, file: &AsciiString, progress: &ProgressRange<'_>) -> Result<bool, Failure> {
        reader_domains::read_file_domains(self, file, progress)
    }
    /// `is_ascii`: guesses the format from the first 134 bytes, then restores the position. A
    /// stream with no seek cannot be restored here: the `unget` branch is expressed as a
    /// relative seek, because this stream interface has no put-back;
    /// `reader.is_ascii(&mut stream, true)`.
    fn is_ascii(&self, stream: &mut dyn IStream, is_seek_available: bool) -> bool {
        reader_domains::head_is_ascii(stream, is_seek_available)
    }
    /// `read_binary`: reads the facet count the header records; false on an error or user break;
    /// `reader.read_binary(&mut stream, &ProgressRange::default())?`.
    fn read_binary(
        &self,
        stream: &mut dyn IStream,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        reader_binary::read_binary_facets(self, stream, progress)
    }
    /// `read_ascii`: reads until `endsolid` or the end of data. `until_pos` only scales progress;
    /// `reader.read_ascii(&mut stream, &mut lines, end, &ProgressRange::default())?`.
    fn read_ascii(
        &self,
        stream: &mut dyn IStream,
        buffer: &mut ReadLineBuffer,
        until_pos: i64,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        reader_ascii::read_ascii_facets(self, stream, buffer, until_pos, progress)
    }
}
impl_transient_interface!(dyn RwStlReader, "RwStlReader", StandardTransient);
