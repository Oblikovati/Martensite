//! `statshape shape [prefix]` (ShapeAnalysisShapeContents), the
//! form without the third `particul` argument, which only prints the counts.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomConicalSurface, GeomCylindricalSurface, GeomPlane, GeomSphericalSurface, GeomSurface,
};
use martensite_tkmath::gp::{Cone3, Cylinder3, Plane3, Sphere3};
use martensite_tkshhealing::shape_analysis::{
    ShapeAnalysisCanonicalRecognition, ShapeAnalysisShapeContents,
};

/// The `Count    Item` table of `statshape`: one `n<TAB>label` row per nonzero count, in
/// the command's order. `statshape(&result).contains("bspsur : BSplineSurface")`.
pub fn statshape(shape: &Shape) -> String {
    assert!(!shape.is_null(), "Shape unknown");
    let mut analyzer = ShapeAnalysisShapeContents::new();
    analyzer
        .perform(shape)
        .unwrap_or_else(|failure| panic!("statshape: {failure:?}"));
    let rows = count_rows(&analyzer);
    let mut text = String::from("Count    Item\n-----    ----\n");
    for (count, label) in rows {
        if count > 0 {
            text.push_str(&format!("{count}{label}\n"));
        }
    }
    text
}

fn count_rows(a: &ShapeAnalysisShapeContents) -> [(i32, &'static str); 27] {
    [
        (a.nb_edges(), "\t EDGE (Oriented)"),
        (a.nb_shared_edges(), "\t EDGE (Shared)"),
        (a.nb_free_edges(), "\t EDGE (Free)"),
        (a.nb_faces(), "\t FACE"),
        (a.nb_free_faces(), "\t FACE (Free)"),
        (a.nb_free_wires(), "\t WIRE (Free)"),
        (a.nb_shells(), "\t SHELL"),
        (a.nb_solids(), "\t SOLID"),
        (a.nb_vertices(), "\t VERTEX (Oriented)"),
        (a.nb_shared_vertices(), "\t VERTEX (Shared)"),
        (a.nb_wires(), "\t WIRE"),
        (
            a.nb_faces_with_several_wires(),
            "\tFace with more than one wire",
        ),
        (a.nb_missing_pcurves(), "\tNo pcurve"),
        (a.nb_solids_with_voids(), "\tSOLID with voids"),
        (a.nb_wires_with_seam(), "\tWire(s) with one seam edge"),
        (
            a.nb_wires_with_several_seams(),
            "\tWire(s) with several seam edges",
        ),
        (a.nb_big_splines(), "\tbigspl : BSpline > 8192 poles"),
        (a.nb_bezier_surfaces(), "\tbezsur : BezierSurface"),
        (a.nb_bspline_surfaces(), "\tbspsur : BSplineSurface"),
        (a.nb_c0_curves(), "\tc0curv : Curve Only C0"),
        (a.nb_c0_surfaces(), "\tc0surf : Surface Only C0"),
        (a.nb_indirect_surfaces(), "\tindsur : Indirect Surface"),
        (a.nb_offset_curves(), "\tofcur  : Offset Curve(s)"),
        (a.nb_offset_surfaces(), "\tofsur  : Offset Surface"),
        (a.nb_trimmed_curves_2d(), "\ttrc2d  : Trimmed Curve2d"),
        (a.nb_trimmed_curves_3d(), "\ttrc3d  : Trimmed Curve3d"),
        (
            a.nb_trimmed_surfaces(),
            "\ttrimsu : RectangularTrimmedSurface",
        ),
    ]
}

/// The count a `statshape` row gives `keyword`, as the grid ends read it with
/// `regexp "(\[0-9\]+)\\s+keyword *: *Label"`: 0 when no row names it.
/// `statshape_count(&text, "bspsur", "BSplineSurface")`.
pub fn statshape_count(text: &str, keyword: &str, label: &str) -> usize {
    for line in text.lines() {
        let Some((count, rest)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(after) = rest.strip_prefix(keyword) else {
            continue;
        };
        let after = after.trim_start_matches(' ');
        let Some(after) = after.strip_prefix(':') else {
            continue;
        };
        if after.trim_start_matches(' ').starts_with(label) {
            return count.parse().unwrap_or(0);
        }
    }
    0
}

/// `getanasurf res shape [pln|cyl|con|sph [tol]]` without a
/// sample surface: ShapeAnalysisCanonicalRecognition of the shell, face, edge or wire;
/// the surface it found (then `Gap = gap`, `%.17g`) or `Cannot get required surface`.
pub fn getanasurf(
    shape: &Shape,
    target: &str,
    tolerance: f64,
) -> (Option<Handle<dyn GeomSurface>>, String) {
    let mut recognition = ShapeAnalysisCanonicalRecognition::with_shape(shape)
        .unwrap_or_else(|failure| panic!("getanasurf: {failure:?}"));
    let found: Option<Handle<dyn GeomSurface>> = match target {
        "cyl" => {
            let mut cylinder = Cylinder3::default();
            recognition
                .is_cylinder(tolerance, &mut cylinder)
                .unwrap()
                .then(|| Handle::new(GeomCylindricalSurface::from_cylinder(cylinder)).cast())
        }
        "con" => {
            let mut cone = Cone3::default();
            recognition
                .is_cone(tolerance, &mut cone)
                .unwrap()
                .then(|| Handle::new(GeomConicalSurface::from_cone(cone)).cast())
        }
        "sph" => {
            let mut sphere = Sphere3::default();
            recognition
                .is_sphere(tolerance, &mut sphere)
                .unwrap()
                .then(|| Handle::new(GeomSphericalSurface::from_sphere(sphere)).cast())
        }
        _ => {
            let mut plane = Plane3::default();
            recognition
                .is_plane(tolerance, &mut plane)
                .unwrap()
                .then(|| Handle::new(GeomPlane::from_pln(plane)).cast())
        }
    };
    let text = match &found {
        Some(_) => format!(
            "Gap = {}\n",
            super::super::printed::g17(recognition.get_gap())
        ),
        None => "Cannot get required surface\n".to_string(),
    };
    (found, text)
}
