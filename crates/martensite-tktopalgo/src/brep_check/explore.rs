//! The explorer loops that every BRepCheck checker repeats.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

/// The sub-shapes of `kind` of `shape` in explorer order (`TopExp_Explorer(S, kind)`).
pub(crate) fn sub_shapes(shape: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    sub_shapes_avoiding(shape, kind, ShapeEnum::Shape)
}

/// The sub-shapes of `kind` of `shape` not inside a sub-shape of `avoid`, in explorer
/// order (`TopExp_Explorer(S, kind, avoid)`; BRepGProp uses it too).
pub(crate) fn sub_shapes_avoiding(
    shape: &Shape,
    kind: ShapeEnum,
    avoid: ShapeEnum,
) -> Result<Vec<Shape>, Failure> {
    let mut explorer = Explorer::new(shape, kind, avoid)?;
    let mut found = Vec::new();
    while explorer.more() {
        found.push(explorer.current().clone());
        explorer.next()?;
    }
    Ok(found)
}

/// The InContext test `for (exp(S, kind); ...) if (Current().IsSame(myShape)) break;
/// if (!exp.More())`: whether `context` holds a sub-shape of `kind` same as `shape`.
pub(crate) fn holds_same(context: &Shape, kind: ShapeEnum, shape: &Shape) -> Result<bool, Failure> {
    Ok(sub_shapes(context, kind)?
        .iter()
        .any(|sub| sub.is_same(shape)))
}

/// The native `IsOriented(S)`: FORWARD or REVERSED.
pub(crate) fn is_oriented(shape: &Shape) -> bool {
    matches!(
        shape.orientation(),
        Orientation::Forward | Orientation::Reversed
    )
}
