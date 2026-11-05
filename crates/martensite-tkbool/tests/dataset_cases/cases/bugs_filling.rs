//! `bugs_filling`: a plate surface built on an initial surface that its constraints leave.
//!
//! The reference run crashes in this operation and catches the signal, which the script
//! declares as a known problem. The library refuses the null plate surface with a checked
//! failure instead; the script's two declared lines match either, so both are the same verdict.
use crate::support::{
    checkview::{CheckView, checkview},
    restore::restore,
    session::Session,
};
use martensite_tkbool::brep_fill::BRepFillCurveConstraint;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorCurve,
    topods::{self},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeomalgo::geom_plate::{
    GeomPlateBuildPlateSurface, GeomPlateCurveConstraint, GeomPlatePointConstraint,
};
use martensite_tkmath::gp::Point3;

const POINTS: [[f64; 3]; 4] = [
    [30., -33.4729635533385, 49.7661550602442],
    [30., -49.6961550602442, 33.3929635533386],
    [23.3333333333333, -50., 30.07],
    [-30., -33.4729635533386, 49.6161550602442],
];
/// The `gplate result 1 4 ...` settings: one curve of order 0, four points.
const CONSTRAINT_TOLERANCE: f64 = 1.0e-4;

fn plate_on_initial_surface(s: &dyn Session) -> GeomPlateBuildPlateSurface {
    let face = topods::face(s.shape("initFace").unwrap()).unwrap();
    let surface = BRepTool::surface(&face).unwrap();
    assert!(!surface.is_null(), "the initial face holds a surface");
    let mut plate =
        GeomPlateBuildPlateSurface::new(3, 15, 2, 1e-5, 1e-4, 0.01, 0.1, false).unwrap();
    plate.load_init_surface(&surface);
    plate
}

fn add_edge_constraint(
    s: &dyn Session,
    plate: &mut GeomPlateBuildPlateSurface,
) -> Handle<GeomPlateCurveConstraint> {
    let edge = topods::edge(s.shape("edge_constraint").unwrap()).unwrap();
    let curve: Handle<dyn Adaptor3dCurve> =
        Handle::new(BRepAdaptorCurve::with_edge(&edge).unwrap()).cast();
    let constraint =
        BRepFillCurveConstraint::with_curve(&curve, 0, 10, CONSTRAINT_TOLERANCE).unwrap();
    plate.add_curve(&constraint);
    constraint
}

fn add_point_constraints(plate: &mut GeomPlateBuildPlateSurface) {
    for [x, y, z] in POINTS {
        let at = Point3::new(x, y, z);
        let point = GeomPlatePointConstraint::with_point(at, 0, CONSTRAINT_TOLERANCE).unwrap();
        plate.add_point(&Handle::new(point));
    }
}

/// `gplate result 1 4 initFace edge_constraint 0 p1 p2 p3 p4`, up to the failure it declares.
fn gplate(s: &dyn Session) -> Result<(), Failure> {
    let mut plate = plate_on_initial_surface(s);
    add_edge_constraint(s, &mut plate);
    add_point_constraints(&mut plate);
    let outcome = plate.perform(&ProgressRange::default()).map(|_| ());
    if outcome.is_err() {
        assert!(
            plate.surface().is_null(),
            "a refused plate holds no surface"
        );
    }
    outcome
}

/// The refusal stays this one: the script's patterns alone would accept any failure.
fn require_null_plate_refusal(failure: &Failure) {
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert_eq!(
        failure.message(),
        "GeomPlate: the plate surface is null; expected a constructed object"
    );
}

pub fn bug23343(s: &mut dyn Session) {
    println!("TODO OCC23343 ALL: An exception was caught");
    println!("TODO OCC23343 ALL: Faulty OCC23343");
    println!("============");
    println!(
        "0023343: Crash (A C C E S S  V I O L A T I O N) in GeomPlate_BuildPlateSurface / MakeApprox if init Surface is set"
    );
    println!("============");
    restore(s, "bug23343_initFace.brep", "initFace");
    restore(s, "bug23343_edge_constraint.brep", "edge_constraint");
    match gplate(s) {
        Err(failure) => {
            require_null_plate_refusal(&failure);
            println!(
                "An exception was caught {}: {}",
                failure.exception_type(),
                failure.message()
            );
            println!(
                "Faulty OCC23343: there is crash (ACCESS VIOLATION) in GeomPlate_BuildPlateSurface"
            );
        }
        Ok(()) => println!(
            "OK OCC23343: there is not crash (ACCESS VIOLATION) in GeomPlate_BuildPlateSurface"
        ),
    }
    checkview(s, &CheckView::of("result"));
}
