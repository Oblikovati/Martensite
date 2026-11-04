//! Stock interface lookup stays independent of pipeline allocation and behavior selection.
macro_rules! stock_pair {
    ($stem:literal) => {
        [
            include_bytes!(concat!("../../shaders/", $stem, ".vert.spv")).as_slice(),
            include_bytes!(concat!("../../shaders/", $stem, ".frag.spv")).as_slice(),
        ]
    };
}
pub(super) fn clipping_shaders(line: bool, point: bool, text: bool) -> [&'static [u8]; 2] {
    if text {
        return stock_pair!("text.clipped");
    }
    if point {
        return stock_pair!("point.clipped");
    }
    if line {
        return stock_pair!("line.clipped");
    }
    stock_pair!("unlit.clipped")
}
pub(super) fn lighting_shaders(clipped: bool, point: bool, line: bool) -> [&'static [u8]; 2] {
    if line {
        return line_lighting_shaders(clipped);
    }
    if point {
        return point_lighting_shaders(clipped);
    }
    surface_lighting_shaders(clipped)
}
fn line_lighting_shaders(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return stock_pair!("lighting.line.clipped");
    }
    stock_pair!("lighting.line")
}
fn point_lighting_shaders(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return stock_pair!("lighting.point.clipped");
    }
    stock_pair!("lighting.point")
}
fn surface_lighting_shaders(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return stock_pair!("lighting.clipped");
    }
    stock_pair!("lighting")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clipped_interface_priority_preserves_all_legacy_bytes() {
        for line in [false, true] {
            assert_eq!(
                clipping_shaders(line, false, true),
                stock_pair!("text.clipped")
            );
            assert_eq!(
                clipping_shaders(line, true, false),
                stock_pair!("point.clipped")
            );
            assert_eq!(
                clipping_shaders(line, true, true),
                stock_pair!("text.clipped")
            );
        }
        assert_eq!(
            clipping_shaders(true, false, false),
            stock_pair!("line.clipped")
        );
        assert_eq!(
            clipping_shaders(false, false, false),
            stock_pair!("unlit.clipped")
        );
    }
    #[test]
    fn lit_interface_priority_preserves_all_legacy_bytes() {
        for point in [false, true] {
            assert_eq!(
                lighting_shaders(false, point, true),
                stock_pair!("lighting.line")
            );
            assert_eq!(
                lighting_shaders(true, point, true),
                stock_pair!("lighting.line.clipped")
            );
        }
        assert_eq!(
            lighting_shaders(false, true, false),
            stock_pair!("lighting.point")
        );
        assert_eq!(
            lighting_shaders(true, true, false),
            stock_pair!("lighting.point.clipped")
        );
        assert_eq!(
            lighting_shaders(false, false, false),
            stock_pair!("lighting")
        );
        assert_eq!(
            lighting_shaders(true, false, false),
            stock_pair!("lighting.clipped")
        );
    }
}
