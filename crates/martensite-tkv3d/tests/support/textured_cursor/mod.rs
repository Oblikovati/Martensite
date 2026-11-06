//! Six genuine group callbacks use unchanged production state and independently retained records.
mod callbacks;
mod observations;
mod state;
use super::state_fixture::TexturedStateFixture;
use callbacks::TexturedCursorGroup;
use martensite_tkernel::standard::*;
use martensite_tkservice::{graphic3d::*, null_driver::NullGraphicDriver};
use martensite_tkv3d::{ais::*, prs_mgr::PrsMgrPresentableObjectMembers, v3d::V3dViewer};
use state::TexturedCursorTrace;
use std::sync::Arc;

/// Execute one exact input through the production helper; `replay_cursor_case("remove-earlier")`.
pub fn replay_cursor_case(name: &'static str) -> Vec<String> {
    let object = TexturedStateFixture::new();
    object.attributes().setup_own_defaults().unwrap();
    object.set_texture_map_off();
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let prs = Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap();
    let trace = TexturedCursorTrace::new(name, object.observe());
    let groups = install_cursor_groups(&trace, &prs);
    ais_textured_shape_update_attributes(&*object, &prs.cast()).unwrap();
    finish_cursor_case(&trace, &groups, &prs);
    trace.rows.read().clone()
}
fn install_cursor_groups(
    trace: &Arc<TexturedCursorTrace>,
    prs: &Handle<Graphic3dStructure>,
) -> [Handle<TexturedCursorGroup>; 3] {
    let source: Handle<dyn Graphic3dStructureSource> = prs.cast();
    let [a, b] = ["A", "B"].map(|role| callbacks::append_cursor_group(trace, &source, role));
    if trace.duplicate {
        // The native derived structure appends the same retained B handle a second time.
        callbacks::append_existing_group(&source, &b);
        observations::record_duplicate_nodes(trace, prs);
    }
    let groups = [a, b, callbacks::append_cursor_group(trace, &source, "C")];
    *trace.victims.write() = Some([
        groups[0].observe().observe_as(),
        groups[2].observe().observe_as(),
    ]);
    groups
}
fn finish_cursor_case(
    trace: &TexturedCursorTrace,
    groups: &[Handle<TexturedCursorGroup>; 3],
    prs: &Graphic3dStructure,
) {
    for group in groups {
        observations::record_final_group(trace, group);
    }
    observations::record_summary(trace, prs);
    // Disarm fixture-only nonowning callback inputs before owned graph teardown.
    *trace.victims.write() = None;
}
