//! What a captured image shows: enough to tell that a capture is not empty.
//!
//! The checks do not compare images with reference frames; they check that the view
//! drew. A capture always holds the blue coordinate axes, so a shape is proved by a pixel of
//! another color than the background and the axes.

const BACKGROUND: [u8; 3] = [0, 0, 0];
const AXES: [u8; 3] = [0, 0, 255];

/// The color census of one captured image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageCensus {
    /// Pixels that are not the background.
    pub drawn: usize,
    /// Drawn pixels that are not the color of the axes.
    pub shape: usize,
}

/// Counts the pixels of a binary portable pixmap, the form a window saves.
/// Example: `census_of(b"P6\n1 1\n255\n\0\0\0")?.drawn == 0`.
pub fn census_of(image: &[u8]) -> Result<ImageCensus, String> {
    let pixels = pixmap_pixels(image)?;
    let (colors, _) = pixels.as_chunks::<3>();
    let drawn = colors.iter().filter(|p| **p != BACKGROUND).count();
    let axes = colors.iter().filter(|p| **p == AXES).count();
    Ok(ImageCensus {
        drawn,
        shape: drawn - axes,
    })
}

fn pixmap_pixels(image: &[u8]) -> Result<&[u8], String> {
    let refuse = || format!("image of {} bytes: expected P6, its size, 255", image.len());
    let mut rest = image;
    let mut fields = [""; 3];
    for field in &mut fields {
        let end = rest.iter().position(|b| *b == b'\n').ok_or_else(refuse)?;
        *field = std::str::from_utf8(&rest[..end]).map_err(|_| refuse())?;
        rest = &rest[end + 1..];
    }
    let size: Vec<usize> = fields[1]
        .split(' ')
        .map(|text| text.parse().map_err(|_| refuse()))
        .collect::<Result<_, _>>()?;
    let expected = size.iter().product::<usize>() * 3;
    let shaped = fields[0] == "P6" && fields[2] == "255" && size.len() == 2;
    if !shaped || rest.len() != expected {
        return Err(refuse());
    }
    Ok(rest)
}

#[test]
fn the_census_separates_background_axes_and_shape() {
    let image = b"P6\n3 1\n255\n\0\0\0\0\0\xff\xff\xff\0";
    let census = census_of(image).unwrap();
    assert_eq!(census, ImageCensus { drawn: 2, shape: 1 });
}

#[test]
fn a_blank_image_counts_nothing() {
    let census = census_of(b"P6\n2 1\n255\n\0\0\0\0\0\0").unwrap();
    assert_eq!(census, ImageCensus { drawn: 0, shape: 0 });
}

#[test]
fn a_truncated_or_foreign_image_is_refused() {
    assert!(census_of(b"P6\n2 1\n255\n\0\0\0").is_err());
    assert!(census_of(b"P5\n1 1\n255\n\0\0\0").is_err());
    assert!(census_of(b"").is_err());
}
