//! Sewing of floating edges and of box faces under each sewing switch (OCC22770).
use super::display_result_view;
use super::prelude::*;
use crate::primitive_shapes::box_at;
use crate::shape_report_checks::{
    ShapeCounts, check_free_bounds, check_max_tolerance, check_section_vertices, check_shape_counts,
};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex, BRepBuilderAPISewing,
};

/// Heading of the sewing regressions.
const SEWING_HEADING: [&str; 4] = ["================", "OCC22770", "================", ""];

/// The sewing parameters: working tolerance, optional minimum tolerance, and the switches
/// (`+c`/`-c` cutting, `+n` non-manifold, `+e` floating edges, `+f`/`-f` faces, ...).
/// Example: `SewingOptions { floating_edges: true, ..SewingOptions::with_tolerance(0.1) }`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SewingOptions {
    /// Working tolerance.
    pub tolerance: f64,
    /// Minimum tolerance; 1e-4 of the working tolerance when not given.
    pub min_tolerance: Option<f64>,
    /// Create the sewed shape.
    pub sewing: bool,
    /// Analyse the input shapes.
    pub analysis: bool,
    /// Cut free edges.
    pub cutting: bool,
    /// Process non-manifold topology.
    pub non_manifold: bool,
    /// Same-parameter processing of edges.
    pub same_parameter: bool,
    /// Sew floating edges.
    pub floating_edges: bool,
    /// Sew faces.
    pub faces: bool,
}

impl SewingOptions {
    /// The default switches with working tolerance `tolerance`.
    /// Example: `SewingOptions::with_tolerance(1.1)`.
    pub const fn with_tolerance(tolerance: f64) -> Self {
        Self {
            tolerance,
            min_tolerance: None,
            sewing: true,
            analysis: true,
            cutting: true,
            non_manifold: false,
            same_parameter: true,
            floating_edges: false,
            faces: true,
        }
    }

    /// The default switches and working tolerance (1e-6).
    pub const DEFAULT: Self = Self::with_tolerance(1.0e-6);
}

/// The sewn inputs of a regression.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SewingInputs {
    /// Edges (1,0,0)-(10,0,0) and (10.01,0,0)-(20,0,0), checked as a section.
    FloatingEdges,
    /// Face 2 of a 1x2x3 box at the origin and face 1 of the same box at (2,0,0).
    FacingBoxFaces,
    /// Faces 2 and 3 of a unit box and face 3 of a 1x2x1 box at (1,0,0).
    AdjacentBoxFaces,
}

/// One sewing regression: inputs, switches and the expected checks of its result.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SewingCase {
    /// The sewn shapes.
    pub inputs: SewingInputs,
    /// The sewing parameters.
    pub options: SewingOptions,
    /// The expected length or area of the result.
    pub mass: MassCheck,
    /// The expected sub-shape counts, vertex-first.
    pub counts: [usize; 9],
    /// The expected maximum tolerance, as the regression writes it.
    pub max_tolerance: &'static str,
    /// The expected number of free boundary edges.
    pub free_edges: usize,
    /// The image dumped.
    pub image: &'static str,
}

/// Sew the inputs, check the result as the regression does, then show it.
/// Example: `sewn_result_view(&case)?`.
pub(crate) fn sewn_result_view(case: &SewingCase) -> Outcome {
    banner(&SEWING_HEADING);
    let shapes = sewing_inputs(case.inputs)?;
    let result = sew(&shapes, &case.options)?;
    check_mass(&result, &case.mass)?;
    check_shape_structure(&result)?;
    if matches!(case.inputs, SewingInputs::FloatingEdges) {
        check_section_vertices(&result)?;
    }
    check_shape_counts(&result, "result", &ShapeCounts::all(case.counts))?;
    check_max_tolerance(&result, case.max_tolerance)?;
    check_shape_counts(&result, "result", &ShapeCounts::shells(case.counts[4]))?;
    check_free_bounds(&result, case.free_edges)?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, &result, case.image)?;
    run.finish()
}

fn sewing_inputs(inputs: SewingInputs) -> Result<Vec<Shape>, Failure> {
    match inputs {
        SewingInputs::FloatingEdges => Ok(vec![
            edge([1., 0., 0.], [10., 0., 0.])?,
            edge([10.01, 0., 0.], [20., 0., 0.])?,
        ]),
        SewingInputs::FacingBoxFaces => {
            let a = faces(&box_at([0.; 3], [1., 2., 3.])?)?;
            let b = faces(&box_at([2., 0., 0.], [1., 2., 3.])?)?;
            Ok(vec![a[1].clone(), b[0].clone()])
        }
        SewingInputs::AdjacentBoxFaces => {
            let a = faces(&box_at([0.; 3], [1., 1., 1.])?)?;
            let b = faces(&box_at([1., 0., 0.], [1., 2., 1.])?)?;
            Ok(vec![a[1].clone(), a[2].clone(), b[2].clone()])
        }
    }
}

/// An edge between two new vertices at `from` and `to`.
fn edge(from: [f64; 3], to: [f64; 3]) -> Result<Shape, Failure> {
    let vertex =
        |[x, y, z]: [f64; 3]| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex();
    let maker = BRepBuilderAPIMakeEdge::from_vertices(&vertex(from)?, &vertex(to)?)?;
    Ok(maker.edge()?.as_ref().clone())
}

/// The distinct faces of `shape` in exploration order, as an explode numbers them.
fn faces(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut seen = MapOfShape::default();
    let mut found = Vec::new();
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            found.push(explorer.current().clone());
        }
        explorer.next()?;
    }
    Ok(found)
}

/// Sew `shapes` with `options`, printing the sewing dump; returns the sewed shape.
pub(super) fn sew(shapes: &[Shape], options: &SewingOptions) -> Result<Shape, Box<dyn Error>> {
    let tolerance = options.tolerance.max(precision::confusion());
    let min_tolerance = options
        .min_tolerance
        .unwrap_or(options.tolerance * 1e-4)
        .max(precision::confusion());
    let mut sewer = BRepBuilderAPISewing::default();
    let o = options;
    sewer.init(tolerance, o.sewing, o.analysis, o.cutting, o.non_manifold);
    sewer.set_same_parameter_mode(o.same_parameter);
    sewer.set_floating_edges_mode(o.floating_edges);
    sewer.set_face_mode(o.faces);
    sewer.set_min_tolerance(min_tolerance);
    sewer.set_max_tolerance(precision::infinite());
    for shape in shapes {
        sewer.add(shape)?;
    }
    sewer.perform(&ProgressRange::default())?;
    let mut dump = Vec::new();
    sewer.dump(&mut dump)?;
    print!("{}", String::from_utf8_lossy(&dump));
    Ok(sewer.sewed_shape().clone())
}
