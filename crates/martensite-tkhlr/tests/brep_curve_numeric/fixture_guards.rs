use super::replay::brep_curve_fixture::{self as fixture, CurveFixtureInput};
use martensite_tkmath::gp::{Direction3, Point3};

pub(crate) struct RecordedCurveInput {
    kind: i32,
    collinear_frame: bool,
    directions_read: usize,
    pub(crate) reads: Vec<&'static str>,
}
impl RecordedCurveInput {
    pub(crate) fn new(kind: i32, collinear_frame: bool) -> Self {
        Self {
            kind,
            collinear_frame,
            directions_read: 0,
            reads: Vec::new(),
        }
    }
}
impl CurveFixtureInput for RecordedCurveInput {
    fn integer(&mut self) -> i32 {
        self.reads.push("kind");
        self.kind
    }
    fn real(&mut self) -> f64 {
        self.reads.push("scalar");
        8.
    }
    fn pnt(&mut self) -> Point3 {
        self.reads.push("point");
        Point3::default()
    }
    fn dir(&mut self) -> Direction3 {
        self.reads.push("direction");
        self.directions_read += 1;
        if self.directions_read == 1 || self.collinear_frame {
            return Direction3::Z;
        }
        Direction3::X
    }
}

#[test]
fn default_projector_skips_all_frame_and_focus_fields() {
    let mut input = RecordedCurveInput::new(2, false);
    let projector = fixture::projector(&mut input).unwrap();
    assert!(!projector.perspective());
    assert_eq!(input.reads, ["kind"]);
}

#[test]
fn perspective_projector_consumes_focus_only_after_a_valid_frame() {
    let mut input = RecordedCurveInput::new(1, false);
    let projector = fixture::projector(&mut input).unwrap();
    assert!(projector.perspective());
    assert_eq!(projector.focus().unwrap(), 8.);
    assert_eq!(
        input.reads,
        ["kind", "point", "direction", "direction", "scalar"]
    );
}

#[test]
fn failed_frames_skip_later_projector_focus_and_conic_radius_reads() {
    let mut projector_input = RecordedCurveInput::new(1, true);
    assert!(fixture::projector(&mut projector_input).is_err());
    assert_eq!(
        projector_input.reads,
        ["kind", "point", "direction", "direction"]
    );
    let mut conic_input = RecordedCurveInput::new(1, true);
    assert!(fixture::geometry(&mut conic_input).is_err());
    assert_eq!(
        conic_input.reads,
        ["kind", "point", "direction", "direction"]
    );
}
