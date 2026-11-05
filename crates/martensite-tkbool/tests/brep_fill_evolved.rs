//! Behavioral checks for BRepFillEvolved recovered from Claude's unfinished worktree.
#[path = "brep_fill_pipe_shell/support.rs"]
mod support;
use martensite_tkbool::brep_fill::BRepFillEvolved;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    geom_abs::JoinType,
    gp::{Axis3, Point3},
};
use martensite_tktopalgo::brep_lib::BRepLibMakeFace;

#[test]
fn default_evolved_has_no_result_or_generated_shapes() {
    let evolved = BRepFillEvolved::default();
    assert!(!evolved.is_done());
    assert!(evolved.shape().is_null());
    assert!(evolved.top().is_null());
    assert!(evolved.bottom().is_null());
    assert!(
        evolved
            .generated_shapes(&Shape::default(), &Shape::default())
            .is_empty()
    );
}

#[test]
fn non_arc_joins_report_native_not_implemented() -> Result<(), Failure> {
    let spine = support::make_square_profile(Point3::new(2., 2., 0.), 2.)?;
    let profile = support::polyline(&[(0., 0., 0.), (0., 0., 1.)])?;
    for join in [JoinType::Tangent, JoinType::Intersection] {
        let mut evolved = BRepFillEvolved::new();
        let error = evolved
            .perform_wire(&spine, &profile, &Axis3::default(), join, false)
            .unwrap_err();
        assert_eq!(error.exception_type(), "Standard_NotImplemented");
        assert_eq!(evolved.join_type(), join);
        assert!(!evolved.is_done());
    }
    Ok(())
}

#[test]
fn vertical_profile_sweeps_four_faces_without_caps() -> Result<(), Failure> {
    let spine = support::make_square_profile(Point3::new(2., 2., 0.), 2.)?;
    let profile = support::polyline(&[(0., 0., 0.), (0., 0., 1.)])?;
    let evolved =
        BRepFillEvolved::from_wire(&spine, &profile, &Axis3::default(), JoinType::Arc, false)?;
    assert!(evolved.is_done());
    assert_eq!(
        Explorer::new(evolved.shape(), ShapeEnum::Face, ShapeEnum::Shape)?.count(),
        4
    );
    assert!(evolved.top().is_null());
    assert!(evolved.bottom().is_null());
    Ok(())
}

#[test]
fn face_spine_solid_has_caps_and_generated_history() -> Result<(), Failure> {
    let spine = support::make_square_profile(Point3::new(2., 2., 0.), 2.)?;
    let face = BRepLibMakeFace::from_wire(&spine, true)?.face()?;
    let profile = support::polyline(&[(0., 0., 0.), (0., 0., 1.)])?;
    let evolved =
        BRepFillEvolved::from_face(&face, &profile, &Axis3::default(), JoinType::Arc, true)?;
    assert!(evolved.is_done());
    assert_eq!(evolved.shape().shape_type()?, ShapeEnum::Solid);
    assert!(!evolved.top().is_null());
    assert!(!evolved.bottom().is_null());
    let spine_edge = Explorer::new(spine.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?
        .current()
        .clone();
    let profile_edge = Explorer::new(profile.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?
        .current()
        .clone();
    assert_eq!(
        evolved.generated_shapes(&spine_edge, &profile_edge).len(),
        1
    );
    Ok(())
}

#[test]
fn outward_profile_preserves_negative_zero_at_a_planar_pcurve_start() -> Result<(), Failure> {
    let spine = support::make_square_profile(Point3::new(2., 2., 0.), 2.)?;
    let profile = support::polyline(&[(0., 0., 0.), (0., -0.5, 1.)])?;
    let evolved =
        BRepFillEvolved::from_wire(&spine, &profile, &Axis3::default(), JoinType::Arc, false)?;
    let mut edges = martensite_tkbrep::toptools::IndexedMapOfShape::default();
    martensite_tkbrep::topexp::map_shapes_of_type(evolved.shape(), ShapeEnum::Edge, &mut edges)?;
    let mut negative_zero_starts = 0;
    for edge in edges.iter() {
        negative_zero_starts += negative_zero_pcurves(&martensite_tkbrep::topods::edge(edge)?)?;
    }
    assert_eq!(negative_zero_starts, 1);
    Ok(())
}

fn negative_zero_pcurves(edge: &martensite_tkbrep::topods::Edge) -> Result<usize, Failure> {
    let mut count = 0;
    for index in 1.. {
        let pcurve = martensite_tkbrep::brep::BRepTool::curve_on_surface_by_index(edge, index)?;
        let Some(curve) = pcurve.pcurve.get() else {
            break;
        };
        if pcurve.first < 0. && pcurve.first > -1e-12 {
            count += usize::from(curve.value(pcurve.first)?.y().to_bits() == (-0.0f64).to_bits());
        }
    }
    Ok(count)
}
