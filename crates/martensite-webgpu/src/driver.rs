//! Public WebGPU driver constructors, factory, and `WebGpuRenderViewExt::capture_async` (#634).
use crate::context::WebGpuContext;
use crate::options::WebGpuOptions;
use crate::renderer::WebGpuRenderer;
use crate::targets::WebGpuDeferredFrame;
use martensite_render_core::redraw_capture::AsyncReadbackState;
use martensite_render_core::{
    FrameCapture, Graphic3dRenderDriver, Graphic3dRenderView, RenderBehaviorConfiguration,
    RenderCoordinateConvention, RenderError, RenderLineClippingBehavior, RenderLineWidthPolicy,
    RenderPipelinePolicies,
};
#[cfg(not(target_arch = "wasm32"))]
use martensite_render_core::{RenderDeviceBackend, RenderDeviceOptions, RenderRasterPolicy};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::AspectDisplayConnection;
use martensite_tkservice::graphic3d::{
    Graphic3dGraphicDriver, Graphic3dGraphicDriverFactory, Graphic3dGraphicDriverFactoryBase,
};
use std::future::Future;

/// Canonical Graphic3d driver factory name for the WebGPU backend.
pub const WEBGPU_DRIVER_NAME: &str = "TKWebGPU";

/// Async and blocking constructors for a WebGPU-backed `Graphic3dRenderDriver` (#634).
pub struct WebGpuRenderDriver;

impl WebGpuRenderDriver {
    /// Asynchronously initialize a WebGPU `Graphic3dRenderDriver` with default policies (#634).
    ///
    /// Example: `let driver = WebGpuRenderDriver::new_async(WebGpuOptions::default()).await?;`.
    pub async fn new_async(
        options: WebGpuOptions,
    ) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
        Self::with_policies_async(
            options,
            RenderPipelinePolicies::default(),
            RenderCoordinateConvention::default(),
            RenderLineWidthPolicy::default(),
            RenderBehaviorConfiguration::default(),
        )
        .await
    }

    /// Asynchronously initialize a WebGPU `Graphic3dRenderDriver` with explicit policies (#634).
    pub async fn with_policies_async(
        options: WebGpuOptions,
        selection: RenderPipelinePolicies,
        coordinates: RenderCoordinateConvention,
        line_widths: RenderLineWidthPolicy,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
        let renderer = WebGpuRenderer::new_async(
            options,
            selection.raster,
            coordinates,
            RenderLineClippingBehavior::default(),
            configuration,
        )
        .await?;
        Graphic3dRenderDriver::from_device(Box::new(renderer), selection, line_widths)
    }

    /// Wrap an already-initialized `WebGpuContext` in a `Graphic3dRenderDriver` (#634).
    pub fn from_context(
        context: WebGpuContext,
        selection: RenderPipelinePolicies,
        coordinates: RenderCoordinateConvention,
        line_widths: RenderLineWidthPolicy,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
        let renderer = WebGpuRenderer::from_context(
            context,
            selection.raster,
            coordinates,
            RenderLineClippingBehavior::default(),
            configuration,
        )?;
        Graphic3dRenderDriver::from_device(Box::new(renderer), selection, line_widths)
    }

    /// Synchronously initialize a WebGPU `Graphic3dRenderDriver` on native desktop platforms (#634).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_blocking(
        options: WebGpuOptions,
    ) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
        crate::realm::block_on(Self::new_async(options))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl RenderDeviceOptions for WebGpuOptions {
    fn create_device(
        self,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Box<dyn RenderDeviceBackend>, RenderError> {
        let renderer =
            WebGpuRenderer::new_blocking(self, raster, coordinates, clipping, configuration)?;
        Ok(Box::new(renderer))
    }
}

/// `Graphic3dGraphicDriverFactory` implementation producing WebGPU drivers (#634).
pub struct WebGpuDriverFactory {
    base: Graphic3dGraphicDriverFactoryBase,
    default_options: SharedState<WebGpuOptions>,
}
martensite_tkernel::impl_transient!(
    WebGpuDriverFactory,
    "Martensite_WebGpuDriverFactory",
    StandardTransient,
    interfaces: [dyn Graphic3dGraphicDriverFactory]
);

impl WebGpuDriverFactory {
    /// Create a WebGPU driver factory named [`WEBGPU_DRIVER_NAME`] with default options.
    pub fn new() -> Self {
        Self {
            base: Graphic3dGraphicDriverFactoryBase::new(&AsciiString::from(WEBGPU_DRIVER_NAME)),
            default_options: SharedState::new(WebGpuOptions::default()),
        }
    }

    /// Return the default `WebGpuOptions` applied to drivers created by this factory.
    pub fn default_options(&self) -> WebGpuOptions {
        *self.default_options.read()
    }

    /// Replace the default `WebGpuOptions` for subsequent driver creations.
    pub fn set_default_options(&self, options: WebGpuOptions) {
        *self.default_options.write() = options;
    }
}

impl Default for WebGpuDriverFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Graphic3dGraphicDriverFactory for WebGpuDriverFactory {
    fn graphic_driver_factory_base(&self) -> &Graphic3dGraphicDriverFactoryBase {
        &self.base
    }

    fn create_driver(
        &self,
        _display: &Handle<AspectDisplayConnection>,
    ) -> Result<Handle<dyn Graphic3dGraphicDriver>, Failure> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let driver =
                WebGpuRenderDriver::new_blocking(self.default_options()).map_err(|error| {
                    Failure::with_message(
                        "Standard_ProgramError",
                        &format!("Error: the WebGPU graphic driver cannot be created: {error}"),
                    )
                })?;
            Ok(driver.cast())
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err(Failure::with_message(
                "Standard_ProgramError",
                "WebGpuDriverFactory::create_driver cannot block on wasm32; expected WebGpuRenderDriver::new_async",
            ))
        }
    }
}

