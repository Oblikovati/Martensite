//! The TShape classes of the eight shape types.
use super::tshape::{TShape, TShapeBase};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Declares a concrete TShape class: its base data, its descriptor, a constructor
/// that applies the native constructor flags, and `EmptyCopy` returning a new
/// instance of the same class, as each native `EmptyCopy` does.
macro_rules! concrete_tshape {
    ($(#[$doc:meta])* $class:ident, $native:literal, $kind:expr, |$base:ident| $defaults:block $(, $source:ident)?) => {
        $(#[$doc])*
        pub struct $class {
            tshape: TShapeBase,
        }
        impl_transient!($class, $native, dyn TShape, interfaces: [dyn TShape $(, dyn $source)?]);
        $(impl $source for $class {})?
        impl $class {
            /// The native constructor: the type and its default flags.
            pub fn new() -> Self {
                let $base = TShapeBase::new($kind);
                $defaults
                Self {
                    tshape: $base,
                }
            }
        }
        impl Default for $class {
            fn default() -> Self {
                Self::new()
            }
        }
        impl TShape for $class {
            fn tshape_base(&self) -> &TShapeBase {
                &self.tshape
            }
            fn empty_copy(&self) -> Result<Handle<dyn TShape>, Failure> {
                Ok(Handle::new(Self::new()).into_upcast())
            }
        }
    };
}

concrete_tshape!(
    /// `TopoDS_TEdge`: its constructor is protected natively (BRep_TEdge derives from
    /// it), but its `EmptyCopy` returns a plain `TopoDS_TEdge`; `TEdge::new()`.
    TEdge, "TopoDS_TEdge", ShapeEnum::Edge, |base| {}, TEdgeSource
);
concrete_tshape!(
    /// `TopoDS_TWire`: an empty wire; `TWire::new()`.
    TWire, "TopoDS_TWire", ShapeEnum::Wire, |base| {}
);
concrete_tshape!(
    /// `TopoDS_TFace`: a face without geometry; `TFace::new()`.
    TFace, "TopoDS_TFace", ShapeEnum::Face, |base| {}, TFaceSource
);
concrete_tshape!(
    /// `TopoDS_TShell`: an empty shell; `TShell::new()`.
    TShell, "TopoDS_TShell", ShapeEnum::Shell, |base| {}
);
concrete_tshape!(
    /// `TopoDS_TSolid`: an empty solid, created not orientable; `TSolid::new()`.
    TSolid, "TopoDS_TSolid", ShapeEnum::Solid, |base| {
        base.set_bit(super::bit_layout::BIT_ORIENTABLE, false);
    }
);
concrete_tshape!(
    /// `TopoDS_TCompSolid`: an empty composite solid; `TCompSolid::new()`.
    TCompSolid, "TopoDS_TCompSolid", ShapeEnum::CompSolid, |base| {}
);
concrete_tshape!(
    /// `TopoDS_TCompound`: an empty compound, created not orientable; `TCompound::new()`.
    TCompound, "TopoDS_TCompound", ShapeEnum::Compound, |base| {
        base.set_bit(super::bit_layout::BIT_ORIENTABLE, false);
    }
);

/// `TopoDS_TVertex`: the default fields of a vertex TShape. Its constructor makes the
/// vertex Closed and Convex. A vertex with `EmptyCopy` implements [`TVertexSource`].
///
/// ```
/// use martensite_tkbrep::topods::TVertex;
/// let vertex = TVertex::new();
/// assert_eq!(vertex.tshape_base().state() & 0x0500, 0x0500);
/// ```
pub struct TVertex {
    tshape: TShapeBase,
}
impl_transient!(TVertex, "TopoDS_TVertex", dyn TShape);

impl TVertex {
    /// The protected `TopoDS_TVertex()`, for derived classes; `TVertex::new()`.
    pub fn new() -> Self {
        Self {
            tshape: Self::initial_base(),
        }
    }

    pub(crate) fn initial_base() -> TShapeBase {
        let tshape = TShapeBase::new(ShapeEnum::Vertex);
        tshape.set_bit(super::bit_layout::BIT_CLOSED, true);
        tshape.set_bit(super::bit_layout::BIT_CONVEX, true);
        tshape
    }

    /// The embedded `TopoDS_TShape` data, which a derived class returns from
    /// `TShape::tshape_base`; `vertex.tshape_base()`.
    pub fn tshape_base(&self) -> &TShapeBase {
        &self.tshape
    }
}

impl Default for TVertex {
    fn default() -> Self {
        Self::new()
    }
}

/// The `TopoDS_TEdge` topology interface. Example: `edge.upcast::<dyn TEdgeSource>()`.
pub trait TEdgeSource: TShape {}
parent_interface!(dyn TEdgeSource, TEdge, [dyn TShape]);

/// The `TopoDS_TFace` topology interface. Example: `face.upcast::<dyn TFaceSource>()`.
pub trait TFaceSource: TShape {}
parent_interface!(dyn TFaceSource, TFace, [dyn TShape]);

/// The `TopoDS_TVertex` topology interface. Example: `vertex.upcast::<dyn TVertexSource>()`.
pub trait TVertexSource: TShape {}
parent_interface!(dyn TVertexSource, TVertex, [dyn TShape]);
