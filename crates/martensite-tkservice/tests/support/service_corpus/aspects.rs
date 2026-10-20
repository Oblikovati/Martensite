//! Graphic3d_Aspects and its primitive classes.
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectFillArea3d, Graphic3dAspectLine3d, Graphic3dAspectMarker3d,
    Graphic3dAspectText3d, Graphic3dAspects, Graphic3dAspectsEdgeMembers,
    Graphic3dAspectsLineMembers, Graphic3dAspectsMembers, Graphic3dAspectsSource,
    Graphic3dAspectsTextMembers,
};

/// The aspects of a corpus kind: 0 Aspects, 1 FillArea3d, 2 Line3d, 3 Marker3d, 4 Text3d.
fn aspects_of(kind: i32) -> Handle<dyn Graphic3dAspectsSource> {
    match kind {
        1 => Handle::new(Graphic3dAspectFillArea3d::default()).cast(),
        2 => Handle::new(Graphic3dAspectLine3d::default()).cast(),
        3 => Handle::new(Graphic3dAspectMarker3d::default()).cast(),
        4 => Handle::new(Graphic3dAspectText3d::default()).cast(),
        _ => Handle::new(Graphic3dAspects::default()).cast(),
    }
}

/// `sv_aspects_dump Kind Depth HasFont Cutoff Angle`.
pub fn dump_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, depth, has_font) = (r.integer(), r.integer(), r.flag());
    let (cutoff, angle) = (r.real() as f32, r.real() as f32);
    let handle = aspects_of(kind);
    let Some(a) = handle.get() else {
        return end(output);
    };
    if has_font {
        a.set_text_font(Handle::new(HAsciiString::new("Arial")));
    }
    a.set_alpha_mode(a.alpha_mode(), cutoff);
    a.set_text_angle(angle);
    let mut dump = DumpStream::default();
    a.dump_json(&mut dump, depth).expect("in-memory dump");
    text(output, dump.bytes());
    int(output, a.line_type() as i32);
    int(output, i32::from(a.line_pattern()));
    real(output, f64::from(a.line_width()));
    int(output, a.marker_type() as i32);
    real(output, f64::from(a.marker_scale()));
    int(output, a.shading_model() as i32);
    int(output, a.interior_style() as i32);
    flag(output, a.is_marker_sprite());
    flag(output, a.to_draw_edges());
    end(output);
}