/// Asynchronous host pixel readback extension on `Graphic3dRenderView` for WebGPU (#634).
pub trait WebGpuRenderViewExt {
    /// Asynchronously map and read back the latest redrawn frame without blocking the event loop.
    ///
    /// Example: `let capture = view.capture_async().await?;`.
    fn capture_async(&self) -> impl Future<Output = Result<FrameCapture, RenderError>>;
}

impl WebGpuRenderViewExt for Graphic3dRenderView {
    async fn capture_async(&self) -> Result<FrameCapture, RenderError> {
        let published = self.latest_redraw_capture()?;
        let state = published.take_for_async_readback()?;
        let (readback, steps) = match state {
            AsyncReadbackState::Ready(capture) => return Ok(capture),
            AsyncReadbackState::Deferred(readback, steps) => (readback, steps),
        };
        let Some(deferred) = readback.as_any().downcast_ref::<WebGpuDeferredFrame>() else {
            let err = RenderError::rejected(
                "non-WebGPU deferred frame",
                "a WebGpuDeferredFrame produced by WebGpuRenderer",
            );
            published.complete_async_readback(Err(err.clone()));
            return Err(err);
        };
        let map_future = {
            let device = self.render_device().read();
            let Some(webgpu) = device
                .as_device()
                .and_then(|d| d.as_any().downcast_ref::<WebGpuRenderer>())
            else {
                let err = RenderError::rejected(
                    device.name(),
                    "a Graphic3dRenderView backed by WebGpuRenderer",
                );
                published.complete_async_readback(Err(err.clone()));
                return Err(err);
            };
            deferred.begin_map_async(webgpu.context())
        };
        let read_result = match map_future {
            Ok(future) => deferred.finish_map_capture(future.await),
            Err(err) => Err(err),
        };
        match read_result {
            Ok(mut capture) => {
                steps.apply(&mut capture);
                published.complete_async_readback(Ok(capture.clone()));
                Ok(capture)
            }
            Err(err) => {
                published.complete_async_readback(Err(err.clone()));
                Err(err)
            }
        }
    }
}
