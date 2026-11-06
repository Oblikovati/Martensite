//! Scalar observations retain actual reference/count semantics without adding target owners.
use super::{callbacks::TexturedCursorGroup, state::*};
use martensite_tkernel::standard::*;
use martensite_tkservice::graphic3d::*;
use std::sync::atomic::Ordering;

/// Observe the actual argument and source slot; `record_callback(trace, group, "enter", argument)`.
pub fn record_callback(
    trace: &TexturedCursorTrace,
    group: &TexturedCursorGroup,
    phase: &str,
    argument: &dyn HandleReference<dyn Graphic3dAspectsSource>,
) {
    trace.record(callback_row(trace, group, phase, argument));
    record_binding(trace, group.role, phase, argument);
}
fn callback_row(
    trace: &TexturedCursorTrace,
    group: &TexturedCursorGroup,
    phase: &str,
    argument: &dyn HandleReference<dyn Graphic3dAspectsSource>,
) -> String {
    let source = HandleFieldRef::new(&trace.object.base.fields, current_cursor_aspect);
    let groups = group.transient_base().ref_count();
    let arguments = argument.observe().ref_count();
    let sources = source.observe().ref_count();
    let same = argument.object_address() == source.object_address();
    let slot = argument.reference_address() == source.reference_address();
    format!(
        "{{\"event\":\"callback\",\"case\":\"{}\",\"role\":\"{}\",\"phase\":\"{phase}\",\"group_owners\":{groups},\"argument_aspect_owners\":{arguments},\"source_aspect_owners\":{sources},\"argument_is_source\":{same},\"argument_slot_is_source_field\":{slot}}}",
        trace.name, group.role
    )
}
/// Observe both actual field targets across B's callback; `record_binding(trace, "B", "rebind-after", argument)`.
pub fn record_binding(
    trace: &TexturedCursorTrace,
    role: &str,
    phase: &str,
    argument: &dyn HandleReference<dyn Graphic3dAspectsSource>,
) {
    if trace.alternate.is_null() {
        return;
    }
    trace.record(binding_row(trace, role, phase, argument));
}
fn binding_row(
    trace: &TexturedCursorTrace,
    role: &str,
    phase: &str,
    argument: &dyn HandleReference<dyn Graphic3dAspectsSource>,
) -> String {
    let source = HandleFieldRef::new(&trace.object.base.fields, current_cursor_aspect);
    let alternate = trace.alternate.object_address();
    let shading = trace.shading.aspect().object_address() == alternate;
    let old = trace.old_external.read();
    let arg_alt = argument.object_address() == alternate;
    let current_alt = source.object_address() == alternate;
    let old_current = old.object_address() == source.object_address();
    let alt_count = trace.alternate.ref_count();
    let old_count = old.ref_count();
    format!(
        "{{\"event\":\"aspect_binding\",\"case\":\"{}\",\"role\":\"{role}\",\"phase\":\"{phase}\",\"argument_is_alt\":{arg_alt},\"current_is_alt\":{current_alt},\"shading_is_alt\":{shading},\"old_external_is_current\":{old_current},\"alternate_owners\":{alt_count},\"old_external_owners\":{old_count}}}",
        trace.name
    )
}
/// End the temporary Aspects owner before counts; `record_final_group(trace, group)`.
pub fn record_final_group(trace: &TexturedCursorTrace, group: &TexturedCursorGroup) {
    let current = trace.object.base.fields.read().aspect.object_address();
    let updated = group.aspects().object_address() == current;
    trace.record(format!(
        concat!(
            "{{\"event\":\"final_group\",\"case\":\"{}\",\"role\":\"{}\",",
            "\"deleted\":{},\"group_owners\":{},\"has_current_aspect\":{}}}"
        ),
        trace.name,
        group.role,
        group.is_deleted(),
        group.transient_base().ref_count(),
        updated
    ));
}
/// Compare the two B nodes: equal payloads at distinct nodes; `record_duplicate_nodes(trace, prs)`.
pub fn record_duplicate_nodes(trace: &TexturedCursorTrace, prs: &Graphic3dStructure) {
    let structure = prs.cstructure();
    let fields = structure.cstructure_base().fields.read();
    let mut cursor = fields.groups.cursor();
    cursor.next(&fields.groups);
    let first = cursor;
    cursor.next(&fields.groups);
    let payload = |at: martensite_tkernel::ncollection::SequenceCursor| {
        at.value(&fields.groups).map(|group| group.object_address())
    };
    let same = payload(first).is_some() && payload(first) == payload(cursor);
    let distinct = first != cursor;
    trace.record(format!(
        concat!(
            "{{\"event\":\"duplicate_nodes\",\"case\":\"{}\",",
            "\"same_payload\":{},\"distinct_nodes\":{}}}"
        ),
        trace.name, same, distinct
    ));
}
/// Count actual retained groups after all callbacks; `record_summary(trace, prs)`.
pub fn record_summary(trace: &TexturedCursorTrace, prs: &Graphic3dStructure) {
    let structure = prs.cstructure();
    let remaining = structure.cstructure_base().fields.read().groups.len();
    let count = trace.object.base.fields.read().aspect.ref_count();
    trace.record(format!(
        concat!(
            "{{\"event\":\"summary\",\"case\":\"{}\",\"remaining_groups\":{},",
            "\"source_aspect_owners\":{},\"edited\":{}}}"
        ),
        trace.name,
        remaining,
        count,
        trace.edited.load(Ordering::Relaxed)
    ));
}
