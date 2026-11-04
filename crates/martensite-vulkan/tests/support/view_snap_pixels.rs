//! Snap-dependent pixels of a view's next frame (`snap_pixels`), for the cross-driver snap
//! coverage rule of the native dump graders (D2, AMD only, accepted 2026-10-28).
use super::snap_pixels::snap_pixels;
use super::tie_edges::SubmittedDraws;
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::v3d::V3dViewSource;
use martensite_vulkan::Graphic3dRenderView;
use std::collections::BTreeSet;

/// Redraw `view` once and list the pixels whose centre lies within the snap bound of a drawn
/// triangle edge; `view_snap_pixels(&scene.view, (128, 128))`.
pub fn view_snap_pixels(
    view: &Handle<dyn V3dViewSource>,
    size: (usize, usize),
) -> BTreeSet<(i32, i32)> {
    let render: Handle<Graphic3dRenderView> = view.get().unwrap().view().cast();
    let render = render
        .get()
        .expect("a Vulkan render view behind the V3d view");
    let record = SubmittedDraws::install(render);
    view.redraw();
    render.set_frame_hooks(None);
    let (width, height) = (size.0 as f64, size.1 as f64);
    snap_pixels(
        &record.triangles(width, height),
        [size.0 as i32, size.1 as i32],
    )
}
