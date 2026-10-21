//! Source-ordered face/shell dispatch, private publication and partial-state controls.
use super::ObservedCylinder;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkprim::brep_prim::{BRepPrimCylinder, BRepPrimOneAxis};

#[derive(Clone, Copy, Debug)]
enum Part {
    Shell,
    Lateral,
    Top,
    Bottom,
    Start,
    End,
}
impl Part {
    fn build(self, cylinder: &mut ObservedCylinder) -> Result<Shape, Failure> {
        match self {
            Self::Shell => Ok(cylinder.shell()?.into()),
            Self::Lateral => Ok(cylinder.lateral_face()?.into()),
            Self::Top => Ok(cylinder.top_face()?.into()),
            Self::Bottom => Ok(cylinder.bottom_face()?.into()),
            Self::Start => Ok(cylinder.start_face()?.into()),
            Self::End => Ok(cylinder.end_face()?.into()),
        }
    }
    fn expected_orientation(self) -> Orientation {
        match self {
            Self::Bottom | Self::End => Orientation::Reversed,
            _ => Orientation::Forward,
        }
    }
    fn expected_counts(self) -> [usize; 9] {
        match self {
            Self::Shell => [3, 3, 14, 8, 4, 3, 14, 1, 0],
            Self::Lateral => [0, 0, 9, 8, 4, 3, 10, 1, 0],
            Self::Top => [2, 0, 4, 3, 0, 0, 7, 0, 0],
            Self::Bottom => [0, 2, 4, 3, 0, 0, 7, 0, 0],
            Self::Start | Self::End => [2, 2, 8, 9, 14, 12, 38, 0, 1],
        }
    }
    fn guard_name(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            _ => "sides",
        }
    }
    fn early_calls(self) -> &'static [&'static str] {
        match self {
            Self::Shell | Self::Lateral => &["empty_lateral"],
            Self::Top => &["top", "value"],
            Self::Bottom => &["bottom", "value"],
            Self::Start | Self::End => &["sides", "max", "min", "value"],
        }
    }
    fn fixture(self, height: f64) -> ObservedCylinder {
        let mut cylinder = ObservedCylinder::new(height, false);
        if matches!(self, Self::Start | Self::End) {
            cylinder.set_angle(std::f64::consts::PI).unwrap();
        }
        cylinder
    }
}
fn assert_part_counts(cylinder: &ObservedCylinder, part: Part) {
    let hooks = [
        "top",
        "bottom",
        "sides",
        "closed",
        "max",
        "min",
        "value",
        "empty_lateral",
        "pcurve",
    ];
    let calls = cylinder.virtual_calls.borrow();
    let actual = hooks.map(|hook| calls.iter().filter(|call| **call == hook).count());
    assert_eq!(actual, part.expected_counts(), "{part:?}: {calls:?}");
}
fn cached_part(cylinder: &mut ObservedCylinder, part: Part) -> Shape {
    let shape = part.build(cylinder).unwrap();
    let calls = cylinder.virtual_calls.borrow().clone();
    assert!(part.build(cylinder).unwrap().is_same(&shape));
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    shape
}
fn assert_locked(cylinder: &mut ObservedCylinder) {
    let failure = cylinder.set_angle(1.0).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(!cylinder.virtual_calls.borrow().is_empty());
}
pub(super) fn topology_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while explorer.more() {
        count += 1;
        explorer.next().unwrap();
    }
    count
}
fn first_wire(shape: &Shape) -> Shape {
    let explorer = Explorer::new(shape, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    assert!(explorer.more());
    explorer.current().clone()
}
#[test]
fn completed_faces_and_shell_publish_stable_identity_and_parameter_locks() {
    for part in [
        Part::Shell,
        Part::Lateral,
        Part::Top,
        Part::Bottom,
        Part::Start,
        Part::End,
    ] {
        let mut cylinder = part.fixture(2.0);
        let shape = cached_part(&mut cylinder, part);
        assert!(!shape.is_null());
        assert_eq!(shape.orientation(), part.expected_orientation());
        assert_part_counts(&cylinder, part);
        assert_locked(&mut cylinder);
    }
}
#[test]
fn empty_lateral_callback_failure_precedes_queries_and_leaves_state_unlocked() {
    for part in [Part::Shell, Part::Lateral] {
        let mut cylinder = part.fixture(2.0);
        cylinder.fail_lateral_face.set(true);
        assert_eq!(
            part.build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert_eq!(*cylinder.virtual_calls.borrow(), ["empty_lateral"]);
        assert!(cylinder.lateral_faces.borrow().is_empty());
        cylinder.set_angle(std::f64::consts::FRAC_PI_2).unwrap();
        cylinder.fail_lateral_face.set(false);
        assert!(!cached_part(&mut cylinder, part).is_null());
        assert_locked(&mut cylinder);
    }
}
#[test]
fn early_plane_or_height_failure_does_not_publish_or_lock_a_face() {
    for part in [Part::Top, Part::Bottom, Part::Start, Part::End] {
        let mut cylinder = part.fixture(2.0);
        cylinder.fail_meridian_read.set(Some(1));
        assert_eq!(
            part.build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert_eq!(*cylinder.virtual_calls.borrow(), part.early_calls());
        cylinder.set_angle(std::f64::consts::FRAC_PI_2).unwrap();
        cylinder.fail_meridian_read.set(None);
        assert!(!cached_part(&mut cylinder, part).is_null());
        assert_locked(&mut cylinder);
    }
}
#[test]
fn failed_lateral_wire_discards_face_publication_and_retry_uses_a_new_face() {
    let mut cylinder = Part::Lateral.fixture(2.0);
    cylinder.fail_meridian_read.set(Some(1));
    assert!(cylinder.lateral_face().is_err());
    let abandoned = cylinder.lateral_faces.borrow()[0].clone();
    assert_eq!(topology_count(&abandoned, ShapeEnum::Wire), 0);
    cylinder.set_angle(std::f64::consts::PI).unwrap();
    cylinder.fail_meridian_read.set(None);
    let completed = cached_part(&mut cylinder, Part::Lateral);
    assert!(!completed.is_same(&abandoned));
    assert_eq!(topology_count(&completed, ShapeEnum::Wire), 1);
    assert_locked(&mut cylinder);
}
#[test]
fn failed_side_pcurve_retains_partial_vertex_locks_and_reuses_the_completed_wire() {
    for part in [Part::Start, Part::End] {
        let mut cylinder = part.fixture(2.0);
        cylinder.fail_pcurve.set(true);
        assert_eq!(
            part.build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        let abandoned = cylinder.bound_faces.borrow()[0].clone();
        assert_eq!(topology_count(&abandoned, ShapeEnum::Wire), 1);
        assert_locked(&mut cylinder);
        cylinder.fail_pcurve.set(false);
        let completed = cached_part(&mut cylinder, part);
        assert!(!completed.is_same(&abandoned));
        assert!(first_wire(&completed).is_same(&first_wire(&abandoned)));
        assert!(cylinder.bound_faces.borrow()[1].is_same(&completed));
    }
}
#[test]
fn shell_face_predicates_select_actual_caps_and_side_faces() {
    for (angle, caps, expected) in [
        (std::f64::consts::TAU, true, 3),
        (std::f64::consts::PI, true, 5),
        (std::f64::consts::TAU, false, 1),
    ] {
        let mut cylinder = Part::Shell.fixture(2.0);
        cylinder.set_angle(angle).unwrap();
        cylinder.top_exists.set(caps);
        cylinder.bottom_exists.set(caps);
        let shell = cached_part(&mut cylinder, Part::Shell);
        assert_eq!(topology_count(&shell, ShapeEnum::Face), expected);
        assert_eq!(shell.closed().unwrap(), caps);
    }
}
#[test]
fn infinite_lateral_and_side_faces_preserve_two_unbounded_wires() {
    for part in [Part::Lateral, Part::Start, Part::End] {
        let mut cylinder = part.fixture(2.0);
        cylinder.cylinder = BRepPrimCylinder::infinite(1.0);
        cylinder.top_exists.set(false);
        cylinder.bottom_exists.set(false);
        if matches!(part, Part::Start | Part::End) {
            cylinder.set_angle(std::f64::consts::PI).unwrap();
        }
        let face = cached_part(&mut cylinder, part);
        assert_eq!(topology_count(&face, ShapeEnum::Wire), 2);
        assert_eq!(topology_count(&face, ShapeEnum::Vertex), 0);
        assert_locked(&mut cylinder);
    }
}

#[test]
fn side_face_orientation_tracks_height_inversion() {
    for (height, start, end) in [
        (2.0, Orientation::Forward, Orientation::Reversed),
        (-2.0, Orientation::Reversed, Orientation::Forward),
    ] {
        for (part, expected) in [(Part::Start, start), (Part::End, end)] {
            let mut cylinder = part.fixture(height);
            let face = topods::face(&cached_part(&mut cylinder, part)).unwrap();
            assert_eq!(face.orientation(), expected);
        }
    }
}

#[test]
fn absent_cap_and_side_guards_reject_before_evaluation_without_parameter_locks() {
    for part in [Part::Top, Part::Bottom, Part::Start, Part::End] {
        let mut cylinder = part.fixture(2.0);
        cylinder.set_angle(std::f64::consts::TAU).unwrap();
        cylinder.top_exists.set(false);
        cylinder.bottom_exists.set(false);
        assert_eq!(
            part.build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert_eq!(*cylinder.virtual_calls.borrow(), [part.guard_name()]);
        cylinder.set_angle(std::f64::consts::FRAC_PI_2).unwrap();
        cylinder.top_exists.set(true);
        cylinder.bottom_exists.set(true);
        assert!(!cached_part(&mut cylinder, part).is_null());
        assert_locked(&mut cylinder);
    }
}

fn contains_face(shell: &Shape, target: &Shape) -> bool {
    let mut faces = Explorer::new(shell, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        if faces.current().is_same(target) {
            return true;
        }
        faces.next().unwrap();
    }
    false
}
fn retained_shell_faces(cylinder: &mut ObservedCylinder) -> [Shape; 3] {
    let before = cylinder.virtual_calls.borrow().clone();
    let retained = [
        cylinder.lateral_face().unwrap().into(),
        cylinder.top_face().unwrap().into(),
        cylinder.bottom_face().unwrap().into(),
    ];
    assert_eq!(*cylinder.virtual_calls.borrow(), before);
    retained
}
#[test]
fn shell_retry_preserves_completed_face_identities_after_a_later_side_failure() {
    let mut cylinder = Part::Shell.fixture(2.0);
    cylinder.set_angle(std::f64::consts::PI).unwrap();
    cylinder.fail_pcurve.set(true);
    assert_eq!(
        cylinder.shell().unwrap_err().exception_type(),
        "Standard_DomainError"
    );
    let retained = retained_shell_faces(&mut cylinder);
    assert_locked(&mut cylinder);
    cylinder.fail_pcurve.set(false);
    let shell = cached_part(&mut cylinder, Part::Shell);
    for face in retained {
        assert!(contains_face(&shell, &face));
    }
    assert_eq!(topology_count(&shell, ShapeEnum::Face), 5);
}
