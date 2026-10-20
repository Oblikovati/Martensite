//! The geometry stage of mesh edges and the point sprite colour functions.
use super::Graphic3dShaderManager;
use super::program_parts::{Declarations, FRAG, GEOM, VERT_FRAG, all, any};
use super::texts_shading::{GEOM_EDGE_HEIGHTS, GEOM_VIEWPORT_TRANSFORM};
use super::version::text;
use crate::graphic3d::Graphic3dShaderFlags as Flags;
use martensite_tkernel::tcollection::AsciiString;

/// `gl_PointCoord` flipped vertically (`THE_VEC2_glPointCoord`).
const POINT_COORD: &str = "vec2 (gl_PointCoord.x, 1.0 - gl_PointCoord.y)";

/// The mesh edge uniforms and the `EdgeDistance` in/out of the geometry stage.
fn declare_mesh_edges(declarations: &mut Declarations) {
    declarations.uniform("vec4 occViewport", GEOM);
    declarations.uniform("bool occIsQuadMode", GEOM);
    declarations.uniform("float occLineWidth", GEOM);
    declarations.uniform("float occLineWidth", FRAG);
    declarations.uniform("float occLineFeather", FRAG);
    declarations.uniform("vec4 occWireframeColor", FRAG);
    declarations.in_out("vec3 EdgeDistance", GEOM | FRAG);
}

/// The copies of the vertex-to-fragment variables for vertex `index` (whole arrays by name).
fn pass_through(declarations: &Declarations, index: usize) -> String {
    let mut source = String::new();
    for variable in declarations
        .in_outs
        .iter()
        .filter(|v| v.stages == VERT_FRAG)
    {
        let name = variable.name.token(" ", 2);
        let name = if name.as_bytes().last() == Some(&b']') {
            name.token("[", 1)
        } else {
            name
        };
        let name = text(&name);
        source.push_str(&format!("\n  geomOut.{name} = geomIn[{index}].{name};"));
    }
    source
}

/// The edge distances of vertex `index`.
const EDGE_DISTANCES: [&str; 3] = [
    "\n  EdgeDistance = vec3 (aHeightABC[0], 0.0, aQuadModeHeightC);",
    "\n  EdgeDistance = vec3 (0.0, aHeightABC[1], aQuadModeHeightC);",
    "\n  EdgeDistance = vec3 (0.0, 0.0, aHeightABC[2]);",
];

/// `prepareGeomMainSrc`: the geometry stage that passes the triangle through, with the edge
/// distances of mesh edges; empty without a geometry stage. Adds the mesh edge declarations.
pub(super) fn prepare_geom_main_src(declarations: &mut Declarations, bits: i32) -> String {
    if !any(bits, Flags::NEEDS_GEOM_SHADER) {
        return String::new();
    }
    let mesh_edges = any(bits, Flags::MESH_EDGES);
    let mut source = String::new();
    if mesh_edges {
        declare_mesh_edges(declarations);
        source.push_str(GEOM_VIEWPORT_TRANSFORM);
    }
    source.push_str("\nvoid main()\n{");
    if mesh_edges {
        source.push_str(GEOM_EDGE_HEIGHTS);
    }
    for (index, distances) in EDGE_DISTANCES.iter().enumerate() {
        source.push_str(&pass_through(declarations, index));
        if mesh_edges {
            source.push_str(distances);
        }
        source.push_str(&format!(
            "\n  gl_Position = gl_in[{index}].gl_Position;\n  EmitVertex();"
        ));
    }
    source.push_str("\n  EndPrimitive();\n}");
    source
}

impl Graphic3dShaderManager {
    /// `pointSpriteAlphaSrc`: `getAlpha()` from the point sprite texture (its RED channel for
    /// alpha sprites with RED-for-ALPHA); `manager.point_sprite_alpha_src(bits)`.
    pub fn point_sprite_alpha_src(&self, bits: i32) -> AsciiString {
        let red = self.snapshot().use_red_alpha;
        AsciiString::from(point_sprite_alpha(bits, red).as_str())
    }
    /// `pointSpriteShadingSrc`: `getColor()` of a point sprite over `base_color` (empty without
    /// a sprite); `manager.point_sprite_shading_src(&"occColor".into(), bits)`.
    pub fn point_sprite_shading_src(&self, base_color: &AsciiString, bits: i32) -> AsciiString {
        let red = self.snapshot().use_red_alpha;
        AsciiString::from(point_sprite_shading(&text(base_color), bits, red).as_str())
    }
}

/// `pointSpriteAlphaSrc` with the RED-for-ALPHA setting `red`.
pub(super) fn point_sprite_alpha(bits: i32, red: bool) -> String {
    let channel = if all(bits, Flags::POINT_SPRITE_A) && red {
        'r'
    } else {
        'a'
    };
    format!(
        "\nfloat getAlpha(void) {{ return occTexture2D(occSamplerPointSprite, {POINT_COORD}).{channel}; }}"
    )
}

/// `pointSpriteShadingSrc` with the RED-for-ALPHA setting `red`.
pub(super) fn point_sprite_shading(base_color: &str, bits: i32, red: bool) -> String {
    let head = format!("\nvec4 getColor(void)\n{{\n  vec4 aColor = {base_color};");
    let tail = "\n  if (aColor.a <= 0.1) discard;\n  return aColor;\n}";
    if all(bits, Flags::POINT_SPRITE_A) {
        let alpha = point_sprite_alpha(bits, red);
        return format!("{alpha}{head}\n  aColor.a = getAlpha();{tail}");
    }
    if all(bits, Flags::POINT_SPRITE) {
        return format!(
            "{head}\n  aColor = occTexture2D(occSamplerPointSprite, {POINT_COORD}) * aColor;{tail}"
        );
    }
    String::new()
}
