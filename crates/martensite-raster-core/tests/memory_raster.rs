//! Display-free raster windows: window state, palette, drawing modes and capture bytes.
use martensite_raster_core::{
    COLOR_NAMES, MemoryWindowSystem, PixelSegment, RasterWindow, RasterWindowSystem,
    ZeroWidthLineRule,
};

/// Captures land in the temporary directory and are removed after reading.
fn captured_bytes(window: &dyn RasterWindow, name: &str) -> Option<Vec<u8>> {
    let file = format!("memory-raster-{name}-{}.ppm", std::process::id());
    let path = std::env::temp_dir().join(file);
    if !window.save(path.to_str().unwrap()) {
        return None;
    }
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    Some(bytes)
}

#[test]
fn windows_keep_their_state_and_never_map() {
    let system = MemoryWindowSystem::new();
    let mut window = system
        .try_create_window("Capture", (5, 8), (16, 12), 0)
        .unwrap();
    assert_eq!(window.position(), (5, 8));
    assert_eq!((window.width(), window.height()), (16, 12));
    window.set_position(-3, 4);
    window.set_title("Renamed");
    window.display();
    window.wait(true);
    window.redraw();
    system.flush();
    assert_eq!(
        (window.position(), window.title()),
        ((-3, 4), "Renamed".to_string())
    );
    assert!(!window.is_mapped());
    assert_eq!(system.next_event().kind, 0);
}

#[test]
fn drawing_starts_in_white_and_ignores_unknown_palette_indices() {
    let system = MemoryWindowSystem::new();
    let mut window = system
        .try_create_memory_window("W", (0, 0), (8, 8))
        .unwrap();
    window.draw_segments(&[PixelSegment::new(0, 0, 7, 0)]);
    assert_eq!(window.pixel(7, 0), Some(0xff_ffff));
    window.set_color(5);
    window.set_color(-1);
    window.set_color(15);
    window.draw_segments(&[PixelSegment::new(0, 1, 7, 1)]);
    assert_eq!(window.pixel(3, 1), Some(0xff_d700));
    assert_eq!(COLOR_NAMES[5], "Gold");
}

#[test]
fn xor_mode_and_clearing_follow_the_raster_operation() {
    let system = MemoryWindowSystem::new();
    let mut window = system
        .try_create_memory_window("W", (0, 0), (8, 8))
        .unwrap();
    window.set_color(1);
    window.draw_segments(&[PixelSegment::new(0, 0, 0, 7)]);
    window.set_raster_op(6);
    window.set_color(3);
    window.draw_segments(&[PixelSegment::new(0, 0, 0, 0)]);
    assert_eq!(window.pixel(0, 0), Some(0xff_00ff));
    window.clear();
    assert_eq!(window.pixel(0, 3), Some(0xff_0000));
    window.set_raster_op(3);
    window.clear();
    assert_eq!(window.rgb_bytes(), vec![0; 8 * 8 * 3]);
}

#[test]
fn text_origins_wrap_like_signed_sixteen_bit_coordinates() {
    let system = MemoryWindowSystem::new();
    let mut wrapped = system
        .try_create_memory_window("A", (0, 0), (20, 20))
        .unwrap();
    let mut direct = system
        .try_create_memory_window("B", (0, 0), (20, 20))
        .unwrap();
    wrapped.draw_string(65536 + 2, 65536 + 14, "Hi");
    direct.draw_string(2, 14, "Hi");
    assert_eq!(wrapped.rgb_bytes(), direct.rgb_bytes());
    assert_ne!(direct.rgb_bytes(), vec![0; 20 * 20 * 3]);
}

#[test]
fn a_capture_is_a_binary_ppm_of_the_window_pixels() {
    let system = MemoryWindowSystem::new();
    let mut window = system
        .try_create_memory_window("W", (0, 0), (16, 16))
        .unwrap();
    window.clear();
    window.set_color(1);
    window.draw_segments(&[PixelSegment::new(1, 2, 12, 2)]);
    let bytes = captured_bytes(&window, "ppm").unwrap();
    assert!(bytes.starts_with(b"P6\n16 16\n255\n"));
    assert_eq!(&bytes[13..], window.rgb_bytes().as_slice());
    assert!(bytes[13..].as_chunks::<3>().0.contains(&[255, 0, 0]));
}

#[test]
fn a_rejected_resize_fails_the_next_capture_until_a_valid_one() {
    let system = MemoryWindowSystem::new();
    let mut window = system
        .try_create_memory_window("W", (0, 0), (8, 8))
        .unwrap();
    window.set_dimension(0, 8);
    assert_eq!((window.width(), window.height()), (8, 8));
    assert_eq!(captured_bytes(&window, "rejected"), None);
    window.set_color(2);
    window.set_raster_op(6);
    window.draw_segments(&[PixelSegment::new(0, 0, 3, 0)]);
    window.set_dimension(4, 3);
    assert_eq!(window.rgb_bytes(), vec![0; 4 * 3 * 3]);
    window.draw_segments(&[PixelSegment::new(0, 0, 0, 0)]);
    window.draw_segments(&[PixelSegment::new(0, 0, 0, 0)]);
    assert_eq!(
        window.pixel(0, 0),
        Some(0x00_ff00),
        "a resize returns to copy mode"
    );
    assert!(
        captured_bytes(&window, "resized")
            .unwrap()
            .starts_with(b"P6\n4 3\n255\n")
    );
}

#[test]
fn the_line_rule_reaches_every_window() {
    let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
    let mut window = system
        .try_create_memory_window("W", (0, 0), (8, 8))
        .unwrap();
    window.draw_segments(&[PixelSegment::new(0, 0, 4, 2)]);
    assert_eq!(window.pixel(1, 0), Some(0xff_ffff));
    let mut software = MemoryWindowSystem::default()
        .try_create_memory_window("W", (0, 0), (8, 8))
        .unwrap();
    software.draw_segments(&[PixelSegment::new(0, 0, 4, 2)]);
    assert_eq!(software.pixel(1, 1), Some(0xff_ffff));
}
