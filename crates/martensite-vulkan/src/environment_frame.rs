//! The view environment one frame hands to the renderer: GlView's cubemap background
//! (myCubeMapBackground drawn with uYCoeff/uZCoeff) and its PBR environment
//! (myCubeMapIBL, myPBREnvRequest and the Graphic3dRenderingParams baking parameters).
use crate::surface_texture::SurfaceImageEncoding;
use std::sync::Arc;

/// Six square faces of a cube map in +X, -X, +Y, -Y, +Z, -Z order, each in its native memory
/// row order (GlTexture::InitCubeMap uploads `ImagePixMap::Data` unchanged).
#[derive(Debug, PartialEq)]
pub(crate) struct CubeMapPixels {
    pub id: Vec<u8>,
    pub revision: usize,
    pub size: u32,
    pub encoding: SurfaceImageEncoding,
    pub faces: [Vec<u8>; 6],
    /// The supplied mip lengths of each compressed side (DDS); empty for uncompressed sides,
    /// whose mipmaps the upload generates.
    pub levels: Vec<usize>,
    pub top_down: bool,
    pub z_inverted: bool,
}

impl CubeMapPixels {
    /// `uYCoeff` and `uZCoeff` of GlView::drawBackground and GlPBREnvironment::Bake:
    /// `[IsTopDown ? 1: -1, ZIsInverted ? -1: 1]`. Example: `pixels.coefficients`.
    pub fn coefficients(&self) -> [i32; 2] {
        [
            if self.top_down { 1 } else { -1 },
            if self.z_inverted { -1 } else { 1 },
        ]
    }
}

/// AspectSkydomeBackground values that GlView::updateSkydomeBg pushes to the
/// SkydomBackground.fs uniforms, plus the cube face size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SkydomeParams {
    pub sun_direction: [f32; 3],
    pub cloudiness: f32,
    pub time: f32,
    pub fogginess: f32,
    pub size: u32,
}

/// The cube map a background or bake reads: an image cube map or the generated skydome.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EnvironmentCubeSource {
    Image(Arc<CubeMapPixels>),
    Skydome(SkydomeParams),
}

impl EnvironmentCubeSource {
    /// The native coefficients; the skydome cube (Graphic3dCubeMapPacked("")) is top-down and
    /// not Z-inverted. Example: `source.coefficients`.
    pub fn coefficients(&self) -> [i32; 2] {
        match self {
            Self::Image(pixels) => pixels.coefficients(),
            Self::Skydome(_) => [1, 1],
        }
    }
}

/// The cubemap background of a frame and its camera: the float product of the projection and
/// orientation matrices that GlBackgroundArray::Render derives for Graphic3dTOB_CUBEMAP.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CubeBackground {
    pub source: EnvironmentCubeSource,
    pub projection_world_view: [f32; 16],
}

/// Graphic3dRenderingParams PbrEnvBakingDiffNbSamples, PbrEnvBakingSpecNbSamples and
/// PbrEnvBakingProbability.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PbrBakingSamples {
    pub diffuse: u32,
    pub specular: u32,
    pub probability: f32,
}

/// The PBR environment state of a PBR frame (GlView::Render "process PBR environment").
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FramePbrEnvironment {
    pub pow2_size: u32,
    pub spec_levels: u32,
    /// myPBREnvRequest: bake (or clear) the environment before this frame.
    pub request: bool,
    /// myCubeMapIBL; `None` clears the environment to unit lighting.
    pub source: Option<EnvironmentCubeSource>,
    pub samples: PbrBakingSamples,
}

/// Identity of a view for the renderer's per-view environment caches; the renderer keeps a weak
/// reference and drops the view's resources once every strong owner is gone.
#[derive(Debug, Default)]
pub(crate) struct ViewEnvironmentToken;

/// The environment of one frame; `frame.environment = Some(environment)`.
#[derive(Clone, Debug)]
pub(crate) struct FrameEnvironment {
    pub view: Arc<ViewEnvironmentToken>,
    pub background: Option<CubeBackground>,
    pub pbr: Option<FramePbrEnvironment>,
    /// The view's cube map that a ray-traced frame samples while the background is no longer
    /// that cube map (GlView keeps myCubeMapBackground and CubemapForBack).
    pub raytrace_cube: Option<EnvironmentCubeSource>,
}

impl PartialEq for FrameEnvironment {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.view, &other.view)
            && self.background == other.background
            && self.pbr == other.pbr
            && self.raytrace_cube == other.raytrace_cube
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(top_down: bool, z_inverted: bool) -> CubeMapPixels {
        CubeMapPixels {
            id: b"cube".to_vec(),
            revision: 0,
            size: 1,
            encoding: SurfaceImageEncoding::Srgb8,
            faces: Default::default(),
            levels: Vec::new(),
            top_down,
            z_inverted,
        }
    }

    #[test]
    fn coefficients_follow_the_native_cubemap_flags() {
        assert_eq!(pixels(true, false).coefficients(), [1, 1]);
        assert_eq!(pixels(false, false).coefficients(), [-1, 1]);
        assert_eq!(pixels(true, true).coefficients(), [1, -1]);
        let skydome = SkydomeParams {
            sun_direction: [0., 1., 0.],
            cloudiness: 0.2,
            time: 0.,
            fogginess: 0.,
            size: 512,
        };
        assert_eq!(
            EnvironmentCubeSource::Skydome(skydome).coefficients(),
            [1, 1]
        );
    }

    #[test]
    fn frame_environments_compare_by_view_identity() {
        let view = Arc::new(ViewEnvironmentToken);
        let first = FrameEnvironment {
            view: view.clone(),
            background: None,
            pbr: None,
            raytrace_cube: None,
        };
        let same = FrameEnvironment {
            view,
            background: None,
            pbr: None,
            raytrace_cube: None,
        };
        let other = FrameEnvironment {
            view: Arc::new(ViewEnvironmentToken),
            background: None,
            pbr: None,
            raytrace_cube: None,
        };
        assert_eq!(first, same);
        assert_ne!(first, other);
    }
}
