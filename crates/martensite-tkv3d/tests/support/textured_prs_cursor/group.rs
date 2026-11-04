//! A labelled null group that records each aspect callback and runs the single B edit.
use super::{PrsCursorEdit, PrsCursorTrace, labelled_presentation};
use martensite_tkernel::standard::*;
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::{graphic3d::*, null_driver::NullGroup};
use martensite_tkv3d::prs_mgr::*;
use std::sync::Arc;

pub struct PrsCursorGroup {
    transient: StandardTransient,
    inner: NullGroup,
    role: &'static str,
    trace: Arc<PrsCursorTrace>,
}
martensite_tkernel::impl_transient!(PrsCursorGroup,"PrsCursorGroup",StandardTransient,transient,interfaces:[dyn Graphic3dGroup]);

/// Append the labelled group to the presentation's group nodes, as the native NewGroup.
/// Example: `install_labelled_group(&trace, &prs, "A")`.
pub fn install_labelled_group(
    trace: &Arc<PrsCursorTrace>,
    prs: &Handle<PrsMgrPresentation>,
    role: &'static str,
) {
    let structure: Handle<dyn Graphic3dStructureSource> = prs.cast();
    let group = Handle::new(PrsCursorGroup {
        transient: StandardTransient::default(),
        inner: NullGroup::new(&structure),
        role,
        trace: trace.clone(),
    });
    let cstructure = prs.cstructure();
    cstructure
        .cstructure_base()
        .fields
        .write()
        .groups
        .append(group.cast());
}
impl PrsCursorGroup {
    fn record_visit(&self) {
        let trace = &self.trace;
        let owners = trace.roles.read()[self.role].ref_count();
        let listed = trace
            .object
            .presentable_base()
            .fields
            .read()
            .presentations
            .len();
        trace.rows.write().push(format!(
            "{{\"event\":\"visit\",\"case\":\"{}\",\"operation\":{},\"role\":\"{}\",\"prs_owners\":{owners},\"presentations\":{listed}}}",
            trace.name, trace.operation, self.role
        ));
    }
    fn edit_presentations(&self) {
        let trace = &self.trace;
        if trace.edit == PrsCursorEdit::Append {
            let appended = labelled_presentation(trace, "D", 3);
            let fields = &trace.object.presentable_base().fields;
            fields.write().presentations.append(appended);
            return;
        }
        let victim = if trace.edit == PrsCursorEdit::RemoveEarlier {
            "A"
        } else {
            "C"
        };
        let address = trace.roles.read()[victim].object_address();
        let mut fields = trace.object.presentable_base().fields.write();
        let offset = fields
            .presentations
            .iter()
            .position(|p| p.object_address() == address)
            .expect("victim presentation not listed; expected a live A/C node");
        // The test keeps its own victim owner, so the removed node is not the last one.
        fields.presentations.remove(offset + 1).unwrap();
    }
}
impl Graphic3dGroup for PrsCursorGroup {
    fn group_base(&self) -> &Graphic3dGroupBase {
        self.inner.group_base()
    }
    fn aspects(&self) -> Handle<dyn Graphic3dAspectsSource> {
        self.inner.aspects()
    }
    fn set_group_primitives_aspect(
        &self,
        aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>,
    ) {
        self.inner.set_group_primitives_aspect(aspect);
        self.record_visit();
        if self.trace.take_edit(self.role) {
            self.edit_presentations();
        }
    }
    fn set_primitives_aspect(&self, aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>) {
        self.inner.set_primitives_aspect(aspect);
    }
    fn synchronize_aspects(&self) {
        self.inner.synchronize_aspects();
    }
    fn replace_aspects(&self, replacements: &Graphic3dAspectsReplaceMap) {
        self.inner.replace_aspects(replacements);
    }
    fn set_stencil_test_options(&self, enabled: bool) {
        self.inner.set_stencil_test_options(enabled);
    }
    fn set_flipping_options(&self, enabled: bool, plane: &Axis2) {
        self.inner.set_flipping_options(enabled, plane);
    }
}
