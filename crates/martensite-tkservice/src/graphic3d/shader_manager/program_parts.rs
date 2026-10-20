//! Shared steps of the program generators: variables, stage attachment, flag tests, clipping
//! and transparency outputs.
use super::snippets::{
    THE_FRAG_CLIP_CHAINS_2, THE_FRAG_CLIP_CHAINS_N, THE_FRAG_CLIP_PLANES_1, THE_FRAG_CLIP_PLANES_2,
    THE_FRAG_CLIP_PLANES_N,
};
use crate::graphic3d::{
    Graphic3dRenderTransparentMethod, Graphic3dShaderFlags as Flags, Graphic3dShaderObject,
    Graphic3dShaderObjectVariable, Graphic3dShaderProgram, Graphic3dTypeOfShaderObject,
};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;

/// The vertex stage bit (`Graphic3d_TOS_VERTEX`).
pub(super) const VERT: i32 = Graphic3dTypeOfShaderObject::Vertex as i32;
/// The geometry stage bit (`Graphic3d_TOS_GEOMETRY`).
pub(super) const GEOM: i32 = Graphic3dTypeOfShaderObject::Geometry as i32;
/// The fragment stage bit (`Graphic3d_TOS_FRAGMENT`).
pub(super) const FRAG: i32 = Graphic3dTypeOfShaderObject::Fragment as i32;
/// The vertex and fragment stage bits.
pub(super) const VERT_FRAG: i32 = VERT | FRAG;

/// The uniforms and stage in/outs of a program under construction (`ShaderVariableList`s).
#[derive(Default)]
pub(super) struct Declarations {
    /// The uniform declarations.
    pub(super) uniforms: Vec<Graphic3dShaderObjectVariable>,
    /// The stage in/out declarations.
    pub(super) in_outs: Vec<Graphic3dShaderObjectVariable>,
}
impl Declarations {
    /// Appends a uniform `name` active in `stages`.
    pub(super) fn uniform(&mut self, name: &str, stages: i32) {
        self.uniforms
            .push(Graphic3dShaderObjectVariable::new(name, stages));
    }
    /// Appends a stage in/out `name` active in `stages`.
    pub(super) fn in_out(&mut self, name: &str, stages: i32) {
        self.in_outs
            .push(Graphic3dShaderObjectVariable::new(name, stages));
    }
}

/// Whether any bit of `flag` is set in `bits`.
pub(super) fn any(bits: i32, flag: Flags) -> bool {
    bits & flag.0 != 0
}
/// Whether every bit of `flag` is set in `bits`.
pub(super) fn all(bits: i32, flag: Flags) -> bool {
    bits & flag.0 == flag.0
}
/// Whether the texture bits of `bits` are exactly `flag`.
pub(super) fn textures_are(bits: i32, flag: Flags) -> bool {
    bits & Flags::HAS_TEXTURES.0 == flag.0
}

/// A new program handle (`new Graphic3d_ShaderProgram()`).
pub(super) fn new_program() -> Handle<Graphic3dShaderProgram> {
    Handle::new(Graphic3dShaderProgram::default())
}
/// The program behind a handle made by [`new_program`].
pub(super) fn program(handle: &Handle<Graphic3dShaderProgram>) -> &Graphic3dShaderProgram {
    handle.get().expect("generated programs are never null")
}

