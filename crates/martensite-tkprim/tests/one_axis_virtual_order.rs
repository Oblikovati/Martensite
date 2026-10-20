//! The real axis-vertex builder must preserve virtual short-circuit and cache order.
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point2;
use martensite_tkprim::brep_prim::{BRepPrimCylinder, BRepPrimOneAxis, BRepPrimOneAxisBase};
use std::cell::{Cell, RefCell};

struct ObservedCylinder {
    cylinder: BRepPrimCylinder,
    closed: bool,
    infinity_reads: RefCell<Vec<&'static str>>,
    virtual_calls: RefCell<Vec<&'static str>>,
    fail_meridian: Cell<bool>,
    fail_meridian_read: Cell<Option<usize>>,
    meridian_reads: Cell<usize>,
    fail_lateral_face: Cell<bool>,
    fail_pcurve: Cell<bool>,
    fail_after_pcurve: Cell<bool>,
    lateral_faces: RefCell<Vec<Face>>,
    bound_faces: RefCell<Vec<Face>>,
    top_exists: Cell<bool>,
    bottom_exists: Cell<bool>,
}
impl ObservedCylinder {
    fn new(height: f64, closed: bool) -> Self {
        Self {
            cylinder: BRepPrimCylinder::from_radius_height(1.0, height),
            closed,
            infinity_reads: RefCell::new(Vec::new()),
            virtual_calls: RefCell::new(Vec::new()),
            fail_meridian: Cell::new(false),
            fail_meridian_read: Cell::new(None),
            meridian_reads: Cell::new(0),
            fail_lateral_face: Cell::new(false),
            fail_pcurve: Cell::new(false),
            fail_after_pcurve: Cell::new(false),
            lateral_faces: RefCell::new(Vec::new()),
            bound_faces: RefCell::new(Vec::new()),
            top_exists: Cell::new(true),
            bottom_exists: Cell::new(true),
        }
    }
    fn reads(&self) -> Vec<&'static str> {
        self.infinity_reads.borrow().clone()
    }
}
impl BRepPrimOneAxis for ObservedCylinder {
    fn one_axis_base(&self) -> &BRepPrimOneAxisBase {
        self.cylinder.one_axis_base()
    }
    fn one_axis_base_mut(&mut self) -> &mut BRepPrimOneAxisBase {
        self.cylinder.one_axis_base_mut()
    }
    fn make_empty_lateral_face(&self) -> Result<Face, Failure> {
        self.virtual_calls.borrow_mut().push("empty_lateral");
        if self.fail_lateral_face.get() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "ObservedCylinder fail_lateral_face=true; expected an enabled Face builder",
            ));
        }
        let face = self.cylinder.make_empty_lateral_face()?;
        self.lateral_faces.borrow_mut().push(face.clone());
        Ok(face)
    }
    fn make_empty_meridian_edge(&self, angle: f64) -> Result<Edge, Failure> {
        self.cylinder.make_empty_meridian_edge(angle)
    }
    fn set_meridian_pcurve(&self, edge: &Edge, face: &Face) -> Result<(), Failure> {
        self.virtual_calls.borrow_mut().push("pcurve");
        self.bound_faces.borrow_mut().push(face.clone());
        if self.fail_pcurve.get() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "ObservedCylinder fail_pcurve=true; expected enabled face pcurve binding",
            ));
        }
        self.cylinder.set_meridian_pcurve(edge, face)?;
        if self.fail_after_pcurve.get() {
            return Err(pcurve_after_binding_failure());
        }
        Ok(())
    }
    fn meridian_value(&self, parameter: f64) -> Result<Point2, Failure> {
        self.virtual_calls.borrow_mut().push("value");
        let read = self.meridian_reads.get() + 1;
        self.meridian_reads.set(read);
        if self.fail_meridian.get() || self.fail_meridian_read.get() == Some(read) {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "ObservedCylinder parameter {parameter}, read {read}; expected an enabled Point2 meridian evaluation"
                ),
            ));
        }
        self.cylinder.meridian_value(parameter)
    }
    fn meridian_closed(&self) -> Result<bool, Failure> {
        self.virtual_calls.borrow_mut().push("closed");
        Ok(self.closed)
    }
    fn has_top(&self) -> Result<bool, Failure> {
        self.virtual_calls.borrow_mut().push("top");
        Ok(self.top_exists.get())
    }
    fn has_bottom(&self) -> Result<bool, Failure> {
        self.virtual_calls.borrow_mut().push("bottom");
        Ok(self.bottom_exists.get())
    }
    fn has_sides(&self) -> bool {
        self.virtual_calls.borrow_mut().push("sides");
        self.cylinder.has_sides()
    }
    fn vmax_infinite(&self) -> bool {
        self.infinity_reads.borrow_mut().push("max");
        self.virtual_calls.borrow_mut().push("max");
        self.cylinder.vmax_infinite()
    }
    fn vmin_infinite(&self) -> bool {
        self.infinity_reads.borrow_mut().push("min");
        self.virtual_calls.borrow_mut().push("min");
        self.cylinder.vmin_infinite()
    }
}
fn pcurve_after_binding_failure() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "ObservedCylinder fail_after_pcurve=true; expected successful completion after real binding",
    )
}
#[test]
fn cached_axis_vertices_do_not_reevaluate_virtual_infinity_queries() {
    let mut cylinder = ObservedCylinder::new(2.0, false);
    let top = cylinder.axis_top_vertex().unwrap();
    assert_eq!(cylinder.reads(), ["max"]);
    assert!(cylinder.axis_top_vertex().unwrap().is_same(&top));
    assert_eq!(cylinder.reads(), ["max"]);
    let bottom = cylinder.axis_bottom_vertex().unwrap();
    assert_eq!(cylinder.reads(), ["max", "max", "min", "min"]);
    assert!(cylinder.axis_bottom_vertex().unwrap().is_same(&bottom));
    assert_eq!(cylinder.reads(), ["max", "max", "min", "min"]);
}
#[test]
fn axis_vertex_sharing_evaluates_only_the_equal_height_infinity_queries() {
    let mut cylinder = ObservedCylinder::new(0.0, false);
    let top = cylinder.axis_top_vertex().unwrap();
    let bottom = cylinder.axis_bottom_vertex().unwrap();
    assert!(top.is_same(&bottom));
    assert_eq!(cylinder.reads(), ["max", "max", "min"]);
}
#[test]
fn closed_meridian_rejection_precedes_virtual_infinity_queries() {
    let mut cylinder = ObservedCylinder::new(2.0, true);
    let top_error = cylinder.axis_top_vertex().unwrap_err();
    let bottom_error = cylinder.axis_bottom_vertex().unwrap_err();
    assert_eq!(top_error.exception_type(), "Standard_DomainError");
    assert_eq!(bottom_error.exception_type(), "Standard_DomainError");
    assert!(cylinder.reads().is_empty());
    cylinder.set_vmax(3.0).unwrap();
    assert_eq!(cylinder.vmax(), 3.0);
}

