//! Layer pass state for render elements: the layer list draws each
//! layer with the opaque filter, counts skipped transparent filling elements, then draws the
//! layers with skipped elements again with the transparent filter. Built-in geometry is
//! lowered once (the Vulkan frame separates blended draws itself); the passes decide which
//! user elements are called and what filter and counter they observe.
use crate::render_element::RenderElementFilter;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RenderElementPhase {
    /// Extraction outside a view frame: every element is called without filtering.
    Unfiltered,
    Opaque,
    Transparent,
}

/// One layer pass; `RenderElementPass::opaque(depth_write)`.
#[derive(Debug)]
pub(crate) struct RenderElementPass {
    phase: RenderElementPhase,
    use_depth_write: bool,
    skipped: AtomicUsize,
    elements: AtomicBool,
    /// The view's environment texture when the layer uses it (`UseEnvironmentTexture`).
    environment: Option<Handle<Graphic3dTextureEnv>>,
    /// The layer's local origin (`Graphic3dZLayerSettings::Origin`), when it has one.
    origin: Option<crate::layer_origin::LayerOrigin>,
    /// Skip groups with transform persistence (`GlRenderFilter_SkipTrsfPersistence`).
    skip_trsf_persistence: bool,
    /// Skip ray-traced arrays (`GlRenderFilter_NonRaytraceableOnly`).
    skip_raytraced: bool,
    /// The frame's context state (device, viewport, resolution, layer polygon offset).
    context: std::sync::Arc<crate::RenderContextState>,
}

impl RenderElementPass {
    /// Direct group extraction without a view frame; `RenderElementPass::unfiltered()`.
    pub fn unfiltered() -> Self {
        Self::new(RenderElementPhase::Unfiltered, true, 0)
    }
    /// The first pass of a layer; `RenderElementPass::opaque(settings.to_enable_depth_write())`.
    pub fn opaque(use_depth_write: bool) -> Self {
        Self::new(RenderElementPhase::Opaque, use_depth_write, 0)
    }
    /// The transparent pass keeps the opaque pass counter, as natively (it is not reset), and
    /// renders with the global depth mask off, so elements never see depth writes.
    pub fn transparent(&self) -> Self {
        Self::new(RenderElementPhase::Transparent, false, self.skipped())
            .with_environment(self.environment.clone())
            .with_layer_origin(self.origin)
            .with_skip_trsf_persistence(self.skip_trsf_persistence)
            .with_skip_raytraced(self.skip_raytraced)
            .with_context_state(self.context.clone())
    }
    /// The raster pass of a ray-traced layer draws only what ray tracing does not
    /// (GlView::render); `RenderElementPass::opaque(true).with_skip_raytraced(true)`.
    pub fn with_skip_raytraced(mut self, skip: bool) -> Self {
        self.skip_raytraced = skip;
        self
    }
    /// Whether ray-traced arrays are skipped; `pass.skips_raytraced()`.
    pub fn skips_raytraced(&self) -> bool {
        self.skip_raytraced
    }
    /// Give elements the frame's context state; `pass.with_context_state(Arc::new(state))`.
    pub fn with_context_state(
        mut self,
        context: std::sync::Arc<crate::RenderContextState>,
    ) -> Self {
        self.context = context;
        self
    }
    /// The frame's context state; `pass.context_state().viewport()`.
    pub fn context_state(&self) -> &crate::RenderContextState {
        &self.context
    }
    /// The shadow-map pass skips transform-persistent groups (GlView::renderShadowMap);
    /// `RenderElementPass::opaque(true).with_skip_trsf_persistence(true)`.
    pub fn with_skip_trsf_persistence(mut self, skip: bool) -> Self {
        self.skip_trsf_persistence = skip;
        self
    }
    /// Whether transform-persistent groups are skipped; `pass.skips_trsf_persistence()`.
    pub fn skips_trsf_persistence(&self) -> bool {
        self.skip_trsf_persistence
    }
    /// Render the layer with the view's environment texture (`GlView::SetTextureEnv`) or
    /// without one; `RenderElementPass::opaque(true).with_environment(Some(environment))`.
    pub fn with_environment(mut self, environment: Option<Handle<Graphic3dTextureEnv>>) -> Self {
        self.environment = environment.filter(|texture| !texture.is_null());
        self
    }
    /// Render the layer around its local origin; `pass.with_layer_origin(LayerOrigin::of(&settings))`.
    pub fn with_layer_origin(mut self, origin: Option<crate::layer_origin::LayerOrigin>) -> Self {
        self.origin = origin;
        self
    }
    /// The layer's local origin; `pass.layer_origin()`.
    pub fn layer_origin(&self) -> Option<crate::layer_origin::LayerOrigin> {
        self.origin
    }
    /// The environment texture of the pass; `pass.environment()`.
    pub fn environment(&self) -> Option<&Handle<Graphic3dTextureEnv>> {
        self.environment.as_ref()
    }
    fn new(phase: RenderElementPhase, use_depth_write: bool, skipped: usize) -> Self {
        Self {
            phase,
            use_depth_write,
            skipped: AtomicUsize::new(skipped),
            elements: AtomicBool::new(false),
            environment: None,
            origin: None,
            skip_trsf_persistence: false,
            skip_raytraced: false,
            context: std::sync::Arc::default(),
        }
    }
    /// The filter elements observe; `pass.filter() == RenderElementFilter::OPAQUE_ONLY`.
    pub fn filter(&self) -> RenderElementFilter {
        match self.phase {
            RenderElementPhase::Unfiltered => RenderElementFilter::EMPTY,
            RenderElementPhase::Opaque => RenderElementFilter::OPAQUE_ONLY,
            RenderElementPhase::Transparent => RenderElementFilter::TRANSPARENT_ONLY,
        }
    }
    pub fn use_depth_write(&self) -> bool {
        self.use_depth_write
    }
    /// Transparent filling elements skipped by the opaque pass so far.
    pub fn skipped(&self) -> usize {
        self.skipped.load(Ordering::Relaxed)
    }
    /// Record that a user element was visited; `pass.note_element()`.
    pub fn note_element(&self) {
        self.elements.store(true, Ordering::Relaxed);
    }
    /// A transparent revisit can only add draws when user elements exist.
    pub fn needs_transparent_pass(&self) -> bool {
        self.skipped() > 0 && self.elements.load(Ordering::Relaxed)
    }
    /// Built-in group geometry is lowered once, outside the transparent pass.
    pub fn lowers_group_geometry(&self) -> bool {
        self.phase != RenderElementPhase::Transparent
    }
    /// Count a built-in filling array the native opaque pass would skip; `pass.note_builtin(fill, transparent)`.
    pub fn note_builtin(&self, fill: bool, transparent: impl FnOnce() -> bool) {
        if self.phase == RenderElementPhase::Opaque && fill && transparent() {
            self.skipped.fetch_add(1, Ordering::Relaxed);
        }
    }
    /// Native `GlWorkspace::ShouldRender` for a user element; counts opaque-pass skips.
    pub fn should_render(&self, fill: bool, transparent: impl FnOnce() -> bool) -> bool {
        match self.phase {
            RenderElementPhase::Unfiltered => true,
            RenderElementPhase::Opaque => self.opaque_accepts(fill && transparent()),
            RenderElementPhase::Transparent => fill && transparent(),
        }
    }
    fn opaque_accepts(&self, transparent_fill: bool) -> bool {
        if transparent_fill {
            self.skipped.fetch_add(1, Ordering::Relaxed);
        }
        !transparent_fill
    }
}

