//! A primitive-array render element.
//! It keeps the Graphic3d buffers, the unique id the driver hands out and the native buffer
//! residency (built on the first submission, kept across `invalidate`, dropped by `release`).
//! Vulkan reads the Graphic3d buffers on every frame, so the client buffers stay attached after
//! the first draw (native `GlCaps::keepArrayData`).
use crate::element_stats::{GroupElementStats, bound_count};
use crate::element_upload::ElementUploadSlot;
use crate::primitive::PrimitiveSubmission;
use crate::{Graphic3dRenderDriver, RenderElement, RenderElementContext, RenderError};
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkservice::graphic3d::{
    Graphic3dBoundBuffer, Graphic3dBufferMembers, Graphic3dBufferSource,
    Graphic3dFrameStatsDataTmp, Graphic3dIndexBufferSource, Graphic3dTypeOfPrimitiveArray as Kind,
};

/// The Graphic3d buffers of one array and its declared type.
#[derive(Clone)]
struct PrimitiveArrayBuffers {
    kind: Kind,
    indices: Handle<dyn Graphic3dIndexBufferSource>,
    attributes: Handle<dyn Graphic3dBufferSource>,
    bounds: Handle<Graphic3dBoundBuffer>,
}

impl Default for PrimitiveArrayBuffers {
    fn default() -> Self {
        Self {
            kind: Kind::Undefined,
            indices: Handle::default(),
            attributes: Handle::default(),
            bounds: Handle::default(),
        }
    }
}

/// One primitive array drawn as a group element; `group.add_element(Arc::new(array))`.
///
/// ```ignore
/// let array = RenderPrimitiveArray::new(&driver, Kind::Triangles, &indices, &attributes, &bounds);
/// assert_eq!(array.draw_mode(), Some(Kind::Triangles));
/// group.add_element(std::sync::Arc::new(array));
/// ```
pub struct RenderPrimitiveArray {
    uid: usize,
    buffers: SharedState<PrimitiveArrayBuffers>,
    upload: ElementUploadSlot,
}

