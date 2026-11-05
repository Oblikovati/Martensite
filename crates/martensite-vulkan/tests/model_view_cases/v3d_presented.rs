//! Shaded presentations of restored shapes from the V3d bugs group.
use super::prelude::*;

/// `bug22304`: a cleaned face shown shaded after fitting has triangles and nodes.
pub(crate) fn v3d_tcleaned_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===========",
        "OCC22304: Incorrect triangulation on face of the attached file",
        "===========",
        "",
    ]);
    let result = read_dataset_shape("OCC22304-face_976_badvisu.brep")?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    check_triangulation(&result, &PRESENT_TRIANGLES_NODES)?;
    run.dump("bug22304.png")?;
    run.finish()
}

/// `bug20373`: a B-spline sphere is shaded, has triangles, its area and a valid shape.
pub(crate) fn bspline_sphere_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC20373: Shading doesn't work on b-spline sphere",
        "============",
        "",
    ]);
    let result = read_dataset_shape("OCC20373.brep")?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    check_triangulation(&result, &PRESENT_TRIANGLES)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 9.92128e+06))?;
    check_shape_valid(&result)?;
    run.dump("bug20373.png")?;
    run.finish()
}

/// `bug4894`: a cylindrical face displayed, cleaned, then shaded has triangles and nodes.
pub(crate) fn cleaned_after_display() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC4894: Problems with sharding in vdisplay",
        "============",
        "",
    ]);
    let result = read_dataset_shape("bug4894_face_cyl")?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    remove_triangulation(&result)?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    check_triangulation(&result, &PRESENT_TRIANGLES_NODES)?;
    run.dump("bug4894.png")?;
    run.finish()
}

/// `bug23400`: a thin ring shaded from the top has no gap.
pub(crate) fn thin_ring_top() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC23400: Triangulation of thin ring contains gap",
        "============",
        "",
    ]);
    let a = read_dataset_shape("bug23400_thinring.brep")?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "a", &a)?;
    run.session().set_standard_projection(TOP)?;
    run.session().fit()?;
    run.dump("bug23400.png")?;
    run.finish()
}

/// `bug23886_1`..`_3`: a notched planar face shaded at a 1 degree angle, then at two own
/// deviation coefficients.
pub(crate) fn notched_face_qualities(
    input: &str,
    images: [&'static str; 3],
) -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR23886: Visualizing planar faces with 'notches' depends heavily on the deflection coefficient",
    ]);
    banner(&["============", ""]);
    let s = read_dataset_shape(input)?;
    let mut run = SessionRun::new()?;
    show(&mut run, "s", &s)?;
    run.session().fit()?;
    run.session().set_default_deviation_angle(1.);
    run.session().set_object_display_mode("s", 1)?;
    run.dump(images[0])?;
    run.session().set_shading_quality("s", 0.00001)?;
    run.dump(images[1])?;
    run.session().set_shading_quality("s", 0.01)?;
    run.dump(images[2])?;
    run.finish()
}

/// `bug288_8`: the face's length, free vertices and sub-shape counts, then shaded.
pub(crate) fn face_counts_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC288: Currently meshing (for shading) of some shapes looks incorrect",
        "================",
        "",
    ]);
    let result = read_dataset_shape("shading_105.brep")?;
    check_mass(&result, &MassCheck::near(MassKind::Length, 265.239))?;
    report_alone_vertices(&result)?;
    check_sub_shape_counts(&result, &[2, 3, 1, 1, 0, 0, 0, 0, 7])?;
    let mut run = SessionRun::new()?;
    remove_triangulation(&result)?;
    run.session().set_display_mode(1)?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.dump("bug288_8.png")?;
    run.finish()
}
