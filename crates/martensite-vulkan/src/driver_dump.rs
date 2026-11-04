//! The `DumpJson` steps shared by the driver objects (`GlGroup`,
//! `GlStructure` and the `GlElement` family). The dumps keep the native class names
//! and field keys, so a dump reads like the one of the OpenGl driver. The Graphic3d base parts
//! (`Graphic3dGroup`, `Graphic3dCStructure`) are written here as well: an override of the
//! trait `dump_json` cannot call the default it replaces.
use crate::element_upload::ElementUpload;
use crate::primitive::PrimitiveSubmission;
use martensite_tkernel::standard::dump::{DumpOutput, DumpStream, dump_real_values};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkservice::font::FontRect;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::json_dump::{
    base_class, class_begin, field_dumped, field_integer, field_pointer,
};
use std::io;

/// The address of an object, as `DUMP_FIELD_VALUE_POINTER` prints it.
pub(crate) fn address_of<T: ?Sized>(object: &T) -> usize {
    std::ptr::from_ref(object).cast::<()>() as usize
}

/// `DUMP_FIELD_VALUES_DUMPED` of a handle: nothing for a null handle.
pub(crate) fn dumped_handle<T: ?Sized + RuntimeTyped>(
    stream: &mut DumpStream,
    depth: i32,
    field: &str,
    handle: &Handle<T>,
    dump: &dyn Fn(&T, &mut DumpStream, i32) -> io::Result<()>,
) -> io::Result<()> {
    match handle.get() {
        Some(value) => field_dumped(stream, depth, field, &|o, d| dump(value, o, d)),
        None => Ok(()),
    }
}

/// `DUMP_CLASS_BEGIN(<class>)` then `DUMP_BASE_CLASS(GlElement)`: the class name
/// and the element base, which is the element address.
pub(crate) fn element_begin(
    stream: &mut DumpStream,
    depth: i32,
    class: &str,
    address: usize,
) -> io::Result<()> {
    class_begin(stream, class)?;
    base_class(stream, depth, &|o, _| element_base(o, address))
}

/// `GlElement::DumpJson`: `"className": "GlElement", "this": <address>`.
pub(crate) fn element_base(stream: &mut DumpStream, address: usize) -> io::Result<()> {
    class_begin(stream, "GlElement")?;
    field_pointer(stream, "this", address)
}

/// `GlAspects::DumpJson` of an aspects element at `address` holding `aspect`.
pub(crate) fn aspects_element(
    stream: &mut DumpStream,
    depth: i32,
    address: usize,
    aspect: &Handle<dyn Graphic3dAspectsSource>,
) -> io::Result<()> {
    element_begin(stream, depth, "GlAspects", address)?;
    dumped_handle(stream, depth, "myAspect.get()", aspect, &|a, o, d| {
        a.dump_json(o, d)
    })?;
    // GlAspects keeps the aspect's shading model; an empty element is unlit.
    let shading = aspect
        .get()
        .map_or(Graphic3dTypeOfShadingModel::Unlit, |a| {
            a.aspects_base().fields.read().shading_model
        });
    field_integer(stream, "myShadingModel", shading as i32)
}

/// The GL draw mode native `GlPrimitiveArray::setDrawMode` derives from a primitive type
/// (`DRAW_MODE_NONE`, -1, without attributes or for an undefined type).
pub(crate) fn gl_draw_mode(kind: Graphic3dTypeOfPrimitiveArray, has_attributes: bool) -> i32 {
    use Graphic3dTypeOfPrimitiveArray as Kind;
    if !has_attributes {
        return -1;
    }
    match kind {
        Kind::Points => 0x0000,
        Kind::Segments => 0x0001,
        Kind::Polylines => 0x0003,
        Kind::Triangles => 0x0004,
        Kind::Trianglestrips => 0x0005,
        Kind::Trianglefans => 0x0006,
        Kind::LinesAdjacency => 0x000A,
        Kind::LineStripAdjacency => 0x000B,
        Kind::TrianglesAdjacency => 0x000C,
        Kind::TriangleStripAdjacency => 0x000D,
        Kind::Quadrangles => 0x0007,
        Kind::Quadranglestrips => 0x0008,
        Kind::Polygons => 0x0009,
        Kind::Undefined => -1,
    }
}

/// `GlRaytrace::IsRaytracedElement`: a primitive array whose GL draw mode is at least
/// `GL_TRIANGLES` (the native comparison also admits the line adjacency modes).
pub(crate) fn is_raytraced_primitive(primitive: &PrimitiveSubmission) -> bool {
    gl_draw_mode(primitive.kind, !primitive.attributes.is_null()) >= 0x0004
}

