//! Selection-buffer images and the display of active sensitive entities
//! in the active view.
use super::*;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat};
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorImageMembers as _;
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage;

/// The pixel format of a selection image: float gray for the depth images, BGR
/// for the colour images; `selection_image_format(kind)`.
pub fn selection_image_format(kind: StdSelectTypeOfSelectionImage) -> ImageFormat {
    let depth = [
        StdSelectTypeOfSelectionImage::NORMALIZED_DEPTH,
        StdSelectTypeOfSelectionImage::NORMALIZED_DEPTH_INVERTED,
        StdSelectTypeOfSelectionImage::UNNORMALIZED_DEPTH,
    ];
    if depth.contains(&kind) {
        ImageFormat::GrayF
    } else {
        ImageFormat::Bgr
    }
}

impl ViewerSession {
    /// Pick every pixel of the active view's window and save
    /// the selection image (first picked entity) to `path`, as PPM without an image library.
    /// Example: `session.selection_dump(&path, StdSelectTypeOfSelectionImage::SURFACE_NORMAL)?`.
    pub fn selection_dump(
        &self,
        path: &std::path::Path,
        kind: StdSelectTypeOfSelectionImage,
    ) -> Result<(), Failure> {
        self.selection_dump_in_format(path, kind, selection_image_format(kind))
    }

    /// Without a selection type: the normalized depth, in the BGR image the dump
    /// starts from. Only an explicit depth type allocates float gray; the two differ once
    /// saved, because BGR truncates `value * 255` per pixel when it is filled, while float gray
    /// is scaled over its range and rounded when it is encoded as PNG.
    /// Example: `session.untyped_selection_dump(&path)?`.
    pub fn untyped_selection_dump(&self, path: &std::path::Path) -> Result<(), Failure> {
        let kind = StdSelectTypeOfSelectionImage::NORMALIZED_DEPTH;
        self.selection_dump_in_format(path, kind, ImageFormat::Bgr)
    }

    fn selection_dump_in_format(
        &self,
        path: &std::path::Path,
        kind: StdSelectTypeOfSelectionImage,
        format: ImageFormat,
    ) -> Result<(), Failure> {
        let view = self.view().view_base();
        let (width, height) = view.window().get().map_or((0, 0), |w| w.size());
        let image = ImageAlienPixMap::default();
        if !image.init_zero(format, width as usize, height as usize, 0, 0) {
            return Err(Failure::with_message(
                "Standard_OutOfMemory",
                &format!("selection image {width}x{height}; expected an allocatable size"),
            ));
        }
        let immediate = view.set_immediate_update(false);
        let filled = self
            .context
            .main_selector()
            .to_pix_map(&image, view, kind, 1);
        view.set_immediate_update(immediate);
        if !filled? {
            return Err(Failure::with_message(
                "Standard_Failure",
                "can't generate selection image",
            ));
        }
        let name = path.to_string_lossy();
        image.save(&AsciiString::from(name.as_ref()))?;
        Ok(())
    }

    /// Display the active sensitive entities in the active view.
    /// Example: `session.display_active_sensitive()?`.
    pub fn display_active_sensitive(&self) -> Result<(), Failure> {
        self.context
            .display_active_sensitive(self.view().view_base())
    }
}
