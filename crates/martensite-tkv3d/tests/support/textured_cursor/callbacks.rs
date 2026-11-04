//! Named virtual groups delegate rendering state to real NullGroup; only callbacks mutate inputs.
use super::{observations, state::*};
use martensite_tkernel::standard::*;
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::{graphic3d::*, null_driver::NullGroup};
use std::sync::Arc;

pub struct TexturedCursorGroup {
    transient: StandardTransient,
    inner: NullGroup,
    pub role: &'static str,
    trace: Arc<TexturedCursorTrace>,
}
martensite_tkernel::impl_transient!(TexturedCursorGroup,"TexturedCursorGroup",StandardTransient,transient,interfaces:[dyn Graphic3dGroup]);
/// Install a named callback using the existing null-driver storage; `append_cursor_group(trace, prs, "A")`.
pub fn append_cursor_group(
    trace: &Arc<TexturedCursorTrace>,
    prs: &Handle<dyn Graphic3dStructureSource>,
    role: &'static str,
) -> Handle<TexturedCursorGroup> {
    let group = Handle::new(TexturedCursorGroup {
        transient: StandardTransient::default(),
        inner: NullGroup::new(prs),
        role,
        trace: trace.clone(),
    });
    append_existing_group(prs, &group);
    group
}
/// Append one more sequence node retaining `group`; `append_existing_group(prs, &b)`.
pub fn append_existing_group(
    prs: &Handle<dyn Graphic3dStructureSource>,
    group: &Handle<TexturedCursorGroup>,
) {
    // Same retained group insertion as NullCStructure::new_group; no replacement traversal.
    prs.cstructure()
        .cstructure_base()
        .fields
        .write()
        .groups
        .append(group.cast());
}
impl TexturedCursorGroup {
    fn edit_other_inputs(&self, argument: &dyn HandleReference<dyn Graphic3dAspectsSource>) {
        if !self.trace.take_edit(self.role) {
            return;
        }
        match self.trace.edit {
            TexturedCursorEdit::RemoveEarlier => self.remove_victim(0),
            TexturedCursorEdit::RemoveNext => self.remove_victim(1),
            TexturedCursorEdit::Append => {
                drop(append_cursor_group(&self.trace, &self.structure(), "D"));
            }
            TexturedCursorEdit::CurrentAspect | TexturedCursorEdit::ShadingAspect => {
                self.rebind_aspect(argument)
            }
            TexturedCursorEdit::None => panic!(
                "cursor edit=None role={:?}; expected an enabled B mutation",
                self.role
            ),
        }
    }
    fn remove_victim(&self, index: usize) {
        let victims = self.trace.victims.read();
        let victims = victims
            .as_ref()
            .expect("callback victims=None; expected live external A/C owners");
        victims[index].remove();
    }
    fn rebind_aspect(&self, argument: &dyn HandleReference<dyn Graphic3dAspectsSource>) {
        let trace = &self.trace;
        *trace.old_external.write() = trace.object.base.fields.read().aspect.clone();
        observations::record_binding(trace, self.role, "rebind-before", argument);
        if trace.edit == TexturedCursorEdit::CurrentAspect {
            trace.object.base.fields.write().aspect = trace.alternate.clone();
        } else {
            trace.shading.set_aspect(trace.alternate.clone());
        }
        observations::record_binding(trace, self.role, "rebind-after", argument);
    }
}
impl Graphic3dGroup for TexturedCursorGroup {
    fn group_base(&self) -> &Graphic3dGroupBase {
        // Delegation keeps the canonical parent/removal state authoritative.
        self.inner.group_base()
    }
    fn aspects(&self) -> Handle<dyn Graphic3dAspectsSource> {
        // The caller ends this actual getter owner before observing counts.
        self.inner.aspects()
    }
    fn set_group_primitives_aspect(
        &self,
        aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>,
    ) {
        observations::record_callback(&self.trace, self, "enter", aspect);
        self.inner.set_group_primitives_aspect(aspect);
        observations::record_callback(&self.trace, self, "stored", aspect);
        self.edit_other_inputs(aspect);
        observations::record_callback(&self.trace, self, "leave", aspect);
    }
    fn set_primitives_aspect(&self, aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>) {
        // Unrelated driver operations remain real state delegation, not stubs.
        self.inner.set_primitives_aspect(aspect);
    }
    fn synchronize_aspects(&self) {
        // Preserve the existing null driver's aspect synchronization behavior.
        self.inner.synchronize_aspects();
    }
    fn replace_aspects(&self, replacements: &Graphic3dAspectsReplaceMap) {
        // Preserve ordinary replacement behavior outside the observed callback.
        self.inner.replace_aspects(replacements);
    }
    fn set_stencil_test_options(&self, enabled: bool) {
        // The test does not synthesize independent stencil state.
        self.inner.set_stencil_test_options(enabled);
    }
    fn set_flipping_options(&self, enabled: bool, plane: &Axis2) {
        // The actual null group owns this unrelated rendering state.
        self.inner.set_flipping_options(enabled, plane);
    }
}