/// `GlPrimitiveArray::DumpJson` of a retained array. The GL vertex and index buffer
/// objects (`myVboIndices`, `myVboAttribs`) have no Vulkan counterpart and are not written:
/// the frame uploads the Graphic3d buffers into its own buffers.
pub(crate) fn primitive_array(
    stream: &mut DumpStream,
    depth: i32,
    address: usize,
    primitive: &PrimitiveSubmission,
    uid: usize,
) -> io::Result<()> {
    element_begin(stream, depth, "GlPrimitiveArray", address)?;
    dumped_handle(
        stream,
        depth,
        "myIndices.get()",
        &primitive.indices,
        &|b, o, d| b.dump_json(o, d),
    )?;
    dumped_handle(
        stream,
        depth,
        "myAttribs.get()",
        &primitive.attributes,
        &|b, o, d| b.dump_json(o, d),
    )?;
    dumped_handle(
        stream,
        depth,
        "myBounds.get()",
        &primitive.bounds,
        &|b, o, d| b.dump_json(o, d),
    )?;
    let mode = gl_draw_mode(primitive.kind, !primitive.attributes.is_null());
    field_integer(stream, "myDrawMode", mode)?;
    let fill = crate::render_element_pass::is_fill_primitive(
        primitive.kind,
        !primitive.attributes.is_null(),
    );
    field_integer(stream, "myIsFillType", i32::from(fill))?;
    let initialized = primitive.upload.is_initialized();
    field_integer(stream, "myIsVboInit", i32::from(initialized))?;
    martensite_tkservice::json_dump::field_unsigned(stream, "myUID", uid as u64)
}

/// `GlText::DumpJson` of a text whose glyph buffers are `upload`: the glyph texture
/// count and the formatter box of the last built layout. The per-texture GL coordinate
/// buffers (`myTCrdsVbo`) are not written; the frame uploads the glyph quads itself.
pub(crate) fn text(
    stream: &mut DumpStream,
    depth: i32,
    address: usize,
    upload: ElementUpload,
) -> io::Result<()> {
    element_begin(stream, depth, "GlText", address)?;
    let (textures, bounds) = match upload {
        ElementUpload::Text(text) => (usize::from(text.glyph_vertices > 0), text.bounds),
        _ => (0, FontRect::default()),
    };
    martensite_tkservice::json_dump::field_unsigned(stream, "myTextures.Size()", textures as u64)?;
    field_dumped(stream, depth, "&myBndBox", &|o, d| bounds.dump_json(o, d))
}

/// `DUMP_FIELD_VALUES_NUMERICAL(theOStream, "<name>", n, ...)`: `"<name>": [v1, ...]`.
pub(crate) fn field_real_values(
    stream: &mut DumpStream,
    name: &str,
    values: &[f64],
) -> io::Result<()> {
    martensite_tkernel::standard::dump::add_values_separator(stream)?;
    stream.write_fragment(format!("\"{name}\": [").as_bytes())?;
    dump_real_values(stream, values)?;
    stream.write_fragment(b"]")
}

/// The body of `Graphic3dGroup::DumpJson` for a driver group.
pub(crate) fn graphic3d_group<G: Graphic3dGroup + ?Sized>(
    group: &G,
    stream: &mut DumpStream,
    depth: i32,
) -> io::Result<()> {
    let f = group.group_base().fields.read().clone();
    class_begin(stream, "Graphic3d_Group")?;
    field_pointer(stream, "this", address_of(group))?;
    dumped_handle(
        stream,
        depth,
        "myTrsfPers.get()",
        &f.trsf_pers,
        &|t, o, d| t.dump_json(o, d),
    )?;
    field_pointer(stream, "myStructure", f.structure.object_address() as usize)?;
    field_dumped(stream, depth, "&myBounds", &|o, _| f.bounds.dump_json(o))?;
    field_integer(stream, "myIsClosed", i32::from(f.is_closed))
}

/// The body of `Graphic3dCStructure::DumpJson` for a driver structure.
pub(crate) fn graphic3d_cstructure<C: Graphic3dCStructure + ?Sized>(
    cstructure: &C,
    stream: &mut DumpStream,
    depth: i32,
) -> io::Result<()> {
    let f = cstructure.cstructure_base().fields.read().clone();
    class_begin(stream, "Graphic3d_CStructure")?;
    for group in &f.groups {
        dumped_handle(stream, depth, "aGroup.get()", group, &|g, o, d| {
            g.dump_json(o, d)
        })?;
    }
    for (name, value) in [
        ("myId", f.id),
        ("myZLayer", f.z_layer),
        ("myPriority", f.priority as i32),
        ("myPreviousPriority", f.previous_priority as i32),
    ] {
        field_integer(stream, name, value)?;
    }
    let flags = [
        ("IsInfinite", f.is_infinite),
        ("stick", f.stick),
        ("highlight", f.highlight),
        ("visible", f.visible),
        ("HLRValidation", f.hlr_validation),
        ("IsForHighlight", f.is_for_highlight),
        ("IsMutable", f.is_mutable),
        ("Is2dText", f.is_2d_text),
    ];
    for (name, value) in flags {
        field_integer(stream, name, i32::from(value))?;
    }
    field_dumped(stream, depth, "&myBndBox", &|o, _| f.bnd_box.dump_json(o))?;
    dumped_handle(stream, depth, "myTrsf.get()", &f.trsf, &|t, o, d| {
        t.dump_json(o, d)
    })?;
    dumped_handle(
        stream,
        depth,
        "myTrsfPers.get()",
        &f.trsf_pers,
        &|t, o, d| t.dump_json(o, d),
    )?;
    dumped_handle(
        stream,
        depth,
        "myClipPlanes.get()",
        &f.clip_planes,
        &|t, o, d| t.dump_json(o, d),
    )?;
    dumped_handle(
        stream,
        depth,
        "myHighlightStyle.get()",
        &f.highlight_style,
        &|t, o, d| t.dump_json(o, d),
    )?;
    field_integer(stream, "myIsCulled", i32::from(f.is_culled))?;
    field_integer(stream, "myBndBoxClipCheck", i32::from(f.bnd_box_clip_check))
}
