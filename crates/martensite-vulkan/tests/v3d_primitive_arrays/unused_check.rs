//! The source defines this helper but never calls it; retain its reads and two HasPixel=1 quirks.
#![allow(dead_code)]
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::{standard::Failure, storage::format_general};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
pub(super) trait PrimitivePixelSource {
    fn pixel_text(&self, x: i32, y: i32) -> Result<String, Failure>;
}
impl PrimitivePixelSource for ViewerSession {
    fn pixel_text(&self, x: i32, y: i32) -> Result<String, Failure> {
        let color = self.pixel_color(x, y)?;
        Ok([color.red(), color.green(), color.blue()]
            .map(|v| format_general(v, 17))
            .join(" "))
    }
}
pub(super) fn check_primitive(
    session: &mut ViewerSession,
    names: &[&str],
    expected: &str,
) -> Result<bool, Failure> {
    session.display_only(names)?;
    session.set_standard_projection(V3dTypeOfOrientation::Zpos)?;
    session.fit()?;
    session.move_cursor(0, 0)?;
    session.move_cursor(0, 0)?;
    let passed = corner_checks(session, expected)?;
    session.update_context(|context| context.display_all(false))?;
    session.set_standard_projection(V3dTypeOfOrientation::Zpos)?;
    session.fit()?;
    Ok(passed)
}
pub(super) fn corner_checks(
    reader: &impl PrimitivePixelSource,
    expected: &str,
) -> Result<bool, Failure> {
    let top = region(reader, expected, 0, 0, 1, false)?;
    let bottom = region(reader, expected, 0, 405, -1, true)?;
    let right = region(reader, expected, 402, 201, 1, true)?;
    let left = region(reader, expected, 101, 201, 1, false)?;
    Ok(top && bottom && right && !left)
}
fn region(
    reader: &impl PrimitivePixelSource,
    expected: &str,
    x: i32,
    y: i32,
    y_step: i32,
    initial: bool,
) -> Result<bool, Failure> {
    let mut found = initial;
    for i in 0..3 {
        for j in 0..3 {
            if reader.pixel_text(x + i, y + y_step * j)? == expected {
                found = true;
            }
        }
    }
    Ok(found)
}
#[cfg(test)]
mod controls {
    use super::*;
    use std::cell::RefCell;
    struct NamedPrimitivePixels {
        colored: Vec<(i32, i32)>,
        reads: RefCell<Vec<(i32, i32)>>,
    }
    impl PrimitivePixelSource for NamedPrimitivePixels {
        fn pixel_text(&self, x: i32, y: i32) -> Result<String, Failure> {
            self.reads.borrow_mut().push((x, y));
            Ok(if self.colored.contains(&(x, y)) {
                "1 1 0"
            } else {
                "0 0 0"
            }
            .to_owned())
        }
    }
    #[test]
    fn unused_source_helper_preserves_all36_reads_and_always_true_middle_regions() {
        let reader = NamedPrimitivePixels {
            colored: vec![(0, 0)],
            reads: RefCell::new(vec![]),
        };
        assert!(corner_checks(&reader, "1 1 0").unwrap());
        assert_eq!(reader.reads.borrow().len(), 36);
        assert!(reader.reads.borrow().contains(&(2, 403)));
    }
    #[test]
    fn unused_source_helper_refuses_missing_first_color_or_present_left_color() {
        for colored in [vec![], vec![(0, 0), (101, 201)]] {
            let reader = NamedPrimitivePixels {
                colored,
                reads: RefCell::new(vec![]),
            };
            assert!(!corner_checks(&reader, "1 1 0").unwrap());
            assert_eq!(reader.reads.borrow().len(), 36);
        }
    }
}
