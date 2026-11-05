//! Restored shapes scaled, re-parameterized or healed, measured and shown in a check view.
use super::prelude::*;
use martensite_tkshhealing::shape_custom::ShapeCustom;
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tktopalgo::brep_lib::same_parameter_shape;

/// `bug32053`: a face scaled by 0.1 shows in its own shaded mode with exact counts.
pub(crate) fn scaled_face_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0032053: Mesh - Invalid mesh after shape scaling",
        "=======",
        "",
    ]);
    let a = read_dataset_shape("bug32053_prob_face.brep")?;
    let result = ShapeCustom::scale_shape(&a, 0.1)?;
    let mut run = SessionRun::new()?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display("result", &result, &request)?;
    run.session().fit()?;
    check_triangulation(&result, &exact_counts(168., 133.))?;
    run.dump("bug32053.png")?;
    run.finish()
}

/// `bug22989`: an edge made same-parameter keeps its length and has no free vertex.
pub(crate) fn same_parameter_edge_length() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC22989", "================", ""]);
    let result = read_dataset_shape("OCC22989-edge-nosameparameter.brep")?;
    same_parameter_shape(&result, 1.0e-7, false)?;
    check_mass(&result, &MassCheck::near(MassKind::Length, 85.0797))?;
    report_alone_vertices(&result)?;
    checkview_3d(&result, "bug22989.png")
}

/// `bug810_1`: a shape healed at precision 1e-7 and maximal tolerance 0.1 keeps its area.
pub(crate) fn healed_shape_area() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================",
        "OCC810",
        "========================",
        "",
    ]);
    let a = read_dataset_shape("OCC810.rle")?;
    let fixer = ShapeFixShape::default();
    fixer.init(a);
    fixer.set_precision(1.0e-7);
    fixer.set_max_tolerance(0.1);
    fixer.perform()?;
    let result = fixer.shape();
    check_mass(&result, &MassCheck::near(MassKind::Area, 3616.53))?;
    check_shape_valid(&result)?;
    checkview_3d(&result, "bug810_1.png")
}

/// `bug6412_1`: an empty shaded view is dumped after the area of the restored shape is checked.
pub(crate) fn area_with_empty_view() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC6412", "================", ""]);
    let result = read_dataset_shape("OCC6412.brep")?;
    let mut run = SessionRun::new()?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 3480.05))?;
    run.dump("bug6412_1.png")?;
    run.finish()
}

/// `bug23410`: a shaded box coloured green reads green, then shows in the check view.
pub(crate) fn green_box_color() -> Result<(), Box<dyn Error>> {
    use martensite_tkernel::quantity::{Color, NamedColor};
    banner(&["============", "CR23410", "============", ""]);
    let result = crate::primitive_shapes::box_sized([100., 100., 100.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.session()
        .set_color("result", Color::named(NamedColor::GREEN))?;
    run.session().check_color(128, 235, [0.00, 0.83, 0.00])?;
    checkview_in(&mut run, &result, "bug23410.png")?;
    run.finish()
}

/// `bug32601`: a cone whose stored triangulation is invalid shows shaded without automatic
/// triangulation, within the case's 30 s CPU limit.
pub(crate) fn bad_triangulation_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0032601: Foundation Classes - Poly_Connect hangs on the given incorrect triangulation",
        "============",
        "",
    ]);
    let a = read_dataset_shape("cone_bad_tri.brep")?;
    let mut run = SessionRun::new()?;
    run.session().set_auto_triangulation(false);
    show(&mut run, "a", &a)?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.finish_without_image()
}

/// `bug22822`: an open wire crossing itself is reported self-intersecting on its plane
/// (`checkselfintersection`), keeps its length and has no free vertex.
pub(crate) fn self_intersecting_open_wire() -> Result<(), Box<dyn Error>> {
    use martensite_tkbrep::topods;
    use martensite_tkernel::precision;
    use martensite_tkshhealing::shape_analysis::ShapeAnalysisWire;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    banner(&["================", "OCC22822", "================", ""]);
    let result = read_dataset_shape("OCC22822-selfIntersectingOpenWire.brep")?;
    let wire = topods::wire(&result)?;
    let face = BRepBuilderAPIMakeFace::from_wire(&wire, true)?.face()?;
    let analyser = ShapeAnalysisWire::from_wire(&wire, &face, precision::confusion())?;
    if analyser.check_self_intersection()? {
        println!("OK OCC22822");
    } else {
        println!("Faulty OCC22822");
    }
    check_mass(&result, &MassCheck::near(MassKind::Length, 579.551))?;
    report_alone_vertices(&result)?;
    check_shape_valid(&result)?;
    checkview_3d(&result, "bug22822.png")
}
