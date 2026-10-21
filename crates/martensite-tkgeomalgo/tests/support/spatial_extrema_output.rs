//! Shared source-bound segment/point publication of GeometryTest::extrema.
use martensite_tkmath::gp::Point3;
#[path = "curve_segment.rs"]
mod curve_segment;
pub(super) use curve_segment::extrema_segment;

pub(super) fn spatial_result_words(index: i32, a: Point3, b: Point3) -> Vec<String> {
    if a.distance(&b) < 1.0e-16 {
        return format!("Extrema {index} is point : {} {} {}", a.x(), a.y(), a.z())
            .split_whitespace()
            .map(str::to_owned)
            .collect();
    }
    let _segment = extrema_segment(a, b);
    vec![format!("ext_{index}")]
}
