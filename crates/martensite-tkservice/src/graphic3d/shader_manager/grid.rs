//! `getGridProgram`: the GPU grid on an arbitrary plane, intersected per fragment in view
//! space from a screen-covering triangle.
use super::Graphic3dShaderManager;
use super::grid_text::{GRID_FRAG, GRID_VERT};
use super::program_parts::{Declarations, FRAG, VERT_FRAG, attach_pair, new_program, program};
use crate::aspect::AspectGraphicsLibrary;
use crate::graphic3d::Graphic3dShaderProgram;
use martensite_tkernel::standard::Handle;

/// The fragment uniforms of the grid program, in declaration order.
const GRID_UNIFORMS: [&str; 29] = [
    "float uScaleX",
    "float uScaleY",
    "float uThickness",
    "vec3 uColor",
    "vec3 uAccentColor",
    "float uAccentScaleX",
    "float uAccentScaleY",
    "float uAccentAngularScale",
    "int uIsDrawAxis",
    "vec3 uPlaneOriginView",
    "vec3 uPlaneRefView",
    "vec3 uPlaneXView",
    "vec3 uPlaneYView",
    "vec3 uPlaneNView",
    "int uGridType",
    "float uAngularScale",
    "int uDrawMode",
    "int uIsPerspective",
    "int uIsZeroToOneDepth",
    "int uIsBackground",
    "float uParallelTolerance",
    "vec2 uLocalOriginShift",
    "vec2 uAccentLocalOriginShift",
    "float uRadialOriginShift",
    "float uAccentRadialOriginShift",
    "vec3 uBounds",
    "int uIsBoundFade",
    "vec2 uArcRange",
    "int uArcBounded",
];

impl Graphic3dShaderManager {
    /// `getGridProgram`: the grid program (`gl_VertexID` and `fwidth` need OpenGL 3.0 or
    /// OpenGL ES 3.0; no id or limits are set); `manager.get_grid_program()`.
    pub fn get_grid_program(&self) -> Handle<Graphic3dShaderProgram> {
        let fields = self.snapshot();
        let mut declarations = Declarations::default();
        // Pass only NDC.xy to the fragment; each fragment reconstructs its view-space ray.
        declarations.in_out("vec2 vNdc", VERT_FRAG);
        GRID_UNIFORMS
            .iter()
            .for_each(|uniform| declarations.uniform(uniform, FRAG));
        let handle = new_program();
        if fields.gapi == AspectGraphicsLibrary::OpenGL {
            let header = if fields.is_gapi_greater_equal(3, 2) {
                "#version 150"
            } else {
                "#version 130"
            };
            program(&handle).set_header(header.into());
        } else if fields.is_gapi_greater_equal(3, 0) {
            program(&handle).set_header("#version 300 es".into());
        }
        attach_pair(&handle, GRID_VERT.into(), GRID_FRAG.into(), &declarations);
        handle
    }
}
