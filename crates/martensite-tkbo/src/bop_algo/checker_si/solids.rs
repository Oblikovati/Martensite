//! `BOPAlgoCheckerSI_1`: the interferences of vertices, edges, faces and solids with
//! the solids of the checked shape (`PerformVZ`, `PerformEZ`, `PerformFZ`, `PerformZZ`).
use super::BOPAlgoCheckerSI;
use crate::bop_ds::{DataStructure, InterfEZ, InterfFZ, InterfVZ, InterfZZ, Pair};
use crate::int_tools::Context;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods;
use martensite_tkernel::ncollection::Map;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{ShapeEnum, State};

fn take_iterator_pairs(
    iterator: &mut crate::bop_ds::BOPDSIterator,
) -> Result<Vec<(i32, i32)>, Failure> {
    let mut pairs = Vec::new();
    while iterator.more() {
        pairs.push(iterator.value()?);
        iterator.next();
    }
    Ok(pairs)
}

/// One vertex/solid classification (`BOPAlgoVertexSolid`): the record names the source
/// vertex, the classified point is its same-domain vertex.
struct VertexSolidTask {
    vertex: i32,
    classified: i32,
    solid: i32,
}

fn distinct_vertex_solid_task(
    structure: &DataStructure,
    vertex: i32,
    solid: i32,
    fence: &mut Map<Pair>,
) -> Option<VertexSolidTask> {
    let mut classified = vertex;
    structure.has_shape_sd_into(vertex, &mut classified);
    fence
        .add(Pair::new(classified, solid))
        .then_some(VertexSolidTask {
            vertex,
            classified,
            solid,
        })
}

/// Classify the vertex point with its own tolerance (`BOPAlgoVertexSolid::Perform`).
fn classify_vertex_solid(
    vertex: &martensite_tkbrep::topods::Vertex,
    solid: &martensite_tkbrep::topods::Solid,
    context: &Context,
) -> Result<State, Failure> {
    let classifier = context.solid_classifier(solid)?;
    let mut classifier = classifier.write();
    classifier.perform(&BRepTool::pnt(vertex)?, BRepTool::vertex_tolerance(vertex)?)?;
    Ok(classifier.state())
}

/// Appends the edge/solid or face/solid interference record.
fn record_shape_solid(structure: &mut DataStructure, kind: ShapeEnum, shape: i32, solid: i32) {
    if kind == ShapeEnum::Edge {
        structure
            .interf_ez()
            .append(InterfEZ::default())
            .set_indices(shape, solid);
    } else {
        structure
            .interf_fz()
            .append(InterfFZ::default())
            .set_indices(shape, solid);
    }
}

#[path = "solids/pair_work.rs"]
mod pair_work;
#[path = "solids/progress.rs"]
mod progress;
#[path = "solids/vertex_work.rs"]
mod vertex_work;
