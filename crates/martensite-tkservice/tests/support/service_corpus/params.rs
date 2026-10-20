//! Graphic3d_RenderingParams, Graphic3d_ZLayerSettings and Graphic3d_PresentationAttributes.
use super::common::{Reader, end, flag, real, text};
use super::print::Output;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::AspectTypeOfHighlightMethod;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectFillArea3d, Graphic3dPresentationAttributes,
    Graphic3dPresentationAttributesMembers, Graphic3dPresentationAttributesSource,
    Graphic3dRenderingParams, Graphic3dZLayerSettings,
};
use std::io;

fn dumped(output: &Output, write: impl FnOnce(&mut DumpStream) -> io::Result<()>) {
    dumped_masked(output, write, &[]);
}

/// The dump with the object of each `"Key": {...}` replaced by `?`, as `sv_masked`.
fn dumped_masked(
    output: &Output,
    write: impl FnOnce(&mut DumpStream) -> io::Result<()>,
    keys: &[&str],
) {
    let mut stream = DumpStream::default();
    if let Err(error) = write(&mut stream) {
        return output.write(format_args!("dump-error:{error} "));
    }
    let mut dump = stream.bytes().to_vec();
    for key in keys {
        dump = masked(&dump, key);
    }
    text(output, &dump);
}

/// The end (exclusive) of the object that opens at `open`.
fn object_end(text: &[u8], open: usize) -> usize {
    let (mut end, mut depth) = (open, 0);
    loop {
        depth += match text[end] {
            b'{' => 1,
            b'}' => -1,
            _ => 0,
        };
        end += 1;
        if depth == 0 || end >= text.len() {
            return end;
        }
    }
}

pub fn masked(text: &[u8], key: &str) -> Vec<u8> {
    let head = format!("\"{key}\": {{").into_bytes();
    let mut out = text.to_vec();
    let mut from = 0;
    while let Some(found) = out[from..]
        .windows(head.len())
        .position(|w| w == head.as_slice())
    {
        let open = from + found + head.len() - 1;
        let end = object_end(&out, open);
        out.splice(open..end, *b"?");
        from += found + 1;
    }
    out
}

fn rendering(output: &Output, r: &mut Reader<'_>) -> Result<(), Failure> {
    let mut params = Graphic3dRenderingParams::new()?;
    params.resolution = r.integer() as u32;
    real(output, f64::from(params.resolution_ratio()));
    dumped_masked(
        output,
        |s| params.dump_json(s, -1),
        &["ChartSize", "Params.Params3d"],
    );
    Ok(())
}

fn layer(output: &Output, r: &mut Reader<'_>) {
    let mut s = Graphic3dZLayerSettings::default();
    s.set_name(&AsciiString::from("layer"));
    s.set_origin(r.xyz());
    s.set_culling_distance(r.real());
    s.set_culling_size(r.real());
    match r.integer() {
        1 => s.set_depth_offset_positive(),
        2 => s.set_depth_offset_negative(),
        _ => {}
    }
    s.set_immediate(r.flag());
    s.set_raytracable(r.flag());
    s.set_environment_texture(r.flag());
    s.set_enable_depth_test(r.flag());
    s.set_enable_depth_write(r.flag());
    s.set_clear_depth(r.flag());
    s.set_render_in_depth_prepass(r.flag());
    flag(output, s.has_culling_distance());
    flag(output, s.has_culling_size());
    flag(output, s.origin_transformation().is_null());
    dumped(output, |stream| s.dump_json(stream, -1));
}

fn attributes(output: &Output, r: &mut Reader<'_>) -> Result<(), Failure> {
    let a = Graphic3dPresentationAttributes::default();
    a.set_method(if r.integer() == 0 {
        AspectTypeOfHighlightMethod::Color
    } else {
        AspectTypeOfHighlightMethod::Boundbox
    });
    let c = r.xyz();
    a.set_color(Color::new(c.x(), c.y(), c.z(), ColorSpace::Rgb)?);
    a.set_transparency(r.real() as f32);
    a.set_z_layer(r.integer());
    a.set_display_mode(r.integer());
    if r.flag() {
        a.set_basic_fill_area_aspect(Handle::new(Graphic3dAspectFillArea3d::default()));
    }
    real(output, f64::from(a.transparency()));
    real(output, f64::from(a.color_rgba().alpha()));
    dumped(output, |s| a.dump_json(s, -1));
    Ok(())
}

/// `sv_params`: the rendering parameters, a Z layer or presentation attributes, with dumps.
pub fn params_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let result = match r.integer() {
        0 => rendering(output, &mut r),
        1 => {
            layer(output, &mut r);
            Ok(())
        }
        _ => attributes(output, &mut r),
    };
    super::tried(output, result);
    end(output);
}
