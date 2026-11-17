use crate::bop_algo::builder::BuilderHooks;
use crate::bop_algo::{BOPAlgoBOP, Operation, ProgressCompletion};
use martensite_tkbrep::{
    bintools::{self, WriteOptions},
    topexp::Explorer,
    topods::Shape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
pub(super) fn cuboid(origin: [f64; 3], extent: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::at_point(
        Point3::new(origin[0], origin[1], origin[2]),
        extent[0],
        extent[1],
        extent[2],
    )
    .unwrap()
    .shape()
    .unwrap()
    .clone()
}
pub(super) fn operation(
    sources: &[Shape],
    kind: Operation,
    history: bool,
    ranged: bool,
) -> BOPAlgoBOP {
    let mut bop = BOPAlgoBOP::new();
    bop.set_operation(kind);
    bop.set_non_destructive(true);
    bop.set_to_fill_history(history);
    bop.add_argument(&sources[0]);
    for tool in &sources[1..] {
        bop.add_tool(tool);
    }
    execute(&mut bop, ranged);
    assert!(!bop.options().has_errors());
    bop
}
fn execute(bop: &mut BOPAlgoBOP, ranged: bool) {
    if ranged {
        assert!(matches!(
            bop.perform_with_progress(&ProgressRange::default()),
            Ok(ProgressCompletion::Completed)
        ));
    } else {
        bop.perform();
    }
}
pub(super) fn entities(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    Explorer::new(shape, kind, ShapeEnum::Shape)
        .unwrap()
        .collect()
}
pub(super) fn covered(bop: &BOPAlgoBOP, parent: &Shape, child: &Shape) -> bool {
    parent.is_same(child)
        || bop
            .modified(parent)
            .unwrap()
            .iter()
            .chain(bop.generated(parent).unwrap().iter())
            .any(|image| image.is_same(child))
}
pub(super) fn require_coverage(bop: &BOPAlgoBOP, sources: &[Shape]) {
    let parents: Vec<_> = sources
        .iter()
        .flat_map(|source| {
            supported()
                .into_iter()
                .flat_map(|kind| entities(source, kind))
        })
        .collect();
    for kind in supported() {
        for child in entities(bop.shape(), kind) {
            assert!(
                parents.iter().any(|parent| covered(bop, parent, &child)),
                "uncovered {kind:?}"
            );
        }
    }
}
fn supported() -> [ShapeEnum; 4] {
    [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Face,
        ShapeEnum::Solid,
    ]
}
pub(super) fn encode(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    bintools::write(
        shape,
        &mut stream,
        WriteOptions::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    stream.seekg(0);
    read_bytes(&mut stream)
}
fn read_bytes(stream: &mut IoStream) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0];
        stream.read(&mut byte);
        if stream.gcount() == 0 {
            break;
        }
        bytes.push(byte[0]);
    }
    bytes
}

pub(super) fn selected_cells_from(bop: &BOPAlgoBOP, source: &Shape) -> usize {
    entities(&bop.rc, ShapeEnum::Solid)
        .iter()
        .filter(|cell| {
            cell.is_same(source)
                || bop
                    .builder()
                    .origins()
                    .seek(cell)
                    .is_some_and(|origins| origins.iter().any(|origin| origin.is_same(source)))
        })
        .count()
}
