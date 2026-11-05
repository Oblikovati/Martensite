//! `RwStlReader::read_binary`.
use super::merge_tool::ReaderMergeTool;
use super::{FACET_BYTES, RwStlReader};
use martensite_tkernel::{
    message::{Message, ProgressRange, ProgressScope},
    osd::IStream,
    standard::Failure,
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Xyz;

const HEADER_BYTES: usize = 84;
const CHUNK_FACETS: i32 = 80;
const VECTOR_BYTES: usize = 12;

/// Reads the header's facet count; it is not checked against the data length.
pub(super) fn read_binary_facets<R: RwStlReader + ?Sized>(
    reader: &R,
    stream: &mut dyn IStream,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let mut header = [0; HEADER_BYTES];
    if stream.read(&mut header) != HEADER_BYTES {
        Message::send_fail_text(&AsciiString::new("Error: Corrupted binary STL file"));
        return Ok(false);
    }
    let nb_facets = i32::from_le_bytes([header[80], header[81], header[82], header[83]]);
    let mut tool = ReaderMergeTool::new(reader, nb_facets);
    let scope = ProgressScope::with_literal(
        progress,
        "Reading binary STL file",
        f64::from(nb_facets),
        false,
    );
    read_facet_chunks(&mut tool, stream, nb_facets, &scope)
}

fn read_facet_chunks<R: RwStlReader + ?Sized>(
    tool: &mut ReaderMergeTool<'_, R>,
    stream: &mut dyn IStream,
    nb_facets: i32,
    scope: &ProgressScope<'_>,
) -> Result<bool, Failure> {
    let mut chunk = vec![0; FACET_BYTES * CHUNK_FACETS as usize];
    let mut nb_read = 0;
    while nb_read < nb_facets && scope.more() {
        let in_chunk = CHUNK_FACETS.min(nb_facets - nb_read);
        let wanted = &mut chunk[..FACET_BYTES * in_chunk as usize];
        if stream.read(wanted) != wanted.len() {
            Message::send_fail_text(&AsciiString::new("Error: binary STL read failed"));
            return Ok(false);
        }
        nb_read += add_chunk_facets(tool, wanted, scope)?;
    }
    Ok(scope.more())
}

/// Adds the facets of one chunk, stopping at a user break as the per-facet loop does.
fn add_chunk_facets<R: RwStlReader + ?Sized>(
    tool: &mut ReaderMergeTool<'_, R>,
    chunk: &[u8],
    scope: &ProgressScope<'_>,
) -> Result<i32, Failure> {
    let mut added = 0;
    for facet in chunk.as_chunks::<FACET_BYTES>().0 {
        if !scope.more() {
            break;
        }
        tool.add_triangle(&facet_nodes(facet))?;
        drop(scope.next(1.0));
        added += 1;
    }
    Ok(added)
}

/// The three nodes after the facet's normal, which the reader skips.
fn facet_nodes(facet: &[u8; FACET_BYTES]) -> [Xyz; 3] {
    [
        stored_vector(&facet[VECTOR_BYTES..]),
        stored_vector(&facet[VECTOR_BYTES * 2..]),
        stored_vector(&facet[VECTOR_BYTES * 3..]),
    ]
}

// TODO(target_endian = "big"): swaps the floats there; every Linux target here is little endian.
fn stored_vector(bytes: &[u8]) -> Xyz {
    let value = |at: usize| {
        let stored = [bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]];
        f64::from(f32::from_le_bytes(stored))
    };
    Xyz::new(value(0), value(4), value(8))
}
