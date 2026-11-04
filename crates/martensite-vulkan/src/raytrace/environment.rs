//! The environment of ray-traced frames (GlView::setUniformState's environment
//! map parameters, bindRaytraceTextures and initRaytraceResources' CubemapForBack): the view's
//! environment texture sampled as a lat-long map, or with a cubemap background the cube map
//! (BACKGROUND_CUBEMAP), lighting path-traced scenes, reflected, and shown as the background.
use super::textures::RaytraceTexture;
use crate::RenderError;
use crate::retained_texture_images::view_texture_images;
use martensite_tkservice::graphic3d::*;

/// `uEnvMapEnabled`, `uEnvMapForBack`, `uYCoeff` and `uZCoeff`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct EnvironmentUniforms {
    pub enabled: bool,
    pub for_background: bool,
    /// `[uYCoeff, uZCoeff]`, set only with a cubemap background.
    pub coefficients: [i32; 2],
}

/// The environment one ray-traced frame samples.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RaytraceEnvironment {
    /// The view's environment texture (binding 11), without a cubemap background.
    pub map: Option<RaytraceTexture>,
    /// CubemapForBack: the frame's background cube map (binding 13) is the environment.
    pub cube: bool,
    pub uniforms: EnvironmentUniforms,
}

/// The environment of `view`: with `cube` (the coefficients of the view's cube map and whether
/// the background is still that cube map) the cube map is the environment, and the background
/// when it is; otherwise the view's environment texture when it loads, shown as the background
/// with UseEnvironmentMapBackground. Example: `raytrace_environment(&*view, None)?`.
pub(crate) fn raytrace_environment(
    view: &dyn Graphic3dCView,
    cube: Option<([i32; 2], bool)>,
) -> Result<RaytraceEnvironment, RenderError> {
    if let Some((coefficients, for_background)) = cube {
        return Ok(cube_environment(coefficients, for_background));
    }
    let map = environment_texture(view)?;
    let params = view.rendering_params();
    Ok(RaytraceEnvironment {
        uniforms: EnvironmentUniforms {
            enabled: map.is_some(),
            for_background: params.ray_tracing.use_environment_map_background,
            coefficients: [0, 0],
        },
        map,
        cube: false,
    })
}

/// The view's cube map: always enabled (its texture is valid once uploaded), shown only while
/// the background type is the cube map (`uEnvMapForBack`).
fn cube_environment(coefficients: [i32; 2], for_background: bool) -> RaytraceEnvironment {
    RaytraceEnvironment {
        map: None,
        cube: true,
        uniforms: EnvironmentUniforms {
            enabled: true,
            for_background,
            coefficients,
        },
    }
}

/// `myTextureEnv`: the frozen environment texture and its sampler, or none.
fn environment_texture(view: &dyn Graphic3dCView) -> Result<Option<RaytraceTexture>, RenderError> {
    let environment = view.texture_env();
    if environment.is_null() {
        return Ok(None);
    }
    let images = view_texture_images(view);
    let Some(image) = images.snapshot_environment(&environment)? else {
        return Ok(None);
    };
    let sampler = images.samplers.policy(&image, &environment.get_params())?;
    Ok(Some(RaytraceTexture { image, sampler }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_cubemap_background_is_the_shown_environment_with_its_coefficients() {
        let environment = cube_environment([-1, 1], true);
        assert!(environment.cube && environment.map.is_none());
        assert_eq!(
            environment.uniforms,
            EnvironmentUniforms {
                enabled: true,
                for_background: true,
                coefficients: [-1, 1],
            }
        );
        // After the background switched away from the cube map (CubemapForBack stays set).
        let kept = cube_environment([1, -1], false);
        assert!(kept.cube && kept.uniforms.enabled && !kept.uniforms.for_background);
    }
}
