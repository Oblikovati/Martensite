//! Exact original E2 driver setup, geometry, naming history and selection recomputation.
use super::{
    execute_box,
    naming_fillet::{execute_fillet, make_fillet},
    naming_function_graph::{add_selection, solve_selection},
    naming_transform::execute_transform,
    selection_driver, set_box_dimensions,
    transform_checks::{E2TransformPrefix, current, e2_transform_prefix, selected_subshape},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{Shape, ShapeIterator},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tfunction::TFunctionFunction;

pub(super) fn collect_shapes(shape: &Shape, shapes: &mut Vec<Shape>) {
    shapes.push(shape.clone());
    for child in ShapeIterator::new(shape, true, true) {
        collect_shapes(&child, shapes);
    }
}
fn collected_selections(
    prefix: &E2TransformPrefix,
    shape: &Shape,
) -> Vec<(Handle<TFunctionFunction>, ShapeEnum)> {
    let mut shapes = Vec::new();
    collect_shapes(shape, &mut shapes);
    assert!(!shapes.is_empty());
    shapes.remove(0);
    shapes
        .into_iter()
        .filter_map(|selected| selection_for_shape(prefix, selected))
        .collect()
}
fn selection_for_shape(
    prefix: &E2TransformPrefix,
    selected: Shape,
) -> Option<(Handle<TFunctionFunction>, ShapeEnum)> {
    let kind = selected.shape_type().unwrap();
    if kind == ShapeEnum::Edge
        && BRepTool::degenerated(&martensite_tkbrep::topods::edge(&selected).unwrap()).unwrap()
    {
        return None;
    }
    let (_object, function) =
        add_selection(&prefix.document, &prefix.object, &selected, true).unwrap();
    Some((function, kind))
}
fn recompute_transforms(
    prefix: &E2TransformPrefix,
    driver: &super::naming_selection_driver::NamingSelectionDriver,
) {
    set_box_dimensions(&prefix.box_function, 290.0);
    execute_box(&prefix.box_function);
    assert_eq!(execute_transform(&prefix.transforms[0]).unwrap(), 0);
    for index in 0..3 {
        assert!(
            !solve_selection(&prefix.selections[index], driver)
                .unwrap()
                .is_null()
        );
        assert_eq!(execute_transform(&prefix.transforms[index + 1]).unwrap(), 0);
    }
}

fn initial_fillet(
    prefix: &E2TransformPrefix,
) -> (Handle<TFunctionFunction>, Handle<TFunctionFunction>) {
    let mirrored = current(&prefix.transforms[3]);
    let (path, path_selection) =
        selected_subshape(&prefix.document, &prefix.object, &mirrored, 1, Some(3));
    (make_fillet(&prefix.object, &path).unwrap(), path_selection)
}
fn assert_resolved_selections(
    selections: Vec<(Handle<TFunctionFunction>, ShapeEnum)>,
    driver: &super::naming_selection_driver::NamingSelectionDriver,
) {
    for (function, case_kind) in selections {
        let selected = solve_selection(&function, driver).unwrap();
        assert!(!selected.is_null());
        assert_eq!(selected.shape_type().unwrap(), case_kind);
    }
}
/// TNamingTool tests.CafNamedShape_E2_SelectionAfterTransformations; `tnaming_tool_selection` runs it.
pub(crate) fn caf_named_shape_e2_selection_after_transformations() {
    super::naming_driver_registration::register_naming_drivers();
    let prefix = e2_transform_prefix();
    let (fillet, path_selection) = initial_fillet(&prefix);
    let filleted = current(&fillet);
    assert!(!filleted.is_null());
    let selections = collected_selections(&prefix, &filleted);
    println!(
        "{{\"event\":\"e2-selected-occurrences\",\"count\":{}}}",
        selections.len()
    );
    let (driver, warnings) = selection_driver();
    recompute_transforms(&prefix, &driver);
    assert!(!solve_selection(&path_selection, &driver).unwrap().is_null());
    assert_eq!(execute_fillet(&fillet).unwrap(), 0);
    assert_resolved_selections(selections, &driver);
    assert!(warnings.0.lock().unwrap().is_empty());
}
