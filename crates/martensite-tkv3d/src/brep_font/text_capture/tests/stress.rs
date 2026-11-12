use super::*;
use std::{sync::Barrier, thread};
mod values;
const WORKERS: usize = 8;
const REPETITIONS: usize = 8;
const STRESS_TEXT: &str = "B8i S\nB8i!";

#[test]
fn concurrent_independent_memory_fonts_match_sequential_bits() {
    let font = asset();
    let baseline = values::snapshot(&capture(STRESS_TEXT));
    let start = Arc::new(Barrier::new(WORKERS));
    let threads: Vec<_> = (0..WORKERS)
        .map(|_| {
            let (font, start, baseline) = (font.clone(), start.clone(), baseline.clone());
            thread::spawn(move || independent_worker(font, start, baseline))
        })
        .collect();
    for worker in threads {
        worker.join().expect("independent capture worker");
    }
}
fn independent_worker(font: OwnedFontTextAsset, start: Arc<Barrier>, baseline: Vec<u64>) {
    start.wait();
    for _ in 0..REPETITIONS {
        let result = capture_owned_font_text(&font, request(STRESS_TEXT), || false).unwrap();
        assert_eq!(result.asset(), &font);
        assert_eq!(values::snapshot(&result), baseline);
    }
}
#[test]
fn concurrent_published_snapshot_reads_and_builds_match_values() {
    let published = Arc::new(capture(STRESS_TEXT));
    let baseline = values::snapshot(&published);
    let geometry = values::built_geometry(&published);
    let start = Arc::new(Barrier::new(WORKERS));
    let threads: Vec<_> = (0..WORKERS)
        .map(|_| {
            let (published, start) = (published.clone(), start.clone());
            let (baseline, geometry) = (baseline.clone(), geometry.clone());
            thread::spawn(move || published_worker(published, start, baseline, geometry))
        })
        .collect();
    for worker in threads {
        worker.join().expect("published snapshot reader/builder");
    }
}
fn published_worker(
    published: Arc<OwnedFontTextCapture>,
    start: Arc<Barrier>,
    baseline: Vec<u64>,
    geometry: Vec<Vec<u64>>,
) {
    start.wait();
    for _ in 0..REPETITIONS {
        assert_eq!(values::snapshot(&published), baseline);
        assert_eq!(values::built_geometry(&published), geometry);
        assert_eq!(values::snapshot(&published), baseline);
    }
}