/// `CreateFromSource(source, kind, uniforms, inOuts, inName, outName, nbGeomVerts)` attached
/// to `handle`; an empty source attaches nothing.
pub(super) fn attach_stage(
    handle: &Handle<Graphic3dShaderProgram>,
    source: String,
    kind: Graphic3dTypeOfShaderObject,
    declarations: &Declarations,
    names: (&str, &str, i32),
) {
    let mut text = AsciiString::from(source.as_str());
    let shader = Graphic3dShaderObject::create_from_stage_source(
        &mut text,
        kind,
        &declarations.uniforms,
        &declarations.in_outs,
        &AsciiString::from(names.0),
        &AsciiString::from(names.1),
        names.2,
    );
    program(handle).attach_shader(&shader);
}
/// The vertex and fragment stages without interface block names.
pub(super) fn attach_pair(
    handle: &Handle<Graphic3dShaderProgram>,
    vertex: String,
    fragment: String,
    declarations: &Declarations,
) {
    attach_stage(
        handle,
        vertex,
        Graphic3dTypeOfShaderObject::Vertex,
        declarations,
        ("", "", 0),
    );
    let kind = Graphic3dTypeOfShaderObject::Fragment;
    attach_stage(handle, fragment, kind, declarations, ("", "", 0));
}
/// The vertex, geometry (`geomIn`/`geomOut` blocks) and fragment stages; three geometry input
/// vertices when the geometry source is not empty.
pub(super) fn attach_with_geometry(
    handle: &Handle<Graphic3dShaderProgram>,
    sources: [String; 3],
    declarations: &Declarations,
) {
    let [vertex, geometry, fragment] = sources;
    let verts = if geometry.is_empty() { 0 } else { 3 };
    let kinds = [
        Graphic3dTypeOfShaderObject::Vertex,
        Graphic3dTypeOfShaderObject::Geometry,
    ];
    attach_stage(handle, vertex, kinds[0], declarations, ("", "", verts));
    attach_stage(
        handle,
        geometry,
        kinds[1],
        declarations,
        ("geomIn", "geomOut", verts),
    );
    let kind = Graphic3dTypeOfShaderObject::Fragment;
    attach_stage(handle, fragment, kind, declarations, ("", "", verts));
}

/// No default sampler, lights or shadow maps, and `clip_planes` clipping planes.
pub(super) fn set_unlit_limits(handle: &Handle<Graphic3dShaderProgram>, clip_planes: i32) {
    set_limits(handle, (0, 0, clip_planes));
}
/// No default sampler, and the (lights, shadow maps, clipping planes) limits.
pub(super) fn set_limits(handle: &Handle<Graphic3dShaderProgram>, limits: (i32, i32, i32)) {
    let program = program(handle);
    program.set_default_sampler(false);
    program.set_nb_lights_max(limits.0);
    program.set_nb_shadow_maps(limits.1);
    program.set_nb_clip_planes_max(limits.2);
}

/// The number of clipping planes and the fragment clipping code of the clip bits of `bits`.
pub(super) fn clip_planes_fragment(bits: i32) -> (i32, &'static str) {
    let chains = any(bits, Flags::CLIP_CHAINS);
    if all(bits, Flags::CLIP_PLANES_N) {
        let code = if chains {
            THE_FRAG_CLIP_CHAINS_N
        } else {
            THE_FRAG_CLIP_PLANES_N
        };
        return (Graphic3dShaderProgram::THE_MAX_CLIP_PLANES_DEFAULT, code);
    }
    if any(bits, Flags::CLIP_PLANES1) {
        return (1, THE_FRAG_CLIP_PLANES_1);
    }
    if any(bits, Flags::CLIP_PLANES2) {
        return (
            2,
            if chains {
                THE_FRAG_CLIP_CHAINS_2
            } else {
                THE_FRAG_CLIP_PLANES_2
            },
        );
    }
    (0, "")
}

/// The fragment outputs of depth peeling (3) or blended (2) order-independent transparency.
pub(super) fn apply_oit_output(handle: &Handle<Graphic3dShaderProgram>, bits: i32) {
    let program = program(handle);
    if any(bits, Flags::OIT_DEPTH_PEELING) {
        program.set_nb_fragment_outputs(3);
        program.set_oit_output(Graphic3dRenderTransparentMethod::DepthPeelingOit);
    } else if any(bits, Flags::WRITE_OIT) {
        program.set_nb_fragment_outputs(2);
        program.set_oit_output(Graphic3dRenderTransparentMethod::BlendOit);
    }
}

/// The alpha test flag and the texture set bits shared by the lit and unlit programs.
pub(super) fn set_alpha_test(handle: &Handle<Graphic3dShaderProgram>, bits: i32) {
    program(handle).set_alpha_test(any(bits, Flags::ALPHA_TEST));
}
