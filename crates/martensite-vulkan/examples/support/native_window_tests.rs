#[test]
fn screen_masks_decode_channels_independently() {
    assert_eq!(super::channel(0xff8800, 0xff0000), 255);
    assert_eq!(super::channel(0xff8800, 0xff00), 136);
    assert_eq!(super::channel(0, 0), 0);
}
