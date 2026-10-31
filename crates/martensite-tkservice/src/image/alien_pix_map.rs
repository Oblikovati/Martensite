//! `Image_AlienPixMap`: an image read from and written to PNG, JPEG, BMP and GIF files, and
//! read from SGI files.
//!
//! Loading recognises the format from the file's leading bytes, else from the extension of
//! its name, and stores the pixels bottom-up with rows padded to four bytes (see
//! `alien_pixels` for the pixel formats). Saving picks the format from the extension; ".ppm"
//! writes binary PPM (P6). Gamma correction is not available.
use super::alien_codec::{self, CodecFormat, DecodeFailure};
use super::{ImageFormat, ImagePixMap};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use std::io::Write;
use std::ops::Deref;

/// Legacy opaque `IWICPalette` marker retained for source compatibility.
/// Active image codecs and palette conversion use the portable image implementation;
/// this marker does not own a COM palette or participate in image loading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageIwicPalette {}

/// An image read from or written to an image file; `ImageAlienPixMap::default().is_top_down()`.
pub struct ImageAlienPixMap {
    base: ImagePixMap,
}
impl Default for ImageAlienPixMap {
    /// An empty bottom-up image, the native default row order without a library.
    fn default() -> Self {
        let base = ImagePixMap::default();
        base.set_top_down(false);
        Self { base }
    }
}
impl Deref for ImageAlienPixMap {
    type Target = ImagePixMap;
    fn deref(&self) -> &ImagePixMap {
        &self.base
    }
}
martensite_tkernel::impl_transient!(ImageAlienPixMap, "Image_AlienPixMap", ImagePixMap, base,
    interfaces:[dyn super::ImagePixMapSource]);

/// A failure message to the default messenger.
fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::from(text));
}

/// The load failure message; `source` is "image" for files and "image stream" for streams.
fn load_failure_text(failure: DecodeFailure, source: &str, name: &AsciiString) -> String {
    let name = String::from_utf8_lossy(name.as_bytes());
    match (failure, source) {
        (DecodeFailure::UnsupportedFile, _) => {
            format!("Error: {source} '{name}' has unsupported file format")
        }
        (DecodeFailure::MissingOrInvalid, "image") => {
            format!("Error: image file '{name}' is missing or invalid")
        }
        (DecodeFailure::MissingOrInvalid, _) => {
            format!("Error: {source} '{name}' is missing or invalid")
        }
        (DecodeFailure::UnsupportedPixels, _) => {
            format!("Error: {source} '{name}' has unsupported pixel format")
        }
    }
}

/// Rows padded to a multiple of four bytes, the stride of a device-independent bitmap.
fn padded_row_bytes(row_bytes: usize) -> usize {
    row_bytes.div_ceil(4) * 4
}

/// Whether the name ends in ".ppm", ignoring case.
fn is_ppm_name(bytes: &[u8]) -> bool {
    bytes.len() >= 4
        && bytes[bytes.len() - 4] == b'.'
        && bytes[bytes.len() - 3..].eq_ignore_ascii_case(b"ppm")
}

/// `static_cast<uint8_t>(channel * 255.0)` on x86-64.
fn ppm_byte(channel: f64) -> u8 {
    crate::cast::int_from_f64(channel * 255.0) as u8
}

