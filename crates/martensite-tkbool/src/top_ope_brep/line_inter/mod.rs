//! Intersection-line state and its indexed topological point records.
use super::{TopOpeBRepTypeLineCurve, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchGLine, IntPatchLine, IntPatchRLine, IntPatchWLine},
    int_surf::IntSurfPntOn2S,
};
mod diagnostics;
mod identity;
pub(crate) use identity::LineInterRecordIdentity;
mod geometry;
mod load;
mod points;
mod transitions;

/// Retains a source intersection and independent mutable point classifications.
///
/// `let line = TopOpeBRepLineInter::new(); assert!(line.ok());`
pub struct TopOpeBRepLineInter {
    record_identity: LineInterRecordIdentity,
    source: Handle<dyn IntPatchLine>,
    geometric: Handle<IntPatchGLine>,
    walking: Handle<IntPatchWLine>,
    restriction: Handle<IntPatchRLine>,
    points: Vec<TopOpeBRepVPointInter<'static>>,
    current_walk: Option<IntSurfPntOn2S>,
    faces: [Face; 2],
    transitions: [TopOpeBRepDSTransition; 2],
    kind: TopOpeBRepTypeLineCurve,
    ok: bool,
    index: i32,
    inl: bool,
    closed: bool,
    has_restriction: bool,
    point_bounds: Option<(i32, i32, i32)>,
    trace: (i32, i32),
    null_shape: Shape,
}

impl Default for TopOpeBRepLineInter {
    fn default() -> Self {
        Self {
            record_identity: LineInterRecordIdentity::default(),
            source: Handle::default(),
            geometric: Handle::default(),
            walking: Handle::default(),
            restriction: Handle::default(),
            points: Vec::new(),
            current_walk: None,
            faces: [Face::default(), Face::default()],
            transitions: [TopOpeBRepDSTransition::default(); 2],
            kind: TopOpeBRepTypeLineCurve::OtherType,
            ok: true,
            index: 0,
            inl: false,
            closed: false,
            has_restriction: false,
            point_bounds: None,
            trace: (0, 0),
            null_shape: Shape::default(),
        }
    }
}

impl TopOpeBRepLineInter {
    /// Starts without a source; `Self::new().nb_vpoint() == 0`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets both intersecting topology faces; `line.set_faces(&first, &second)`.
    pub fn set_faces(&mut self, first: &Face, second: &Face) {
        self.faces = [first.clone(), second.clone()];
    }
    /// Reads the classification kind; `line.type_line_curve()`.
    pub fn type_line_curve(&self) -> TopOpeBRepTypeLineCurve {
        self.kind
    }
    /// Counts recorded vertices; `line.nb_vpoint()`.
    pub fn nb_vpoint(&self) -> i32 {
        self.points.len().saturating_sub(1) as i32
    }
    /// Changes the validity flag; `line.set_ok(false)`.
    pub fn set_ok(&mut self, value: bool) {
        self.ok = value;
    }
    /// Reads validity; `line.ok()`.
    pub fn ok(&self) -> bool {
        self.ok
    }
    /// Changes the caller's index; `line.set_index(7)`.
    pub fn set_index(&mut self, value: i32) {
        self.index = value;
    }
    /// Reads the caller's index; `line.index()`.
    pub fn index(&self) -> i32 {
        self.index
    }
    /// Reads the identical-parameter flag; `line.inl()`.
    pub fn inl(&self) -> bool {
        self.inl
    }
    /// Reads geometric endpoint closure; `line.is_v_closed()`.
    pub fn is_v_closed(&self) -> bool {
        self.closed
    }
    /// Reads whether a kept point lies on a restriction; `line.has_vp_on_r()`.
    pub fn has_vp_on_r(&self) -> bool {
        self.has_restriction
    }
    /// Retains trace face indices; `line.set_trace_index(4, 8)`.
    pub fn set_trace_index(&mut self, first: i32, second: i32) {
        self.trace = (first, second);
    }
    /// Reads trace face indices; `line.get_trace_index()`.
    pub fn get_trace_index(&self) -> (i32, i32) {
        self.trace
    }
    /// Exposes the walking-line handle; `line.line_w()`.
    pub fn line_w(&self) -> &Handle<IntPatchWLine> {
        &self.walking
    }
    /// Exposes the geometric-line handle; `line.line_g()`.
    pub fn line_g(&self) -> &Handle<IntPatchGLine> {
        &self.geometric
    }
    /// Exposes the restriction-line handle; `line.line_r()`.
    pub fn line_r(&self) -> &Handle<IntPatchRLine> {
        &self.restriction
    }
}

fn failure(member: &str, value: impl std::fmt::Display) -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        &format!(
            "TopOpeBRepLineInter::{member}: {value}; expected an initialized compatible intersection line"
        ),
    )
}
