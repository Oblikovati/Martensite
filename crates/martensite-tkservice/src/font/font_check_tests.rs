//! `check_font` over real FreeType faces: the face owner's record borrow on a real font,
//! and the failure paths that must never build an owner (invalid bytes, a missing file).
//! FreeType runs through FFI, so these are native tests rather than Miri tests.
use super::check_font;
use crate::font::FontFTLibrary;
use martensite_tkernel::tcollection::AsciiString;
use std::path::PathBuf;

/// A file with `bytes` in a fresh temporary directory; the directory is removed on drop.
struct TemporaryFile {
    directory: PathBuf,
    path: PathBuf,
}

impl TemporaryFile {
    fn new(name: &str, bytes: &[u8]) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "martensite-font-check-{}-{name}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("temporary directory");
        let path = directory.join(name);
        std::fs::write(&path, bytes).expect("temporary font file");
        Self { directory, path }
    }

    fn text(&self) -> AsciiString {
        AsciiString::from_view(self.path.to_string_lossy().as_bytes())
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn checked(path: &AsciiString) -> (bool, usize) {
    let mut fonts = Vec::new();
    let found = check_font(&mut fonts, &FontFTLibrary::new(), path, -1).expect("check_font");
    (found, fonts.len())
}

#[test]
fn a_real_font_is_described_through_the_borrowed_face_record() {
    let font = TemporaryFile::new("fallback.woff", include_bytes!("dejavu_sans_latin.woff"));
    // Opening and dropping the owner repeatedly exercises FT_New_Face and FT_Done_Face.
    for _ in 0..32 {
        assert_eq!(checked(&font.text()), (true, 1));
    }
}

#[test]
fn invalid_font_bytes_open_no_face() {
    let invalid = TemporaryFile::new("invalid.ttf", b"not a font, only text");
    assert_eq!(checked(&invalid.text()), (false, 0));
}

#[test]
fn a_missing_file_opens_no_face() {
    let missing = AsciiString::from_view(b"/nonexistent/martensite-font-check/missing.ttf");
    assert_eq!(checked(&missing), (false, 0));
}