impl ImageAlienPixMap {
    /// False: loaded images are stored bottom-up.
    ///
    /// Example: `ImageAlienPixMap::is_top_down_default() == false`.
    pub fn is_top_down_default() -> bool {
        false
    }
    /// `Load(theFileName)`: decodes a PNG, JPEG, BMP, GIF or SGI file; on failure the image is
    /// empty and a message names the file. Example: `pixmap.load(&AsciiString::from("a.png"))`.
    pub fn load(&self, file_name: &AsciiString) -> bool {
        self.load_bytes(None, file_name)
    }
    /// `Load(std::istream&, theFileName)`: decodes the rest of the stream; the name only
    /// helps when the stream has no recognised signature.
    ///
    /// Example: `pixmap.load_stream(&mut std::io::Cursor::new(bytes), &name)`.
    pub fn load_stream(&self, stream: &mut dyn std::io::Read, file_name: &AsciiString) -> bool {
        self.base.clear();
        let mut bytes = Vec::new();
        let read = stream
            .read_to_end(&mut bytes)
            .ok()
            .map(|_| bytes.as_slice());
        self.store_decoded(
            alien_codec::decode(read, file_name.as_bytes()),
            "image stream",
            file_name,
        )
    }
    /// `Load(theData, theLength, theFileName)`: decodes `data`, or the named file when
    /// `data` is `None`. Example: `pixmap.load_bytes(Some(&png_bytes), &name)`.
    pub fn load_bytes(&self, data: Option<&[u8]>, file_name: &AsciiString) -> bool {
        self.base.clear();
        let file = match data {
            Some(_) => None,
            None => std::fs::read(String::from_utf8_lossy(file_name.as_bytes()).as_ref()).ok(),
        };
        let bytes = data.or(file.as_deref());
        self.store_decoded(
            alien_codec::decode(bytes, file_name.as_bytes()),
            "image",
            file_name,
        )
    }
    /// Copies decoded rows into bottom-up, four-byte-aligned storage, or reports the failure.
    fn store_decoded(
        &self,
        decoded: Result<super::alien_pixels::DecodedRows, DecodeFailure>,
        source: &str,
        file_name: &AsciiString,
    ) -> bool {
        let rows = match decoded {
            Ok(rows) => rows,
            Err(failure) => {
                send_fail(&load_failure_text(failure, source, file_name));
                return false;
            }
        };
        let packed = rows.width * ImagePixMap::size_pixel_bytes_of(rows.format);
        if !self.base.init_trash(
            rows.format,
            rows.width,
            rows.height,
            padded_row_bytes(packed),
        ) {
            return false;
        }
        self.base.set_top_down(false);
        let mut state = self.base.change_data();
        for (row, pixels) in rows.pixels.chunks_exact(packed).enumerate() {
            if let Some(target) = state.data.change_row(row) {
                target[..packed].copy_from_slice(pixels);
            }
        }
        true
    }
    /// `Save(theFileName)`: writes the format named by the extension (PNG, JPEG, BMP, GIF or
    /// PPM); false for an empty image or another extension.
    ///
    /// Example: `pixmap.save(&AsciiString::from("a.png"))?`.
    pub fn save(&self, file_name: &AsciiString) -> Result<bool, Failure> {
        self.save_bytes(None, file_name)
    }
    /// `Save(std::ostream&, theExtension)`: writes the format named by `extension` ("png",
    /// ".png" or a file name) to the stream.
    ///
    /// Example: `pixmap.save_stream(&mut out, &AsciiString::from("png"))`.
    pub fn save_stream(&self, stream: &mut dyn Write, extension: &AsciiString) -> bool {
        match self.encoded(extension.as_bytes()) {
            Ok(Some(bytes)) => stream.write_all(&bytes).is_ok(),
            _ => false,
        }
    }
    /// `Save(theBuffer, theLength, theFileName)`: encodes as named by the extension, into
    /// `buffer` when given (failing with a message when it is too small), else into the file.
    /// Float pixels outside [0, 1] raise `Standard_OutOfRange` when written as PPM, as the
    /// colour constructor does.
    ///
    /// Example: `pixmap.save_bytes(None, &name)?`.
    pub fn save_bytes(
        &self,
        buffer: Option<&mut [u8]>,
        file_name: &AsciiString,
    ) -> Result<bool, Failure> {
        let Some(bytes) = self.encoded(file_name.as_bytes())? else {
            return Ok(false);
        };
        let Some(buffer) = buffer else {
            let path = String::from_utf8_lossy(file_name.as_bytes()).into_owned();
            return Ok(std::fs::write(path, bytes).is_ok());
        };
        if bytes.len() > buffer.len() {
            send_fail("Error: memory buffer too small for storing image");
            return Ok(false);
        }
        buffer[..bytes.len()].copy_from_slice(&bytes);
        Ok(true)
    }
    /// The file bytes in the format `name` selects; `None` for an empty image, an unknown
    /// extension or pixels without an 8-bit form in that format.
    fn encoded(&self, name: &[u8]) -> Result<Option<Vec<u8>>, Failure> {
        if self.is_empty() {
            return Ok(None);
        }
        if is_ppm_name(name) || name.eq_ignore_ascii_case(b"ppm") {
            return self.ppm_bytes().map(Some);
        }
        let Some(format) = CodecFormat::from_name(name) else {
            return Ok(None);
        };
        Ok(alien_codec::encode(&self.base, format).ok())
    }
    /// `savePPM`: the "P6" header, then the RGB bytes of every pixel, top row first.
    fn ppm_bytes(&self) -> Result<Vec<u8>, Failure> {
        let mut contents = format!(
            "P6\n{} {}\n255\n",
            self.width() as i32,
            self.height() as i32
        )
        .into_bytes();
        for row in 0..self.height() {
            for col in 0..self.width() {
                let rgb = *self.pixel_color(col as i32, row as i32, false)?.rgb();
                contents.extend([rgb.red(), rgb.green(), rgb.blue()].map(ppm_byte));
            }
        }
        Ok(contents)
    }
    /// `InitTrash`: as `Image_PixMap::InitTrash`.
    ///
    /// Example: `pixmap.init_trash(ImageFormat::Rgb, 4, 4, 0)`.
    pub fn init_trash(
        &self,
        format: ImageFormat,
        size_x: usize,
        size_y: usize,
        size_row_bytes: usize,
    ) -> bool {
        self.base.init_trash(format, size_x, size_y, size_row_bytes)
    }
    /// `InitCopy`: same format only (no conversion), copied in one block when the rows
    /// match, else row by row; a different format clears and fails.
    ///
    /// Example: `pixmap.init_copy(&other)`.
    pub fn init_copy(&self, copy: &ImagePixMap) -> bool {
        if std::ptr::eq(&self.base, copy) {
            return false;
        }
        let source = copy.data().clone();
        if !self.init_trash(
            source.format,
            source.data.size_x,
            source.data.size_y,
            source.data.size_row_bytes,
        ) {
            return false;
        }
        if self.format() != source.format {
            self.clear();
            return false;
        }
        let mut target = self.change_data();
        if target.data.size_row_bytes == source.data.size_row_bytes
            && target.data.top_to_down == source.data.top_to_down
        {
            let count = target.data.size().min(source.data.size());
            target.data.change_data()[..count].copy_from_slice(&source.data.data()[..count]);
            return true;
        }
        let row_bytes = target.data.size_row_bytes.min(source.data.size_row_bytes);
        for row in 0..target.data.size_y {
            if let (Some(to), Some(from)) = (target.data.change_row(row), source.data.row(row)) {
                to[..row_bytes].copy_from_slice(&from[..row_bytes]);
            }
        }
        true
    }
    /// `InitWrapper`: private natively, where it overrides the base member to forbid
    /// wrapping foreign memory: it clears and fails, shadowing `ImagePixMap::init_wrapper`.
    ///
    /// Example: `pixmap.init_wrapper(ImageFormat::Gray, None, 1, 1, 0) == false`.
    pub fn init_wrapper(
        &self,
        _format: ImageFormat,
        _bytes: Option<Vec<u8>>,
        _size_x: usize,
        _size_y: usize,
        _size_row_bytes: usize,
    ) -> bool {
        self.clear();
        false
    }
    /// `Clear`: releases the pixels (no library image to release).
    ///
    /// Example: `pixmap.clear()`.
    pub fn clear(&self) {
        self.base.clear();
    }
    /// `AdjustGamma`: gamma correction is not available, so always false;
    /// `pixmap.adjust_gamma(2.2)`.
    pub fn adjust_gamma(&self, _gamma: f64) -> bool {
        false
    }
}

#[cfg(test)]
#[path = "alien_pix_map_tests.rs"]
mod tests;
