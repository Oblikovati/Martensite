use super::super::{BRepFontBuilderTextPlan, BRepFontPlanarRegion};
use super::types::*;
use martensite_tkmath::gp::Xy;
use martensite_tkservice::font::FontGlyphOutline;
use std::{fmt, sync::Arc};

impl fmt::Debug for OwnedFontTextAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFontTextAsset")
            .field("id", &self.id)
            .field("bytes", &self.bytes.len())
            .field("face", &self.face)
            .finish()
    }
}
impl OwnedFontTextAsset {
    /// Freeze exact content and collection face selector; admission occurs at capture.
    /// Example: `OwnedFontTextAsset::new(FontTextAssetId(1), bytes, 0)`.
    pub fn new(id: FontTextAssetId, bytes: Arc<[u8]>, face: i32) -> Self {
        Self { id, bytes, face }
    }
    /// Caller identity, not a content hash; `asset.id()`.
    pub fn id(&self) -> FontTextAssetId {
        self.id
    }
    /// Exact immutable font content; `asset.bytes()`.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Collection face selector actually used to open this memory font; `asset.face()`.
    pub fn face(&self) -> i32 {
        self.face
    }
    /// Verify a persisted binding by exact identity, bytes and face, without a hash registry.
    /// Example: `asset.matches_binding(&other)`.
    pub fn matches_binding(&self, other: &Self) -> bool {
        self == other
    }
}
impl OwnedFontTextGlyph {
    /// Exact content binding retained even when this prototype is cloned; `glyph.asset()`.
    pub fn asset(&self) -> &OwnedFontTextAsset {
        &self.asset
    }
    /// Frozen capture recipe, including design-unit tolerance; `glyph.options()`.
    pub fn options(&self) -> FontTextCaptureOptions {
        self.options
    }
    /// Primary-face glyph index, never zero; `glyph.index()`.
    pub fn index(&self) -> u32 {
        self.index
    }
    /// Raw analytic line/quadratic/cubic contours, before regularization; `glyph.outline()`.
    pub fn outline(&self) -> &FontGlyphOutline {
        &self.outline
    }
    /// Frozen processed-source region, with no claimed raw-segment lineage; `glyph.region()`.
    pub fn region(&self) -> &BRepFontPlanarRegion {
        &self.region
    }
}
impl OwnedFontTextOccurrence {
    /// Caller-authored identity and UTF-8 span; `occurrence.source()`.
    pub fn source(&self) -> &FontTextSource {
        &self.source
    }
    /// Exact source scalar; `occurrence.scalar()`.
    pub fn scalar(&self) -> char {
        self.scalar
    }
    /// Frozen placement in model units; `occurrence.position()`.
    pub fn position(&self) -> &Xy {
        &self.position
    }
    /// Successful ink/no-ink distinction; `occurrence.kind()`.
    pub fn kind(&self) -> FontTextOccurrenceKind {
        self.kind
    }
    /// Immutable prototype for a resolved glyph; `occurrence.glyph()`.
    pub fn glyph(&self) -> Option<&OwnedFontTextGlyph> {
        self.glyph.as_deref()
    }
}
impl OwnedFontTextCapture {
    /// Authoritative font bytes and face binding; `capture.asset()`.
    pub fn asset(&self) -> &OwnedFontTextAsset {
        &self.asset
    }
    /// Frozen source and effective settings; `capture.request()`.
    pub fn request(&self) -> &FontTextCaptureRequest {
        &self.request
    }
    /// Source-ordered occurrences, including successful no-ink outcomes; `capture.occurrences()`.
    pub fn occurrences(&self) -> &[OwnedFontTextOccurrence] {
        &self.occurrences
    }
    /// Existing modeling-builder input; `BRepFontBuilder::build_text(capture.plan())`.
    pub fn plan(&self) -> &BRepFontBuilderTextPlan {
        &self.plan
    }
    /// Actual retained counts and completed checkpoints; `capture.work()`.
    pub fn work(&self) -> FontTextCaptureWork {
        self.work
    }
    /// Frozen recipe version; regenerate only by an explicit new capture.
    /// Example: `assert_eq!(capture.recipe_version(), 1)`.
    pub fn recipe_version(&self) -> u32 {
        1
    }
}

impl fmt::Display for FontTextCaptureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "owned font text capture refused: {:?}",
            self.failure
        )
    }
}
impl std::error::Error for FontTextCaptureError {}
