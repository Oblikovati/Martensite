//! Graphic3d_ClipPlane.
use super::common::{Reader, dir, end, flag, int, pnt, real};
use super::material::material_name;
use super::print::Output;
use super::tried;
use martensite_tkernel::ncollection::{Vec3, Vec4};
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Plane3;
use martensite_tkservice::aspect::AspectHatchStyle;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsRenderMembers, Graphic3dBndBox3d, Graphic3dClipPlane, Graphic3dMaterialAspect,
    Graphic3dTexture2D, Graphic3dTextureMap,
};

fn vec4(output: &Output, v: &Vec4<f64>) {
    [v.x, v.y, v.z, v.w]
        .into_iter()
        .for_each(|value| real(output, value));
}
fn read_vec4(r: &mut Reader<'_>) -> Vec4<f64> {
    let [x, y, z, w] = [0; 4].map(|_| r.real());
    Vec4::new(x, y, z, w)
}

/// `Kind A B C D` as the native `sv_clip_make`.
fn make(kind: i32, e: Vec4<f64>) -> Result<Graphic3dClipPlane, Failure> {
    let from_plane = || Plane3::from_equation(e.x, e.y, e.z, e.w);
    match kind {
        1 => Graphic3dClipPlane::from_equation(e),
        2 => Graphic3dClipPlane::from_plane(&from_plane()?),
        _ => {
            let plane = Graphic3dClipPlane::default();
            match kind {
                0 => plane.set_equation(e)?,
                3 => plane.set_plane(&from_plane()?)?,
                _ => {}
            }
            Ok(plane)
        }
    }
}

/// The handle of a built plane, or `None` after printing the raised exception type.
fn made(
    output: &Output,
    plane: Result<Graphic3dClipPlane, Failure>,
) -> Option<Handle<Graphic3dClipPlane>> {
    let mut built = None;
    super::print::show(output, plane, |plane| built = Some(Handle::new(plane)));
    built
}

fn state(output: &Output, p: &Graphic3dClipPlane) {
    vec4(output, &p.get_equation());
    vec4(output, &p.reversed_equation());
    let plane = p.to_plane();
    pnt(output, plane.location());
    dir(output, plane.axis().direction());
    dir(output, plane.x_axis().direction());
    dir(output, plane.y_axis().direction());
    int(output, p.m_count_equation() as i32);
    flag(output, p.is_on());
    flag(output, p.is_chain());
    int(output, p.nb_chain_next_planes());
}

/// Reads the chain planes and links them after `head`; returns the last plane.
fn read_chain(
    output: &Output,
    r: &mut Reader<'_>,
    head: &Handle<Graphic3dClipPlane>,
) -> Handle<Graphic3dClipPlane> {
    let mut last = head.clone();
    for _ in 0..r.integer() {
        let equation = read_vec4(r);
        tried(
            output,
            Graphic3dClipPlane::from_equation(equation).and_then(|plane| {
                let plane = Handle::new(plane);
                Graphic3dClipPlane::set_chain_next_plane(&last, plane.clone())?;
                last = plane;
                Ok(())
            }),
        );
    }
    last
}

fn probe_points(output: &Output, r: &mut Reader<'_>, head: &Graphic3dClipPlane) {
    for _ in 0..r.integer() {
        let point = read_vec4(r);
        int(output, head.probe_point(&point) as i32);
        int(output, head.probe_point_halfspace(&point) as i32);
        flag(output, head.is_point_out_halfspace(&point));
    }
}

fn probe_boxes(output: &Output, r: &mut Reader<'_>, head: &Graphic3dClipPlane) {
    for _ in 0..r.integer() {
        let [x0, y0, z0, x1, y1, z1] = [0; 6].map(|_| r.real());
        let bounds = Graphic3dBndBox3d::from_corners(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1));
        int(output, head.probe_box(&bounds) as i32);
        flag(output, head.probe_box_touch(&bounds));
        int(output, head.probe_box_halfspace(&bounds) as i32);
        flag(output, head.is_box_full_out_halfspace(&bounds));
        flag(output, head.is_box_full_in_halfspace(&bounds));
        int(output, head.probe_box_max_point_halfspace(&bounds) as i32);
    }
}

/// `sv_clip`: a plane, a chain after it, its point and box tests, then the chain cut.
pub fn clip_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    let Some(head) = made(output, make(kind, read_vec4(&mut r))) else {
        return end(output);
    };
    state(output, &head);
    let last = read_chain(output, &mut r, &head);
    int(output, head.nb_chain_next_planes());
    flag(output, head.is_chain());
    int(output, last.nb_chain_next_planes());
    flag(output, last.chain_previous_plane().is_null());
    tried(output, last.set_on(true));
    probe_points(output, &mut r, &head);
    probe_boxes(output, &mut r, &head);
    cut_chain(output, &head);
    end(output);
}

