//! Development check: memory windows capture the same bytes as live X server windows.
//!
//! Run with a display: `cargo test -p martensite-raster-core --test memory_x11_equivalence
//! -- --ignored --nocapture`. The frame-buffer case needs an unaccelerated server (for example
//! `Xwayland -shm` or `Xvfb`); the accelerated case needs a GL-accelerated server and draws in
//! copy mode only, because GPU logic operations are not repeatable from run to run.
#![cfg(target_os = "linux")]
use martensite_raster_core::{
    MemoryWindowSystem, PixelSegment, RasterFailureContext, RasterWindow, RasterWindowSystem,
    VirtualWindowSystem, ZeroWidthLineRule,
};
use std::sync::atomic::{AtomicUsize, Ordering};

/// What a scene may use: xor drawing and coordinates near the 16-bit limits.
#[derive(Clone, Copy)]
struct SceneOptions {
    xor: bool,
    extreme: bool,
}

/// A small deterministic generator so every run draws the same scene.
struct SceneRandom(u64, SceneOptions);

impl SceneRandom {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, bound: u64) -> i32 {
        (self.next() % bound) as i32
    }
    fn coordinate(&mut self, extent: i32) -> i32 {
        match self.below(10) {
            0 if self.1.extreme => self.below(65536) - 32768,
            0..=2 => self.below(extent as u64 * 3) - extent,
            _ => self.below(extent as u64 + 8) - 4,
        }
    }
    fn segment(&mut self, extent: i32) -> PixelSegment {
        let start = [self.coordinate(extent), self.coordinate(extent)];
        if self.below(3) == 0 {
            let reach = 1 + self.below(12);
            let end = [
                start[0] + self.below(2 * reach as u64 + 1) - reach,
                start[1] + self.below(2 * reach as u64 + 1) - reach,
            ];
            return PixelSegment::new(start[0], start[1], end[0], end[1]);
        }
        PixelSegment::new(
            start[0],
            start[1],
            self.coordinate(extent),
            self.coordinate(extent),
        )
    }
}

/// Draw one scene; return how many segments and strings it drew.
fn draw_scene(
    window: &mut dyn RasterWindow,
    seed: u64,
    extent: i32,
    options: SceneOptions,
) -> [usize; 2] {
    let mut random = SceneRandom(seed, options);
    let mut drawn = [0, 0];
    window.clear();
    for _ in 0..400 {
        match random.below(20) {
            0 => window.set_color(random.below(16) - 1),
            1 => window.set_raster_op(if options.xor && random.below(2) == 0 {
                6
            } else {
                3
            }),
            2 => {
                draw_label(window, &mut random, extent);
                drawn[1] += 1;
            }
            3 if random.below(40) == 0 => window.clear(),
            _ => {
                let count = 1 + random.below(30) as usize;
                let segments: Vec<_> = (0..count).map(|_| random.segment(extent)).collect();
                window.draw_segments(&segments);
                drawn[0] += count;
            }
        }
    }
    drawn
}

fn draw_label(window: &mut dyn RasterWindow, random: &mut SceneRandom, extent: i32) {
    let text: String = (0..1 + random.below(12))
        .map(|_| char::from(32 + random.below(95) as u8))
        .collect();
    window.draw_string(random.coordinate(extent), random.coordinate(extent), &text);
}

fn saved_bytes(window: &dyn RasterWindow, name: &str) -> Vec<u8> {
    // Tests run in parallel threads, so every capture needs its own file.
    static CAPTURES: AtomicUsize = AtomicUsize::new(0);
    let serial = CAPTURES.fetch_add(1, Ordering::Relaxed);
    let file = format!(
        "raster-equivalence-{name}-{}-{serial}.png",
        std::process::id()
    );
    let path = std::env::temp_dir().join(file);
    assert!(
        window.save(path.to_str().unwrap()),
        "save failed for {name}"
    );
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    bytes
}

fn differing_pixels(server: &[u8], memory: &[u8], width: usize) -> Vec<(usize, usize)> {
    let header = server.len() - width * width * 3;
    let pixel = |bytes: &[u8], index: usize| {
        bytes
            .get(header + index * 3..header + index * 3 + 3)
            .map(<[u8]>::to_vec)
    };
    (0..width * width)
        .filter(|index| pixel(server, *index) != pixel(memory, *index))
        .map(|index| (index % width, index / width))
        .collect()
}

/// Draw the same 24 scenes on both systems; return the number of differing captures.
fn mismatched_captures(rule: ZeroWidthLineRule, options: SceneOptions) -> usize {
    let server = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let memory = MemoryWindowSystem::with_line_rule(rule);
    let (mut mismatches, mut totals) = (0, [0, 0]);
    for (seed, extent) in (1..=24u64).zip([800, 400].into_iter().cycle()) {
        let mut native = server.create_window("Native", (0, 0), (extent, extent), 0);
        let mut owned = memory.create_window("Memory", (0, 0), (extent, extent), 0);
        let drawn = draw_scene(native.as_mut(), seed * 7919, extent, options);
        totals = [totals[0] + drawn[0], totals[1] + drawn[1]];
        draw_scene(owned.as_mut(), seed * 7919, extent, options);
        let expected = saved_bytes(native.as_ref(), "native");
        let actual = saved_bytes(owned.as_ref(), "memory");
        if expected != actual {
            mismatches += 1;
            let pixels = differing_pixels(&expected, &actual, extent as usize);
            let first = &pixels[..pixels.len().min(8)];
            println!(
                "seed {seed} extent {extent}: {} pixels differ, first {first:?}",
                pixels.len()
            );
        }
    }
    println!(
        "24 scenes, {} segments, {} strings, {mismatches} differing captures",
        totals[0], totals[1]
    );
    mismatches
}

#[test]
#[ignore = "requires a software-rendering X server in DISPLAY"]
fn frame_buffer_rule_matches_software_server_captures() {
    let options = SceneOptions {
        xor: true,
        extreme: true,
    };
    assert_eq!(
        mismatched_captures(ZeroWidthLineRule::FrameBuffer, options),
        0
    );
}

#[test]
#[ignore = "requires a GL-accelerated X server in DISPLAY"]
fn accelerated_rule_matches_gl_server_copy_captures() {
    let options = SceneOptions {
        xor: false,
        extreme: false,
    };
    assert_eq!(
        mismatched_captures(ZeroWidthLineRule::Accelerated, options),
        0
    );
}
