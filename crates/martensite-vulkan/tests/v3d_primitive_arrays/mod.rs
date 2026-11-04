//! All four buffer/layout states recreate the actual arrays, without claiming two Vulkan storage paths.
mod arrays;
#[cfg(test)]
mod cpu;
mod scene;
mod specification;
mod unused_check;
use crate::session_run::{SessionRun, banner};
type Outcome = Result<(), Box<dyn std::error::Error>>;
const IMAGES: [&str; 4] = [
    "bug23226_vbooff.png",
    "bug23226_vbooff_noninter.png",
    "bug23226_vboon.png",
    "bug23226_vboon_noninter.png",
];
/// Execute every array/bounds/index command and both final color assertions; `primitive_layouts()?`.
pub(crate) fn primitive_layouts() -> Outcome {
    heading();
    let _status = 0;
    let mut run = SessionRun::new()?;
    capture_layouts(&mut run)?;
    run.session().check_color(200, 200, [0., 1., 0.])?;
    let stat = run.session().check_color(220, 200, [0., 1., 1.])?;
    println!("BUG {} OCC22583", if stat { "OK" } else { "FAULTY" });
    run.finish()
}
fn capture_layouts(run: &mut SessionRun) -> Outcome {
    for (index, (enabled, non_interleaved)) in
        [(false, false), (false, true), (true, false), (true, true)]
            .into_iter()
            .enumerate()
    {
        scene::stage(run.session(), enabled, non_interleaved)?;
        run.dump(IMAGES[index])?;
    }
    Ok(())
}
fn heading() {
    banner(&[
        "============",
        "OCC23226: Extend GlContext to store map of shared GPU resources",
        "this test performs automatic test of primitives array objects by pixel checking",
        "this test ALSO DUMPS two result images (the primitives on the images should have",
        "same contours, WIREFRAME OBJECTS ARE IN YELLOW-RED COLORS, SHADED OBJECTS ARE BLUE-GREEN COLORS)",
        "============",
        "",
    ]);
}