fn cut_chain(output: &Output, head: &Handle<Graphic3dClipPlane>) {
    let second = head.chain_next_plane();
    tried(
        output,
        Graphic3dClipPlane::set_chain_next_plane(head, Handle::default()),
    );
    int(output, head.nb_chain_next_planes());
    flag(output, head.is_chain());
    int(output, head.m_count_equation() as i32);
    if let Some(second) = second.get() {
        flag(output, second.chain_previous_plane().is_null());
        tried(output, second.set_on(false));
        flag(output, second.is_on());
    }
}

fn capping_state(output: &Output, p: &Graphic3dClipPlane) {
    let color = p.capping_color();
    [color.red(), color.green(), color.blue()]
        .into_iter()
        .for_each(|value| real(output, value));
    int(output, p.capping_material().name() as i32);
    flag(output, p.is_hatch_on());
    let has_hatch = p.capping_custom_hatch().is_some();
    flag(output, has_hatch);
    if has_hatch {
        super::print::show(output, p.capping_hatch(), |style| int(output, style as i32));
    }
    flag(output, p.capping_texture().is_some());
    flag(output, p.capping_aspect().to_map_texture());
    flag(output, p.to_use_object_material());
    flag(output, p.to_use_object_texture());
    flag(output, p.to_use_object_shader());
    flag(output, p.to_use_object_properties());
    flag(output, p.is_capping());
    int(output, p.m_count_aspect() as i32);
    int(output, p.m_count_equation() as i32);
}

/// The hatch style of a corpus value (0..=12).
fn hatch(value: i32) -> AspectHatchStyle {
    use AspectHatchStyle as H;
    [
        H::Solid,
        H::GridDiagonal,
        H::GridDiagonalWide,
        H::Grid,
        H::GridWide,
        H::Diagonal45,
        H::Diagonal135,
        H::Horizontal,
        H::Vertical,
        H::Diagonal45Wide,
        H::Diagonal135Wide,
        H::HorizontalWide,
        H::VerticalWide,
    ][value as usize]
}

fn capping_texture(with: bool) -> Handle<dyn Graphic3dTextureMap> {
    if !with {
        return Handle::default();
    }
    let texture = Graphic3dTexture2D::from_file(&AsciiString::from("capping.png"));
    Handle::new(texture).cast::<dyn Graphic3dTextureMap>()
}

/// `R G B`: SetCappingColor of an RGB color (an out-of-range one raises).
fn set_color(output: &Output, r: &mut Reader<'_>, p: &Graphic3dClipPlane) {
    let [red, green, blue] = [0; 3].map(|_| r.real());
    tried(
        output,
        Color::new(red, green, blue, ColorSpace::Rgb).and_then(|c| p.set_capping_color(c)),
    );
}

/// `Flag On`: sets the object material (0), texture (1) or shader (2) flag.
fn set_flag(r: &mut Reader<'_>, p: &Graphic3dClipPlane) {
    let (which, on) = (r.integer(), r.flag());
    match which {
        0 => p.set_use_object_material(on),
        1 => p.set_use_object_texture(on),
        _ => p.set_use_object_shader(on),
    }
}

/// Applies one capping operation of the corpus; returns the plane to continue with.
fn capping_op(
    output: &Output,
    r: &mut Reader<'_>,
    p: Handle<Graphic3dClipPlane>,
) -> Handle<Graphic3dClipPlane> {
    match r.integer() {
        1 => set_color(output, r, &p),
        2 => p.set_capping_material(Graphic3dMaterialAspect::new(material_name(r.integer()))),
        3 => p.set_capping_hatch(hatch(r.integer())),
        4 => p.set_capping_hatch_on(),
        5 => p.set_capping_hatch_off(),
        6 => set_flag(r, &p),
        7 => p.set_capping(r.flag()),
        8 => return p.clone_plane(),
        9 => tried(output, p.set_capping_texture(capping_texture(r.flag()))),
        _ => tried(output, p.set_equation(read_vec4(r))),
    }
    p
}

/// `sv_clip_capping`: capping operations on a default plane, the state after each.
pub fn capping_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut plane = Handle::new(Graphic3dClipPlane::default());
    for _ in 0..r.integer() {
        plane = capping_op(output, &mut r, plane);
        capping_state(output, &plane);
    }
    end(output);
}
