//! Graphic3d_CullingTool.
use super::camera::read_camera;
use super::common::{Reader, end, flag, int, real};
use super::print::Output;
use martensite_tkernel::ncollection::{Mat4, Vec3, Vec4};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{Graphic3dCullingContext, Graphic3dCullingTool};

fn vec3(output: &Output, v: &Vec3<f64>) {
    [v.x, v.y, v.z]
        .into_iter()
        .for_each(|value| real(output, value));
}

fn boxes(
    output: &Output,
    r: &mut Reader<'_>,
    tool: &Graphic3dCullingTool,
    ctx: &Graphic3dCullingContext,
) {
    for _ in 0..r.integer() {
        let (a, b) = (r.xyz(), r.xyz());
        let (min, max) = (
            Vec3::new(a.x(), a.y(), a.z()),
            Vec3::new(b.x(), b.y(), b.z()),
        );
        flag(output, tool.is_out_frustum(&min, &max, None));
        let mut in1 = true;
        flag(output, tool.is_out_frustum(&min, &max, Some(&mut in1)));
        flag(output, in1);
        let mut in2 = true;
        flag(output, tool.is_too_distant(ctx, &min, &max, Some(&mut in2)));
        flag(output, in2);
        flag(output, tool.is_too_small(ctx, &min, &max));
        let mut in3 = true;
        flag(output, tool.is_culled(ctx, &min, &max, Some(&mut in3)));
        flag(output, in3);
        flag(output, tool.is_culled(ctx, &min, &max, None));
    }
}

/// `sv_culling`: a culling tool over a camera, its context and the box tests.
pub fn culling_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let camera = Handle::new(read_camera(output, &mut r));
    let t = r.xyz();
    let (w, h) = (r.integer(), r.integer());
    let [ratio, distance, size] = [0; 3].map(|_| r.real());
    let mut model = Mat4::<f64>::identity();
    (0..3).for_each(|row| model.set_value(row, 3, [t.x(), t.y(), t.z()][row]));
    let mut tool = Graphic3dCullingTool::default();
    tool.set_view_volume(&camera, &model);
    tool.set_viewport_size(w, h, ratio);
    tool.cache_clip_pts_projections();
    let mut ctx = Graphic3dCullingContext::default();
    tool.set_culling_distance(&mut ctx, distance);
    tool.set_culling_size(&mut ctx, size);
    real(output, ctx.dist_cull);
    real(output, ctx.size_cull2);
    int(output, tool.viewport_width());
    int(output, tool.viewport_height());
    vec3(output, tool.camera_eye());
    vec3(output, tool.camera_direction());
    real(
        output,
        tool.signed_plane_point_distance(
            &Vec4::new(1.0, 2.0, 2.0, -3.0),
            &Vec4::new(t.x(), t.y(), t.z(), 1.0),
        ),
    );
    boxes(output, &mut r, &tool, &ctx);
    end(output);
}
