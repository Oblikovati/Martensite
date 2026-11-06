//! Save and consume a real document view without an interpreter or alternate renderer.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkv3d::v3d::V3dViewSource;
use martensite_vulkan::Graphic3dRenderView;
use std::path::PathBuf;

struct SavedDocumentView {
    directory: PathBuf,
}
impl SavedDocumentView {
    fn new(case: &str) -> Self {
        let directory =
            std::env::temp_dir().join(format!("martensite-{case}-{}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        Self { directory }
    }
}
impl Drop for SavedDocumentView {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.directory).unwrap();
    }
}

pub fn read_case_saved_view(view: &Handle<dyn V3dViewSource>, case: &str) -> Vec<u8> {
    read_case_saved_view_named(view, case, &format!("{case}.ppm"))
}

pub fn read_case_saved_view_named(
    view: &Handle<dyn V3dViewSource>,
    case: &str,
    file: &str,
) -> Vec<u8> {
    let rendered = view.get().unwrap().view().cast::<Graphic3dRenderView>();
    assert!(!rendered.is_null());
    rendered.try_redraw().unwrap();
    let saved = SavedDocumentView::new(case);
    let path = saved.directory.join(file);
    let was_immediate = view.set_immediate_update(false);
    let result = view.dump(path.to_str().unwrap(), &Graphic3dBufferType::Rgb);
    view.set_immediate_update(was_immediate);
    assert!(result.unwrap());
    std::fs::read(path).unwrap()
}
