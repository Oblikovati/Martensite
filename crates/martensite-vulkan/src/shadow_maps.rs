//! Shadow maps GlView::Redraw: one depth map per enabled shadow-casting
//! light (the cast-shadows flag; resolution and bias come from the rendering parameters),
//! rendered from the light camera of GlShadowMap::UpdateCamera before the scene and sampled
//! by the Phong-family and PBR fragments (LightShadow.glsl, backend/shadow_maps.rs).
use crate::RenderError;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::graphic3d::*;

/// One rendered shadow map (GlShadowMap): the light matrix (`ProjectionMatrixF *
/// OrientationMatrixF` of the light camera) and the depth texels, bottom row first as an OpenGL
/// texture.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderShadowMap {
    /// The casting light (GlShadowMap::LightSource); ShadowMapDump finds maps by its name.
    pub(crate) light: Handle<Graphic3dCLight>,
    pub(crate) light_matrix: [f32; 16],
    pub(crate) depth: Vec<f32>,
}
impl RenderShadowMap {
    /// The column-major world-to-light-clip matrix the fragments sample with
    /// (GlShadowMap::LightSourceMatrix); `map.light_source_matrix[15]`.
    pub fn light_source_matrix(&self) -> [f32; 16] {
        self.light_matrix
    }
    /// The depth texels, bottom row first, `resolution` texels per row; `map.depth()[0]`.
    pub fn depth(&self) -> &[f32] {
        &self.depth
    }
}
/// The view's shadow maps in caster order with their resolution and bias
/// (GlShadowMapArray).
#[derive(Clone, Debug, PartialEq)]
pub struct RenderShadowMaps {
    pub(crate) resolution: u32,
    pub(crate) bias: f32,
    pub(crate) maps: Vec<RenderShadowMap>,
}
impl RenderShadowMaps {
    /// Texels per side of every map (`ShadowMapResolution`); `maps.resolution()`.
    pub fn resolution(&self) -> u32 {
        self.resolution
    }
    /// The depth bias of the comparisons (GlShadowMap::ShadowMapBias); `maps.bias`.
    pub fn bias(&self) -> f32 {
        self.bias
    }
    /// One map per shadow-casting light, in caster order; `maps.maps()[0]`.
    pub fn maps(&self) -> &[RenderShadowMap] {
        &self.maps
    }
}

/// The light cameras of a view, kept between frames as GlShadowMapArray keeps its maps,
/// and the maps of the last frame.
#[derive(Default)]
pub(crate) struct ShadowCameras {
    cameras: Vec<Handle<Graphic3dCamera>>,
    pub(crate) rendered: Option<std::sync::Arc<RenderShadowMaps>>,
    /// The control-capture offset of the maps' rasterization, in texels (zero natively).
    raster_shift: [f64; 2],
}
impl ShadowCameras {
    /// One camera per caster, keeping the existing ones (`myShadowMaps->Resize(.., true)`).
    fn resize(&mut self, count: usize) -> &[Handle<Graphic3dCamera>] {
        self.cameras.truncate(count);
        while self.cameras.len() < count {
            self.cameras.push(Handle::new(Graphic3dCamera::default()));
        }
        &self.cameras
    }
}

/// The casting lights when the view renders shadow maps (GlView::Redraw "allocate shadow
/// maps"): shadows enabled, a positive resolution and at least one enabled caster.
fn shadow_casters(view: &dyn Graphic3dCView) -> Vec<Handle<Graphic3dCLight>> {
    let params = view.rendering_params();
    let lights = view.lights();
    let Some(set) = lights.get() else {
        return Vec::new();
    };
    set.update_revision();
    // Ray tracing traces its own shadow rays (GlView::Redraw toUseShadowMap).
    if !params.ray_tracing.is_shadow_enabled
        || params.shadow_map_resolution <= 0
        || params.method == Graphic3dRenderingMode::Raytracing
        || set.nb_cast_shadows() <= 0
    {
        return Vec::new();
    }
    // Graphic3dLightSet::IterationFilter_ActiveShadowCasters.
    set.lights()
        .into_iter()
        .filter(|light| {
            light.is_enabled()
                && light.light_type() != Graphic3dTypeOfLightSource::Ambient
                && light.to_cast_shadows()
        })
        .collect()
}

