//! The reading members of `RWStl`.
use super::collector::MeshCollector;
use super::reader::DEFAULT_MERGE_ANGLE;
use super::reader_domains::{LINE_BUFFER_BYTES, stream_end};
use super::{RwStl, RwStlReader};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::Sequence,
    osd::{CadPath, File, IStream, IosOpenMode, SeekDir, SharedIStream, default_file_system},
    standard::{Failure, Handle, ReadLineBuffer},
    tcollection::AsciiString,
};
use martensite_tkmath::poly::Triangulation;
use std::sync::PoisonError;

pub(super) fn read_binary_mode() -> IosOpenMode {
    IosOpenMode::IN | IosOpenMode::BINARY
}

fn open_path(path: &CadPath) -> Result<Option<SharedIStream>, Failure> {
    default_file_system().open_istream(&path.system_name(), read_binary_mode(), 0, None)
}

impl RwStl {
    /// Reads a file with every node merged;
    /// `RwStl::read_file(&"part.stl".into(), &ProgressRange::default())?`.
    pub fn read_file(
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        Self::read_file_with_angle(file, DEFAULT_MERGE_ANGLE, progress)
    }
    /// The triangles read, or a null handle.
    /// The result of the read is ignored: data read before an error is still returned;
    /// `RwStl::read_file_with_angle(&"part.stl".into(), FRAC_PI_4, &ProgressRange::default())?`.
    pub fn read_file_with_angle(
        file: &AsciiString,
        merge_angle: f64,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        let reader = MeshCollector::with_merge_angle(merge_angle);
        reader.read(file, progress)?;
        reader.triangulation()
    }
    /// One triangulation per
    /// domain; `RwStl::read_file_domains(&"a.stl".into(), FRAC_PI_2, &mut list, &progress)?`.
    pub fn read_file_domains(
        file: &AsciiString,
        merge_angle: f64,
        list: &mut Sequence<Handle<Triangulation>>,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let reader = MeshCollector::multi_domain(merge_angle);
        reader.read(file, progress)?;
        list.clear(None);
        reader.move_domains_to(list);
        Ok(())
    }
    /// A null handle when the file does not exist;
    /// `RwStl::read_path(&CadPath::new("part.stl"), &ProgressRange::default())?`.
    pub fn read_path(
        file: &CadPath,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        if !File::with_path(file.clone()).exists() {
            return Ok(Handle::default());
        }
        Self::read_file(&file.system_name(), progress)
    }
    /// A null handle on an error;
    /// `RwStl::read_binary(&CadPath::new("part.stl"), &ProgressRange::default())?`.
    pub fn read_binary(
        file: &CadPath,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        let Some(opened) = open_path(file)? else {
            return Ok(Handle::default());
        };
        let mut guard = opened.lock().unwrap_or_else(PoisonError::into_inner);
        read_binary_on(&mut *guard, &MeshCollector::default(), progress)
    }
    /// A null handle on an error;
    /// `RwStl::read_ascii(&CadPath::new("part.stl"), &ProgressRange::default())?`.
    pub fn read_ascii(
        file: &CadPath,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        let Some(opened) = open_path(file)? else {
            return Ok(Handle::default());
        };
        let mut guard = opened.lock().unwrap_or_else(PoisonError::into_inner);
        read_ascii_on(&mut *guard, &MeshCollector::default(), progress)
    }
    /// `read_binary_stream`: `RwStl::read_binary_stream(&mut stream, FRAC_PI_2, &progress)?`.
    pub fn read_binary_stream(
        stream: &mut dyn IStream,
        merge_angle: f64,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        read_binary_on(
            stream,
            &MeshCollector::with_merge_angle(merge_angle),
            progress,
        )
    }
    /// `read_ascii_stream`: reads from the start of the stream;
    /// `RwStl::read_ascii_stream(&mut stream, FRAC_PI_2, &progress)?`.
    pub fn read_ascii_stream(
        stream: &mut dyn IStream,
        merge_angle: f64,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        read_ascii_on(
            stream,
            &MeshCollector::with_merge_angle(merge_angle),
            progress,
        )
    }
    /// `read_stream`: text when the data starts with `solid`, else binary;
    /// `RwStl::read_stream(&mut stream, FRAC_PI_2, &progress)?`.
    pub fn read_stream(
        stream: &mut dyn IStream,
        merge_angle: f64,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<Triangulation>, Failure> {
        let original = stream.tellg();
        let mut header = [0; 5];
        let nb_read = stream.read(&mut header);
        stream.seekg(original, SeekDir::Beg);
        if nb_read == header.len() && header == *b"solid" {
            return Self::read_ascii_stream(stream, merge_angle, progress);
        }
        Self::read_binary_stream(stream, merge_angle, progress)
    }
}

fn read_binary_on(
    stream: &mut dyn IStream,
    reader: &MeshCollector,
    progress: &ProgressRange<'_>,
) -> Result<Handle<Triangulation>, Failure> {
    if !reader.read_binary(stream, progress)? {
        return Ok(Handle::default());
    }
    reader.triangulation()
}

fn read_ascii_on(
    stream: &mut dyn IStream,
    reader: &MeshCollector,
    progress: &ProgressRange<'_>,
) -> Result<Handle<Triangulation>, Failure> {
    let end = stream_end(stream);
    let mut lines = ReadLineBuffer::new(LINE_BUFFER_BYTES);
    if !reader.read_ascii(stream, &mut lines, end, progress)? {
        return Ok(Handle::default());
    }
    reader.triangulation()
}
