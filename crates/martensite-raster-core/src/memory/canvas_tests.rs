use super::*;

fn canvas(size: [i32; 2]) -> PixelCanvas {
    let mut canvas = PixelCanvas::new(size, ZeroWidthLineRule::FrameBuffer);
    canvas.set_foreground(0xff_0000);
    canvas
}

fn inked(canvas: &PixelCanvas) -> Vec<[i32; 2]> {
    let [width, height] = canvas.size();
    (0..height)
        .flat_map(|y| (0..width).map(move |x| [x, y]))
        .filter(|point| canvas.pixel(*point) != Some(0))
        .collect()
}

#[test]
fn a_new_canvas_is_black_and_copies() {
    let mut canvas = canvas([3, 2]);
    assert!(inked(&canvas).is_empty());
    canvas.plot([1, 1]);
    assert_eq!(canvas.pixel([1, 1]), Some(0xff_0000));
    canvas.plot([3, 0]);
    assert_eq!(canvas.pixel([3, 0]), None);
}

#[test]
fn clearing_fills_pixel_zero_through_the_current_operation() {
    let mut canvas = canvas([2, 2]);
    canvas.plot([0, 0]);
    canvas.set_operation(6);
    canvas.clear();
    assert_eq!(canvas.pixel([0, 0]), Some(0xff_0000));
    canvas.set_operation(3);
    canvas.clear();
    assert!(inked(&canvas).is_empty());
}

#[test]
fn xor_drawing_twice_restores_the_pixels() {
    let mut canvas = canvas([8, 8]);
    canvas.draw_segment([0, 0], [7, 3]);
    canvas.set_operation(6);
    canvas.set_foreground(0x00_ff00);
    canvas.draw_segment([0, 0], [7, 3]);
    assert_eq!(canvas.pixel([0, 0]), Some(0xff_ff00));
    canvas.draw_segment([0, 0], [7, 3]);
    assert_eq!(canvas.pixel([0, 0]), Some(0xff_0000));
    canvas.set_operation(99);
    canvas.draw_segment([0, 0], [7, 3]);
    assert_eq!(canvas.pixel([0, 0]), Some(0xff_ff00));
}

#[test]
fn a_glyph_hangs_from_its_baseline_origin() {
    let mut canvas = canvas([12, 16]);
    canvas.draw_text([1, 12], b"A");
    let pixels = inked(&canvas);
    // Row 2 of "A" (baseline - 9) holds only column 2; its crossbar is row 7 (baseline - 4).
    assert_eq!(pixels.first(), Some(&[3, 3]));
    assert!((1..6).all(|x| pixels.contains(&[x, 8])));
    assert_eq!(pixels.iter().map(|p| p[1]).max(), Some(11));
    assert_eq!(
        pixels.len(),
        FIXED_GLYPHS[0x41]
            .iter()
            .map(|row| row.count_ones() as usize)
            .sum()
    );
}

#[test]
fn text_advances_six_pixels_and_stops_at_nul() {
    let mut pair = canvas([20, 16]);
    pair.draw_text([0, 12], b"AA\0A");
    let mut single = canvas([20, 16]);
    single.draw_text([6, 12], b"A");
    single.draw_text([0, 12], b"A");
    assert_eq!(inked(&pair), inked(&single));
    let mut space = canvas([20, 16]);
    space.draw_text([-3, 12], b" ");
    assert!(inked(&space).is_empty());
}

#[test]
fn glyphs_are_clipped_at_every_edge() {
    let mut canvas = canvas([4, 4]);
    canvas.draw_text([-2, 2], b"MM");
    canvas.draw_text([2, 14], b"M");
    assert!(
        inked(&canvas)
            .iter()
            .all(|[x, y]| (0..4).contains(x) && (0..4).contains(y))
    );
    assert!(!inked(&canvas).is_empty());
}

#[test]
fn rgb_bytes_are_top_down_red_green_blue() {
    let mut canvas = canvas([2, 2]);
    canvas.set_foreground(0x12_3456);
    canvas.plot([1, 0]);
    assert_eq!(
        canvas.rgb_bytes(),
        [0, 0, 0, 0x12, 0x34, 0x56, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(canvas.line_rule(), ZeroWidthLineRule::FrameBuffer);
}

#[test]
fn saving_follows_the_extension_with_exact_ppm_bytes() {
    let mut canvas = canvas([3, 2]);
    canvas.plot([2, 1]);
    let path = std::env::temp_dir().join(format!("memory-canvas-{}.ppm", std::process::id()));
    assert_eq!(canvas.save(path.to_str().unwrap()), Ok(true));
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    let mut expected = b"P6\n3 2\n255\n".to_vec();
    expected.extend(canvas.rgb_bytes());
    assert_eq!(bytes, expected);
    let path = std::env::temp_dir().join(format!("memory-canvas-{}.png", std::process::id()));
    assert_eq!(canvas.save(path.to_str().unwrap()), Ok(true));
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}
