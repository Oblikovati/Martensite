//! Image differences count pixels or connected non-border regions without a viewer.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::image::{ImageDiff, ImageFormat, ImagePixMap};

fn white_image_with_black_pixels(
    width: usize,
    height: usize,
    black: &[(usize, usize)],
) -> Handle<ImagePixMap> {
    let mut bytes = vec![255; width * height * 3];
    for &(x, y) in black {
        let offset = (y * width + x) * 3;
        bytes[offset..offset + 3].fill(0);
    }
    let image = ImagePixMap::default();
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(bytes), width, height, 0));
    image.set_top_down(true);
    Handle::new(image)
}

fn assert_difference_counts(
    first: &Handle<ImagePixMap>,
    second: &Handle<ImagePixMap>,
    pixels: i32,
    regions: i32,
) -> Result<(), Failure> {
    let comparison = ImageDiff::default();
    assert!(comparison.init(first, second, false)?);
    comparison.set_color_tolerance(0.01);
    assert_eq!(comparison.compare()?, pixels);
    comparison.set_border_filter_on(true);
    assert_eq!(comparison.compare()?, regions);
    assert_eq!(comparison.compare()?, regions);
    comparison.set_border_filter_on(false);
    assert_eq!(comparison.compare()?, pixels);
    Ok(())
}

#[test]
fn thin_boundary_difference_is_filtered_without_crossing_image_bounds() -> Result<(), Failure> {
    let white = white_image_with_black_pixels(4, 4, &[]);
    let boundary = white_image_with_black_pixels(4, 4, &[(0, 0), (0, 1), (0, 2)]);
    assert_difference_counts(&white, &boundary, 3, 0)?;
    assert_difference_counts(&boundary, &white, 3, 0)
}

#[test]
fn one_pixel_wide_images_keep_connected_lines_without_solid_neighbours() -> Result<(), Failure> {
    let white = white_image_with_black_pixels(1, 3, &[]);
    let black = white_image_with_black_pixels(1, 3, &[(0, 0), (0, 1), (0, 2)]);
    assert_difference_counts(&white, &black, 3, 1)?;
    assert_difference_counts(&black, &white, 3, 1)
}

#[test]
fn separated_solid_blocks_form_two_regions_in_both_directions() -> Result<(), Failure> {
    let white = white_image_with_black_pixels(7, 3, &[]);
    let blocks = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (5, 0),
        (6, 0),
        (5, 1),
        (6, 1),
    ];
    let altered = white_image_with_black_pixels(7, 3, &blocks);
    assert_difference_counts(&white, &altered, 8, 2)?;
    assert_difference_counts(&altered, &white, 8, 2)
}

#[test]
fn isolated_pixel_differences_do_not_form_connected_regions() -> Result<(), Failure> {
    let white = white_image_with_black_pixels(4, 4, &[]);
    let isolated = white_image_with_black_pixels(4, 4, &[(0, 0), (3, 3)]);
    assert_difference_counts(&white, &isolated, 2, 0)?;
    assert_difference_counts(&isolated, &white, 2, 0)?;
    assert_difference_counts(&white, &white, 0, 0)
}
