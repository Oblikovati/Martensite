//! TopoDSToStep: shapes to the STEP topological entities. A translator is a value with `init`,
//! `is_done` and `Value`; `TopoDSToStepTool` holds what the translations of one shape share.
mod builder;
mod faceted_tool;
mod make_brep_with_voids;
mod make_faceted_brep;
mod make_faceted_brep_and_brep_with_voids;
mod make_geometric_curve_set;
mod make_manifold_solid_brep;
mod make_shell_based_surface_model;
mod make_step_edge;
mod make_step_face;
mod make_step_vertex;
mod make_step_wire;
mod make_tessellated_item;
mod package;
mod pcurves;
mod tessellated_items;
mod tool;
mod torus_as_revolution;
mod wireframe_builder;

pub use builder::TopoDSToStepBuilder;
pub use faceted_tool::TopoDSToStepFacetedTool;
pub use make_brep_with_voids::TopoDSToStepMakeBrepWithVoids;
pub use make_faceted_brep::TopoDSToStepMakeFacetedBrep;
pub use make_faceted_brep_and_brep_with_voids::TopoDSToStepMakeFacetedBrepAndBrepWithVoids;
pub use make_geometric_curve_set::TopoDSToStepMakeGeometricCurveSet;
pub use make_manifold_solid_brep::TopoDSToStepMakeManifoldSolidBrep;
pub use make_shell_based_surface_model::TopoDSToStepMakeShellBasedSurfaceModel;
pub use make_step_edge::TopoDSToStepMakeStepEdge;
pub use make_step_face::TopoDSToStepMakeStepFace;
pub use make_step_vertex::TopoDSToStepMakeStepVertex;
pub use make_step_wire::TopoDSToStepMakeStepWire;
pub use make_tessellated_item::TopoDSToStepMakeTessellatedItem;
pub use package::{
    TopoDSToStep, TopoDSToStepBuilderError, TopoDSToStepFacetedError, TopoDSToStepMakeEdgeError,
    TopoDSToStepMakeFaceError, TopoDSToStepMakeVertexError, TopoDSToStepMakeWireError,
    TopoDSToStepRoot,
};
pub use tool::{TopoDSToStepShapeMap, TopoDSToStepTool};
pub use wireframe_builder::TopoDSToStepWireframeBuilder;

pub(crate) use package::{add_warning, step_model_of};

/// Declares the `is_done`, `Value` and `tessellated_value` of a solid translator, whose `Value`
/// raises a not done failure while there is no result.
macro_rules! topo_ds_to_step_brep_class {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $entity:ty) => {
        $(#[$meta])*
        #[derive(Clone, Default)]
        pub struct $class {
            /// Root.
            pub root: $crate::topo_ds_to_step::TopoDSToStepRoot,
            result: martensite_tkernel::standard::Handle<$entity>,
            tessellated: martensite_tkernel::standard::Handle<$crate::step_visual::StepVisualTessellatedItem>,
        }
        impl $class {
            /// `is_done`.
            pub fn is_done(&self) -> bool {
                self.root.is_done()
            }
            /// `Value`.
            ///
            /// # Errors
            /// a not done failure when the translation gave no result.
            pub fn value(
                &self,
            ) -> Result<martensite_tkernel::standard::Handle<$entity>, martensite_tkernel::standard::Failure>
            {
                self.root.check_done(concat!($recorded, "::Value()"))?;
                Ok(self.result.clone())
            }
            /// `tessellated_value`: the optional tessellated result.
            ///
            /// # Errors
            /// a not done failure when the translation gave no result.
            pub fn tessellated_value(
                &self,
            ) -> Result<
                martensite_tkernel::standard::Handle<$crate::step_visual::StepVisualTessellatedItem>,
                martensite_tkernel::standard::Failure,
            > {
                self.root.check_done(concat!($recorded, "::TessellatedValue()"))?;
                Ok(self.tessellated.clone())
            }
        }
    };
}
pub(crate) use topo_ds_to_step_brep_class;
