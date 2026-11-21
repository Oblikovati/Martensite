//! The typed shapes and the `TopoDS` casts.
use super::errors::type_mismatch;
use super::{Shape, ShapeView};
use martensite_tkernel::ncollection::HashKey;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{self, ShapeEnum};
use std::ops::{Deref, DerefMut};

/// The check of every `TopoDS` cast: a null shape or a shape of the requested type.
pub(super) fn check_cast(
    shape: &ShapeView<'_>,
    kind: ShapeEnum,
    cast: &str,
) -> Result<(), Failure> {
    let Some(tshape) = shape.tshape().get() else {
        return Ok(());
    };
    let found = tshape.shape_type();
    if found != kind {
        return Err(type_mismatch(cast, top_abs::shape_type_to_string(found)));
    }
    Ok(())
}

/// Declares a typed shape (a `TopoDS_Shape` subclass with no field) and its cast.
macro_rules! typed_shape {
    ($(#[$doc:meta])* $class:ident, $view:ident, $kind:expr, $cast:ident, $copy:ident, $native_cast:literal) => {
        $(#[$doc])*
        pub type $class = $view<'static>;

        #[doc = concat!("The fields of `", $native_cast, "` during a scoped read.")]
        #[derive(PartialEq, Eq, Hash)]
        pub struct $view<'a>(ShapeView<'a>);

        impl $view<'static> {
            /// Wrap an owning shape without a type check. Example: `let typed = Edge::unchecked(shape);`.
            pub fn unchecked(shape: Shape) -> Self {
                Self(shape)
            }
            /// The underlying owning shape. Example: `typed.into_shape()`.
            pub fn into_shape(self) -> Shape {
                self.0
            }
        }
        impl $view<'_> {
            /// Acquire an owning typed copy from a scoped shape.
            #[doc = concat!("Example: `view.", stringify!($copy), "()`.")]
            pub fn $copy(&self) -> $class {
                $view(self.0.to_shape())
            }
        }
        impl Clone for $view<'static> {
            fn clone(&self) -> Self {
                self.$copy()
            }
        }
        impl Default for $view<'static> {
            fn default() -> Self {
                Self(Shape::default())
            }
        }
        impl<'a> Deref for $view<'a> {
            type Target = ShapeView<'a>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $view<'static> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<'a> AsRef<ShapeView<'a>> for $view<'a> {
            fn as_ref(&self) -> &ShapeView<'a> {
                &self.0
            }
        }
        impl From<$class> for Shape {
            fn from(typed: $class) -> Shape {
                typed.0
            }
        }
        impl std::fmt::Debug for $view<'_> {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.debug_tuple(stringify!($class)).field(&self.0).finish()
            }
        }

        impl HashKey for $view<'_> {
            const WALKS_IN_ORDER_ADDED: bool = true;
            fn kernel_hash(&self) -> usize {
                self.0.hash_code()
            }
        }

        #[doc = concat!("`", $native_cast, "(S)`: `S` as a typed shape; Standard_TypeMismatch when \
                         `S` is not null and has another type. This owned convenience conversion \
                         does not implement the native reference-returning overloads.")]
        pub fn $cast(shape: &ShapeView<'_>) -> Result<$class, Failure> {
            check_cast(shape, $kind, $native_cast)?;
            Ok($view(shape.to_shape()))
        }
    };
}

typed_shape!(
    /// `TopoDS_Vertex`; `topods::vertex(&shape)?`.
    Vertex, VertexView, ShapeEnum::Vertex, vertex, to_vertex, "TopoDS::Vertex"
);
typed_shape!(
    /// `TopoDS_Edge`; `topods::edge(&shape)?`.
    Edge, EdgeView, ShapeEnum::Edge, edge, to_edge, "TopoDS::Edge"
);
typed_shape!(
    /// `TopoDS_Wire`; `topods::wire(&shape)?`.
    Wire, WireView, ShapeEnum::Wire, wire, to_wire, "TopoDS::Wire"
);
typed_shape!(
    /// `TopoDS_Face`; `topods::face(&shape)?`.
    Face, FaceView, ShapeEnum::Face, face, to_face, "TopoDS::Face"
);
typed_shape!(
    /// `TopoDS_Shell`; `topods::shell(&shape)?`.
    Shell, ShellView, ShapeEnum::Shell, shell, to_shell, "TopoDS::Shell"
);
typed_shape!(
    /// `TopoDS_Solid`; `topods::solid(&shape)?`.
    Solid, SolidView, ShapeEnum::Solid, solid, to_solid, "TopoDS::Solid"
);
typed_shape!(
    /// `TopoDS_CompSolid`; `topods::comp_solid(&shape)?`.
    CompSolid, CompSolidView, ShapeEnum::CompSolid, comp_solid, to_comp_solid, "TopoDS::CompSolid"
);
typed_shape!(
    /// `TopoDS_Compound`; `topods::compound(&shape)?`.
    Compound, CompoundView, ShapeEnum::Compound, compound, to_compound, "TopoDS::Compound"
);

impl<'a> EdgeView<'a> {
    pub(super) fn scoped(shape: ShapeView<'a>) -> Self {
        Self(shape)
    }
}

impl<'a> FaceView<'a> {
    pub(super) fn scoped(shape: ShapeView<'a>) -> Self {
        Self(shape)
    }
}
