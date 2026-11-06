//! `TopoDSToStep` (the static decoders and `AddResult`), `TopoDSToStepRoot` and the error codes.
use super::TopoDSToStepTool;
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::transfer::{TransferFinderProcess, TransferSimpleBinderOfTransient};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeMapper};

/// Declares an error enumeration, its default (the constructors start at the error), and
/// the member that decodes it into the text.
macro_rules! writer_error {
    ($(#[$meta:meta])* $name:ident, $decode:ident, $default:ident, { $($value:ident => $text:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = $text]
                $value,
            )+
        }
        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }
        impl TopoDSToStep {
            #[doc = concat!("`", stringify!($decode), "`: the text of the error.")]
            pub fn $decode(error: $name) -> Handle<HAsciiString> {
                let text = match error {
                    $($name::$value => $text,)+
                };
                Handle::new(HAsciiString::new(text))
            }
        }
    };
}

/// `TopoDSToStep`: the static decoders of the error codes, and `AddResult`.
///
/// ```
/// use martensite_tkdestep::topo_ds_to_step::{TopoDSToStep, TopoDSToStepBuilderError};
/// let text = TopoDSToStep::decode_builder_error(TopoDSToStepBuilderError::NoFaceMapped);
/// assert_eq!(text.string().as_bytes(), b"None of the Shell Faces has been mapped");
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct TopoDSToStep;

writer_error!(
    /// Builder error.
    TopoDSToStepBuilderError, decode_builder_error, BuilderOther, {
        BuilderDone => "Builder Done",
        NoFaceMapped => "None of the Shell Faces has been mapped",
        BuilderOther => "Other Error in Builder",
    }
);
writer_error!(
    /// Make face error.
    TopoDSToStepMakeFaceError, decode_face_error, FaceOther, {
        FaceDone => "Face Done",
        InfiniteFace => "The Face has no Outer Wire",
        NonManifoldFace => "The Face is Internal or External",
        NoWireMapped => "None of the Face Wires has been mapped",
        FaceOther => "Other Error in Make STEP face",
    }
);
writer_error!(
    /// Make wire error.
    TopoDSToStepMakeWireError, decode_wire_error, WireOther, {
        WireDone => "Wire Done",
        NonManifoldWire => "The Wire is Internal or External",
        WireOther => "Other Error in Make STEP wire",
    }
);
writer_error!(
    /// Make edge error.
    TopoDSToStepMakeEdgeError, decode_edge_error, EdgeOther, {
        EdgeDone => "Edge Done",
        NonManifoldEdge => "The Edge is Internal or External",
        EdgeOther => "Other Error in Make STEP Edge",
    }
);
writer_error!(
    /// Make vertex error.
    TopoDSToStepMakeVertexError, decode_vertex_error, VertexOther, {
        VertexDone => "Vertex Done",
        VertexOther => "Other Error in Make STEP Vertex",
    }
);

/// `TopoDSToStepFacetedError`: why a shape cannot be written as a faceted model.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TopoDSToStepFacetedError {
    /// The shape is faceted.
    #[default]
    FacetedDone,
    /// A surface is neither a plane nor a flat B-spline or Bezier surface.
    SurfaceNotPlane,
    /// A curve on a surface is not linear.
    PCurveNotLinear,
}

impl TopoDSToStep {
    /// `AddResult(FP, Shape, ent)`: binds `ent` to the shape's mapper, after what it already has.
    pub fn add_result(
        fp: &Handle<TransferFinderProcess>,
        shape: &Shape,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        let result = TransferSimpleBinderOfTransient::new();
        result.set_result(ent)?;
        let result = Handle::new(result);
        let mapper = TransferBRep::shape_mapper(fp, shape)?;
        let process = step_live(fp, "TopoDSToStep::AddResult")?;
        let binder = process.find(&mapper.cast());
        match binder.get() {
            None => process.bind(&mapper.cast(), &result.cast()),
            Some(live) => {
                live.add_result(&result.cast());
                Ok(())
            }
        }
    }
    /// `AddResult(FP, Tool)`: every entity of the tool's map.
    pub fn add_tool_results(
        fp: &Handle<TransferFinderProcess>,
        tool: &TopoDSToStepTool,
    ) -> Result<(), Failure> {
        for (shape, ent) in tool.map().items() {
            Self::add_result(fp, shape, ent)?;
        }
        Ok(())
    }
}

/// `TopoDSToStepRoot`: the state common to the translators; a tolerance of 0.0001 by default.
///
/// ```
/// use martensite_tkdestep::topo_ds_to_step::TopoDSToStepRoot;
/// let mut root = TopoDSToStepRoot::new();
/// assert!(!root.is_done() && root.tolerance() == 0.0001);
/// *root.tolerance_mut() = 0.5;
/// assert_eq!(root.tolerance(), 0.5);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TopoDSToStepRoot {
    pub(super) done: bool,
    toler: f64,
}

impl Default for TopoDSToStepRoot {
    fn default() -> Self {
        Self {
            done: false,
            toler: 0.0001,
        }
    }
}

impl TopoDSToStepRoot {
    /// `TopoDSToStepRoot()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `tolerance`, to read.
    pub fn tolerance(&self) -> f64 {
        self.toler
    }
    /// `tolerance`, to change: the member returns a reference.
    pub fn tolerance_mut(&mut self) -> &mut f64 {
        &mut self.toler
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// a not done failure while the translator named `member` has no result.
    pub(crate) fn check_done(&self, member: &str) -> Result<(), Failure> {
        if self.done {
            return Ok(());
        }
        Err(martensite_tkernel::std_fail::not_done(&format!(
            "{member} - no result"
        )))
    }
}

/// `FP->AddWarning(new TransferBRepShapeMapper(shape), message)`.
pub(crate) fn add_warning(
    fp: &Handle<TransferFinderProcess>,
    shape: &Shape,
    message: &str,
) -> Result<(), Failure> {
    let mapper = Handle::new(TransferBRepShapeMapper::new(shape));
    step_live(fp, "TopoDSToStep")?.add_warning_text(&mapper.cast(), message, "");
    Ok(())
}

/// `down_cast<StepDataStepModel>(FP->Model())`; a null one is the null dereference.
pub(crate) fn step_model_of(
    fp: &Handle<TransferFinderProcess>,
) -> Result<Handle<StepDataStepModel>, Failure> {
    let model: Handle<StepDataStepModel> = step_live(fp, "TopoDSToStep")?.model().cast();
    step_live(&model, "TopoDSToStep: the model of the finder process")?;
    Ok(model)
}
