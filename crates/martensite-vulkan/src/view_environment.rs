//! GlView's background image and image based lighting state: SetBackgroundImage,
//! SetImageBasedLighting, the skydome texture of updateSkydomeBg and the PBR environment request
//! that GlView::Render consumes. The retained-view forwarding macro has no exclusion list,
//! so, as for the statistics, the inherent methods of RenderRetainedView take precedence over
//! the forwarded defaults (which store the image and ignore image based lighting).
use crate::environment_frame::*;
use crate::{RenderError, error::rejected, view_statistics::RenderRetainedView};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// Renderer-side environment state of one view.
#[derive(Default)]
pub(crate) struct ViewEnvironmentState {
    token: Arc<ViewEnvironmentToken>,
    /// myPBREnvRequest.
    request: bool,
    /// The texture of myCubeMapParams: an image cube map, or the skydome once generated.
    skydome: Option<SkydomeParams>,
    snapshots: Vec<Arc<CubeMapPixels>>,
}

impl RenderRetainedView {
    /// `GlView::SetBackgroundImage`: a 2D texture or a cube map background; with
    /// `to_update_pbr_env` the cube map (or none) also becomes the IBL source, requesting a new
    /// PBR environment when its id changes. Example: `view.set_background_image(&cube, true)`.
    pub fn set_background_image(
        &self,
        texture: &Handle<dyn Graphic3dTextureMap>,
        to_update_pbr_env: bool,
    ) {
        let texture = done_texture(texture);
        let cube = texture.cast::<dyn Graphic3dCubeMap>();
        if to_update_pbr_env {
            self.replace_ibl_cube_map(&cube);
        }
        let mut fields = self.cview_base().fields.write();
        if texture.is_null() {
            if matches!(
                fields.background_type,
                Graphic3dTypeOfBackground::Texture | Graphic3dTypeOfBackground::Cubemap
            ) {
                fields.background_type = Graphic3dTypeOfBackground::None;
            }
            return;
        }
        if cube.is_null() {
            fields.background_type = Graphic3dTypeOfBackground::Texture;
            fields.background_image = texture;
            return;
        }
        fields.background_type = Graphic3dTypeOfBackground::Cubemap;
        fields.cube_map_background = cube;
        self.environment.write().skydome = None;
    }

    /// `GlView::SetImageBasedLighting`: the background cube map becomes the IBL source,
    /// or none when disabled or without a cubemap background. Example:
    /// `view.set_image_based_lighting(true)`.
    pub fn set_image_based_lighting(&self, to_enable_ibl: bool) {
        let fields = self.cview_base().fields.read();
        let enabled = to_enable_ibl && fields.background_type == Graphic3dTypeOfBackground::Cubemap;
        let source = if enabled {
            fields.cube_map_background.clone()
        } else {
            Handle::default()
        };
        drop(fields);
        if !enabled && self.cview_base().fields.read().cube_map_ibl.is_null() {
            return;
        }
        self.replace_ibl_cube_map(&source);
    }

    /// Set myCubeMapIBL and raise myPBREnvRequest when its id changes.
    fn replace_ibl_cube_map(&self, cube: &Handle<dyn Graphic3dCubeMap>) {
        let mut fields = self.cview_base().fields.write();
        if cube_id(&fields.cube_map_ibl) != cube_id(cube) {
            self.environment.write().request = true;
        }
        fields.cube_map_ibl = cube.clone();
    }

    /// The environment of the next frame of `view`; consumes the skydome update and, for PBR
    /// views, the environment request. Example: `retained.frame_environment(view)?`.
    pub fn frame_environment(
        &self,
        view: &dyn Graphic3dCView,
    ) -> Result<Option<FrameEnvironment>, RenderError> {
        let background = self.cube_background(view)?;
        let pbr = self.frame_pbr_environment(view)?;
        if background.is_none() && pbr.is_none() {
            return Ok(None);
        }
        Ok(Some(FrameEnvironment {
            view: self.environment.read().token.clone(),
            background,
            pbr,
            raytrace_cube: None,
        }))
    }

    /// CubemapForBack of a ray-traced frame (GlView::initRaytraceResources): set once a
    /// cube map background was set, which GlView keeps (myCubeMapBackground) when the
    /// background type changes. The texture is myCubeMapParams' (the skydome once generated),
    /// `[uYCoeff, uZCoeff]` follow the cube map and `uEnvMapForBack` the background type. When
    /// the background is no longer the cube map, `environment` gains the cube to upload.
    /// Example: `let cube = retained.raytrace_cube(view, &mut environment)?;`.
    pub fn raytrace_cube(
        &self,
        view: &dyn Graphic3dCView,
        environment: &mut Option<FrameEnvironment>,
    ) -> Result<Option<([i32; 2], bool)>, RenderError> {
        let cube = self.cview_base().fields.read().cube_map_background.clone();
        if cube.is_null() {
            return Ok(None);
        }
        let coefficients = self.image_source(&cube)?.coefficients();
        let shown = view.background_type() == Graphic3dTypeOfBackground::Cubemap;
        let uploaded = environment.as_ref().is_some_and(|e| e.background.is_some());
        if !uploaded {
            let source = self.background_source()?;
            let frame = environment.get_or_insert_with(|| FrameEnvironment {
                view: self.environment.read().token.clone(),
                background: None,
                pbr: None,
                raytrace_cube: None,
            });
            frame.raytrace_cube = Some(source);
        }
        Ok(Some((coefficients, shown)))
    }