impl RenderPrimitiveArray {
    /// An array without buffers that draws nothing (`GlPrimitiveArray(theDriver)`).
    /// Example: `RenderPrimitiveArray::empty(&driver).draw_mode == None`.
    pub fn empty(driver: &Graphic3dRenderDriver) -> Self {
        Self {
            uid: driver.next_primitive_array_uid(),
            buffers: SharedState::default(),
            upload: ElementUploadSlot::default(),
        }
    }
    /// An array over Graphic3d buffers; an index buffer without elements is dropped, as the
    /// native constructor does. Example: `RenderPrimitiveArray::new(&driver, kind, &i, &a, &b)`.
    pub fn new(
        driver: &Graphic3dRenderDriver,
        kind: Kind,
        indices: &Handle<dyn Graphic3dIndexBufferSource>,
        attributes: &Handle<dyn Graphic3dBufferSource>,
        bounds: &Handle<Graphic3dBoundBuffer>,
    ) -> Self {
        let array = Self::empty(driver);
        let dummy = indices.get().is_some_and(|buffer| buffer.nb_elements() < 1);
        *array.buffers.write() = PrimitiveArrayBuffers {
            kind,
            indices: if dummy {
                Handle::default()
            } else {
                indices.clone()
            },
            attributes: attributes.clone(),
            bounds: bounds.clone(),
        };
        array
    }
    /// The unique id the driver assigned (`GetUID`); `array.uid()`.
    pub fn uid(&self) -> usize {
        self.uid
    }
    /// The type drawn, None when nothing is drawn: no attributes or an undefined type
    /// (`DrawMode`, `DRAW_MODE_NONE`). Example: `array.draw_mode()`.
    pub fn draw_mode(&self) -> Option<Kind> {
        let buffers = self.buffers.read();
        (!buffers.attributes.is_null() && buffers.kind != Kind::Undefined).then_some(buffers.kind)
    }
    /// The index buffer, null when drawing all vertices (`Indices`); `array.indices()`.
    pub fn indices(&self) -> Handle<dyn Graphic3dIndexBufferSource> {
        self.buffers.read().indices.clone()
    }
    /// The vertex attributes (`Attributes`); `array.attributes()`.
    pub fn attributes(&self) -> Handle<dyn Graphic3dBufferSource> {
        self.buffers.read().attributes.clone()
    }
    /// The bounds of strips and polylines (`Bounds`); `array.bounds()`.
    pub fn bounds(&self) -> Handle<Graphic3dBoundBuffer> {
        self.buffers.read().bounds.clone()
    }
    /// Whether a submission built the device buffers (`IsInitialized`); `array.is_initialized()`.
    pub fn is_initialized(&self) -> bool {
        self.upload.is_initialized()
    }
    /// Build the buffers again on the next submission (`Invalidate`); `array.invalidate()`.
    pub fn invalidate(&self) {
        self.upload.invalidate();
    }
    /// Drop the built buffers; the next submission builds them again (`Release`).
    /// Example: `array.release()`.
    pub fn release(&self) {
        self.upload.release();
    }
    /// Release, then draw other buffers (`InitBuffers`); unlike the constructor this keeps an
    /// empty index buffer. Example: `array.init_buffers(Kind::Polylines, &i, &a, &b)`.
    pub fn init_buffers(
        &self,
        kind: Kind,
        indices: &Handle<dyn Graphic3dIndexBufferSource>,
        attributes: &Handle<dyn Graphic3dBufferSource>,
        bounds: &Handle<Graphic3dBoundBuffer>,
    ) {
        self.release();
        *self.buffers.write() = PrimitiveArrayBuffers {
            kind,
            indices: indices.clone(),
            attributes: attributes.clone(),
            bounds: bounds.clone(),
        };
    }
    fn submission(&self) -> Option<PrimitiveSubmission> {
        let kind = self.draw_mode()?;
        let buffers = self.buffers.read();
        Some(PrimitiveSubmission {
            kind,
            indices: buffers.indices.clone(),
            attributes: buffers.attributes.clone(),
            bounds: buffers.bounds.clone(),
            upload: self.upload.clone(),
            uid: self.uid,
        })
    }
    fn element_stats(&self) -> GroupElementStats {
        GroupElementStats::Primitive {
            kind: self.draw_mode().unwrap_or(Kind::Undefined),
            bounds: bound_count(&self.buffers.read().bounds),
            upload: self.upload.current(),
        }
    }
}

impl RenderElement for RenderPrimitiveArray {
    /// Draw with the current aspects; arrays without a draw mode emit nothing.
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        if let Some(primitive) = self.submission() {
            context.add_primitive_submission(primitive);
        }
        Ok(())
    }
    fn is_fill_draw_mode(&self) -> bool {
        self.draw_mode()
            .is_some_and(|kind| !crate::element_stats::is_line_array(kind) && kind != Kind::Points)
    }
    fn estimated_data_size(&self) -> usize {
        self.upload.current().estimated_bytes()
    }
    fn update_draw_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp, detailed: bool) {
        self.element_stats().draw_stats(detailed).apply_to(stats);
    }
    fn is_raytraced_element(&self) -> bool {
        self.submission()
            .is_some_and(|primitive| crate::driver_dump::is_raytraced_primitive(&primitive))
    }
    /// `GlPrimitiveArray::DumpJson`: the element, the Graphic3d buffers, the GL draw
    /// mode, fill type, buffer residency and unique id; `array.dump_json(&mut stream, -1)?`.
    fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        let buffers = self.buffers.read().clone();
        let primitive = PrimitiveSubmission {
            kind: buffers.kind,
            indices: buffers.indices,
            attributes: buffers.attributes,
            bounds: buffers.bounds,
            upload: self.upload.clone(),
            uid: self.uid,
        };
        let address = crate::driver_dump::address_of(self);
        crate::driver_dump::primitive_array(stream, depth, address, &primitive, self.uid)
    }
}
