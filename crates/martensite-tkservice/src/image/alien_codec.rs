//! The image file formats `ImageAlienPixMap` reads and writes: PNG, JPEG, BMP and GIF,
//! through the `image` crate, and SGI files, which are only read (see `alien_sgi`). This
//! module is the only user of that crate.
//!
//! A format is recognised from the leading bytes of the file, else from the extension of the
//! name. Pixel conversion between decoded images and pixmap rows lives in `alien_pixels`.
use super::alien_palette::indexed_kind;
use super::alien_pixels::{DecodedRows, decoded_rows, encodable_image};
use super::alien_sgi::{decode_sgi, has_sgi_signature, is_sgi_extension};
use super::{ImageFormat, ImagePixMap};
use image::ImageFormat as FileFormat;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use std::io::Cursor;

/// A file format `ImageAlienPixMap` reads and writes; `CodecFormat::from_name(b"a.png")`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CodecFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
}

/// Why a file did not load; each maps to one failure message of `ImageAlienPixMap::load`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DecodeFailure {
    /// Neither the signature nor the name gives a supported format.
    UnsupportedFile,
    /// The file is absent, unreadable or not a valid image of its format.
    MissingOrInvalid,
    /// The decoded pixels have no pixmap format.
    UnsupportedPixels,
}

/// Why a pixmap was not written; `EncodeFailure::Encoder` when the encoder refuses it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EncodeFailure {
    /// The pixmap format has no 8-bit form in this file format.
    UnsupportedPixels(ImageFormat),
    /// The encoder refused the pixels.
    Encoder,
}

/// Extensions per format, compared without case.
const EXTENSIONS: [(&[u8], CodecFormat); 7] = [
    (b"png", CodecFormat::Png),
    (b"jpg", CodecFormat::Jpeg),
    (b"jpeg", CodecFormat::Jpeg),
    (b"jpe", CodecFormat::Jpeg),
    (b"jif", CodecFormat::Jpeg),
    (b"bmp", CodecFormat::Bmp),
    (b"gif", CodecFormat::Gif),
];

impl CodecFormat {
    /// The format whose signature starts `bytes`; `CodecFormat::from_signature(b"GIF89a")`.
    pub(super) fn from_signature(bytes: &[u8]) -> Option<Self> {
        match image::guess_format(bytes).ok()? {
            FileFormat::Png => Some(Self::Png),
            FileFormat::Jpeg => Some(Self::Jpeg),
            FileFormat::Bmp => Some(Self::Bmp),
            FileFormat::Gif => Some(Self::Gif),
            _ => None,
        }
    }
    /// The format named by the text after the last dot, or by the whole text when it has
    /// no dot, so that a bare extension works too: `CodecFormat::from_name(b"PNG")`.
    pub(super) fn from_name(name: &[u8]) -> Option<Self> {
        let extension = name_extension(name);
        EXTENSIONS
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(extension))
            .map(|&(_, format)| format)
    }
    fn file_format(self) -> FileFormat {
        match self {
            Self::Png => FileFormat::Png,
            Self::Jpeg => FileFormat::Jpeg,
            Self::Bmp => FileFormat::Bmp,
            Self::Gif => FileFormat::Gif,
        }
    }
}

/// The text after the last dot of `name`, or all of it without a dot; `name_extension(b"a.png")`.
fn name_extension(name: &[u8]) -> &[u8] {
    match name.iter().rposition(|&b| b == b'.') {
        Some(dot) => &name[dot + 1..],
        None => name,
    }
}

/// Whether to read the file as SGI: by its signature, else, when no other signature matches,
/// by the extension of `name`. Example: `reads_as_sgi(Some(&[1, 0xda]), b"a.png")`.
fn reads_as_sgi(bytes: Option<&[u8]>, name: &[u8]) -> bool {
    match bytes {
        Some(bytes) if has_sgi_signature(bytes) => true,
        Some(bytes) if CodecFormat::from_signature(bytes).is_some() => false,
        _ => is_sgi_extension(name_extension(name)),
    }
}

/// Decodes `bytes` (absent when the file could not be read) as the format of its signature,
/// else of `name`. Example: `decode(Some(&png_bytes), b"a.png")?.format`.
pub(super) fn decode(bytes: Option<&[u8]>, name: &[u8]) -> Result<DecodedRows, DecodeFailure> {
    if reads_as_sgi(bytes, name) {
        return decode_sgi(bytes.ok_or(DecodeFailure::MissingOrInvalid)?);
    }
    let signature = bytes.and_then(CodecFormat::from_signature);
    let format = signature
        .or_else(|| CodecFormat::from_name(name))
        .ok_or(DecodeFailure::UnsupportedFile)?;
    let bytes = bytes.ok_or(DecodeFailure::MissingOrInvalid)?;
    let decoded = image::load_from_memory_with_format(bytes, format.file_format())
        .map_err(|_| DecodeFailure::MissingOrInvalid)?;
    decoded_rows(&decoded, indexed_kind(bytes)).ok_or(DecodeFailure::UnsupportedPixels)
}

/// The file bytes of `pixmap` in `format`. Example: `encode(&pixmap, CodecFormat::Png)?`.
pub(super) fn encode(pixmap: &ImagePixMap, format: CodecFormat) -> Result<Vec<u8>, EncodeFailure> {
    let image =
        encodable_image(pixmap, format).ok_or(EncodeFailure::UnsupportedPixels(pixmap.format()))?;
    let mut out = Cursor::new(Vec::new());
    let written = match format {
        // Quality 75 with 4:2:0 chroma: the usual default of JPEG writers.
        CodecFormat::Jpeg => image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 75)
            .encode_image(&image)
            .is_ok(),
        CodecFormat::Png => write_compressed_png(&image, &mut out),
        _ => image.write_to(&mut out, format.file_format()).is_ok(),
    };
    if !written {
        return Err(EncodeFailure::Encoder);
    }
    Ok(out.into_inner())
}

// #347: Fast compression can produce oversized files for repetitive pixel patterns.
fn write_compressed_png(image: &image::DynamicImage, out: &mut Cursor<Vec<u8>>) -> bool {
    let encoder = PngEncoder::new_with_quality(out, CompressionType::Default, FilterType::Adaptive);
    image.write_with_encoder(encoder).is_ok()
}

#[cfg(test)]
#[path = "alien_codec_tests.rs"]
mod tests;
