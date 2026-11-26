use super::*;
use std::{cell::RefCell, collections::BTreeMap, path::PathBuf};
#[derive(Default)]
struct FakeFrameImageStore {
    files: RefCell<BTreeMap<PathBuf, Vec<u8>>>,
    fail_write: bool,
    fail_rename: bool,
}
impl FrameImageStore for FakeFrameImageStore {
    fn write(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        if self.fail_write {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "named fake denied write",
            ));
        }
        self.files.borrow_mut().insert(path.into(), bytes.into());
        Ok(())
    }
    fn write_image(&self, path: &Path, image: &ImagePixMap) -> io::Result<()> {
        assert!(image.is_top_down());
        assert_eq!((image.width(), image.height()), (1, 1));
        assert_eq!(image.pixel_color(0, 0, false).unwrap().rgb().green(), 1.);
        self.write(path, b"named fake serialized image")
    }
    fn rename(&self, temporary: &Path, target: &Path) -> io::Result<()> {
        if self.fail_rename {
            return Err(io::Error::other("named fake denied rename"));
        }
        let bytes = self
            .files
            .borrow_mut()
            .remove(temporary)
            .expect("complete temporary image");
        self.files.borrow_mut().insert(target.into(), bytes);
        Ok(())
    }
}
fn captured_green() -> FrameCapture {
    FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![0, 255, 0, 255],
        depth: vec![1.],
    }
}
#[test]
fn publishes_owned_color_depth_and_image_without_partial_targets() {
    let store = FakeFrameImageStore::default();
    publish_capture(Path::new("frame"), &captured_green(), &store).unwrap();
    let files = store.files.borrow();
    assert_eq!(files.len(), 3);
    assert_eq!(files[Path::new("frame.rgba")], [0, 255, 0, 255]);
    assert_eq!(files[Path::new("frame.depth")], 1_f32.to_le_bytes());
    assert_eq!(
        files[Path::new("frame.ppm")],
        b"named fake serialized image"
    );
}
#[test]
fn failure_and_malformed_capture_never_publish_success() {
    let store = FakeFrameImageStore {
        fail_write: true,
        ..Default::default()
    };
    let error = publish_capture(Path::new("frame"), &captured_green(), &store).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    let mut malformed = captured_green();
    malformed.rgba.clear();
    let error = publish_capture(Path::new("frame"), &malformed, &store).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert!(store.files.borrow().is_empty());
}
#[test]
fn failed_rename_preserves_existing_published_bytes() {
    let store = FakeFrameImageStore {
        fail_rename: true,
        ..Default::default()
    };
    store
        .files
        .borrow_mut()
        .insert("frame.rgba".into(), vec![9; 4]);
    assert!(publish_capture(Path::new("frame"), &captured_green(), &store).is_err());
    assert_eq!(store.files.borrow()[Path::new("frame.rgba")], [9; 4]);
    assert!(!store.files.borrow().contains_key(Path::new("frame.ppm")));
}
#[test]
fn published_image_storage_is_isolated_from_source_mutation() {
    let mut frame = captured_green();
    let image = owned_capture_image(&frame).unwrap();
    frame.rgba.fill(0);
    assert!(image.is_top_down());
    assert_eq!(image.pixel_color(0, 0, false).unwrap().rgb().green(), 1.);
}
#[test]
fn extreme_extent_returns_invalid_data_without_publishing() {
    let store = FakeFrameImageStore::default();
    let frame = FrameCapture {
        width: u32::MAX,
        height: u32::MAX,
        rgba: Vec::new(),
        depth: Vec::new(),
    };
    let error = publish_capture(Path::new("extreme"), &frame, &store).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("extent=4294967295x4294967295"));
    assert!(
        error
            .to_string()
            .contains("representable usize RGBA byte length")
    );
    assert!(store.files.borrow().is_empty());
}
#[test]
fn malformed_depth_reports_actual_lengths_and_expected_shape() {
    let store = FakeFrameImageStore::default();
    let mut frame = captured_green();
    frame.depth.clear();
    let error = publish_capture(Path::new("depth"), &frame, &store).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "RGBA bytes=4, depth values=0, extent=1x1; expected 4 RGBA bytes and 1 depth values for positive extent"
    );
    assert!(store.files.borrow().is_empty());
}
