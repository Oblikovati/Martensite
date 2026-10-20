//! `Graphic3d_ShaderManager`: generates the standard GLSL programs (text only) for a graphics
//! API and version. The programs are the description a GLSL driver compiles; the Vulkan
//! driver does not need them.
//!
//! The native class exposes its generators as protected members for driver subclasses; here
//! they are public methods of the same names.
mod background;
mod geometry;
mod glsl;
mod gouraud;
mod grid;
mod grid_text;
mod lighting;
mod phong;
mod program_parts;
mod snippets;
mod texts_programs;
mod texts_shading;
mod unlit;
mod utility;
mod version;

pub use lighting::Graphic3dLightingOptions;
pub use phong::Graphic3dPhongVariant;

use crate::aspect::AspectGraphicsLibrary;
use martensite_tkernel::standard::{SharedState, StandardTransient};

/// GLSL syntax extensions (`Graphic3d_GlslExtension`).
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Graphic3dGlslExtension {
    /// OpenGL ES 2.0 extension `GL_OES_standard_derivatives`.
    GlOesStandardDerivatives = 0,
    /// OpenGL ES 2.0 extension `GL_EXT_shader_texture_lod`.
    GlExtShaderTextureLod = 1,
    /// OpenGL ES 2.0 extension `GL_EXT_frag_depth`.
    GlExtFragDepth = 2,
    /// OpenGL 2.0 extension `GL_EXT_gpu_shader4`.
    GlExtGpuShader4 = 3,
}
/// The number of GLSL extensions (`Graphic3d_GlslExtension_NB`).
pub const GRAPHIC3D_GLSL_EXTENSION_NB: i32 = Graphic3dGlslExtension::GlExtGpuShader4 as i32 + 1;

/// The protected fields of `Graphic3d_ShaderManager`.
#[derive(Clone, Debug, PartialEq)]
pub struct Graphic3dShaderManagerFields {
    /// The graphics API (`myGapi`).
    pub gapi: AspectGraphicsLibrary,
    /// The API version major/minor pair (`myGapiVersion`).
    pub gapi_version: [i32; 2],
    /// The available GLSL extensions (`myGlslExtensions`).
    pub glsl_extensions: [bool; GRAPHIC3D_GLSL_EXTENSION_NB as usize],
    /// Flat shading usage (`myHasFlatShading`).
    pub has_flat_shading: bool,
    /// Reverse the flat shading normal (`myToReverseDFdxSign`).
    pub to_reverse_dfdx_sign: bool,
    /// Always set `gl_PointSize` (`mySetPointSize`).
    pub set_point_size: bool,
    /// Use the RED channel instead of ALPHA (`myUseRedAlpha`).
    pub use_red_alpha: bool,
    /// Emulate depth clamping in GLSL (`myToEmulateDepthClamp`).
    pub to_emulate_depth_clamp: bool,
    /// The tracked sRGB state (`mySRgbState`).
    pub srgb_state: bool,
}

/// The standard GLSL program generator.
///
/// Example: `let manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
/// manager.set_gapi_version(3, 2); manager.get_std_program_font().get_id()` is `occt_font0000`.
pub struct Graphic3dShaderManager {
    transient: StandardTransient,
    state: SharedState<Graphic3dShaderManagerFields>,
}
martensite_tkernel::impl_transient!(
    Graphic3dShaderManager,
    "Graphic3d_ShaderManager",
    StandardTransient,
    transient
);

impl Graphic3dShaderManager {
    /// An empty manager for `gapi`: flat shading on, depth clamp emulation on, `gl_PointSize`
    /// set only on OpenGL ES, no extension; `Graphic3dShaderManager::new(gapi)`.
    pub fn new(gapi: AspectGraphicsLibrary) -> Self {
        let fields = Graphic3dShaderManagerFields {
            gapi,
            gapi_version: [0, 0],
            glsl_extensions: [false; GRAPHIC3D_GLSL_EXTENSION_NB as usize],
            has_flat_shading: true,
            to_reverse_dfdx_sign: false,
            set_point_size: gapi == AspectGraphicsLibrary::OpenGLES,
            use_red_alpha: false,
            to_emulate_depth_clamp: true,
            srgb_state: true,
        };
        Self {
            transient: StandardTransient::default(),
            state: SharedState::new(fields),
        }
    }
    /// The protected fields, as a native subclass sees them; `manager.fields().read().gapi`.
    pub fn fields(&self) -> &SharedState<Graphic3dShaderManagerFields> {
        &self.state
    }
    /// A copy of the fields the generators read.
    fn snapshot(&self) -> Graphic3dShaderManagerFields {
        self.state.read().clone()
    }
}

