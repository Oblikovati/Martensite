//! Restored shapes triangulated by their shaded presentation, counted and dumped.
use super::prelude::*;

/// A restored shape checked, cleaned, displayed, switched to its own shaded mode, optionally
/// fitted, counted and dumped.
struct OwnShadedCase {
    input: &'static str,
    check_shape: bool,
    fit: bool,
    counts: TriangulationExpectation,
    image: &'static str,
}

impl OwnShadedCase {
    /// The checked and fitted form shared by most regressions.
    const fn fitted(
        input: &'static str,
        counts: TriangulationExpectation,
        image: &'static str,
    ) -> Self {
        Self {
            input,
            check_shape: true,
            fit: true,
            counts,
            image,
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let result = read_dataset_shape(self.input)?;
        if self.check_shape {
            check_shape_valid(&result)?;
        }
        remove_triangulation(&result)?;
        let mut run = SessionRun::new()?;
        show(&mut run, "result", &result)?;
        run.session().set_object_display_mode("result", 1)?;
        if self.fit {
            run.session().fit()?;
        }
        check_triangulation(&result, &self.counts)?;
        run.dump(self.image)?;
        run.finish()
    }
}

/// `bug19_2`: a cleaned shape in its own shaded mode, not fitted, has 50 triangles.
pub(crate) fn unfitted_own_shaded() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC19", "================", ""]);
    let case = OwnShadedCase {
        fit: false,
        ..OwnShadedCase::fitted("OCC19.brep", exact_counts(50., 52.), "bug19_2.png")
    };
    case.run()
}

/// `bug288_2`: the shaded face is not meshed into the faulty 1170 triangles and 647 nodes.
pub(crate) fn own_shaded_not_faulty() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================",
        " OCC288 ",
        "(case 2)",
        "========================",
        "",
    ]);
    let counts = TriangulationExpectation {
        triangles: CountExpectation::Not(1170.),
        nodes: CountExpectation::Not(647.),
        ..TriangulationExpectation::default()
    };
    OwnShadedCase::fitted("shading_105.brep", counts, "bug288_2.png").run()
}

/// `bug364`: a face invisible in shading is meshed into 96 triangles.
pub(crate) fn invisible_face_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================",
        "BUC61000: Face is invisible in the shading mode in the 3D View.",
    ]);
    banner(&[" OCC364 ", "========================", ""]);
    OwnShadedCase::fitted("OCC364.brep", exact_counts(96., 96.), "bug364.png").run()
}

/// `bug258_1`: shading of the first restored face.
pub(crate) fn shading_face_143() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================",
        " OCC258 ",
        "(case 1)",
        "========================",
        "",
    ]);
    OwnShadedCase::fitted("shading_143.brep", exact_counts(78., 82.), "bug258_1.png").run()
}

/// `bug258_2`: shading of the second restored face.
pub(crate) fn shading_face_144() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========================",
        " OCC258 ",
        "(case 2)",
        "========================",
        "",
    ]);
    OwnShadedCase::fitted("shading_144.brep", exact_counts(78., 80.), "bug258_2.png").run()
}

/// `bug6412`: a cleaned shape shown in its own shaded mode has triangles and nodes.
pub(crate) fn own_shaded_present() -> Result<(), Box<dyn Error>> {
    banner(&["========", "OCC6412", "========", ""]);
    let case = OwnShadedCase {
        check_shape: false,
        ..OwnShadedCase::fitted("OCC6412.brep", PRESENT_TRIANGLES_NODES, "bug6412.png")
    };
    case.run()
}

/// `bug31258`: a cleaned shape shown shaded is triangulated by the presentation with exact counts.
pub(crate) fn restored_shaded_counts() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0031258: Mesh - the reference implementation 7.4.0 VIS get wrong render data",
        "=======",
        "",
    ]);
    let result = read_dataset_shape("bug31258.brep")?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    check_triangulation(&result, &exact_counts(166., 102.))?;
    run.dump("bug31258.png")?;
    run.finish()
}

/// `bug26716`: a revolved shape with a shifted wire is meshed by its shaded presentation.
pub(crate) fn shifted_wire_shaded() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC26716: Mesher cannot create a mesh of brep model due to ShapeFix_Wire::FixShifted()",
        "============",
        "",
    ]);
    let result = read_dataset_shape("bug26716_rev.brep")?;
    check_shape_valid(&result)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.dump("bug26716.png")?;
    check_triangulation(&result, &PRESENT_TRIANGLES)?;
    run.finish()
}

/// `bug26359`: a parabolic face shown shaded in an axonometric view has a deflection.
pub(crate) fn parabola_face_shaded() -> Result<(), Box<dyn Error>> {
    banner(&["============", "CR26359", "============", ""]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    let p = read_dataset_shape("bug26359_parabola.brep")?;
    show(&mut run, "p", &p)?;
    run.session().fit()?;
    let counts = TriangulationExpectation {
        deflection: CountExpectation::Present,
        ..PRESENT_TRIANGLES_NODES
    };
    check_triangulation(&p, &counts)?;
    run.dump("bug26359.png")?;
    run.finish()
}