#[test]
fn failed_lateral_wire_stops_before_lower_limit_query_and_leaves_parameters_unlocked() {
    let mut cylinder = ObservedCylinder::new(2.0, false);
    cylinder.fail_meridian.set(true);
    let error = cylinder.lateral_wire().unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert_eq!(cylinder.reads(), ["max"]);
    cylinder.set_angle(std::f64::consts::PI).unwrap();
    cylinder.fail_meridian.set(false);
    assert!(!cylinder.lateral_wire().unwrap().is_null());
    assert_eq!(
        cylinder.set_angle(1.0).unwrap_err().exception_type(),
        "Standard_DomainError"
    );
}
#[test]
fn lateral_wire_evaluates_upper_edge_before_lower_limit_and_caches_completed_state() {
    let mut cylinder = ObservedCylinder::new(2.0, false);
    let wire = cylinder.lateral_wire().unwrap();
    let calls = cylinder.virtual_calls.borrow().clone();
    let first_value = calls.iter().position(|event| *event == "value").unwrap();
    let first_min = calls.iter().position(|event| *event == "min").unwrap();
    assert!(first_value < first_min, "virtual calls: {calls:?}");
    assert!(cylinder.lateral_wire().unwrap().is_same(&wire));
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
}

type CylinderWire = fn(&mut ObservedCylinder) -> Result<martensite_tkbrep::topods::Wire, Failure>;
fn cap_wire_cases() -> [(&'static str, CylinderWire); 2] {
    [
        ("top", ObservedCylinder::top_wire),
        ("bottom", ObservedCylinder::bottom_wire),
    ]
}
fn side_wire_cases() -> [(&'static str, CylinderWire); 2] {
    [
        ("bottom", ObservedCylinder::start_wire),
        ("top", ObservedCylinder::end_wire),
    ]
}
fn assert_wire_retry_and_cached_lock(cylinder: &mut ObservedCylinder, build: CylinderWire) {
    cylinder.set_angle(std::f64::consts::FRAC_PI_2).unwrap();
    cylinder.fail_meridian_read.set(None);
    let wire = build(cylinder).unwrap();
    let calls = cylinder.virtual_calls.borrow().clone();
    assert!(build(cylinder).unwrap().is_same(&wire));
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    assert_eq!(
        cylinder.set_angle(1.0).unwrap_err().exception_type(),
        "Standard_DomainError"
    );
}
#[test]
fn cap_wire_edge_failure_precedes_optional_side_queries() {
    for (cap, build) in cap_wire_cases() {
        let mut cylinder = ObservedCylinder::new(2.0, false);
        cylinder.fail_meridian_read.set(Some(1));
        assert_eq!(
            build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert_eq!(*cylinder.virtual_calls.borrow(), [cap, "closed", "value"]);
        assert_wire_retry_and_cached_lock(&mut cylinder, build);
    }
}
#[test]
fn side_wire_equal_height_evaluation_precedes_first_cap_query() {
    for (_, build) in side_wire_cases() {
        let mut cylinder = ObservedCylinder::new(2.0, false);
        cylinder.set_angle(std::f64::consts::PI).unwrap();
        cylinder.fail_meridian_read.set(Some(3));
        assert_eq!(
            build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert_eq!(
            *cylinder.virtual_calls.borrow(),
            [
                "sides", "max", "min", "value", "value", "max", "min", "value"
            ]
        );
        assert_wire_retry_and_cached_lock(&mut cylinder, build);
    }
}
#[test]
fn side_wire_first_edge_failure_precedes_axis_and_opposite_cap_queries() {
    for (cap, build) in side_wire_cases() {
        let mut cylinder = ObservedCylinder::new(2.0, false);
        cylinder.set_angle(std::f64::consts::PI).unwrap();
        cylinder.fail_meridian_read.set(Some(5));
        assert_eq!(
            build(&mut cylinder).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        let expected = [
            "sides", "max", "min", "value", "value", "max", "min", "value", "value", cap, cap,
            "sides", "value",
        ];
        assert_eq!(*cylinder.virtual_calls.borrow(), expected);
        assert_wire_retry_and_cached_lock(&mut cylinder, build);
    }
}
fn assert_wire_call_counts(calls: &[&str], expected: [usize; 7]) {
    let hooks = ["top", "bottom", "sides", "closed", "max", "min", "value"];
    let actual = hooks.map(|hook| calls.iter().filter(|call| **call == hook).count());
    assert_eq!(actual, expected, "virtual calls: {calls:?}");
}
#[test]
fn cap_wire_success_preserves_hook_counts_and_cached_identity() {
    for (cap, build) in cap_wire_cases() {
        let mut cylinder = ObservedCylinder::new(2.0, false);
        let wire = build(&mut cylinder).unwrap();
        let calls = cylinder.virtual_calls.borrow().clone();
        let caps = if cap == "top" { [1, 0] } else { [0, 1] };
        assert_wire_call_counts(&calls, [caps[0], caps[1], 3, 3, 0, 0, 5]);
        assert_eq!(calls[..4], [cap, "closed", "value", "value"]);
        assert_eq!(calls.last(), Some(&"sides"));
        assert!(build(&mut cylinder).unwrap().is_same(&wire));
        assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    }
}
#[test]
fn side_wire_success_preserves_cap_order_hook_counts_and_cached_identity() {
    for (cap, build) in side_wire_cases() {
        let mut cylinder = ObservedCylinder::new(2.0, false);
        cylinder.set_angle(std::f64::consts::PI).unwrap();
        let wire = build(&mut cylinder).unwrap();
        let calls = cylinder.virtual_calls.borrow().clone();
        assert_wire_call_counts(&calls, [2, 2, 7, 9, 9, 8, 28]);
        let caps: Vec<_> = calls
            .iter()
            .copied()
            .filter(|call| matches!(*call, "top" | "bottom"))
            .collect();
        let opposite = if cap == "top" { "bottom" } else { "top" };
        assert_eq!(caps, [cap, cap, opposite, opposite]);
        assert!(build(&mut cylinder).unwrap().is_same(&wire));
        assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    }
}

fn assert_partial_wire_lock_and_retry(build: CylinderWire) {
    let mut cylinder = ObservedCylinder::new(2.0, false);
    cylinder.set_angle(std::f64::consts::PI).unwrap();
    cylinder.fail_meridian_read.set(Some(12));
    assert_eq!(
        build(&mut cylinder).unwrap_err().exception_type(),
        "Standard_DomainError"
    );
    assert_eq!(
        cylinder.set_angle(1.0).unwrap_err().exception_type(),
        "Standard_DomainError"
    );
    cylinder.fail_meridian_read.set(None);
    let wire = build(&mut cylinder).unwrap();
    let calls = cylinder.virtual_calls.borrow().clone();
    assert!(build(&mut cylinder).unwrap().is_same(&wire));
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
}
#[test]
fn partially_built_cap_and_side_wires_keep_vertex_parameter_locks_and_retry() {
    for (_, build) in cap_wire_cases().into_iter().chain(side_wire_cases()) {
        assert_partial_wire_lock_and_retry(build);
    }
}

#[path = "support/one_axis_face_contracts.rs"]
mod face_contracts;

#[path = "support/one_axis_meridian_branches.rs"]
mod meridian_branches;

#[path = "support/one_axis_mutation_contracts.rs"]
mod mutation_contracts;

#[path = "support/one_axis_cap_guards.rs"]
mod cap_guards;
