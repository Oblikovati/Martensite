//! The interactive-object trait implementations shared by every concrete dimension.

/// Implements `AisInteractiveObject` (a dimension), `PrsMgrPresentableObject` (display mode 0,
/// `Compute` through the inherent `compute_dimension`) and `SelectMgrSelectableObject`
/// (`PrsDimDimension::ComputeSelection`) for a struct with a `base: PrsDimDimensionBase`.
macro_rules! prs_dim_dimension_impls {
    ($type:ty) => {
        impl $crate::ais::AisInteractiveObject for $type {
            fn interactive_base(&self) -> &$crate::ais::AisInteractiveObjectBase {
                &self.base.interactive
            }
            /// `Type`: a dimension.
            fn kind_of_interactive(&self) -> $crate::ais::AisKindOfInteractive {
                $crate::ais::AisKindOfInteractive::Dimension
            }
        }
        impl $crate::prs_mgr::PrsMgrPresentableObject for $type {
            fn presentable_base(&self) -> &$crate::prs_mgr::PrsMgrPresentableObjectBase {
                &self.base.interactive.presentable
            }
            fn compute(
                &self,
                _: &dyn martensite_tkernel::standard::HandleReference<
                    $crate::prs_mgr::PrsMgrPresentationManager,
                >,
                prs: &dyn martensite_tkernel::standard::HandleReference<
                    dyn martensite_tkservice::graphic3d::Graphic3dStructureSource,
                >,
                mode: i32,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                self.compute_dimension(&prs.clone_handle(), mode)
            }
            /// `AcceptDisplayMode`: the whole dimension only.
            fn accept_display_mode(&self, mode: i32) -> bool {
                mode == $crate::prs_dim::PrsDimComputeMode::All as i32
            }
            $crate::ais_presentable_overrides!();
        }
        impl $crate::select_mgr::SelectMgrSelectableObject for $type {
            fn selectable_base(&self) -> &$crate::select_mgr::SelectMgrSelectableObjectBase {
                &self.base.interactive.selectable
            }
            fn compute_selection(
                &self,
                selection: &dyn martensite_tkernel::standard::HandleReference<
                    $crate::select_mgr::SelectMgrSelection,
                >,
                mode: i32,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::prs_dim::prs_dim_dimension_compute_selection(
                    self,
                    &selection.clone_handle(),
                    mode,
                )
            }
        }
    };
}
pub(crate) use prs_dim_dimension_impls;

/// Implements `AisInteractiveObject` (a relation), `PrsMgrPresentableObject` (display mode 0,
/// `Compute` through the inherent `compute_relation`, the relation `SetColor`/`UnsetColor`)
/// and `SelectMgrSelectableObject` (through the inherent `compute_relation_selection`) for a
/// struct with a `base: PrsDimRelationBase`.
macro_rules! prs_dim_relation_impls {
    ($type:ty) => {
        impl $crate::ais::AisInteractiveObject for $type {
            fn interactive_base(&self) -> &$crate::ais::AisInteractiveObjectBase {
                &self.base.interactive
            }
            /// `Type`: a relation.
            fn kind_of_interactive(&self) -> $crate::ais::AisKindOfInteractive {
                $crate::ais::AisKindOfInteractive::Relation
            }
        }
        impl $crate::prs_mgr::PrsMgrPresentableObject for $type {
            fn presentable_base(&self) -> &$crate::prs_mgr::PrsMgrPresentableObjectBase {
                &self.base.interactive.presentable
            }
            fn compute(
                &self,
                _: &dyn martensite_tkernel::standard::HandleReference<
                    $crate::prs_mgr::PrsMgrPresentationManager,
                >,
                prs: &dyn martensite_tkernel::standard::HandleReference<
                    dyn martensite_tkservice::graphic3d::Graphic3dStructureSource,
                >,
                mode: i32,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                self.compute_relation(&prs.clone_handle(), mode)
            }
            /// `AcceptDisplayMode`: mode 0 only.
            fn accept_display_mode(&self, mode: i32) -> bool {
                mode == 0
            }
            fn set_color(
                &self,
                color: martensite_tkernel::quantity::Color,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::prs_dim::prs_dim_relation_set_color(self, color)
            }
            fn unset_color(&self) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::prs_dim::prs_dim_relation_unset_color(self)
            }
            $crate::ais_presentable_overrides!();
        }
        impl $crate::select_mgr::SelectMgrSelectableObject for $type {
            fn selectable_base(&self) -> &$crate::select_mgr::SelectMgrSelectableObjectBase {
                &self.base.interactive.selectable
            }
            fn compute_selection(
                &self,
                selection: &dyn martensite_tkernel::standard::HandleReference<
                    $crate::select_mgr::SelectMgrSelection,
                >,
                mode: i32,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                self.compute_relation_selection(&selection.clone_handle(), mode)
            }
        }
    };
}
pub(crate) use prs_dim_relation_impls;

/// A concrete relation: the struct (transient, `base: PrsDimRelationBase` and `fields` of a
/// state type), its RTTI, the interactive traits, `PrsDimRelationSource` with the optional
/// `KindOfDimension` and `IsMovable` overrides, and `alloc` binding a new handle.
macro_rules! prs_dim_relation_class {
    ($type:ident, $native:literal, $fields:ty, $doc:literal
        $(, kind = $kind:expr)? $(, movable = $movable:expr)?) => {
        prs_dim_relation_class!($type, $native, $fields, $doc,
            parent = dyn $crate::prs_dim::PrsDimRelationSource,
            interfaces = [dyn $crate::prs_dim::PrsDimRelationSource, dyn $crate::ais::AisInteractiveObject,
                dyn $crate::select_mgr::SelectMgrSelectableObject, dyn $crate::prs_mgr::PrsMgrPresentableObject]
            $(, kind = $kind)? $(, movable = $movable)?);
    };
    ($type:ident, $native:literal, $fields:ty, $doc:literal,
        parent = $parent:ty, interfaces = [$($interface:ty),+]
        $(, kind = $kind:expr)? $(, movable = $movable:expr)?) => {
        #[doc = $doc]
        pub struct $type {
            transient: martensite_tkernel::standard::StandardTransient,
            pub(super) base: $crate::prs_dim::PrsDimRelationBase,
            /// The class fields.
            pub fields: martensite_tkernel::standard::SharedState<$fields>,
        }
        martensite_tkernel::impl_transient!($type, $native, $parent, transient,
            interfaces: [$($interface),+]);
        super::dimension_impls::prs_dim_relation_impls!($type);
        impl $crate::prs_dim::PrsDimRelationSource for $type {
            fn relation_base(&self) -> &$crate::prs_dim::PrsDimRelationBase {
                &self.base
            }
            $(
                fn kind_of_dimension(&self) -> $crate::prs_dim::PrsDimKindOfDimension {
                    $kind
                }
            )?
            $(
                fn is_movable(&self) -> bool {
                    $movable
                }
            )?
        }
        impl $type {
            /// A bound handle on the default relation state and the given fields.
            fn alloc(fields: $fields) -> martensite_tkernel::standard::Handle<Self> {
                let relation = martensite_tkernel::standard::Handle::new(Self {
                    transient: martensite_tkernel::standard::StandardTransient::default(),
                    base: $crate::prs_dim::PrsDimRelationBase::new(
                        $crate::prs_mgr::PrsMgrTypeOfPresentation3d::AllView,
                    ),
                    fields: martensite_tkernel::standard::SharedState::new(fields),
                });
                $crate::prs_mgr::prs_mgr_presentable_bind(&relation.cast());
                relation
            }
        }
    };
}
pub(crate) use prs_dim_relation_class;
