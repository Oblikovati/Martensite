//! Retain the authored RasterBox geometry in an owned drawing-only batch context.
use martensite_geometry_raster::BoxGlyph;
use martensite_raster_core::{PaletteColor, PaletteKind, RasterViewer, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bnd::{BoundingBox3, OrientedBoundingBox};
use std::rc::Rc;

pub(super) struct BoundingPreview {
    pub drawable: Rc<BoxGlyph>,
    pub context: RasterViewer,
}
impl BoundingPreview {
    /// Keep the real orange preview through source END; `BoundingPreview::new(&bounds)?`.
    pub(super) fn new(bounds: &BoundingBox3) -> Result<Self, Failure> {
        let drawable = Rc::new(BoxGlyph::new(
            OrientedBoundingBox::from_box(bounds),
            PaletteColor::new(PaletteKind::Orange),
        ));
        // The original reference runs the reference command host -b; its command-host viewer is batch, even with a V3d view.
        let mut context = RasterViewer::with_adapter(None);
        let displayed: Rc<dyn RasterDrawable> = drawable.clone();
        context.display(&displayed)?;
        Ok(Self { drawable, context })
    }
}
