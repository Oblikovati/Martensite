//! The commands of the bug scripts that read, copy and take apart named shapes.
use super::session::Session;
use crate::common_operation::restored_shape;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

/// Reads the shape stream of a `.brep` file whose root may be of any type.
fn restored_any(bytes: &[u8]) -> Shape {
    let root = root_type(bytes);
    restored_shape(bytes, root)
}

fn root_type(bytes: &[u8]) -> ShapeEnum {
    let end = bytes.iter().position(u8::is_ascii_whitespace).unwrap();
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    martensite_tkbrep::brep_tools::read(
        &mut shape,
        &mut stream,
        &martensite_tkbrep::brep::BRepBuilder::default(),
        &martensite_tkernel::message::ProgressRange::default(),
    )
    .unwrap();
    shape.shape_type().unwrap()
}

/// `restore file name`; `restore(s, "a", include_bytes!("a.brep"))`.
pub fn restore(session: &mut dyn Session, name: &str, bytes: &[u8]) {
    session.bind(name, &restored_any(bytes));
}

/// The shape bound to `name`, which the script needs bound.
pub fn bound(session: &dyn Session, name: &str) -> Shape {
    session
        .shape(name)
        .unwrap_or_else(|| panic!("{name}: expected a bound shape"))
        .clone()
}

/// `tcopy original copy`; `tcopy(s, "a12", "a21")`.
pub fn tcopy(session: &mut dyn Session, original: &str, copy: &str) {
    let source = bound(session, original);
    let mut maker = BRepBuilderAPICopy::with_shape(&source, true, false).unwrap();
    session.bind(copy, maker.shape().unwrap());
}

/// `copy original copy`: the same shape under a second name; `copy(s, "r_1", "result")`.
pub fn copy(session: &mut dyn Session, original: &str, copy: &str) {
    let shape = bound(session, original);
    session.bind(copy, &shape);
}

/// `renamevar from to`; `renamevar(s, "a_1", "a")` rebinds `a` to what `a_1` held.
pub fn renamevar(session: &mut dyn Session, from: &str, to: &str) {
    copy(session, from, to);
}

fn bind_numbered(session: &mut dyn Session, name: &str, parts: &[Shape]) -> Vec<String> {
    let mut names = Vec::with_capacity(parts.len());
    for (index, part) in parts.iter().enumerate() {
        names.push(format!("{name}_{}", index + 1));
        session.bind(&names[index], part);
    }
    names
}

/// `explode name`: binds `name_1`, `name_2`, ... to the direct children and returns the names.
/// Example: `explode_children(s, "s")`.
pub fn explode_children(session: &mut dyn Session, name: &str) -> Vec<String> {
    let shape = bound(session, name);
    let children: Vec<Shape> = ShapeIterator::new(&shape, true, true).collect();
    bind_numbered(session, name, &children)
}

/// `explode name kind`: the distinct sub-shapes of `kind` in the order of a first visit, without
/// `name` itself. Example: `explode_kind(s, "a", ShapeEnum::Edge)`.
pub fn explode_kind(session: &mut dyn Session, name: &str, kind: ShapeEnum) -> Vec<String> {
    let shape = bound(session, name);
    let mut seen = vec![shape.clone()];
    let mut explorer = martensite_tkbrep::topexp::Explorer::new(&shape, kind, ShapeEnum::Shape);
    let explorer = explorer.as_mut().unwrap();
    while explorer.more() {
        if !seen.iter().any(|known| known.is_same(explorer.current())) {
            seen.push(explorer.current().clone());
        }
        explorer.next().unwrap();
    }
    seen.remove(0);
    bind_numbered(session, name, &seen)
}
