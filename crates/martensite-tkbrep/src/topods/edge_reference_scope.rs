//! Bounded Martensite edge views avoid topology copies hidden by the owned convenience cast.
use super::{
    Edge, EdgeView, ShapeView, shape_reference_scope::scoped_reference_shape, typed::check_cast,
};
use martensite_tkernel::standard::{Failure, catch_native_failure};
use martensite_tkg3d::top_abs::ShapeEnum;
impl Edge {
    /// Lend a checked edge for one read-only operation without owning another TShape.
    /// Example: `Edge::with_shape_reference(&shape, |edge| Ok(edge.nb_children()))?`.
    ///
    /// ```compile_fail
    /// use martensite_tkbrep::topods::{Edge,Shape};
    /// let shape=Shape::default();
    /// let escaped=Edge::with_shape_reference(&shape,|edge|Ok(edge));
    /// ```
    pub fn with_shape_reference<R>(
        shape: &ShapeView<'_>,
        project: impl FnOnce(&EdgeView<'_>) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        with_typed_shape_reference(
            shape,
            ShapeEnum::Edge,
            "TopoDS::Edge",
            EdgeView::scoped,
            project,
        )
    }
}

pub(super) fn with_typed_shape_reference<'a, T, R>(
    shape: &'a ShapeView<'_>,
    kind: ShapeEnum,
    cast: &str,
    wrap: fn(ShapeView<'a>) -> T,
    project: impl FnOnce(&T) -> Result<R, Failure>,
) -> Result<R, Failure> {
    catch_native_failure(|| {
        check_cast(shape, kind, cast)?;
        let frame = scoped_reference_shape(shape.tshape(), shape.location(), shape.orientation());
        let typed = wrap(frame);
        project(&typed)
    })
}
