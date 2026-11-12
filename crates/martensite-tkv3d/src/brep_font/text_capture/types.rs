use super::super::{BRepFontBuilderTextPlan, BRepFontPlanarRegion, BRepFontRegularizerStatus};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Xy;
use martensite_tkservice::font::FontGlyphOutline;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment, Graphic3dVerticalTextAlignment,
};
use std::{ops::Range, sync::Arc};

/// Application-assigned asset identity; content equality also requires exact bytes and face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontTextAssetId(pub u64);
/// Application-assigned occurrence identity, unique within one capture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontTextOccurrenceId(pub u64);
/// Immutable font content. No path, discovery fallback, or backend handle is retained.
#[derive(Clone, PartialEq, Eq)]
pub struct OwnedFontTextAsset {
    pub(super) id: FontTextAssetId,
    pub(super) bytes: Arc<[u8]>,
    pub(super) face: i32,
}
/// One caller-authored identity and exactly one UTF-8 scalar span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontTextSource {
    /// Stable identity supplied by the caller.
    pub id: FontTextOccurrenceId,
    /// Half-open byte span, in source order with complete coverage.
    pub utf8_span: Range<usize>,
}
/// Bounds on retained input/output counts and cooperative checkpoints, not native call cost.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontTextCaptureLimits {
    /// Maximum font content bytes.
    pub font_bytes: usize,
    /// Maximum UTF-8 text bytes.
    pub text_bytes: usize,
    /// Maximum scalar occurrences.
    pub sources: usize,
    /// Maximum distinct resolved glyphs.
    pub glyphs: usize,
    /// Maximum retained raw segments across distinct glyphs.
    pub segments: usize,
    /// Maximum retained raw contours across distinct glyphs.
    pub contours: usize,
    /// Maximum logical control points, counting segment endpoints independently.
    pub points: usize,
    /// Maximum vertices, curves, uses, loops, holes and material regions retained together.
    pub processed_entities: usize,
    /// Maximum cooperative checkpoints.
    pub checkpoints: usize,
}
/// Explicit layout and frozen-region recipe; primary face only, no hinting or synthetic style.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontTextCaptureOptions {
    /// Model height, finite and positive.
    pub size: f64,
    /// Model topology tolerance, finite and positive.
    pub tolerance: f64,
    /// Frozen regularizer tolerance in font design units, finite and positive.
    pub outline_tolerance: f64,
    /// Horizontal scale, finite and positive.
    pub width_scaling: f32,
    /// Horizontal anchor.
    pub horizontal: Graphic3dHorizontalTextAlignment,
    /// Vertical anchor.
    pub vertical: Graphic3dVerticalTextAlignment,
    /// Requested retained-storage and checkpoint limits.
    pub limits: FontTextCaptureLimits,
}
/// Owned text, caller identities and effective settings for one strict capture.
#[derive(Clone, Debug)]
pub struct FontTextCaptureRequest {
    /// Exact UTF-8 source. Only space and LF are admitted control/separator characters.
    pub text: Arc<str>,
    /// Complete scalar coverage in source order.
    pub sources: Vec<FontTextSource>,
    /// Explicit operation settings.
    pub options: FontTextCaptureOptions,
}
/// Counts charged once per retained prototype, plus operation checkpoints.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FontTextCaptureWork {
    /// Completed cooperative checkpoints.
    pub checkpoints: usize,
    /// Distinct raw glyph prototypes.
    pub glyphs: usize,
    /// Raw segments.
    pub segments: usize,
    /// Raw contours.
    pub contours: usize,
    /// Logical control points.
    pub points: usize,
    /// Frozen processed entities.
    pub processed_entities: usize,
}
/// Explicit successful no-ink outcomes remain distinct from missing glyph failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontTextOccurrenceKind {
    /// Nonempty raw glyph outline.
    Outlined,
    /// Source U+0020, no glyph requested.
    Space,
    /// Source LF, no glyph requested.
    LineBreak,
    /// Resolved nonzero glyph with an empty raw outline.
    EmptyGlyph,
}
/// Frozen processed source boundary, not an original-segment lineage receipt (#425).
#[derive(Clone, Debug)]
pub struct OwnedFontTextGlyph {
    pub(super) asset: OwnedFontTextAsset,
    pub(super) options: FontTextCaptureOptions,
    pub(super) index: u32,
    pub(super) outline: FontGlyphOutline,
    pub(super) region: Arc<BRepFontPlanarRegion>,
}
/// One source occurrence and its immutable glyph prototype, if any.
#[derive(Clone, Debug)]
pub struct OwnedFontTextOccurrence {
    pub(super) source: FontTextSource,
    pub(super) scalar: char,
    pub(super) position: Xy,
    pub(super) kind: FontTextOccurrenceKind,
    pub(super) glyph: Option<Arc<OwnedFontTextGlyph>>,
}
/// Published snapshot; replay uses these frozen outlines, placements and regions.
#[derive(Clone, Debug)]
pub struct OwnedFontTextCapture {
    pub(super) asset: OwnedFontTextAsset,
    pub(super) request: FontTextCaptureRequest,
    pub(super) occurrences: Vec<OwnedFontTextOccurrence>,
    pub(super) plan: BRepFontBuilderTextPlan,
    pub(super) work: FontTextCaptureWork,
}
/// Failure class retains the offending values; backend rejection has no guessed cause.
#[derive(Clone, Debug)]
pub enum FontTextCaptureFailure {
    /// Invalid setting and required shape.
    InvalidInput {
        /// Setting whose supplied value was refused.
        field: &'static str,
        /// Offending supplied value or source binding.
        observed: String,
        /// Required input shape.
        expected: &'static str,
    },
    /// Missing or malformed scalar source binding.
    Source {
        /// Zero-based source scalar position.
        ordinal: usize,
        /// Offending supplied value or source binding.
        observed: Option<FontTextSource>,
        /// Required input shape.
        expected: &'static str,
    },
    /// A bound source scalar is outside the admitted control-character policy.
    UnsupportedScalar {
        /// Exact caller identity and UTF-8 span, retained after consuming the request.
        source: FontTextSource,
        /// Offending source scalar.
        scalar: char,
        /// Required scalar policy.
        expected: &'static str,
    },
    /// Requested retained count or checkpoint limit was exceeded.
    Limit {
        /// Retained-count or checkpoint resource name.
        resource: &'static str,
        /// Offending supplied value or source binding.
        observed: usize,
        /// Requested maximum count.
        maximum: usize,
    },
    /// Cooperative cancellation was requested.
    Cancelled,
    /// FreeType could not be initialized.
    BackendUnavailable,
    /// Memory face open/configuration rejected this content and face selector.
    FaceRejected {
        /// Requested collection face selector.
        face: i32,
        /// Exact input font byte count.
        bytes: usize,
    },
    /// No primary glyph was resolved; glyph zero is never published as success.
    MissingGlyph {
        /// Caller-authored occurrence identity.
        source: FontTextOccurrenceId,
        /// Exact source scalar.
        scalar: char,
    },
    /// Backend rejected a resolved nonzero outline.
    OutlineRejected {
        /// Caller-authored occurrence identity.
        source: FontTextOccurrenceId,
        /// Resolved nonzero primary-face glyph index.
        glyph: u32,
    },
    /// Existing regularizer refused this outline.
    Regularizer {
        /// Caller-authored occurrence identity.
        source: FontTextOccurrenceId,
        /// Resolved nonzero primary-face glyph index.
        glyph: u32,
        /// Existing regularizer refusal status.
        status: BRepFontRegularizerStatus,
    },
    /// Existing allocation/layout API returned its original typed failure.
    Native {
        /// Existing allocation or layout operation.
        operation: &'static str,
        /// Original owned typed failure.
        cause: Box<Failure>,
    },
    /// Existing builder rejected the complete frozen plan.
    PlanRejected,
}
/// Failure carries requested settings and work completed before refusal; never partial success.
#[derive(Clone, Debug)]
pub struct FontTextCaptureError {
    /// Typed failure and source context.
    pub failure: FontTextCaptureFailure,
    /// Requested limits and counts at refusal.
    pub diagnostics: Box<FontTextCaptureDiagnostics>,
}

/// Retained refusal context, separate from its typed cause.
#[derive(Clone, Debug)]
pub struct FontTextCaptureDiagnostics {
    /// Caller asset identity for the refused operation.
    pub asset_id: FontTextAssetId,
    /// Requested collection face selector.
    pub face: i32,
    /// Exact font input length.
    pub font_bytes: usize,
    /// Exact UTF-8 input length.
    pub text_bytes: usize,
    /// Requested scalar source records.
    pub sources: usize,
    /// Original requested limits.
    pub limits: FontTextCaptureLimits,
    /// Counters at refusal, not a temporary-allocation peak.
    pub work: FontTextCaptureWork,
}
