//! Behaviour tests of `Image_VideoRecorder`.
//!
//! OpenVideoFile, InvalidParameters and WriteFrames test the codec backend (their bodies are
//! `#ifdef HAVE_FFMPEG`); the media backend is not part of Martensite, so they are
//! not-applicable rows. image_video_recorder_disabled_backend.rs pins the recorder without
//! a backend.
#![allow(non_snake_case, non_camel_case_types)]
use martensite_tkernel::standard::Handle;
use martensite_tkservice::image::{ImageVideoParams, ImageVideoRecorder};

/// The fixture: a recorder created in SetUp and closed in TearDown.
struct Image_VideoRecorderTest {
    myRecorder: Handle<ImageVideoRecorder>,
}
impl Image_VideoRecorderTest {
    fn set_up() -> Self {
        Self {
            myRecorder: Handle::new(ImageVideoRecorder::default()),
        }
    }
}
impl Drop for Image_VideoRecorderTest {
    fn drop(&mut self) {
        if let Some(recorder) = self.myRecorder.get() {
            recorder.close();
        }
    }
}

#[test]
fn Image_VideoRecorderTest__DefaultConstructor() {
    let fixture = Image_VideoRecorderTest::set_up();
    assert!(!fixture.myRecorder.is_null());
    assert_eq!(0, fixture.myRecorder.get().unwrap().frame_count());
}

#[test]
fn Image_VideoRecorderTest__VideoParamsStructure() {
    let _fixture = Image_VideoRecorderTest::set_up();
    let mut params = ImageVideoParams::default();
    assert_eq!(0, params.width);
    assert_eq!(0, params.height);
    assert_eq!(0, params.fps_num);
    assert_eq!(1, params.fps_den);
    assert!(params.format.as_bytes().is_empty());
    assert!(params.video_codec.as_bytes().is_empty());
    assert!(params.pixel_format.as_bytes().is_empty());
    params.set_framerate(30);
    assert_eq!(30, params.fps_num);
    assert_eq!(1, params.fps_den);
    params.set_framerate_fraction(25, 2);
    assert_eq!(25, params.fps_num);
    assert_eq!(2, params.fps_den);
}

#[test]
fn Image_VideoRecorderTest__CloseWithoutOpen() {
    let fixture = Image_VideoRecorderTest::set_up();
    let recorder = fixture.myRecorder.get().unwrap();
    recorder.close();
    assert_eq!(0, recorder.frame_count());
}
