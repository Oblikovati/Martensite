//! The STEP selections and the counter directly on two read files, as a run of the same
//! calls gave.
use martensite_tkdestep::step_control::{StepControlController, StepControlReader};
use martensite_tkdestep::step_selections::{
    STEPSelectionsAssemblyExplorer, STEPSelectionsCounter, STEPSelectionsSelectAssembly,
    STEPSelectionsSelectFaces, STEPSelectionsSelectGSCurves, STEPSelectionsSelectInstances,
};
use martensite_tkdestep::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkxsbase::if_select::{
    IFSelectReturnStatus, IFSelectSelectDeduct, IFSelectSelectModelEntities, IFSelectSelection,
};
use martensite_tkxsbase::interface::{
    InterfaceGraph, InterfaceInterfaceModel, InterfaceModelEntities,
};
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};
use std::path::PathBuf;

/// The assembly of the dataset, with materials.
const ASSEMBLY: &str = "as1-oc-214-mat.stp";
/// One open-shell box.
const ONE_SOLID: &str = "bug25169_box-openshell.stp";

fn dataset_file(name: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step");
    root.join(name).to_string_lossy().into_owned()
}

struct ReadFile {
    model: Handle<dyn InterfaceInterfaceModel>,
    graph: InterfaceGraph,
    /// The session owns the model: dropping it empties the model.
    _session: Handle<XSControlWorkSession>,
}

fn read(name: &str) -> ReadFile {
    StepControlController::init().expect("controller");
    let mut reader = StepControlReader::new().expect("reader");
    assert_eq!(
        reader.read_file(&dataset_file(name)).expect("read"),
        IFSelectReturnStatus::RetDone
    );
    let session = reader.xs_reader().ws();
    let model = session.model();
    let hgraph = session.graph().expect("a graph");
    let graph = hgraph.get().expect("a graph").graph().clone();
    ReadFile {
        model,
        graph,
        _session: session,
    }
}

fn counted(file: &ReadFile) -> [i32; 10] {
    let mut counter = STEPSelectionsCounter::new();
    for rank in 1..=file.model.nb_entities() {
        let entity = file.model.value(rank).expect("an entity");
        if entity.is_kind(StepShapeShapeDefinitionRepresentation::static_type()) {
            counter.count(&file.graph, &entity).expect("a count");
        }
    }
    [
        counter.nb_instances_of_faces(),
        counter.nb_source_faces(),
        counter.nb_instances_of_shells(),
        counter.nb_source_shells(),
        counter.nb_instances_of_solids(),
        counter.nb_source_solids(),
        counter.nb_instances_of_edges(),
        counter.nb_source_edges(),
        counter.nb_instances_of_wires(),
        counter.nb_source_wires(),
    ]
}

/// A selection over every entity of the model: how many it gives.
fn nb_selected<S: IFSelectSelectDeduct + IFSelectSelection>(selection: S, file: &ReadFile) -> i32 {
    selection.set_input(&Handle::new(IFSelectSelectModelEntities::default()).cast());
    selection
        .root_result(&file.graph)
        .expect("a result")
        .nb_entities()
}

#[test]
fn an_assembly_counts_instances_above_its_sources() {
    let file = read(ASSEMBLY);
    assert_eq!(file.model.nb_entities(), 6484);
    assert_eq!(counted(&file), [106, 53, 10, 5, 10, 5, 0, 0, 0, 0]);
}

#[test]
fn one_solid_counts_each_of_its_entities_once() {
    let file = read(ONE_SOLID);
    assert_eq!(file.model.nb_entities(), 350);
    assert_eq!(counted(&file), [6, 6, 1, 1, 1, 1, 0, 0, 0, 0]);
}

#[test]
fn the_explorer_finds_the_assemblies_and_their_links() {
    let file = read(ASSEMBLY);
    let explorer = STEPSelectionsAssemblyExplorer::new(&file.graph).expect("an explorer");
    assert_eq!(explorer.nb_assemblies(), 14);
    let links: Vec<i32> = (1..=14)
        .map(|rank| {
            let root = explorer.root(rank).expect("a root");
            root.get()
                .expect("a component")
                .get_list()
                .get()
                .map_or(0, |list| list.sequence(|items| items.iter().count()) as i32)
        })
        .collect();
    assert_eq!(links, [4, 3, 0, 0, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn the_explorer_of_one_solid_finds_one_root_with_no_link() {
    let file = read(ONE_SOLID);
    let explorer = STEPSelectionsAssemblyExplorer::new(&file.graph).expect("an explorer");
    assert_eq!(explorer.nb_assemblies(), 1);
    let root = explorer.root(1).expect("a root");
    assert_eq!(
        root.get()
            .expect("a component")
            .get_list()
            .get()
            .map_or(0, |list| list.sequence(|items| items.iter().count()) as i32),
        0
    );
}

#[test]
fn the_selections_on_an_assembly() {
    let file = read(ASSEMBLY);
    assert_eq!(nb_selected(STEPSelectionsSelectFaces::default(), &file), 53);
    assert_eq!(
        nb_selected(STEPSelectionsSelectGSCurves::default(), &file),
        0
    );
    assert_eq!(
        nb_selected(STEPSelectionsSelectInstances::default(), &file),
        49208
    );
    assert_eq!(
        nb_selected(STEPSelectionsSelectAssembly::default(), &file),
        13
    );
}

#[test]
fn the_selections_on_one_solid() {
    let file = read(ONE_SOLID);
    assert_eq!(nb_selected(STEPSelectionsSelectFaces::default(), &file), 6);
    assert_eq!(
        nb_selected(STEPSelectionsSelectGSCurves::default(), &file),
        0
    );
    assert_eq!(
        nb_selected(STEPSelectionsSelectInstances::default(), &file),
        895
    );
    assert_eq!(
        nb_selected(STEPSelectionsSelectAssembly::default(), &file),
        0
    );
}
