//! Named colors of the standard capture palette as 24-bit `0xRRGGBB` pixels.
use crate::{COLOR_NAMES, MAXCOLOR};

/// The pixels a 24-bit TrueColor X server allocates for [`COLOR_NAMES`], in palette order.
/// They are the X11 `rgb.txt` values of those names (note X11 "Green" is `#00ff00` and
/// "Maroon" is `#b03060`), checked against a live server when this table was written.
pub(super) const PALETTE_PIXELS: [u32; MAXCOLOR] = [
    0xff_ffff, // White
    0xff_0000, // Red
    0x00_ff00, // Green
    0x00_00ff, // Blue
    0x00_ffff, // Cyan
    0xff_d700, // Gold
    0xff_00ff, // Magenta
    0xb0_3060, // Maroon
    0xff_a500, // Orange
    0xff_c0cb, // Pink
    0xfa_8072, // Salmon
    0xee_82ee, // Violet
    0xff_ff00, // Yellow
    0xf0_e68c, // Khaki
    0xff_7f50, // Coral
];

/// Resolve a palette color name, ignoring ASCII case as the X color database does.
///
/// Example: `named_pixel("gold") == Some(0xffd700)`; names outside the palette give `None`.
pub(super) fn named_pixel(name: &str) -> Option<u32> {
    COLOR_NAMES
        .iter()
        .position(|known| known.eq_ignore_ascii_case(name))
        .map(|index| PALETTE_PIXELS[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_names_resolve_to_their_server_pixels() {
        assert_eq!(named_pixel("White"), Some(0xff_ffff));
        assert_eq!(named_pixel("green"), Some(0x00_ff00));
        assert_eq!(named_pixel("MAROON"), Some(0xb0_3060));
        assert_eq!(named_pixel("Coral"), Some(0xff_7f50));
        assert_eq!(named_pixel("Chartreuse"), None);
    }

    #[test]
    fn every_palette_name_has_a_distinct_pixel() {
        let pixels: Vec<u32> = COLOR_NAMES.iter().filter_map(|n| named_pixel(n)).collect();
        assert_eq!(pixels, PALETTE_PIXELS);
        let mut unique = pixels.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), MAXCOLOR);
    }
}
