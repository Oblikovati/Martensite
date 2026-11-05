//! Whole-map extension access preserves indexed-list behavior and rebuild visibility.
use super::{CURVE_KEYS, TopOpeBRepBuildBuilder, TopOpeBRepDSBuildTool, connected_edges};
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSHDataStructure;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator, Wire};
use martensite_tkernel::ncollection::{DataMap, List};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeWire;

fn one_edge(edge: &Shape) -> List<Shape> {
    List::from_iter([edge.clone()])
}

fn populated_builder(edge: &Shape) -> TopOpeBRepBuildBuilder {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    for key in CURVE_KEYS {
        builder
            .change_curve_edge_results()
            .bind(key, one_edge(edge));
    }
    builder
}

#[test]
fn whole_map_accessors_alias_and_indexed_mutation_updates_the_same_list() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut builder = populated_builder(&first);
    let shared = std::ptr::from_ref(builder.curve_edge_results());
    let mutable = std::ptr::from_mut(builder.change_curve_edge_results());
    assert_eq!(shared, mutable.cast_const());
    builder.change_new_edges(1).append(second.clone());
    let direct = builder.curve_edge_results().find(&1)?;
    assert_eq!(
        std::ptr::from_ref(direct),
        std::ptr::from_ref(builder.new_edges(1))
    );
    let parts: Vec<&Shape> = direct.iter().collect();
    assert_eq!(parts.len(), 2);
    assert!(parts[0].is_same(&first) && parts[1].is_same(&second));
    assert!(builder.change_curve_edge_results().unbind(&1));
    assert!(builder.new_edges(1).is_empty());
    Ok(())
}

fn assert_map_keys(builder: &TopOpeBRepBuildBuilder) {
    let mut keys: Vec<i32> = builder
        .curve_edge_results()
        .items()
        .map(|(key, _)| *key)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, CURVE_KEYS);
}

#[test]
fn replacing_the_whole_map_replaces_all_indexed_results() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut builder = populated_builder(&first);
    assert_map_keys(&builder);
    let mut replacement = DataMap::default();
    replacement.bind(9, one_edge(&second));
    *builder.change_curve_edge_results() = replacement;
    assert_eq!(builder.curve_edge_results().len(), 1);
    for key in CURVE_KEYS {
        assert!(builder.new_edges(key).is_empty());
    }
    assert!(
        builder
            .new_edges(9)
            .iter()
            .next()
            .expect("one edge")
            .is_same(&second)
    );
    Ok(())
}

#[test]
fn rebuilding_clears_the_same_live_map_object() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut builder = populated_builder(&edge);
    let before = std::ptr::from_ref(builder.curve_edge_results());
    builder.build_edges(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    assert_eq!(before, std::ptr::from_ref(builder.curve_edge_results()));
    assert!(builder.curve_edge_results().is_empty());
    Ok(())
}

struct CurveWireAssembly {
    builder: TopOpeBRepBuildBuilder,
}

impl CurveWireAssembly {
    fn wire(&self) -> Result<Wire, Failure> {
        let mut keys: Vec<i32> = self
            .builder
            .curve_edge_results()
            .items()
            .map(|(key, _)| *key)
            .collect();
        keys.sort_unstable();
        let mut maker = BRepBuilderAPIMakeWire::new();
        for key in keys {
            for edge in self.builder.curve_edge_results().find(&key)?.iter() {
                maker.add_edge(&topods::edge(edge)?)?;
            }
        }
        maker.wire()
    }
}

#[test]
fn an_external_operation_builds_a_wire_from_the_stored_curve_results() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut assembly = CurveWireAssembly {
        builder: TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new()),
    };
    assembly
        .builder
        .change_curve_edge_results()
        .bind(10, one_edge(&second));
    assembly
        .builder
        .change_curve_edge_results()
        .bind(-4, one_edge(&first));
    let wire = assembly.wire()?;
    let edges: Vec<Shape> = ShapeIterator::new(wire.as_ref(), true, true).collect();
    assert_eq!(edges.len(), 2);
    assert!(edges[0].is_same(&first) && edges[1].is_same(&second));
    assert_eq!(assembly.builder.curve_edge_results().len(), 2);
    Ok(())
}
