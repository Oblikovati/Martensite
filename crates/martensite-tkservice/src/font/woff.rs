//! Safe WOFF 1.0 to SFNT (TTF/OTF) unpacker backed by pure-Rust `flate2` (#632).
//!
//! Unpacks the bundled `dejavu_sans_latin.woff` fallback font and any
//! user-supplied WOFF1 font bytes without `unsafe` or native `libfreetype`.

use flate2::read::ZlibDecoder;
use std::io::Read;

const WOFF_SIGNATURE: [u8; 4] = *b"wOFF";
const WOFF_HEADER_LEN: usize = 44;
const WOFF_ENTRY_LEN: usize = 20;
const SFNT_HEADER_LEN: usize = 12;
const SFNT_ENTRY_LEN: usize = 16;
const MAX_SFNT_BYTES: usize = 64 * 1024 * 1024;

/// Unpacks WOFF 1.0 bytes into a valid SFNT container, or clones raw SFNT/TTC
/// bytes unchanged. Returns an error message on malformed WOFF headers or corrupt
/// zlib streams.
pub(crate) fn decode_woff1_or_sfnt(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < 4 {
        return Err(format!(
            "font buffer has {} bytes; expected at least 4 header bytes",
            input.len()
        ));
    }
    if input[..4] != WOFF_SIGNATURE {
        return Ok(input.to_vec());
    }
    unpack_woff1(input)
}

#[derive(Clone, Copy)]
struct WoffTableEntry {
    tag: [u8; 4],
    offset: usize,
    comp_length: usize,
    orig_length: usize,
    orig_checksum: u32,
}

fn read_u16(bytes: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([bytes[at], bytes[at + 1]])
}

fn read_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn unpack_woff1(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < WOFF_HEADER_LEN {
        return Err(format!(
            "WOFF1 buffer has {} bytes; expected at least {WOFF_HEADER_LEN}",
            input.len()
        ));
    }
    let flavor = read_u32(input, 4);
    let num_tables = usize::from(read_u16(input, 12));
    let total_sfnt_size = read_u32(input, 16) as usize;
    if num_tables == 0 || total_sfnt_size > MAX_SFNT_BYTES {
        return Err(format!(
            "WOFF1 header has num_tables={num_tables}, total_sfnt_size={total_sfnt_size}; expected non-empty font under {MAX_SFNT_BYTES} bytes"
        ));
    }
    let entries = parse_directory(input, num_tables)?;
    assemble_sfnt(input, flavor, total_sfnt_size, &entries)
}

fn parse_directory(input: &[u8], num_tables: usize) -> Result<Vec<WoffTableEntry>, String> {
    let dir_end = WOFF_HEADER_LEN
        .checked_add(num_tables.saturating_mul(WOFF_ENTRY_LEN))
        .ok_or_else(|| "WOFF1 table directory overflowed usize".to_string())?;
    if dir_end > input.len() {
        return Err(format!(
            "WOFF1 table directory ends at {dir_end} beyond input length {}",
            input.len()
        ));
    }
    let mut entries = Vec::with_capacity(num_tables);
    for idx in 0..num_tables {
        let base = WOFF_HEADER_LEN + idx * WOFF_ENTRY_LEN;
        let entry = WoffTableEntry {
            tag: [
                input[base],
                input[base + 1],
                input[base + 2],
                input[base + 3],
            ],
            offset: read_u32(input, base + 4) as usize,
            comp_length: read_u32(input, base + 8) as usize,
            orig_length: read_u32(input, base + 12) as usize,
            orig_checksum: read_u32(input, base + 16),
        };
        let end = entry
            .offset
            .checked_add(entry.comp_length)
            .ok_or_else(|| "WOFF1 table slice overflowed".to_string())?;
        if end > input.len() || entry.comp_length > entry.orig_length {
            return Err(format!(
                "WOFF1 table {:?} has invalid bounds offset={} comp={} orig={} input={}",
                entry.tag,
                entry.offset,
                entry.comp_length,
                entry.orig_length,
                input.len()
            ));
        }
        entries.push(entry);
    }
    entries.sort_by_key(|entry| entry.tag);
    Ok(entries)
}

fn assemble_sfnt(
    input: &[u8],
    flavor: u32,
    hint_size: usize,
    entries: &[WoffTableEntry],
) -> Result<Vec<u8>, String> {
    let num_tables = entries.len() as u16;
    let max_pow2 = num_tables.leading_zeros();
    let entry_selector = 15u16.saturating_sub(max_pow2 as u16);
    let search_range = (1u16 << entry_selector) * 16;
    let range_shift = num_tables * 16 - search_range;

    let header_and_dir = SFNT_HEADER_LEN + entries.len() * SFNT_ENTRY_LEN;
    let mut out = Vec::with_capacity(hint_size.max(header_and_dir));
    out.extend_from_slice(&flavor.to_be_bytes());
    out.extend_from_slice(&num_tables.to_be_bytes());
    out.extend_from_slice(&search_range.to_be_bytes());
    out.extend_from_slice(&entry_selector.to_be_bytes());
    out.extend_from_slice(&range_shift.to_be_bytes());
    out.resize(header_and_dir, 0);

    for (idx, entry) in entries.iter().enumerate() {
        let table_offset = out.len();
        let compressed = &input[entry.offset..entry.offset + entry.comp_length];
        if entry.comp_length == entry.orig_length {
            out.extend_from_slice(compressed);
        } else {
            decompress_table(compressed, entry.orig_length, &mut out)?;
        }
        while !out.len().is_multiple_of(4) {
            out.push(0);
        }
        let dir_pos = SFNT_HEADER_LEN + idx * SFNT_ENTRY_LEN;
        out[dir_pos..dir_pos + 4].copy_from_slice(&entry.tag);
        out[dir_pos + 4..dir_pos + 8].copy_from_slice(&entry.orig_checksum.to_be_bytes());
        out[dir_pos + 8..dir_pos + 12].copy_from_slice(&(table_offset as u32).to_be_bytes());
        out[dir_pos + 12..dir_pos + 16].copy_from_slice(&(entry.orig_length as u32).to_be_bytes());
    }
    Ok(out)
}

fn decompress_table(src: &[u8], expected_len: usize, out: &mut Vec<u8>) -> Result<(), String> {
    let start = out.len();
    let mut decoder = ZlibDecoder::new(src);
    decoder
        .read_to_end(out)
        .map_err(|error| format!("WOFF1 zlib decompression failed: {error}"))?;
    let actual = out.len() - start;
    if actual != expected_len {
        return Err(format!(
            "WOFF1 table decompressed to {actual} bytes; expected {expected_len}"
        ));
    }
    Ok(())
}
