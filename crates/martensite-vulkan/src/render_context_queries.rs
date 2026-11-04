//! The view-derived queries of `RenderContextState`: what an element reads from the native
//! `GlContext`, `GlShaderManager` and `GlView` while it renders:
//! the frame camera, the window size, the local origin of the layer, the shading model of the
//! view and the per-primitive choice derived from it, the colour encoding of the frame and the
//! transparency test of an aspect.
use crate::RenderContextState;
use Graphic3dTypeOfShadingModel as Model;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

impl RenderContextState {
    /// The camera of the frame (`GlContext::Camera`); `state.camera.get`.
    pub fn camera(&self) -> &Handle<Graphic3dCamera> {
        &self.camera
    }
    /// The window width and height of the view (`GlWorkspace::Width`, `Height`).
    pub fn window_size(&self) -> [i32; 2] {
        self.window_size
    }
    /// The local origin the current layer renders around (`GlView::LocalOrigin`,
    /// `GlShaderManager::LocalOrigin`); zero outside an origin layer.
    pub fn local_origin(&self) -> [f64; 3] {
        self.local_origin
    }
    /// The view shading model after the PBR fallback (`GlShaderManager::ShadingModel`).
    pub fn shading_model(&self) -> Graphic3dTypeOfShadingModel {
        self.shading_model
    }
    /// PBR shading is available on the device and pipeline (`GlContext::HasPBR`).
    pub fn has_pbr(&self) -> bool {
        self.has_pbr
    }
    /// The frame renders into sRGB attachments (`GlContext::ToRenderSRGB`).
    pub fn to_render_srgb(&self) -> bool {
        self.srgb
    }
    /// The window target is sRGB-ready (`GlContext::IsWindowSRGB`).
    pub fn is_window_srgb(&self) -> bool {
        self.srgb
    }
    /// The bound target encodes linear colours on write (`GlContext::IsFrameBufferSRGB`).
    pub fn is_frame_buffer_srgb(&self) -> bool {
        self.srgb
    }
    /// A linear colour as the shaders of this frame expect it: unchanged for an sRGB target,
    /// gamma-encoded otherwise (`GlContext::Vec4FromQuantityColor`).
    pub fn vec4_from_quantity_color(&self, color: [f32; 4]) -> [f32; 4] {
        match self.srgb {
            true => Self::vec4_linear_from_quantity_color(color),
            false => Self::vec4_srgb_from_quantity_color(color),
        }
    }
    /// The linear colour itself (`GlContext::Vec4LinearFromQuantityColor`).
    pub fn vec4_linear_from_quantity_color(color: [f32; 4]) -> [f32; 4] {
        color
    }
    /// The linear colour encoded as sRGB (`GlContext::Vec4sRGBFromQuantityColor`).
    pub fn vec4_srgb_from_quantity_color(color: [f32; 4]) -> [f32; 4] {
        ColorRgba::linear_to_srgb_vec4(color)
    }
    /// Whether `aspects` under `highlight` need blending (`GlContext::CheckIsTransparent`).
    pub fn check_is_transparent(
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        highlight: &Handle<dyn Graphic3dPresentationAttributesSource>,
    ) -> bool {
        crate::render_element_pass::check_is_transparent(aspects, highlight)
    }
    /// The blending test with the front and back alphas it read (the native out parameters);
    /// `let (blend, [front, back]) = RenderContextState::check_is_transparent_alphas(&a, &h);`.
    pub fn check_is_transparent_alphas(
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        highlight: &Handle<dyn Graphic3dPresentationAttributesSource>,
    ) -> (bool, [f32; 2]) {
        crate::render_element_pass::check_is_transparent_alphas(aspects, highlight)
    }
}

impl RenderContextState {
    /// The model a face draw uses (`GlShaderManager::ChooseFaceShadingModel`): the custom
    /// model or the view's, facet models without nodal normals, Phong models without PBR.
    pub fn choose_face_shading_model(&self, custom: Model, has_nodal_normals: bool) -> Model {
        match self.custom_or_view(custom) {
            model @ (Model::Default | Model::Unlit | Model::PhongFacet) => model,
            model @ (Model::Gouraud | Model::Phong) if has_nodal_normals => model,
            Model::Gouraud | Model::Phong => Model::PhongFacet,
            Model::Pbr if has_nodal_normals => self.pbr_fallback(Model::Pbr),
            Model::Pbr | Model::PbrFacet => self.pbr_fallback(Model::PbrFacet),
        }
    }
    /// The model a line draw uses (`ChooseLineShadingModel`): unlit without nodal normals.
    pub fn choose_line_shading_model(&self, custom: Model, has_nodal_normals: bool) -> Model {
        if !has_nodal_normals {
            return Model::Unlit;
        }
        match self.custom_or_view(custom) {
            model @ (Model::Gouraud | Model::Phong) => model,
            Model::Pbr => self.pbr_fallback(Model::Pbr),
            _ => Model::Unlit,
        }
    }
    /// The model a marker draw uses, as for lines (`ChooseMarkerShadingModel`).
    pub fn choose_marker_shading_model(&self, custom: Model, has_nodal_normals: bool) -> Model {
        self.choose_line_shading_model(custom, has_nodal_normals)
    }
    /// The Phong model of a PBR model when PBR is not allowed
    /// (`GlShaderManager::PBRShadingModelFallback`); `fallback(Pbr, false) == Phong`.
    pub fn pbr_shading_model_fallback(model: Model, is_pbr_allowed: bool) -> Model {
        match (model, is_pbr_allowed) {
            (_, true) => model,
            (Model::Pbr, false) => Model::Phong,
            (Model::PbrFacet, false) => Model::PhongFacet,
            (other, false) => other,
        }
    }
    fn custom_or_view(&self, custom: Model) -> Model {
        match custom {
            Model::Default => self.shading_model,
            other => other,
        }
    }
    /// `IsPbrAllowed`: the view itself shades with a PBR model.
    fn pbr_fallback(&self, model: Model) -> Model {
        let allowed = matches!(self.shading_model, Model::Pbr | Model::PbrFacet);
        Self::pbr_shading_model_fallback(model, allowed)
    }
}
