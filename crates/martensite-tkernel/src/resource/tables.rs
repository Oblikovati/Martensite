//! The Unicode conversion tables, as reviewed table data; do not edit by hand.
//!
//! Sparse blobs hold sorted little-endian `(index, value)` u16 pairs for the nonzero
//! entries; dense blobs hold every entry little-endian. `(bytes, FNV-1a-64)` per table.
/// Unicode to Shift-JIS (sparse, 28172 bytes).
pub(super) const UNISJIS: (&[u8], u64) = (include_bytes!("tables/unisjis.bin"), 0x2ba05d2119487396);
/// Shift-JIS to Unicode (sparse, 28148 bytes).
pub(super) const SJISUNI: (&[u8], u64) = (include_bytes!("tables/sjisuni.bin"), 0x80728a7a85cf17df);
/// Unicode to GB2312 (sparse, 29776 bytes).
pub(super) const UNIGB: (&[u8], u64) = (include_bytes!("tables/unigb.bin"), 0x839f5a0e38643e4d);
/// GB2312 to Unicode (sparse, 29780 bytes).
pub(super) const GBUNI: (&[u8], u64) = (include_bytes!("tables/gbuni.bin"), 0x61ce81a209648b4d);
/// GBK to Unicode (dense, 47880 bytes).
pub(super) const GBKUNI: (&[u8], u64) = (include_bytes!("tables/gbkuni.bin"), 0x0f53fa814d716022);
/// Big5 to Unicode (dense, 79128 bytes).
pub(super) const BIG5UNI: (&[u8], u64) = (include_bytes!("tables/big5uni.bin"), 0x770b6fa0631178df);
/// The single-byte code pages (dense, 4864 bytes).
pub(super) const CODE_PAGES: (&[u8], u64) =
    (include_bytes!("tables/code_pages.bin"), 0xe23c0a568fa6787a);
