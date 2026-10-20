//! The file input and output helpers of `Image_Texture`.
use super::ImageTextureBase;
use martensite_tkernel::message::Message;
use martensite_tkernel::osd::{IStream, IosOpenMode, SeekDir, SharedIStream, default_file_system};
use martensite_tkernel::tcollection::AsciiString;
use std::io::{Read, Write};

/// The `std::istream&` that `Image_AlienPixMap::Load` reads, from its current position;
/// `pixmap.load_stream(&mut IStreamReader(&mut *stream), &path)`.
pub(super) struct IStreamReader<'a>(pub(super) &'a mut dyn IStream);
impl Read for IStreamReader<'_> {
    /// `read(buf, n)`: the bytes extracted (`gcount`); a short read leaves the stream failed.
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        Ok(IStream::read(self.0, out))
    }
}

pub(super) fn fail<P: AsRef<[u8]>>(parts: &[P]) {
    let text: Vec<u8> = parts
        .iter()
        .flat_map(|part| part.as_ref().iter().copied())
        .collect();
    Message::send_fail_text(&AsciiString::from_bytes(&text));
}

/// Opens a file through the default file system, `None` when it cannot be opened.
pub(super) fn open_input(path: &AsciiString) -> Option<SharedIStream> {
    let mode = IosOpenMode::IN | IosOpenMode::BINARY;
    default_file_system()
        .open_istream(path, mode, 0, None)
        .ok()
        .flatten()
}

/// The image format of the first bytes ("png", "jpg", ... or "").
pub(super) fn probe_format(header: &[u8; 20]) -> &'static str {
    let starts = |magic: &[u8]| header.starts_with(magic);
    match () {
        _ if starts(b"\x89PNG\r\n\x1A\n") => "png",
        _ if starts(b"\xFF\xD8\xFF") => "jpg",
        _ if starts(b"GIF87a") || starts(b"GIF89a") => "gif",
        _ if starts(b"II\x2A\x00") || starts(b"MM\x00\x2A") => "tiff",
        _ if starts(b"BM") => "bmp",
        _ if starts(b"RIFF") && &header[8..12] == b"WEBP" => "webp",
        _ if starts(b"DDS ") => "dds",
        _ => "",
    }
}

/// The file branch of `WriteImage(std::ostream&)`: copies the file range (or the whole
/// file) in 4096-byte chunks.
pub(super) fn copy_file_range(
    base: &ImageTextureBase,
    output: &mut dyn Write,
    file: &AsciiString,
) -> bool {
    let path = base.image_path.as_bytes();
    let Some(stream) = open_input(&base.image_path) else {
        fail(&[b"Error: Unable to open file ", path, b"!"]);
        return false;
    };
    let mut input = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut length = base.length;
    if base.offset >= 0 {
        input.seekg(base.offset, SeekDir::Beg);
        if !input.good() {
            fail(&[
                b"Error: Image is defined with invalid file offset '",
                path,
                b"'",
            ]);
            return false;
        }
    } else {
        input.seekg(0, SeekDir::End);
        length = input.tellg();
        input.seekg(0, SeekDir::Beg);
    }
    let mut chunk = vec![0u8; 4096];
    let mut position = 0i64;
    while position < length {
        let size = (length - position).min(4096) as usize;
        if input.read(&mut chunk[..size]) < size {
            fail(&[b"Error: unable to read image file '", path, b"'"]);
            return false;
        }
        if output.write_all(&chunk[..size]).is_err() {
            fail(&[b"File '", file.as_bytes(), b"' can not be written"]);
            return false;
        }
        position += 4096;
    }
    true
}

/// Reads the probe bytes of a file texture, or the failure message.
pub(super) fn read_probe(
    base: &ImageTextureBase,
    header: &mut [u8; 20],
) -> Result<(), Vec<Vec<u8>>> {
    let path = base.image_path.as_bytes().to_vec();
    let Some(stream) = open_input(&base.image_path) else {
        return Err(vec![
            b"Error: Unable to open file '".to_vec(),
            path,
            b"'".to_vec(),
        ]);
    };
    let mut input = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if base.offset >= 0 {
        input.seekg(base.offset, SeekDir::Beg);
        if !input.good() {
            return Err(vec![
                b"Error: Image is defined with invalid file offset '".to_vec(),
                path,
                b"'".to_vec(),
            ]);
        }
    }
    if input.read(header) < header.len() {
        return Err(vec![
            b"Error: unable to read image file '".to_vec(),
            path,
            b"'".to_vec(),
        ]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reader continues from the position `seekg` set, as the native stream does.
    #[test]
    fn istream_reader_reads_from_the_seeked_position() {
        let name = format!("martensite-istream-reader-{}", std::process::id());
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, b"0123456789").unwrap();
        let shared = open_input(&AsciiString::from(path.to_str().unwrap())).unwrap();
        let mut stream = shared.lock().unwrap();
        stream.seekg(4, SeekDir::Beg);
        let mut bytes = Vec::new();
        IStreamReader(&mut *stream).read_to_end(&mut bytes).unwrap();
        drop(stream);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(bytes, b"456789");
    }
}
