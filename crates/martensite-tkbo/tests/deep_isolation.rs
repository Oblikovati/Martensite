//! Deep isolation test suite verifying that source mutations after Boolean operations
//! never mutate result vertices, edges, tolerances, locations, or validity (#25, #457).
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BuilderHooks, Operation};
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::fmt::Write as _;

struct ImmediateCancel;

impl ProgressIndicatorHooks for ImmediateCancel {
    fn show(&self, _scope: &ProgressScope<'_>, _position: f64, _force: bool) {}
    fn user_break(&self) -> bool {
        true
    }
}

fn snapshot_shape(shape: &Shape) -> Result<String, Failure> {
    let mut out = String::new();
    let mut vex = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while vex.more() {
        let v = topods::vertex(vex.current())?;
        let p = BRepTool::pnt(&v)?;
        let tol = BRepTool::vertex_tolerance(&v)?;
        let _ = writeln!(
            out,
            "v:{:016x}:{:016x}:{:016x}:{:016x}",
            p.x().to_bits(),
            p.y().to_bits(),
            p.z().to_bits(),
            tol.to_bits()
        );
        vex.next()?;
    }
    let mut eex = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while eex.more() {
        let e = topods::edge(eex.current())?;
        let (first, last) = BRepTool::range(&e)?;
        let tol = BRepTool::edge_tolerance(&e)?;
        let _ = writeln!(
            out,
            "e:{:016x}:{:016x}:{:016x}",
            first.to_bits(),
            last.to_bits(),
            tol.to_bits()
        );
        eex.next()?;
    }
    Ok(out)
}

fn mutate_source_vertices_and_tolerances(source: &mut Shape) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let mut vex = Explorer::new(source, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while vex.more() {
        let v = topods::vertex(vex.current())?;
        let p = BRepTool::pnt(&v)?;
        let shifted = Point3::new(p.x() + 5.0, p.y() - 3.0, p.z() + 2.0);
        builder.update_vertex_point(&v, shifted, 0.25)?;
        vex.next()?;
    }
    let mut trsf = Transform3::default();
    trsf.set_translation(Vector3::new(100.0, 200.0, 300.0));
    source.move_by(&Location::new(trsf), false)?;
    Ok(())
}

#[test]
fn non_destructive_boolean_result_survives_source_vertex_and_location_mutation()
-> Result<(), Failure> {
    let mut box_a = BRepPrimAPIMakeBox::from_sizes(20., 20., 20.)?
        .shape()?
        .clone();
    let tool = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(10., 10., -2.), Direction3::Z)?,
        4.,
        24.,
    )
    .shape()?
    .clone();
    let source_before = snapshot_shape(&box_a)?;
    let tool_before = snapshot_shape(&tool)?;

    let mut cut = BRepAlgoAPICut::new();
    assert!(
        cut.non_destructive(),
        "Booleans must default to non-destructive mode (#26)"
    );
    let mut args = martensite_tkernel::ncollection::List::default();
    args.append(box_a.clone());
    let mut tools = martensite_tkernel::ncollection::List::default();
    tools.append(tool.clone());
    cut.set_arguments(&args);
    cut.set_tools(&tools);
    cut.build()?;
    let result = cut.shape().clone();
    assert!(!result.is_null(), "cut produced null shape");

    // Sources must be byte-identical after the operation.
    assert_eq!(snapshot_shape(&box_a)?, source_before);
    assert_eq!(snapshot_shape(&tool)?, tool_before);

    // Snapshot and validate result BEFORE mutating source.
    let result_before = snapshot_shape(&result)?;
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?,
        "result must be valid before source mutation"
    );

    // Mutate every source vertex position, tolerance, and root location (#457, #25).
    mutate_source_vertices_and_tolerances(&mut box_a)?;

    // Result must remain byte-identical and B-rep valid after source mutation.
    assert_eq!(
        snapshot_shape(&result)?,
        result_before,
        "result geometry/tolerances changed after source vertex mutation (#457)"
    );
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?,
        "result failed BRepCheckAnalyzer after source vertex mutation (#457)"
    );
    Ok(())
}

#[test]
fn source_is_unchanged_after_boolean_failure_and_cancellation() -> Result<(), Failure> {
    let source = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)?
        .shape()?
        .clone();
    let before = snapshot_shape(&source)?;

    // Failure case: missing tool argument.
    let mut failing = BOPAlgoBOP::new();
    failing.set_non_destructive(true);
    failing.set_operation(Operation::Cut);
    failing.add_argument(&source);
    failing.perform();
    assert_eq!(snapshot_shape(&source)?, before);

    // Cancellation case: immediate user break.
    let indicator = ProgressIndicator::new(Box::new(ImmediateCancel));
    let range = indicator.start();
    let mut cancelled = BOPAlgoBOP::new();
    cancelled.set_non_destructive(true);
    cancelled.set_operation(Operation::Fuse);
    cancelled.add_argument(&source);
    cancelled.add_tool(&source);
    let _ = cancelled.perform_with_progress(&range);
    assert_eq!(snapshot_shape(&source)?, before);
    Ok(())
}

#[test]
fn sealed_operation_result_covers_prism_and_boolean_history_and_errors() -> Result<(), Failure> {
    use martensite_tkbrep::operation::{FailureClass, OperationKind};
    use martensite_tkmath::gp::Vector3;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;

    // #6: bad input on prism returns typed BadInput error with affected entity.
    let box_shape = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)?
        .shape()?
        .clone();
    let bad = BRepPrimAPIMakePrism::execute_operation(&box_shape, Vector3::new(0., 0., 0.), true)
        .unwrap_err();
    assert_eq!(bad.operation(), OperationKind::Prism);
    assert_eq!(bad.failure_class(), FailureClass::BadInput);
    assert_eq!(bad.affected_entities().len(), 1);

    // #18 & #27: real prism produces sealed OperationResult with Derived history.
    let face = martensite_tkbrep::topexp::Explorer::new(
        &box_shape,
        martensite_tkg3d::top_abs::ShapeEnum::Face,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )?
    .current()
    .clone();
    let prism_res =
        BRepPrimAPIMakePrism::execute_operation(&face, Vector3::new(0., 0., 5.), true).unwrap();
    assert_eq!(prism_res.operation(), OperationKind::Prism);
    assert!(!prism_res.shape().is_null());
    assert!(!prism_res.history().is_empty());

    // #18 & #27: real Boolean Cut seals detached result and monotonic revision.
    let tool = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(5., 5., -2.), Direction3::Z)?,
        2.,
        14.,
    )
    .shape()?
    .clone();
    let mut cut = BRepAlgoAPICut::new();
    let mut args = martensite_tkernel::ncollection::List::default();
    args.append(box_shape.clone());
    let mut tools = martensite_tkernel::ncollection::List::default();
    tools.append(tool);
    cut.set_arguments(&args);
    cut.set_tools(&tools);
    cut.build()?;
    let bool_res = cut.seal_result(OperationKind::Boolean).unwrap();
    assert!(bool_res.revision() > prism_res.revision());
    assert!(!bool_res.shape().is_null());
    assert!(!bool_res.history().is_empty());
    Ok(())
}
