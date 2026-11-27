//! User render elements: custom elements appended with
//! `GlGroup::AddElement` and drawn while `GlWorkspace` carries the current state.
//! A Vulkan element does not issue API calls; it reads the state an element sees natively and
//! emits geometry that the group lowers exactly like its own primitive arrays and texts.
use crate::{RenderError, primitive::PrimitiveSubmission};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// Native `GlRenderFilter` bits seen by an element; `RenderElementFilter::OPAQUE_ONLY`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderElementFilter(u32);
impl RenderElementFilter {
    /// No filtering (`GlRenderFilter_Empty`).
    pub const EMPTY: Self = Self(0x000);
    /// Opaque elements and every non-filling element (`GlRenderFilter_OpaqueOnly`).
    pub const OPAQUE_ONLY: Self = Self(0x001);
    /// Semitransparent filling elements only (`GlRenderFilter_TransparentOnly`).
    pub const TRANSPARENT_ONLY: Self = Self(0x002);
    /// Elements outside ray tracing only (`GlRenderFilter_NonRaytraceableOnly`).
    pub const NON_RAYTRACEABLE_ONLY: Self = Self(0x004);
    /// Filling elements only (`GlRenderFilter_FillModeOnly`).
    pub const FILL_MODE_ONLY: Self = Self(0x008);
    /// Skip transform-persistent groups (`GlRenderFilter_SkipTrsfPersistence`).
    pub const SKIP_TRSF_PERSISTENCE: Self = Self(0x010);
    /// The native integer value; `RenderElementFilter::TRANSPARENT_ONLY.bits() == 2`.
    pub fn bits(self) -> u32 {
        self.0
    }
    /// Test one or more bits; `filter.contains(RenderElementFilter::OPAQUE_ONLY)`.
    pub fn contains(self, bits: Self) -> bool {
        self.0 & bits.0 == bits.0
    }
}

/// A user element retained by a group; `group.add_element(Arc::new(MyElement))`.
///
/// ```ignore
/// struct Marker;
/// impl RenderElement for Marker {
///     fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
///         context.add_primitive_array(kind, &indices, &attributes, &bounds);
///         Ok(())
///     }
/// }
/// ```
pub trait RenderElement: Send + Sync {
    /// Emit this element's geometry for the current state (`GlElement::Render`).
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError>;
    /// Whether the element fills surfaces, so transparency filters apply (`IsFillDrawMode`).
    fn is_fill_draw_mode(&self) -> bool {
        false
    }
    /// Estimated retained bytes (`EstimatedDataSize`); `element.estimated_data_size()`.
    fn estimated_data_size(&self) -> usize {
        0
    }
    /// Re-read aspects after the group synchronized them (`SynchronizeAspects`).
    fn synchronize_aspects(&self) {}
    /// Add memory counters (`UpdateMemStats`); frame statistics call it for every element of
    /// a counted structure, hidden or not. By default the estimated size counts as geometry.
    fn update_mem_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp) {
        stats[Graphic3dFrameStatsCounter::EstimatedBytesGeom] += self.estimated_data_size();
    }
    /// Add draw counters (`UpdateDrawStats`) for an element of a visible structure, after its
    /// memory counters; `detailed` asks for primitive counts. Natively a no-op by default.
    fn update_draw_stats(&self, _stats: &mut Graphic3dFrameStatsDataTmp, _detailed: bool) {}
    /// Whether the element holds ray-traced geometry (`GlRaytrace::IsRaytracedElement`):
    /// only primitive arrays drawn as triangles do; a group receiving one becomes ray-traceable.
    fn is_raytraced_element(&self) -> bool {
        false
    }
    /// Write the element's `DumpJson` (`GlElement::DumpJson`): by default the class
    /// `GlElement` and the element address. Example: `element.dump_json(&mut stream, -1)?`.
    fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        _depth: i32,
    ) -> std::io::Result<()> {
        crate::driver_dump::element_base(stream, crate::driver_dump::address_of(self))
    }
}

/// Geometry an element emits, kept with the aspects current at emission.
#[derive(Clone)]
pub(crate) enum RenderElementGeometry {
    Primitives(PrimitiveSubmission),
    Text(
        Handle<Graphic3dText>,
        crate::element_upload::ElementUploadSlot,
        TextPlacement,
    ),
}

/// Where a text's glyphs are placed: projected from the 3D anchor, or in the 2D frame of
/// `GlText::Set2D` (the text model replaces the world-view, no depth test).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextPlacement {
    Scene,
    Pixel,
}

/// Frame state an element sees; built by the group for each call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RenderElementState {
    pub allow_face_culling: bool,
    pub filter: RenderElementFilter,
    pub use_z_buffer: bool,
    pub use_depth_write: bool,
    pub skipped_transparent: usize,
}

/// The workspace view of one element call; `context.interior_color()`.
pub struct RenderElementContext<'a> {
    pub(crate) gl_state: crate::RenderContextState,
    pub(crate) aspects: &'a mut Handle<dyn Graphic3dAspectsSource>,
    pub(crate) highlight: Handle<dyn Graphic3dPresentationAttributesSource>,
    pub(crate) state: RenderElementState,
    pub(crate) emitted: Vec<(Handle<dyn Graphic3dAspectsSource>, RenderElementGeometry)>,
}
