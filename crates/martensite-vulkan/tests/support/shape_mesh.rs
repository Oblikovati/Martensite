//! Face triangulation of a shape: removing it, computing it incrementally, and checking its
//! counts and deflection against expected values with the regressions' messages.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkmesh::imesh_tools::MeshParameters;
#[path = "shape_mesh/checktrinfo_reference.rs"]
mod checktrinfo_reference;

/// Remove the triangulations and polygons stored on the shape's faces and edges.
/// Example: `remove_triangulation(&shape)?`.
pub fn remove_triangulation(shape: &Shape) -> Result<(), Failure> {
    brep_tools::clean(shape, false)
}

/// Remove the surfaces and curves of the shape, keeping its triangulations and polygons, so
/// they become its only representation; `remove_geometry_only(&shape)?`.
pub fn remove_geometry_only(shape: &Shape) -> Result<(), Failure> {
    brep_tools::clean_geometry(shape)
}

/// Linear and angular deflection of an incremental meshing.
/// Example: `MeshRequest::new(0.1).with_angle_degrees(5.)`.
#[derive(Clone, Copy, Debug)]
pub struct MeshRequest {
    /// Linear deflection, clamped to the confusion tolerance.
    pub deflection: f64,
    /// Angular deflection in degrees; `None` keeps the mesher's default.
    pub angle_degrees: Option<f64>,
    /// Deflection relative to the edge size.
    pub relative: bool,
    /// Faces meshed in parallel.
    pub parallel: bool,
    /// A coarser request replaces a finer triangulation instead of keeping it.
    pub decrease: bool,
}

impl MeshRequest {
    /// An absolute, sequential request of `deflection`; `MeshRequest::new(0.1)`.
    pub fn new(deflection: f64) -> Self {
        Self {
            deflection,
            angle_degrees: None,
            relative: false,
            parallel: false,
            decrease: false,
        }
    }

    /// The same request allowed to replace a finer triangulation; `request.decreasing()`.
    pub fn decreasing(self) -> Self {
        Self {
            decrease: true,
            ..self
        }
    }

    /// The same request with an angular deflection in degrees; `request.with_angle_degrees(5.)`.
    pub fn with_angle_degrees(self, angle: f64) -> Self {
        Self {
            angle_degrees: Some(angle),
            ..self
        }
    }

    pub(crate) fn parameters(&self) -> MeshParameters {
        let deflection = self.deflection.max(precision::confusion());
        let mut parameters = MeshParameters {
            deflection,
            deflection_interior: deflection,
            relative: self.relative,
            in_parallel: self.parallel,
            allow_quality_decrease: self.decrease,
            ..MeshParameters::default()
        };
        if let Some(angle) = self.angle_degrees {
            parameters.angle = angle * std::f64::consts::PI / 180.;
        }
        parameters
    }
}

/// Triangulate every face of the shape with the incremental mesher; `mesh_shape(&s, &request)?`.
pub fn mesh_shape(shape: &Shape, request: &MeshRequest) -> Result<(), Failure> {
    BRepMeshIncrementalMesh::with_parameters(
        shape,
        &request.parameters(),
        &ProgressRange::default(),
    )?;
    Ok(())
}

/// Counts of the triangulations stored on a shape's faces.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TriangulationSummary {
    /// Faces explored.
    pub faces: i32,
    /// Faces without a triangulation.
    pub empty_faces: i32,
    /// Triangles over all faces.
    pub triangles: i32,
    /// Nodes over all faces.
    pub nodes: i32,
    /// The largest deflection of a face triangulation.
    pub max_deflection: f64,
}

impl TriangulationSummary {
    /// Summarize the face triangulations of `shape`; `TriangulationSummary::of(&shape)?.triangles`.
    pub fn of(shape: &Shape) -> Result<Self, Failure> {
        let mut summary = Self::default();
        let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() {
            summary.add_face(&topods::face(faces.current())?)?;
            faces.next()?;
        }
        Ok(summary)
    }

