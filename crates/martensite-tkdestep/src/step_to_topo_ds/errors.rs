//! The error codes of the translators and their decoding (`StepToTopoDS`).
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::HAsciiString;

/// Declares an error enumeration of two values.
macro_rules! translator_error_enum {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum $name {
            /// The translation is done.
            Done,
            /// The translation failed.
            #[default]
            Other,
        }
    };
}

/// Declares an error enumeration of two values and the member that decodes it.
macro_rules! translator_error {
    ($(#[$meta:meta])* $name:ident, $decode:ident, $done_text:literal, $other_text:literal) => {
        translator_error_enum!($(#[$meta])* $name);
        impl StepToTopoDS {
            #[doc = concat!("`", stringify!($decode), "`: the text of the error.")]
            pub fn $decode(error: $name) -> Handle<HAsciiString> {
                let text = match error {
                    $name::Done => $done_text,
                    $name::Other => $other_text,
                };
                Handle::new(HAsciiString::new(text))
            }
        }
    };
}

/// `StepToTopoDS`: the static decoders of the error codes.
///
/// ```
/// use martensite_tkdestep::step_to_topo_ds::{StepToTopoDS, StepToTopoDSBuilderError};
/// use martensite_tkernel::tcollection::AsciiString;
/// let text = StepToTopoDS::decode_builder_error(StepToTopoDSBuilderError::Done);
/// assert_eq!(*text.get().unwrap().string(), AsciiString::new("Builder Done"));
/// ```
pub struct StepToTopoDS;

translator_error!(
    /// Builder error.
    StepToTopoDSBuilderError, decode_builder_error, "Builder Done", "Builder Other"
);
translator_error!(
    /// Translate shell error.
    StepToTopoDSTranslateShellError, decode_shell_error, "Translate Shell Done", "Translate Shell Other"
);
translator_error!(
    /// Translate face error.
    StepToTopoDSTranslateFaceError, decode_face_error, "Translate Face Done", "Translate Face Other"
);
translator_error!(
    /// Translate edge error.
    StepToTopoDSTranslateEdgeError, decode_edge_error, "Translate Edge Done", "Translate Edge Other"
);
translator_error!(
    /// Translate vertex error.
    StepToTopoDSTranslateVertexError, decode_vertex_error, "Translate Vertex Done", "Translate Vertex Other"
);
translator_error!(
    /// Translate vertex loop error.
    StepToTopoDSTranslateVertexLoopError, decode_vertex_loop_error, "Translate VertexLoop Done", "Translate VertexLoop Other"
);
translator_error!(
    /// Translate poly loop error.
    StepToTopoDSTranslatePolyLoopError, decode_poly_loop_error, "Translate PolyLoop Done", "Translate PolyLoop Other"
);
translator_error_enum!(
    /// `StepToTopoDSTranslateEdgeLoopError`: without a decoder.
    StepToTopoDSTranslateEdgeLoopError
);
translator_error_enum!(
    /// `StepToTopoDSTranslateSolidError`: without a decoder.
    StepToTopoDSTranslateSolidError
);

/// Geometric tool error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepToTopoDSGeometricToolError {
    /// The computation is done.
    Done,
    /// The edge is degenerated.
    IsDegenerated,
    /// The pcurve is missing.
    HasNoPCurve,
    /// The 3D parameters are wrong.
    Wrong3dParameters,
    /// A point has no projection on the curve.
    NoProjectiOnCurve,
    /// Another failure.
    #[default]
    Other,
}

impl StepToTopoDS {
    /// `decode_geometric_tool_error`: the text of the error.
    pub fn decode_geometric_tool_error(error: StepToTopoDSGeometricToolError) -> &'static str {
        match error {
            StepToTopoDSGeometricToolError::Done => GEOMETRIC_TOOL_TEXTS[0],
            StepToTopoDSGeometricToolError::IsDegenerated => GEOMETRIC_TOOL_TEXTS[1],
            StepToTopoDSGeometricToolError::HasNoPCurve => GEOMETRIC_TOOL_TEXTS[2],
            StepToTopoDSGeometricToolError::Wrong3dParameters => GEOMETRIC_TOOL_TEXTS[3],
            StepToTopoDSGeometricToolError::NoProjectiOnCurve => GEOMETRIC_TOOL_TEXTS[4],
            StepToTopoDSGeometricToolError::Other => GEOMETRIC_TOOL_TEXTS[5],
        }
    }
}

/// The texts of `decode_geometric_tool_error`, in the order of the enumeration.
const GEOMETRIC_TOOL_TEXTS: [&str; 6] = [
    " Geometric Tool is done",
    " an Edge is degenerated",
    " SurfaceCurve does not contain a PCurve lying on the BasisSurface",
    " the update of 3D-Parameters failed",
    " the projection of a VertexPoint on the curve3d failed",
    " GeometricTool failed",
];