/// GlShadowMap::UpdateCamera without an origin: aim `camera` at the view's bounds from
/// `light`; returns the light matrix, or `None` for lights without a shadow camera.
fn update_camera(
    view: &dyn Graphic3dCView,
    camera: &Graphic3dCamera,
    light: &Graphic3dCLight,
) -> Result<Option<Mat4<f32>>, RenderError> {
    let failure = |error| crate::error::rejected(error, "a valid shadow map camera");
    let min_max = view.min_max_values(false).map_err(failure)?;
    let view_camera = crate::view_render_camera::owned_render_camera(view);
    match light.light_type() {
        Graphic3dTypeOfLightSource::Directional => {
            let direction = light.direction().map_err(failure)?;
            let mut dir = [direction.x(), direction.y(), direction.z(), 0.];
            if light.is_headlight() {
                let inverse = view_camera
                    .orientation_matrix()
                    .inverted()
                    .map_err(failure)?;
                dir = (inverse
                    * martensite_tkernel::ncollection::Vec4::new(dir[0], dir[1], dir[2], dir[3]))
                .data();
            }
            camera.set_zero_to_one_depth(view_camera.is_zero_to_one_depth());
            camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
            camera.set_direction(&Direction3::new(dir[0], dir[1], dir[2]).map_err(failure)?);
            set_light_up(camera)?;
            // Fitting the entire scene to the light, as natively.
            if camera
                .fit_min_max(
                    &min_max,
                    10. * martensite_tkernel::precision::confusion(),
                    false,
                )
                .map_err(failure)?
            {
                let dimensions = camera.view_dimensions();
                camera.set_scale((dimensions.x() * 1.1).max(dimensions.y() * 1.1));
            }
            camera.z_fit_all(1., &min_max, &min_max).map_err(failure)?;
        }
        Graphic3dTypeOfLightSource::Spot => {
            camera.set_zero_to_one_depth(view_camera.is_zero_to_one_depth());
            camera.set_projection_type(Graphic3dCameraProjection::Perspective);
            let position = light.position();
            let mut light_box = BoundingBox3::default();
            light_box.set_point(position);
            let corners = (min_max.corner_min(), min_max.corner_max());
            let diagonal = match corners {
                (Ok(min), Ok(max)) => min.distance(&max),
                _ => 0.,
            };
            let distance = min_max.distance(&light_box) + diagonal;
            camera.set_distance(distance);
            camera.move_eye_to(&position);
            camera.set_direction_from_eye(&light.direction().map_err(failure)?);
            set_light_up(camera)?;
            camera.set_z_range(1., distance).map_err(failure)?;
        }
        // Ambient lights have no map; positional lights would need a cube map (not natively).
        _ => return Ok(None),
    }
    Ok(Some(
        camera.projection_matrix_f() * camera.orientation_matrix_f(),
    ))
}
/// `SetUp(!Direction().IsParallel(DY, Angular) ? DY : DX); OrthogonalizeUp()`.
fn set_light_up(camera: &Graphic3dCamera) -> Result<(), RenderError> {
    let parallel = camera
        .direction()
        .is_parallel(Direction3::Y, martensite_tkernel::precision::angular());
    camera.set_up(if parallel {
        &Direction3::X
    } else {
        &Direction3::Y
    });
    camera
        .orthogonalize_up()
        .map_err(|error| crate::error::rejected(error, "an orthogonal shadow camera up"))
}