    fn add_face(&mut self, face: &topods::Face) -> Result<(), Failure> {
        self.faces += 1;
        let (triangulation, _) = BRepTool::triangulation(face, mesh_purpose::NONE)?;
        let Some(triangulation) = triangulation.get() else {
            self.empty_faces += 1;
            return Ok(());
        };
        self.triangles += triangulation.nb_triangles();
        self.nodes += triangulation.nb_nodes();
        self.max_deflection = self.max_deflection.max(triangulation.deflection());
        Ok(())
    }
}

/// What a triangulation count must be.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CountExpectation {
    /// No requirement.
    #[default]
    Any,
    /// Strictly positive.
    Present,
    /// Within `tolerance_absolute + tolerance_relative * |value|` of `value`.
    Near {
        value: f64,
        tolerance_absolute: f64,
        tolerance_relative: f64,
    },
    /// Anything but `value`.
    Not(f64),
}

impl CountExpectation {
    /// Exactly `value`; `CountExpectation::exactly(509.)`.
    pub fn exactly(value: f64) -> Self {
        Self::Near {
            value,
            tolerance_absolute: 0.,
            tolerance_relative: 0.,
        }
    }
}

/// Expected triangulation counts and deflection; unset fields are not checked.
/// Example: `TriangulationExpectation { triangles: CountExpectation::Present, ..Default::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct TriangulationExpectation {
    /// The face count.
    pub faces: CountExpectation,
    /// The triangle count.
    pub triangles: CountExpectation,
    /// The node count.
    pub nodes: CountExpectation,
    /// The maximal deflection.
    pub deflection: CountExpectation,
    /// An upper bound of the maximal deflection.
    pub max_deflection: Option<f64>,
}

/// Check the shape's triangulation, printing an error line per failed expectation and a
/// confirmation per passed value check; `check_triangulation(&shape, &expected)?`.
pub fn check_triangulation(
    shape: &Shape,
    expected: &TriangulationExpectation,
) -> Result<(), Failure> {
    let summary = TriangulationSummary::of(shape)?;
    check_count("Number of faces", f64::from(summary.faces), expected.faces);
    check_count(
        "Number of triangles",
        f64::from(summary.triangles),
        expected.triangles,
    );
    check_count("Number of nodes", f64::from(summary.nodes), expected.nodes);
    check_count(
        "Maximal deflection",
        summary.max_deflection,
        expected.deflection,
    );
    if expected
        .max_deflection
        .is_some_and(|bound| summary.max_deflection > bound)
    {
        println!("Error: Maximal deflection is too big");
    }
    Ok(())
}

fn check_count(name: &str, value: f64, expected: CountExpectation) {
    match expected {
        CountExpectation::Any => {}
        CountExpectation::Present if value <= 0. => println!("Error: {name} is equal to 0"),
        CountExpectation::Present => {}
        CountExpectation::Not(excluded) if value == excluded => {
            println!("Error: {name} is equal to {excluded} but it should not");
        }
        CountExpectation::Not(_) => {}
        CountExpectation::Near {
            value: reference,
            tolerance_absolute,
            tolerance_relative,
        } => check_real(
            name,
            value,
            reference,
            tolerance_absolute,
            tolerance_relative,
        ),
    }
}

/// Compare a real with its expected value within absolute and relative tolerances.
/// Example: `check_real("Number of nodes", 327., 327., 0., 0.)` prints a confirmation.
pub fn check_real(
    name: &str,
    value: f64,
    expected: f64,
    tolerance_absolute: f64,
    tolerance_relative: f64,
) {
    if (value - expected).abs() > tolerance_absolute + tolerance_relative * expected.abs() {
        println!("Error: {name} = {value} is not equal to expected {expected}");
    } else {
        println!("Check of {name} OK: value = {value}, expected = {expected}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_request_converts_degrees_and_clamps_deflection() {
        let parameters = MeshRequest::new(0.).with_angle_degrees(180.).parameters();
        assert_eq!(parameters.deflection, precision::confusion());
        assert_eq!(parameters.deflection_interior, precision::confusion());
        assert_eq!(parameters.angle, std::f64::consts::PI);
    }

    #[test]
    fn empty_shape_has_no_faces() {
        assert_eq!(
            TriangulationSummary::of(&Shape::default()).unwrap(),
            TriangulationSummary::default()
        );
    }
}
