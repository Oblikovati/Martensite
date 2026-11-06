//! Hold explicit owners symmetrically; observing a callback must not clone a group/aspect.
use super::super::state_fixture::TexturedStateFixture;
use martensite_tkernel::standard::*;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{
    ais::*, prs_mgr::PrsMgrPresentableObjectMembers, prs3d::Prs3dShadingAspect,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TexturedCursorEdit {
    None,
    RemoveEarlier,
    RemoveNext,
    Append,
    CurrentAspect,
    ShadingAspect,
}
pub struct TexturedCursorTrace {
    pub name: &'static str,
    pub edit: TexturedCursorEdit,
    /// B is appended twice: two sequence nodes retain the same group.
    pub duplicate: bool,
    pub object: HandleObserver<TexturedStateFixture>,
    pub victims: SharedState<Option<[HandleObserver<dyn Graphic3dGroup>; 2]>>,
    pub rows: SharedState<Vec<String>>,
    pub edited: AtomicBool,
    pub old_external: SharedState<Handle<Graphic3dAspectFillArea3d>>,
    pub alternate: Handle<Graphic3dAspectFillArea3d>,
    pub shading: Handle<Prs3dShadingAspect>,
}
impl TexturedCursorTrace {
    /// Retain only the owners present in the Native input; `TexturedCursorTrace::new(name, object.observe())`.
    pub fn new(name: &'static str, object: HandleObserver<TexturedStateFixture>) -> Arc<Self> {
        let edit = selected_cursor_edit(name);
        let (alternate, shading) = cursor_rebind_owners(edit, &object);
        Arc::new(Self {
            name,
            edit,
            duplicate: name.starts_with("duplicate-"),
            object,
            victims: SharedState::new(None),
            rows: SharedState::default(),
            edited: AtomicBool::new(false),
            old_external: SharedState::default(),
            alternate,
            shading,
        })
    }
    /// Select only the one B mutation; `trace.take_edit("B")`.
    pub fn take_edit(&self, role: &str) -> bool {
        let enabled = role == "B" && self.edit != TexturedCursorEdit::None;
        enabled && !self.edited.swap(true, Ordering::Relaxed)
    }
    /// Preserve record order outside geometry locks; `trace.record(row)`.
    pub fn record(&self, row: String) {
        // Records contain scalars/strings only, never intrusive handle owners.
        self.rows.write().push(row);
    }
}
fn selected_cursor_edit(name: &str) -> TexturedCursorEdit {
    match name {
        "unchanged" | "duplicate-active-unchanged" => TexturedCursorEdit::None,
        "remove-earlier" | "duplicate-active-remove-earlier" => TexturedCursorEdit::RemoveEarlier,
        "remove-next" => TexturedCursorEdit::RemoveNext,
        "append" => TexturedCursorEdit::Append,
        "rebind-current-aspect" => TexturedCursorEdit::CurrentAspect,
        "rebind-shading-aspect" => TexturedCursorEdit::ShadingAspect,
        _ => panic!("cursor case={name:?}; expected one of eight retained Native cases"),
    }
}
fn cursor_rebind_owners(
    edit: TexturedCursorEdit,
    object: &TexturedStateFixture,
) -> (
    Handle<Graphic3dAspectFillArea3d>,
    Handle<Prs3dShadingAspect>,
) {
    if matches!(
        edit,
        TexturedCursorEdit::CurrentAspect | TexturedCursorEdit::ShadingAspect
    ) {
        return (
            Handle::new(Graphic3dAspectFillArea3d::default()),
            object.attributes().shading_aspect(),
        );
    }
    (Handle::default(), Handle::default())
}
/// Lend the genuine protected source slot; `HandleFieldRef::new(&object.base.fields, current_cursor_aspect)`.
pub fn current_cursor_aspect(
    fields: &AisTexturedShapeFields,
) -> &Handle<Graphic3dAspectFillArea3d> {
    // No clone/cast: callback slot identity must describe the actual retained field.
    &fields.aspect
}
