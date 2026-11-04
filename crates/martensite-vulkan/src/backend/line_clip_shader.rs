//! Version-two line interfaces preserve the original stock modules for replay.
use crate::RenderError;
macro_rules! line_pair {
    ($stem:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $stem, ".vert.spv")).as_slice(),
            include_bytes!(concat!(
                "../../shaders/",
                $stem,
                ".viewport-clip-v2.vert.spv"
            ))
            .as_slice(),
        )
    };
}
const LINE_VERTICES: [(&[u8], &[u8]); 8] = [
    line_pair!("line"),
    line_pair!("line.clipped"),
    line_pair!("line.fixed"),
    line_pair!("line.fixed.clipped"),
    line_pair!("lighting.line"),
    line_pair!("lighting.line.clipped"),
    line_pair!("lighting.fixed.line"),
    line_pair!("lighting.fixed.line.clipped"),
];
pub(super) fn select_admitted_line_vertex(
    original: &'static [u8],
    admitted: Result<bool, RenderError>,
) -> Result<&'static [u8], RenderError> {
    if !admitted? {
        return Ok(original);
    }
    viewport_line_vertex(original)
}
fn viewport_line_vertex(original: &[u8]) -> Result<&'static [u8], RenderError> {
    let mut matches = LINE_VERTICES
        .iter()
        .filter(|(before, _)| *before == original);
    let Some((_, selected)) = matches.next() else {
        return Err(RenderError::rejected(
            original.len(),
            "a known stock line vertex module",
        ));
    };
    if matches.any(|(_, candidate)| candidate != selected) {
        return Err(RenderError::rejected(
            original.len(),
            "one unambiguous viewport-clipped line module",
        ));
    }
    Ok(selected)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_stock_line_vertex_selects_its_additive_interface() {
        for (before, after) in LINE_VERTICES {
            assert_eq!(viewport_line_vertex(before).unwrap(), after);
            assert_ne!(before, after);
            assert!(clip_distance_capability(after));
            assert!(!clip_distance_capability(before));
        }
        assert!(viewport_line_vertex(include_bytes!("../../shaders/unlit.vert.spv")).is_err());
    }
    #[test]
    fn refused_admission_never_returns_the_legacy_vertex_module() {
        let before = LINE_VERTICES[0].0;
        let failure = crate::RenderError::rejected(
            "shaderClipDistance=false",
            "admitted viewport clipping without a legacy fallback",
        );
        assert_eq!(
            select_admitted_line_vertex(before, Err(failure.clone())).unwrap_err(),
            failure
        );
        assert_eq!(
            select_admitted_line_vertex(before, Ok(false)).unwrap(),
            before
        );
        assert_eq!(
            select_admitted_line_vertex(before, Ok(true)).unwrap(),
            LINE_VERTICES[0].1
        );
    }
    fn clip_distance_capability(bytes: &[u8]) -> bool {
        let words: Vec<u32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|part| u32::from_le_bytes(*part))
            .collect();
        let mut cursor = 5;
        while cursor < words.len() {
            let count = (words[cursor] >> 16) as usize;
            assert!(count > 0 && cursor + count <= words.len());
            if words[cursor] & 0xffff == 17 && words[cursor + 1] == 32 {
                return true;
            }
            cursor += count;
        }
        false
    }
}
