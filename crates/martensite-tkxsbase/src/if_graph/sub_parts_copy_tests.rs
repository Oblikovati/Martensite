//! Constructor evaluation order and failed-construction ownership are observable contracts.
use super::{IFGraphSubPartsEvaluate, IFGraphSubPartsIterator};
use crate::interface::{InterfaceGraph, interface_error, test_norm::fake_chain};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};

struct EvaluatedParts {
    parts: IFGraphSubPartsIterator,
    replacement: Handle<StandardTransient>,
    evaluations: usize,
    fail: bool,
}

impl IFGraphSubPartsEvaluate for EvaluatedParts {
    fn sub_parts(&self) -> &IFGraphSubPartsIterator {
        &self.parts
    }
    fn sub_parts_mut(&mut self) -> &mut IFGraphSubPartsIterator {
        &mut self.parts
    }
    fn evaluate(&mut self) -> Result<(), Failure> {
        self.evaluations += 1;
        self.parts.thegraph.reset_status()?;
        self.parts.set_part_num(1)?;
        self.parts.get_from_entity(&self.replacement, false)?;
        if self.fail {
            return Err(interface_error("owned evaluation failure"));
        }
        Ok(())
    }
}

fn evaluated_source(
    graph: &InterfaceGraph,
    replacement: &Handle<StandardTransient>,
    fail: bool,
) -> EvaluatedParts {
    let mut parts = IFGraphSubPartsIterator::new(graph, false).unwrap();
    parts.add_part();
    EvaluatedParts {
        parts,
        replacement: replacement.clone(),
        evaluations: 0,
        fail,
    }
}

fn assert_evaluated_copy(copy: &IFGraphSubPartsIterator, entities: &[Handle<StandardTransient>]) {
    assert_eq!((copy.nb_parts(), copy.part_num()), (1, 0));
    assert_eq!(copy.entity_part_num(&entities[0]).unwrap(), 1);
    assert_eq!(copy.entity_part_num(&entities[1]).unwrap(), 1);
    assert!(!copy.is_single().unwrap());
}

#[test]
fn copy_captures_graph_before_virtual_source_evaluation_and_exhausts_source() {
    let (model, entities) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let mut source = evaluated_source(&graph, &entities[1], false);
    source.parts.get_from_entity(&entities[0], false).unwrap();
    let copy = IFGraphSubPartsIterator::from_other(&mut source).unwrap();
    assert_eq!(source.evaluations, 1);
    assert!(!source.more().unwrap());
    assert!(!source.parts.is_loaded(&entities[0]).unwrap());
    assert!(source.parts.is_loaded(&entities[1]).unwrap());
    assert_evaluated_copy(&copy, &entities);
}

#[test]
fn failed_virtual_evaluation_preserves_source_effects_without_leaking_model_owner() {
    let (model, entities) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let mut source = evaluated_source(&graph, &entities[1], true);
    let before = model.get().unwrap().transient_base().ref_count();
    let error = IFGraphSubPartsIterator::from_other(&mut source)
        .err()
        .unwrap();
    assert_eq!(error.exception_type(), "InterfaceInterfaceError");
    assert_eq!(error.message(), "owned evaluation failure");
    assert_eq!(source.evaluations, 1);
    assert!(source.parts.is_loaded(&entities[1]).unwrap());
    assert_eq!(model.get().unwrap().transient_base().ref_count(), before);
}

#[test]
fn empty_source_copy_is_exhausted_without_first_entity_storage() {
    let (model, _) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let mut source = IFGraphSubPartsIterator::new(&graph, false).unwrap();
    let mut copy = IFGraphSubPartsIterator::from_other(&mut source).unwrap();
    assert_eq!((copy.nb_parts(), copy.part_num()), (0, 0));
    assert!(!source.more().unwrap());
    assert!(!copy.more().unwrap());
    assert_eq!(
        copy.first_entity().err().unwrap().exception_type(),
        "Standard_NoSuchObject"
    );
    copy.start().unwrap();
    assert!(!copy.more().unwrap());
}
