//! Shapes meshed explicitly by the incremental mesher, counted, then shown and dumped.
use super::prelude::*;

/// `bug26372`: a dataset shape meshed at 0.1 has exact counts, then shows shaded.
pub(crate) fn incmesh_shaded_counts() -> Result<(), Box<dyn Error>> {
    banner(&["========", "OCC26372", "========", ""]);
    let result = read_dataset_shape("bug26372_RP.brep")?;
    mesh_shape(&result, &MeshRequest::new(0.1))?;
    check_triangulation(&result, &exact_counts(1041., 530.))?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.dump("bug26372.png")?;
    run.finish()
}

/// `bug24127`: a B-spline face meshed at 1 has 20 triangles, 21 nodes and a known deflection.
pub(crate) fn bspline_face_deflection() -> Result<(), Box<dyn Error>> {
    banner(&["================", "CR24127", "================", ""]);
    let f = bspline_face(&NARROW_SPLINE)?;
    mesh_shape(&f, &MeshRequest::new(1.))?;
    let expected = TriangulationExpectation {
        // The regression literal 0.70238336519888955 rounds to this double.
        deflection: near(0.7023833651988896, 1.0e-3, 0.01),
        ..exact_counts(20., 21.)
    };
    check_triangulation(&f, &expected)?;
    let mut run = SessionRun::new()?;
    show(&mut run, "f", &f)?;
    run.session().fit()?;
    run.dump("bug24127.png")?;
    run.finish()
}

/// `bug24959_1`: the B-spline face meshed at 1 keeps its deflection within 1, shown from top.
pub(crate) fn bspline_face_bounded() -> Result<(), Box<dyn Error>> {
    bspline_face_top(&NARROW_SPLINE, 1., "bug24959_1.png")
}

/// `bug24959_2`: a longer B-spline face meshed at 1 keeps its deflection within its bound.
pub(crate) fn long_bspline_face_bounded() -> Result<(), Box<dyn Error>> {
    bspline_face_top(&LONG_SPLINE, 2.4039063417856825, "bug24959_2.png")
}

/// Mesh the face of `spline` at 1, bound its deflection, show it shaded from the top.
fn bspline_face_top(
    spline: &SplineSurface,
    bound: f64,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC24959", "================", ""]);
    let result = bspline_face(spline)?;
    mesh_shape(&result, &MeshRequest::new(1.))?;
    let expected = TriangulationExpectation {
        max_deflection: Some(bound),
        ..TriangulationExpectation::default()
    };
    check_triangulation(&result, &expected)?;
    let mut run = SessionRun::new()?;
    show(&mut run, "result", &result)?;
    run.session().set_display_mode(1)?;
    run.session().set_standard_projection(TOP)?;
    run.session().fit()?;
    run.dump(image)?;
    run.finish()
}

/// `bug32471`: a planar face meshed at 0.01 has a zero deflection; shown without
/// re-triangulation.
pub(crate) fn planar_face_zero_deflection() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0032471: Mesh - Deflection of the triangulation is not recomputed for planar face",
        "=======",
    ]);
    banner(&[""]);
    let result = read_dataset_shape("bug32471.brep")?;
    mesh_shape(&result, &MeshRequest::new(0.01))?;
    let expected = TriangulationExpectation {
        // The literal 0 selects boolean false in the reference scene CheckCommands, not numeric zero.
        deflection: CountExpectation::from_checktrinfo_reference("0", 0., 1e-7)?,
        ..exact_counts(2., 4.)
    };
    check_triangulation(&result, &expected)?;
    let mut run = SessionRun::new()?;
    run.session().set_auto_triangulation(false);
    run.session().set_display_mode(1)?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.dump("bug32471.png")?;
    run.finish()
}

/// `buc60707`: a cleaned fillet shape meshed at 0.1 has triangles and shows shaded.
pub(crate) fn fillet_shape_shaded() -> Result<(), Box<dyn Error>> {
    banner(&["===========", "BUC60707", "===========", ""]);
    let result = read_dataset_shape("buc60707a.brep")?;
    check_shape_valid(&result)?;
    remove_triangulation(&result)?;
    mesh_shape(&result, &MeshRequest::new(0.1))?;
    check_triangulation(&result, &PRESENT_TRIANGLES)?;
    checkview_3d(&result, "buc60707.png")
}

/// `bug31461`: a sphere meshed finer keeps its mesh at a coarser request unless forced to
/// decrease, and a geometry-only clean keeps the triangulation.
pub(crate) fn forced_coarser_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0031461: Mesh - Add possibility to force the meshing of the shape",
        "=======",
    ]);
    banner(&[""]);
    let s = sphere(10.)?;
    mesh_shape(&s, &MeshRequest::new(0.01))?;
    check_triangulation(&s, &exact_counts(10108., 5106.))?;
    mesh_shape(&s, &MeshRequest::new(0.1).decreasing())?;
    check_triangulation(&s, &exact_counts(978., 507.))?;
    remove_geometry_only(&s)?;
    mesh_shape(&s, &MeshRequest::new(0.01))?;
    remove_triangulation(&s)?;
    check_triangulation(&s, &exact_counts(978., 507.))?;
    finish_without_view()
}

/// `bug32767`: a cleaned shape with seams meshes at 0.01 without hanging, with exact counts.
pub(crate) fn seam_edges_split() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "0032767: Mesh - incorrect splitting of edges of seams leading to hang \\[since the reference implementation 7.4.0\\]\t",
        "================",
    ]);
    banner(&[""]);
    let s = read_dataset_shape("bug32767.brep")?;
    remove_triangulation(&s)?;
    mesh_shape(&s, &MeshRequest::new(0.01))?;
    let expected = TriangulationExpectation {
        // The regression literal 0.035378595363471328 rounds to this double.
        deflection: near(0.03537859536347133, 1e-6, 0.),
        ..exact_counts(1512., 909.)
    };
    check_triangulation(&s, &expected)?;
    finish_without_view()
}
