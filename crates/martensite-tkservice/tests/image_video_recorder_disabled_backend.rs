//! The recorder without a codec backend: open accepts any parameters without allocating a
//! frame, frame pushes fail and the frame count stays zero. The inputs are the dimensions,
//! rates, file names and codec strings of the recorder tests whose bodies need a
//! codec backend; these tests pin the disabled behavior and claim nothing about encoding.
use martensite_tkservice::image::{ImageVideoParams, ImageVideoRecorder};

#[test]
fn recorder_without_codec_backend_accepts_open_without_allocating_frame() {
    let recorder = ImageVideoRecorder::default();
    let params = disabled_backend_params(320, 240, 15);
    assert!(recorder.open("test_video.avi", &params));
    assert_eq!(
        (
            recorder.change_frame().width(),
            recorder.change_frame().height()
        ),
        (0, 0)
    );
    assert!(recorder.change_frame().is_empty());
    assert_eq!(recorder.frame_count(), 0);
    assert_eq!(recorder.get_codec_context(), 0);
    recorder.close();
}

#[test]
fn recorder_without_codec_backend_retains_disabled_invalid_parameter_behavior() {
    let recorder = ImageVideoRecorder::default();
    assert!(recorder.open("invalid_test.avi", &ImageVideoParams::default()));
    assert!(recorder.change_frame().is_empty());
    assert_eq!(recorder.frame_count(), 0);
}

#[test]
fn recorder_without_codec_backend_rejects_each_frame_without_advancing_count() {
    let recorder = ImageVideoRecorder::default();
    let params = disabled_backend_params(160, 120, 10);
    assert!(recorder.open("test_frames.avi", &params));
    for _ in 0..3 {
        assert!(!recorder.push_frame());
        assert_eq!(recorder.frame_count(), 0);
    }
    recorder.close();
    assert_eq!(recorder.frame_count(), 0);
}

fn disabled_backend_params(width: i32, height: i32, fps: i32) -> ImageVideoParams {
    let mut params = ImageVideoParams {
        width,
        height,
        format: "avi".into(),
        video_codec: "mpeg4".into(),
        pixel_format: "yuv420p".into(),
        ..Default::default()
    };
    params.set_framerate(fps);
    params
}
