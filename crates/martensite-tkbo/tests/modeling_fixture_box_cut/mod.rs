mod entity_history;
mod geometry;
mod history;
mod measures;
#[path = "../solid_shell_progress_public/snapshot.rs"]
mod snapshot;
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;

pub(super) fn direct_and_prepared() {
    let inputs = geometry::operands();
    let direct = direct_cut(&inputs);
    check(&direct, &inputs);
    let prepared_inputs = geometry::operands();
    let mut filler =
        crate::common_operation::intersections(&prepared_inputs[0], &prepared_inputs[1]);
    let prepared = crate::common_operation::performed_with_filler(
        &prepared_inputs[0],
        &prepared_inputs[1],
        Operation::Cut,
        &mut filler,
    );
    check(&prepared, &prepared_inputs);
    assert_eq!(
        geometry::signature(direct.shape()),
        geometry::signature(prepared.shape())
    );
}

fn direct_cut(inputs: &[Shape; 2]) -> BOPAlgoBOP {
    crate::common_operation::performed_direct(
        &inputs[0],
        &inputs[1],
        Operation::Cut,
        precision::confusion(),
    )
}

pub(super) fn non_destructive() {
    let inputs = geometry::operands();
    let before = snapshot::shapes(inputs.iter());
    let mut operation =
        crate::common_operation::configured_prepared(&inputs[0], &inputs[1], Operation::Cut);
    operation.set_non_destructive(true);
    operation.perform();
    check(&operation, &inputs);
    let after = snapshot::shapes(inputs.iter());
    snapshot::assert_unchanged("successful box cut", &before, &after);
}

fn check(operation: &BOPAlgoBOP, inputs: &[Shape; 2]) {
    assert!(
        !operation.options().has_errors(),
        "box cut report contains errors"
    );
    assert!(!operation.shape().is_null(), "box cut returned null shape");
    assert!(
        operation.has_history(),
        "box cut must retain actual history"
    );
    geometry::require_box(operation.shape(), [0., 1., 0., 2., 0., 2.]);
    measures::require_closed_form(operation.shape(), 16., 4.);
    history::require_relations(operation, inputs);
    entity_history::require_supported_entities(operation, inputs);
}

pub(super) fn failed_non_destructive() {
    let inputs = geometry::operands();
    let before = snapshot::shapes(inputs.iter());
    let mut operation = BOPAlgoBOP::new();
    operation.add_argument(&inputs[0]);
    operation.set_operation(Operation::Cut);
    operation.set_non_destructive(true);
    operation.perform();
    assert!(
        operation.options().has_errors(),
        "cut without tool must report failure"
    );
    assert!(
        operation.shape().is_null(),
        "failed cut must not publish geometry"
    );
    let after = snapshot::shapes(inputs.iter());
    snapshot::assert_unchanged("failed missing-tool cut", &before, &after);
}
