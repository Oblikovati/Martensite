//! Labelled presentations whose B group edits the object's live presentation sequence.
//!
//! Mirrors `the recorded textured_presentation_cursor capture`: the test owns A, B and
//! C once each besides their sequence nodes, D (appended by B) has only its node owner, and
//! the trace keeps non-owning observers so a recorded count is never raised by the record.
mod group;
use super::state_fixture::TexturedStateFixture;
use group::install_labelled_group;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::*;
use martensite_tkservice::{graphic3d::*, null_driver::NullGraphicDriver};
use martensite_tkv3d::{ais::*, prs_mgr::*, prs3d::Prs3dDrawer, v3d::V3dViewer};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrsCursorEdit {
    None,
    RemoveEarlier,
    RemoveNext,
    Append,
}
pub struct PrsCursorTrace {
    pub name: &'static str,
    pub operation: i32,
    pub edit: PrsCursorEdit,
    pub object: HandleObserver<TexturedStateFixture>,
    pub manager: Handle<PrsMgrPresentationManager>,
    pub roles: SharedState<BTreeMap<&'static str, HandleObserver<PrsMgrPresentation>>>,
    pub rows: SharedState<Vec<String>>,
    pub edit_pending: AtomicBool,
}
impl PrsCursorTrace {
    /// Take the single B edit; `if trace.take_edit("B") { ... }`.
    pub fn take_edit(&self, role: &str) -> bool {
        role == "B"
            && self.edit != PrsCursorEdit::None
            && self.edit_pending.swap(false, Ordering::SeqCst)
    }
    fn record(&self, row: String) {
        self.rows.write().push(row);
    }
}

/// Run one recorded case through the appearance overrides; `replay_prs_cursor_case("append", 0)`.
pub fn replay_prs_cursor_case(name: &'static str, operation: i32) -> Vec<String> {
    let object = prepared_object(operation);
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let trace = Arc::new(PrsCursorTrace {
        name,
        operation,
        edit: selected_edit(name),
        object: object.observe(),
        manager: PrsMgrPresentationManager::new(&viewer.structure_manager()),
        roles: SharedState::new(BTreeMap::new()),
        rows: SharedState::new(Vec::new()),
        edit_pending: AtomicBool::new(true),
    });
    let mode_a = if name == "earlier-mode0" { 0 } else { 3 };
    let a = labelled_presentation(&trace, "A", mode_a);
    let b = labelled_presentation(&trace, "B", 3);
    let c = labelled_presentation(&trace, "C", 3);
    append_nodes(&object, &[&a, &b, &c], name.starts_with("duplicate-active"));
    change_appearance(&object, operation);
    finish_case(&trace, &object, [("A", &a), ("B", &b), ("C", &c)]);
    let rows = trace.rows.read().clone();
    // Disarm the non-owning observers before the owned graph is dropped.
    object.presentable_base().fields.write().presentations = Default::default();
    trace.roles.write().clear();
    rows
}
fn selected_edit(name: &str) -> PrsCursorEdit {
    match name {
        "remove-earlier" | "duplicate-active-remove-earlier" => PrsCursorEdit::RemoveEarlier,
        "remove-next" => PrsCursorEdit::RemoveNext,
        "append" => PrsCursorEdit::Append,
        _ => PrsCursorEdit::None,
    }
}
fn prepared_object(operation: i32) -> Handle<TexturedStateFixture> {
    let object = TexturedStateFixture::new();
    object.attributes().setup_own_defaults().unwrap();
    object.set_texture_map_off();
    let linked = Handle::new(Prs3dDrawer::new());
    linked.setup_own_defaults().unwrap();
    object.attributes().link_to(&linked);
    if operation == 2 {
        let gold = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
        ais_shape_set_material(&*object, &gold).unwrap();
    }
    object
}
/// Create a presentation of `mode` with one labelled group; `labelled_presentation(&trace, "D", 3)`.
pub fn labelled_presentation(
    trace: &Arc<PrsCursorTrace>,
    role: &'static str,
    mode: i32,
) -> Handle<PrsMgrPresentation> {
    let object: Handle<dyn PrsMgrPresentableObject> = (*trace.object).clone().cast();
    let prs = PrsMgrPresentation::new(&trace.manager, &object, mode).unwrap();
    drop(object);
    trace.roles.write().insert(role, prs.observe());
    install_labelled_group(trace, &prs, role);
    prs
}
fn append_nodes(
    object: &TexturedStateFixture,
    prs: &[&Handle<PrsMgrPresentation>; 3],
    repeat: bool,
) {
    let mut fields = object.presentable_base().fields.write();
    fields.presentations.append(prs[0].clone());
    fields.presentations.append(prs[1].clone());
    if repeat {
        // The same B handle in a second node, as the native Append of the same handle.
        fields.presentations.append(prs[1].clone());
    }
    fields.presentations.append(prs[2].clone());
}
fn change_appearance(object: &TexturedStateFixture, operation: i32) {
    let gold = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
    match operation {
        0 => ais_textured_shape_set_color(object, Color::named(NamedColor::RED)).unwrap(),
        1 => ais_textured_shape_set_material(object, &gold).unwrap(),
        _ => ais_textured_shape_unset_material(object).unwrap(),
    }
}
fn finish_case(
    trace: &PrsCursorTrace,
    object: &TexturedStateFixture,
    roles: [(&str, &Handle<PrsMgrPresentation>); 3],
) {
    for (role, prs) in roles {
        let current: Handle<dyn Graphic3dAspectsSource> = object.base.fields.read().aspect.cast();
        // End the temporary aspects owner before the count is read.
        let has_current = prs.groups()[0].aspects() == current;
        trace.record(format!(
            "{{\"event\":\"final\",\"case\":\"{}\",\"operation\":{},\"role\":\"{role}\",\"prs_owners\":{},\"has_current_aspect\":{has_current}}}",
            trace.name, trace.operation, prs.ref_count()
        ));
    }
    trace.record(format!(
        "{{\"event\":\"summary\",\"case\":\"{}\",\"operation\":{},\"presentations\":{},\"edited\":{}}}",
        trace.name,
        trace.operation,
        object.presentable_base().fields.read().presentations.len(),
        !trace.edit_pending.load(Ordering::SeqCst)
    ));
}
