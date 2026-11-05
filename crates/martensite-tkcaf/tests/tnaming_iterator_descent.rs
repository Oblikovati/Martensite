//! Independent evolution-chain checks: iterator construction descends one generation.
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNewShapeIterator, TNamingOldShapeIterator};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdf::{TdfData, TdfLabel};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

struct EvolutionChain {
    _framework: Handle<TdfData>,
    root: TdfLabel,
    vertices: [Shape; 3],
}

fn chain_vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0.0, 0.0))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

fn evolution_chain() -> EvolutionChain {
    let framework = TdfData::new();
    let root = framework.root();
    let vertices = [0.0, 1.0, 2.0].map(chain_vertex);
    for (index, pair) in vertices.windows(2).enumerate() {
        let label = root
            .find_child(i32::try_from(index + 1).unwrap(), true)
            .unwrap();
        TNamingBuilder::new(&label)
            .unwrap()
            .modify(&pair[0], &pair[1])
            .unwrap();
    }
    EvolutionChain {
        _framework: framework,
        root,
        vertices,
    }
}

#[test]
fn descendant_constructor_advances_to_next_generation_without_moving_source() {
    let chain = evolution_chain();
    let source = TNamingNewShapeIterator::from_label(&chain.vertices[0], &chain.root).unwrap();
    let mut descendant = TNamingNewShapeIterator::from_new_shape_iterator(&source).unwrap();
    assert!(source.shape().unwrap().is_same(&chain.vertices[1]));
    assert!(descendant.shape().unwrap().is_same(&chain.vertices[2]));
    descendant.next().unwrap();
    assert!(!descendant.more());
    assert!(source.more());
}

#[test]
fn ancestor_constructor_advances_to_previous_generation_without_moving_source() {
    let chain = evolution_chain();
    let source = TNamingOldShapeIterator::from_label(&chain.vertices[2], &chain.root).unwrap();
    let mut ancestor = TNamingOldShapeIterator::from_old_shape_iterator(&source).unwrap();
    assert!(source.shape().unwrap().is_same(&chain.vertices[1]));
    assert!(ancestor.shape().unwrap().is_same(&chain.vertices[0]));
    ancestor.next().unwrap();
    assert!(!ancestor.more());
    assert!(source.more());
}

#[test]
fn descendant_constructor_ends_after_last_generation() {
    let chain = evolution_chain();
    let source = TNamingNewShapeIterator::from_label(&chain.vertices[1], &chain.root).unwrap();
    let descendant = TNamingNewShapeIterator::from_new_shape_iterator(&source).unwrap();
    assert!(!descendant.more());
    assert!(source.shape().unwrap().is_same(&chain.vertices[2]));
}

#[test]
fn ancestor_constructor_ends_before_first_generation() {
    let chain = evolution_chain();
    let source = TNamingOldShapeIterator::from_label(&chain.vertices[1], &chain.root).unwrap();
    let ancestor = TNamingOldShapeIterator::from_old_shape_iterator(&source).unwrap();
    assert!(!ancestor.more());
    assert!(source.shape().unwrap().is_same(&chain.vertices[0]));
}