/// Native `GlPrimitiveArray` fill types; an array without attributes draws nothing.
pub(crate) fn is_fill_primitive(kind: Graphic3dTypeOfPrimitiveArray, has_attributes: bool) -> bool {
    use Graphic3dTypeOfPrimitiveArray as Kind;
    has_attributes
        && matches!(
            kind,
            Kind::Triangles
                | Kind::Trianglestrips
                | Kind::Trianglefans
                | Kind::TrianglesAdjacency
                | Kind::TriangleStripAdjacency
                | Kind::Quadrangles
                | Kind::Quadranglestrips
                | Kind::Polygons
        )
}

/// Native `GlContext::CheckIsTransparent`: the highlight fill aspect replaces the
/// current one; a plain highlight color supplies both alphas; BlendAuto tests the alphas.
pub(crate) fn check_is_transparent(
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    highlight: &Handle<dyn Graphic3dPresentationAttributesSource>,
) -> bool {
    check_is_transparent_alphas(aspects, highlight).0
}
/// The test with the front and back alphas it read (native out parameters, 1 without aspects).
pub(crate) fn check_is_transparent_alphas(
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    highlight: &Handle<dyn Graphic3dPresentationAttributesSource>,
) -> (bool, [f32; 2]) {
    let fill = highlight.get().map(|style| style.basic_fill_area_aspect());
    let source: Handle<dyn Graphic3dAspectsSource> = match &fill {
        Some(aspect) if !aspect.is_null() => aspect.clone().cast(),
        _ => aspects.clone(),
    };
    let Some(aspect) = source.get() else {
        return (false, [1.; 2]);
    };
    let alphas = match (highlight.get(), &fill) {
        (Some(style), Some(fill)) if fill.is_null() => [style.color_rgba().alpha(); 2],
        _ => material_alphas(aspect),
    };
    let blend = match aspect.alpha_mode() {
        Graphic3dAlphaMode::BlendAuto => alphas.iter().any(|alpha| *alpha < 1.),
        mode => mode == Graphic3dAlphaMode::Blend,
    };
    (blend, alphas)
}
fn material_alphas(aspect: &dyn Graphic3dAspectsSource) -> [f32; 2] {
    let front = aspect.front_material().alpha();
    let back = match aspect.distinguish() {
        true => aspect.back_material().alpha(),
        false => front,
    };
    [front, back]
}
#[cfg(test)]
#[path = "render_element_pass_tests.rs"]
mod tests;