/// The public settings.
impl Graphic3dShaderManager {
    /// Whether the API version is at least `major.minor`; `manager.is_gapi_greater_equal(3, 0)`.
    pub fn is_gapi_greater_equal(&self, major: i32, minor: i32) -> bool {
        self.state.read().is_gapi_greater_equal(major, minor)
    }
    /// The API major version; `manager.gapi_version_major()`.
    pub fn gapi_version_major(&self) -> i32 {
        self.state.read().gapi_version[0]
    }
    /// The API minor version; `manager.gapi_version_minor()`.
    pub fn gapi_version_minor(&self) -> i32 {
        self.state.read().gapi_version[1]
    }
    /// Sets the API version; `manager.set_gapi_version(3, 2)`.
    pub fn set_gapi_version(&self, major: i32, minor: i32) {
        self.state.write().gapi_version = [major, minor];
    }
    /// Whether single-channel textures use RED instead of ALPHA; `manager.use_red_alpha()`.
    pub fn use_red_alpha(&self) -> bool {
        self.state.read().use_red_alpha
    }
    /// Sets the RED-for-ALPHA flag; `manager.set_use_red_alpha(true)`.
    pub fn set_use_red_alpha(&self, use_red_alpha: bool) {
        self.state.write().use_red_alpha = use_red_alpha;
    }
    /// Whether flat shading is available (true by default); `manager.has_flat_shading()`.
    pub fn has_flat_shading(&self) -> bool {
        self.state.read().has_flat_shading
    }
    /// Whether flat shading reverses the normal (false by default); `manager.to_reverse_dfdx_sign()`.
    pub fn to_reverse_dfdx_sign(&self) -> bool {
        self.state.read().to_reverse_dfdx_sign
    }
    /// Sets flat shading usage and normal reversal; `manager.set_flat_shading(true, false)`.
    pub fn set_flat_shading(&self, to_use: bool, to_reverse_sign: bool) {
        let mut state = self.state.write();
        state.has_flat_shading = to_use;
        state.to_reverse_dfdx_sign = to_reverse_sign;
    }
    /// Whether depth clamping is emulated (true by default); `manager.to_emulate_depth_clamp()`.
    pub fn to_emulate_depth_clamp(&self) -> bool {
        self.state.read().to_emulate_depth_clamp
    }
    /// Sets depth clamp emulation; `manager.set_emulate_depth_clamp(false)`.
    pub fn set_emulate_depth_clamp(&self, to_emulate: bool) {
        self.state.write().to_emulate_depth_clamp = to_emulate;
    }
    /// Whether `extension` is available; `manager.has_glsl_extension(extension)`.
    pub fn has_glsl_extension(&self, extension: Graphic3dGlslExtension) -> bool {
        self.state.read().glsl_extensions[extension as usize]
    }
    /// Marks `extension` available or not (`EnableGlslExtension(ext, true)` by default);
    /// `manager.enable_glsl_extension(extension, true)`.
    pub fn enable_glsl_extension(&self, extension: Graphic3dGlslExtension, to_enable: bool) {
        self.state.write().glsl_extensions[extension as usize] = to_enable;
    }
}

impl Graphic3dShaderManagerFields {
    /// `IsGapiGreaterEqual` on the fields.
    fn is_gapi_greater_equal(&self, major: i32, minor: i32) -> bool {
        self.gapi_version[0] > major
            || (self.gapi_version[0] == major && self.gapi_version[1] >= minor)
    }
    /// Whether `extension` is available.
    fn has(&self, extension: Graphic3dGlslExtension) -> bool {
        self.glsl_extensions[extension as usize]
    }
    /// Whether the API is OpenGL ES.
    fn is_gles(&self) -> bool {
        self.gapi == AspectGraphicsLibrary::OpenGLES
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
