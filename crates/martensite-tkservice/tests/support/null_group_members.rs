//! Share only real null-group delegation across the named lifecycle callback fixtures.
macro_rules! forward_null_group_members {
    ($inner:ident) => {
        fn group_base(&self) -> &martensite_tkservice::graphic3d::Graphic3dGroupBase {
            // All parent and removal state comes from the real null group.
            self.$inner.group_base()
        }
        fn aspects(
            &self,
        ) -> martensite_tkernel::standard::Handle<
            dyn martensite_tkservice::graphic3d::Graphic3dAspectsSource,
        > {
            // This fixture owns no separate presentation state.
            self.$inner.aspects()
        }
        fn set_group_primitives_aspect(
            &self,
            aspect: &dyn martensite_tkernel::standard::HandleReference<
                dyn martensite_tkservice::graphic3d::Graphic3dAspectsSource,
            >,
        ) {
            // Unobserved aspect methods retain the real null driver's behavior.
            self.$inner.set_group_primitives_aspect(aspect);
        }
        fn set_primitives_aspect(
            &self,
            aspect: &dyn martensite_tkernel::standard::HandleReference<
                dyn martensite_tkservice::graphic3d::Graphic3dAspectsSource,
            >,
        ) {
            // The callback is limited to its lifecycle boundary, without replacing other driver operations.
            self.$inner.set_primitives_aspect(aspect);
        }
        fn synchronize_aspects(&self) {
            // Delegate required methods to actual driver state instead of stubbing them.
            self.$inner.synchronize_aspects();
        }
        fn replace_aspects(
            &self,
            replacements: &martensite_tkservice::graphic3d::Graphic3dAspectsReplaceMap,
        ) {
            // Preserve the inherited aspect replacement path.
            self.$inner.replace_aspects(replacements);
        }
        fn set_stencil_test_options(&self, enabled: bool) {
            // The existing group keeps stencil state and semantics.
            self.$inner.set_stencil_test_options(enabled);
        }
        fn set_flipping_options(&self, enabled: bool, plane: &martensite_tkmath::gp::Axis2) {
            // Reuse the actual group implementation for unrelated view state.
            self.$inner.set_flipping_options(enabled, plane);
        }
    };
}
pub(super) use forward_null_group_members;
