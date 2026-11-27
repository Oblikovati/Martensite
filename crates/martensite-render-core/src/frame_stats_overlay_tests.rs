//! The counters text aligns toward its persistence corner, as GlFrameStatsPrs::Update.
use super::*;

#[test]
fn default_counters_align_left_top() {
    let params = Graphic3dRenderingParams::new().unwrap();
    let (height, horizontal, vertical) = counters_text_key(&params);
    assert_eq!(height, 16);
    assert_eq!(horizontal, Graphic3dHorizontalTextAlignment::Left);
    assert_eq!(vertical, Graphic3dVerticalTextAlignment::Top);
}
