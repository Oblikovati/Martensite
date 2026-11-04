//! User elements retained by a group: `GlGroup::AddElement` and the element branch of
//! `GlGroup::Render`, which filters through `GlWorkspace::ShouldRender`.
use super::{Graphic3dDrawGroup, GroupCommand, GroupGeometry};
use crate::render_element::{RenderElementContext, RenderElementGeometry, RenderElementState};
use crate::render_element_pass::{RenderElementPass, check_is_transparent};
use crate::{RenderElement, RenderError};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// Geometry an element emitted, with the aspects current at emission.
pub(super) type EmittedGeometry = (Handle<dyn Graphic3dAspectsSource>, GroupGeometry);

/// Workspace state shared by the elements of one group visit.
pub(super) struct ElementVisit<'a> {
    pub highlight: &'a Handle<dyn Graphic3dPresentationAttributesSource>,
    pub allow_face_culling: bool,
    pub pass: &'a RenderElementPass,
    /// The context state of the group: the frame's, with the structure's matrix stacks.
    pub context: Option<crate::RenderContextState>,
}

impl Graphic3dDrawGroup {
    /// Append a user element after the existing commands (`GlGroup::AddElement`).
    /// Example: `group.add_element(Arc::new(MyElement))`.
    pub fn add_element(&self, element: Arc<dyn RenderElement>) {
        if self.is_deleted() {
            return;
        }
        let raytraced = element.is_raytraced_element();
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Element(element));
        if raytraced {
            self.note_raytraced_element();
        }
        self.update();
    }
    pub(super) fn synchronize_elements(&self) {
        for command in &self.submissions.read().commands {
            if let GroupCommand::Element(element) = command {
                element.synchronize_aspects();
            }
        }
    }
    /// Call one element when the pass filter accepts it; return what it emitted.
    pub(super) fn call_element(
        &self,
        element: &dyn RenderElement,
        aspects: &mut Handle<dyn Graphic3dAspectsSource>,
        visit: &ElementVisit<'_>,
    ) -> Result<Vec<EmittedGeometry>, RenderError> {
        visit.pass.note_element();
        let transparent = || check_is_transparent(aspects, visit.highlight);
        if !visit
            .pass
            .should_render(element.is_fill_draw_mode(), transparent)
        {
            return Ok(vec![]);
        }
        let mut gl_state = visit.context.clone().unwrap_or_default();
        gl_state.apply_aspects(aspects, visit.allow_face_culling);
        let mut context = RenderElementContext {
            gl_state,
            aspects,
            highlight: visit.highlight.clone(),
            state: element_state(visit),
            emitted: vec![],
        };
        element.render(&mut context)?;
        Ok(context.emitted.into_iter().map(group_geometry).collect())
    }
}
fn element_state(visit: &ElementVisit<'_>) -> RenderElementState {
    RenderElementState {
        allow_face_culling: visit.allow_face_culling,
        filter: visit.pass.filter(),
        // GlView enables the depth buffer for every layer; only the trihedron disables it.
        use_z_buffer: true,
        use_depth_write: visit.pass.use_depth_write(),
        skipped_transparent: visit.pass.skipped(),
    }
}
fn group_geometry(
    (aspects, geometry): (Handle<dyn Graphic3dAspectsSource>, RenderElementGeometry),
) -> EmittedGeometry {
    let geometry = match geometry {
        RenderElementGeometry::Primitives(primitive) => GroupGeometry::Primitives(primitive),
        RenderElementGeometry::Text(text, upload, placement) => {
            GroupGeometry::Text(text, upload, placement)
        }
    };
    (aspects, geometry)
}