impl crate::Graphic3dRenderView {
    /// Render the shadow map of every caster before the scene (GlView::Redraw, "draw
    /// shadow maps"); `None` when the view renders none. Example: `view.shadow_maps(extent)?`.
    pub(crate) fn shadow_maps(
        &self,
        window: [u32; 2],
    ) -> Result<Option<std::sync::Arc<RenderShadowMaps>>, RenderError> {
        let casters = shadow_casters(self);
        let mut cameras = self.retained.shadow_cameras.write();
        cameras.rendered = None;
        if casters.is_empty() {
            return Ok(None);
        }
        let params = self.rendering_params();
        let resolution = params.shadow_map_resolution as u32;
        let clipping = self.retained.render_clip_planes();
        let mut maps = Vec::with_capacity(casters.len());
        let shift = cameras.raster_shift;
        for (light, camera) in casters.iter().zip(cameras.resize(casters.len())) {
            let texels = resolution as usize * resolution as usize;
            let Some(matrix) = update_camera(self, camera, light)? else {
                // Not rendered natively; never sampled (only directional and spot lights are).
                maps.push(RenderShadowMap {
                    light: light.clone(),
                    light_matrix: Mat4::<f32>::identity().data(),
                    depth: vec![1.; texels],
                });
                continue;
            };
            let raster = shifted_raster_camera(camera, shift, resolution);
            let frame = crate::view_scene::shadow_frame_for_view(
                self,
                (resolution, window),
                raster.as_ref().unwrap_or(camera),
                (clipping, self.device.read().device_limits()),
            )?;
            let capture = self.device.write().render(&frame)?;
            maps.push(RenderShadowMap {
                light: light.clone(),
                light_matrix: matrix.data(),
                depth: bottom_up_rows(&capture.depth, resolution as usize),
            });
        }
        let rendered = std::sync::Arc::new(RenderShadowMaps {
            resolution,
            bias: params.shadow_map_bias,
            maps,
        });
        cameras.rendered = Some(rendered.clone());
        Ok(Some(rendered))
    }
    /// Testing only, not product API: offset the rasterization of the shadow maps, not their
    /// sampling, by `texels` of a map (x along the map's columns, y along its rows); zero, the
    /// default, renders as natively. The control captures of the cross-driver shadow grading:
    /// a vertex snapped to a different sub-pixel grid moves the map's raster by less than one
    /// snap step. Example: `view.set_shadow_map_raster_shift([1. / 512., 0.])?`.
    #[doc(hidden)]
    pub fn set_shadow_map_raster_shift(&self, texels: [f64; 2]) -> Result<(), RenderError> {
        if !texels.iter().all(|t| t.abs() < 1.) {
            return Err(crate::error::rejected(
                texels,
                "a shadow map raster shift below one texel on each axis",
            ));
        }
        self.retained.shadow_cameras.write().raster_shift = texels;
        Ok(())
    }
    /// The shadow maps the last redraw rendered and sampled (GlView's `myShadowMaps`),
    /// `None` when it rendered none; `view.rendered_shadow_maps.map(|maps| maps.bias)`.
    pub fn rendered_shadow_maps(&self) -> Option<std::sync::Arc<RenderShadowMaps>> {
        self.retained.shadow_cameras.read().rendered.clone()
    }
}
/// A copy of the light `camera` whose projection is preceded by a clip-space translation of
/// `shift` texels of a `resolution`-texel map, or `None` without a shift.
fn shifted_raster_camera(
    camera: &Handle<Graphic3dCamera>,
    shift: [f64; 2],
    resolution: u32,
) -> Option<Handle<Graphic3dCamera>> {
    let source = camera.get()?;
    if shift == [0., 0.] {
        return None;
    }
    let mut translation = martensite_tkernel::ncollection::Mat4::<f64>::identity();
    // One texel spans 2 / resolution of normalized device coordinates; scaled by w in clip space.
    translation.set_value(0, 3, 2. * shift[0] / f64::from(resolution));
    translation.set_value(1, 3, 2. * shift[1] / f64::from(resolution));
    let shifted = Graphic3dCamera::default();
    shifted.copy(source);
    shifted.set_custom_mono_projection(&(translation * source.projection_matrix()));
    Some(Handle::new(shifted))
}
/// Captures are top row first; OpenGL depth textures hold the bottom row first.
fn bottom_up_rows(depth: &[f32], width: usize) -> Vec<f32> {
    depth.chunks(width).rev().flatten().copied().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn capture_rows_are_reversed_into_texture_rows() {
        let rows = super::bottom_up_rows(&[1., 2., 3., 4., 5., 6.], 2);
        assert_eq!(rows, vec![5., 6., 3., 4., 1., 2.]);
    }
    #[test]
    fn shifted_raster_cameras_move_clip_x_and_y_by_the_texel_fraction() {
        use martensite_tkernel::ncollection::Vec4;
        use martensite_tkernel::standard::Handle;
        use martensite_tkservice::graphic3d::*;
        let camera = Handle::new(Graphic3dCamera::default());
        camera.set_projection_type(Graphic3dCameraProjection::Perspective);
        assert!(super::shifted_raster_camera(&camera, [0., 0.], 256).is_none());
        let shifted = super::shifted_raster_camera(&camera, [0.5, -0.25], 256).unwrap();
        let point = Vec4::new(0.3, -0.2, -5., 1.);
        let ndc = |m: martensite_tkernel::ncollection::Mat4<f64>| {
            let clip = (m * camera.orientation_matrix() * point).data();
            [clip[0] / clip[3], clip[1] / clip[3], clip[2] / clip[3]]
        };
        let (plain, moved) = (
            ndc(camera.projection_matrix()),
            ndc(shifted.projection_matrix()),
        );
        let expected = [plain[0] + 1. / 256., plain[1] - 0.5 / 256., plain[2]];
        assert!(
            (0..3).all(|i| (moved[i] - expected[i]).abs() < 1e-12),
            "{moved:?}"
        );
    }
    #[test]
    fn rendered_maps_expose_resolution_bias_matrix_and_texels() {
        let map = super::RenderShadowMap {
            light: martensite_tkernel::standard::Handle::default(),
            light_matrix: [2.; 16],
            depth: vec![0.25; 4],
        };
        let maps = super::RenderShadowMaps {
            resolution: 2,
            bias: 0.005,
            maps: vec![map],
        };
        assert_eq!((maps.resolution(), maps.bias()), (2, 0.005));
        assert_eq!(maps.maps()[0].light_source_matrix(), [2.; 16]);
        assert_eq!(maps.maps()[0].depth(), &[0.25; 4]);
    }
}
