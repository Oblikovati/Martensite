//! Native FindSPErrorEdges/FindOtherErrors/FixSameParameter decision order.
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, ShapeIterator},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::{
    brep_check::{BRepCheckAnalyzer, BRepCheckResult, BRepCheckStatus},
    brep_lib::same_parameter_shape,
};
type Status = BRepCheckStatus;
type Checker = Handle<dyn BRepCheckResult>;
fn parameter_error(status: Status) -> bool {
    matches!(
        status,
        Status::InvalidSameParameterFlag | Status::InvalidCurveOnSurface
    )
}
fn other_error(status: Status) -> bool {
    status != Status::NoError && !parameter_error(status)
}
fn context_statuses(checker: &Checker) -> Result<Vec<(Shape, Vec<Status>)>, Failure> {
    checker.init_context_iterator()?;
    let mut contexts = Vec::new();
    while checker.more_shape_in_context() {
        contexts.push((
            checker.contextual_shape()?,
            checker.status_on_shape()?.iter().copied().collect(),
        ));
        checker.next_shape_in_context();
    }
    Ok(contexts)
}
fn face_parameter_edges(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &mut IndexedMapOfShape,
) -> Result<(), Failure> {
    for edge in Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)? {
        let checker = analyzer.result(&edge)?;
        if checker.is_null() || edges.contains(&edge) {
            continue;
        }
        for (context, statuses) in context_statuses(&checker)? {
            if context.is_same(shape) && statuses.into_iter().any(parameter_error) {
                edges.add(edge.clone());
            }
        }
    }
    Ok(())
}
fn find_parameter_edges(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &mut IndexedMapOfShape,
) -> Result<(), Failure> {
    for child in ShapeIterator::new(shape, true, true) {
        find_parameter_edges(&child, analyzer, edges)?;
    }
    match shape.shape_type()? {
        ShapeEnum::Face => face_parameter_edges(shape, analyzer, edges)?,
        ShapeEnum::Edge => own_parameter_edge(shape, analyzer, edges)?,
        _ => (),
    }
    Ok(())
}
fn own_parameter_edge(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &mut IndexedMapOfShape,
) -> Result<(), Failure> {
    if analyzer
        .result(shape)?
        .status()?
        .iter()
        .copied()
        .any(parameter_error)
    {
        edges.add(shape.clone());
    }
    Ok(())
}
fn first_error(statuses: &[Status]) -> bool {
    statuses
        .first()
        .is_some_and(|status| *status != Status::NoError)
}
fn context_error(checker: &Checker) -> Result<bool, Failure> {
    Ok(context_statuses(checker)?.iter().any(|(_, statuses)| {
        statuses
            .first()
            .is_some_and(|status| *status != Status::NoError)
    }))
}
fn edge_context_error(
    edge: &Shape,
    face: &Shape,
    shape: &Shape,
    checker: &Checker,
    edges: &IndexedMapOfShape,
) -> Result<bool, Failure> {
    for (context, statuses) in context_statuses(checker)? {
        if !context.is_same(face) && !context.is_same(shape) {
            continue;
        }
        if !first_error(&statuses) {
            continue;
        }
        if !edges.contains(edge) || statuses.into_iter().any(other_error) {
            return Ok(true);
        }
    }
    Ok(false)
}
fn face_other_errors(
    face: &Shape,
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &IndexedMapOfShape,
) -> Result<Option<bool>, Failure> {
    let mut found = None;
    for edge in Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)? {
        let checker = analyzer.result(&edge)?;
        if checker.is_null() {
            continue;
        }
        found = Some(false);
        if edge_context_error(&edge, face, shape, &checker, edges)? {
            return Ok(Some(true));
        }
    }
    Ok(found)
}
fn own_error_contexts(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &IndexedMapOfShape,
    checker: &Checker,
) -> Result<bool, Failure> {
    let mut other_found = true;
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)? {
        if let Some(found) = face_other_errors(&face, shape, analyzer, edges)? {
            if found {
                return Ok(true);
            }
            other_found = false;
        }
    }
    if !other_found {
        return context_error(checker);
    }
    Ok(other_found)
}
fn find_other_errors(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &IndexedMapOfShape,
) -> Result<bool, Failure> {
    for child in ShapeIterator::new(shape, true, true) {
        if find_other_errors(&child, analyzer, edges)? {
            return Ok(true);
        }
    }
    checker_errors(shape, analyzer, edges)
}
fn checker_errors(
    shape: &Shape,
    analyzer: &BRepCheckAnalyzer,
    edges: &IndexedMapOfShape,
) -> Result<bool, Failure> {
    let checker = analyzer.result(shape)?;
    if checker.is_null() {
        return Ok(false);
    }
    let statuses = checker.status()?;
    if edges.contains(shape) || statuses.is_empty() {
        return Ok(statuses.iter().copied().any(other_error));
    }
    if statuses.first().unwrap() != &Status::NoError {
        return own_error_contexts(shape, analyzer, edges, &checker);
    }
    contextual_kind_error(shape, &checker)
}
fn contextual_kind_error(shape: &Shape, checker: &Checker) -> Result<bool, Failure> {
    if matches!(
        shape.shape_type()?,
        ShapeEnum::Vertex | ShapeEnum::Edge | ShapeEnum::Wire | ShapeEnum::Face | ShapeEnum::Shell
    ) {
        return context_error(checker);
    }
    Ok(false)
}
/// Repair only identified parameter errors when no other native defects remain; `fix_same_parameter(...)`.
pub(super) fn fix_same_parameter(
    shape: &Shape,
    analyzer: &mut BRepCheckAnalyzer,
) -> Result<bool, Failure> {
    let mut edges = IndexedMapOfShape::default();
    find_parameter_edges(shape, analyzer, &mut edges)?;
    if find_other_errors(shape, analyzer, &edges)? {
        return Ok(false);
    }
    for edge in edges.iter() {
        same_parameter_shape(edge, precision::confusion(), true)?;
    }
    if edges.is_empty() {
        return Ok(false);
    }
    analyzer.init(shape, true)?;
    analyzer.is_valid()
}

#[cfg(test)]
mod checks {
    use super::*;
    use martensite_tkbrep::{
        brep::{BRepBuilder, BRepTool},
        topods,
    };
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    #[test]
    fn repairs_only_flagged_edge_and_reinitializes_the_native_analyzer() {
        let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
        let shape = maker.shape().unwrap().clone();
        let edge =
            Iterator::next(&mut Explorer::new(&shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap())
                .unwrap();
        let edge = topods::edge(&edge).unwrap();
        BRepBuilder::default().same_parameter(&edge, false).unwrap();
        let mut analyzer = BRepCheckAnalyzer::new(&shape, true, false, false).unwrap();
        assert!(!analyzer.is_valid().unwrap());
        assert!(fix_same_parameter(&shape, &mut analyzer).unwrap());
        assert!(BRepTool::same_parameter(&edge).unwrap());
        assert!(analyzer.is_valid().unwrap());
    }
    #[test]
    fn valid_geometry_needs_no_repair_and_status_filter_retains_other_failures() {
        let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
        let shape = maker.shape().unwrap().clone();
        let mut analyzer = BRepCheckAnalyzer::new(&shape, true, false, false).unwrap();
        assert!(!fix_same_parameter(&shape, &mut analyzer).unwrap());
        assert!(other_error(Status::No3DCurve));
        assert!(!other_error(Status::InvalidCurveOnSurface));
        assert!(parameter_error(Status::InvalidSameParameterFlag));
    }
}
