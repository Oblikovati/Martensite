//! External composition reads and edits the actual operation-local builder state.
use martensite_tkbool::top_ope_brep_build::{
    BuilderExtensionState, BuilderExtensionStateMut, BuilderResultArray, TopOpeBRepBuildBuilder,
};
use martensite_tkbool::top_ope_brep_ds::{TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape, Solid};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Handle;

#[path = "builder_extension/control.rs"]
mod control;
#[path = "builder_extension/results.rs"]
mod results;
#[path = "builder_extension/shapes.rs"]
mod shapes;

fn builder() -> TopOpeBRepBuildBuilder {
    TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new())
}

fn distinct_shape() -> Shape {
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    compound.into_shape()
}

fn shape_list(shape: &Shape) -> List<Shape> {
    let mut list = List::default();
    list.append(shape.clone());
    list
}

#[test]
fn builder_extension_views_preserve_compiler_thread_traits() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<BuilderExtensionState<'static>>();
    send_sync::<BuilderExtensionStateMut<'static>>();
    send_sync::<TopOpeBRepBuildBuilder>();
}
