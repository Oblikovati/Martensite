//! Owned identity/revisions of the target selected for an actual redraw attempt.
use super::*;

#[derive(Clone)]
pub(crate) struct RedrawTargetStamp {
    pub(crate) lighting: crate::driver_lighting_capability::LightingSelection,
    fbo: Handle<StandardTransient>,
    binding: Arc<()>,
    viewport: Option<([i32; 4], Arc<()>)>,
    window: Handle<dyn AspectWindow>,
}

#[derive(Clone)]
pub(crate) struct RedrawPublicationTarget {
    pub started: RedrawTargetStamp,
    pub published: RedrawTargetStamp,
}

impl RedrawTargetStamp {
    pub fn observe(view: &Graphic3dRenderView) -> Self {
        let (fbo, binding) = view.retained.fbo_binding_stamp();
        let viewport = fbo
            .cast::<RenderFrameBuffer>()
            .get()
            .map(|fbo| fbo.viewport_stamp());
        Self {
            lighting: view.lighting_selection.read().clone(),
            fbo,
            binding,
            viewport,
            window: view.window(),
        }
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.fbo.object_address() == other.fbo.object_address()
            && self.window.object_address() == other.window.object_address()
            && Arc::ptr_eq(&self.binding, &other.binding)
            && same_viewport(&self.viewport, &other.viewport)
    }
}

fn same_viewport(
    first: &Option<([i32; 4], Arc<()>)>,
    second: &Option<([i32; 4], Arc<()>)>,
) -> bool {
    match (first, second) {
        (Some((dimensions, revision)), Some((other, epoch))) => {
            dimensions == other && Arc::ptr_eq(revision, epoch)
        }
        (None, None) => true,
        _ => false,
    }
}