    fn cube_background(
        &self,
        view: &dyn Graphic3dCView,
    ) -> Result<Option<CubeBackground>, RenderError> {
        if view.background_type() != Graphic3dTypeOfBackground::Cubemap {
            return Ok(None);
        }
        self.consume_skydome_update();
        let source = self.background_source()?;
        Ok(Some(CubeBackground {
            source,
            projection_world_view: background_projection_world_view(view)?,
        }))
    }

    /// GlView::updateSkydomeBg: myToUpdateSkydome makes the skydome the background texture.
    fn consume_skydome_update(&self) {
        let mut fields = self.cview_base().fields.write();
        if !std::mem::take(&mut fields.to_update_skydome) {
            return;
        }
        self.environment.write().skydome = Some(skydome_params(&fields.skydome_aspect));
    }

    fn background_source(&self) -> Result<EnvironmentCubeSource, RenderError> {
        if let Some(skydome) = self.environment.read().skydome {
            return Ok(EnvironmentCubeSource::Skydome(skydome));
        }
        let cube = self.cview_base().fields.read().cube_map_background.clone();
        self.image_source(&cube)
    }

    fn image_source(
        &self,
        cube: &Handle<dyn Graphic3dCubeMap>,
    ) -> Result<EnvironmentCubeSource, RenderError> {
        let mut state = self.environment.write();
        let pixels =
            crate::environment_cube_pixels::cached_cube_map_pixels(&mut state.snapshots, cube)?;
        Ok(EnvironmentCubeSource::Image(pixels))
    }

    /// GlView::Render "process PBR environment", only for PBR and PbrFacet views.
    fn frame_pbr_environment(
        &self,
        view: &dyn Graphic3dCView,
    ) -> Result<Option<FramePbrEnvironment>, RenderError> {
        let params = view.rendering_params();
        if !matches!(
            params.shading_model,
            Graphic3dTypeOfShadingModel::Pbr | Graphic3dTypeOfShadingModel::PbrFacet
        ) {
            return Ok(None);
        }
        if view.background_type() == Graphic3dTypeOfBackground::Cubemap {
            self.consume_skydome_update();
        }
        let source = self.ibl_source()?;
        Ok(Some(FramePbrEnvironment {
            pow2_size: u32::try_from(params.pbr_env_pow2_size).unwrap_or(0),
            spec_levels: u32::try_from(params.pbr_env_spec_map_nb_levels).unwrap_or(0),
            request: std::mem::take(&mut self.environment.write().request),
            source,
            samples: baking_samples(&params)?,
        }))
    }

    /// myCubeMapIBL: the background's texture when it is the background cube map.
    fn ibl_source(&self) -> Result<Option<EnvironmentCubeSource>, RenderError> {
        let fields = self.cview_base().fields.read();
        let ibl = fields.cube_map_ibl.clone();
        let shared =
            !ibl.is_null() && ibl.object_address() == fields.cube_map_background.object_address();
        drop(fields);
        if ibl.is_null() {
            return Ok(None);
        }
        if shared {
            return self.background_source().map(Some);
        }
        self.image_source(&ibl).map(Some)
    }
}

/// A null or not loadable texture is no background (GlView reports the failure).
fn done_texture(texture: &Handle<dyn Graphic3dTextureMap>) -> Handle<dyn Graphic3dTextureMap> {
    match texture.get() {
        Some(map) if map.is_done() => texture.clone(),
        _ => Handle::default(),
    }
}

fn cube_id(cube: &Handle<dyn Graphic3dCubeMap>) -> Vec<u8> {
    cube.get()
        .map(|map| map.get_id().as_bytes().to_vec())
        .unwrap_or_default()
}

fn skydome_params(aspect: &martensite_tkservice::aspect::AspectSkydomeBackground) -> SkydomeParams {
    let sun = aspect.sun_direction();
    SkydomeParams {
        sun_direction: [sun.x() as f32, sun.y() as f32, sun.z() as f32],
        cloudiness: aspect.cloudiness(),
        time: aspect.time_parameter(),
        fogginess: aspect.fogginess(),
        size: u32::try_from(aspect.size()).unwrap_or(0),
    }
}

fn baking_samples(params: &Graphic3dRenderingParams) -> Result<PbrBakingSamples, RenderError> {
    let count = |value: i32| {
        u32::try_from(value).map_err(|e| rejected((value, e), "a non-negative sample count"))
    };
    if !(0.0..=1.0).contains(&params.pbr_env_baking_probability) {
        return Err(rejected(
            params.pbr_env_baking_probability,
            "a baking probability in [0, 1]",
        ));
    }
    Ok(PbrBakingSamples {
        diffuse: count(params.pbr_env_baking_diff_nb_samples)?,
        specular: count(params.pbr_env_baking_spec_nb_samples)?,
        probability: params.pbr_env_baking_probability,
    })
}

/// GlBackgroundArray::Render for Graphic3dTOB_CUBEMAP: a copy of the view camera with the
/// Z range (0.01, 1), the eye moved to the origin and a perspective projection (stereo views
/// are rejected before), as the float product projection * orientation.
fn background_projection_world_view(view: &dyn Graphic3dCView) -> Result<[f32; 16], RenderError> {
    let owned = crate::view_render_camera::owned_render_camera(view);
    let camera = Graphic3dCamera::from_camera(&owned);
    camera
        .set_z_range(0.01, 1.0)
        .map_err(|e| rejected(e.to_string(), "a valid background camera Z range"))?;
    camera.move_eye_to(&martensite_tkmath::gp::Point3::new(0.0, 0.0, 0.0));
    camera.set_projection_type(Graphic3dCameraProjection::Perspective);
    Ok((camera.projection_matrix_f() * camera.orientation_matrix_f()).data())
}

#[cfg(test)]
#[path = "view_environment_tests.rs"]
mod tests;
