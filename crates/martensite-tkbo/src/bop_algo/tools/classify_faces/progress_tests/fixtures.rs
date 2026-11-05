//! Independently declared planar roles have membership [[0], [1]], third outside.
use super::super::{FaceClassificationJob, InParts, ShapeBoxMap, SolidInternalFaces};
use crate::int_tools::Context;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};

/// Retain authored input roles independently of output order; `fixture.membership(&parts)`.
pub(super) struct AnalyticalClassification {
    pub solids: List<Shape>,
    pub faces: List<Shape>,
    boxes: ShapeBoxMap,
    internal: SolidInternalFaces,
}

impl AnalyticalClassification {
    /// Build the native analytical box/plane corpus; `AnalyticalClassification::new()`.
    pub fn new() -> Self {
        let recipe = "two";
        Self::for_recipe(recipe)
    }

    /// Build one fixed native-observed empty/candidate domain; `AnalyticalClassification::for_recipe("empty_faces")`.
    pub fn for_recipe(recipe: &str) -> Self {
        Self {
            solids: recipe_solids(recipe),
            faces: recipe_faces(recipe),
            boxes: Default::default(),
            internal: Default::default(),
        }
    }

    /// Borrow actual roles into the existing classifier; `fixture.job(&context, &mut parts, true)`.
    pub fn job<'a>(
        &'a self,
        context: &'a Context,
        parts: &'a mut InParts,
        parallel: bool,
    ) -> FaceClassificationJob<'a> {
        FaceClassificationJob {
            faces: &self.faces,
            solids: &self.solids,
            context,
            parts,
            boxes: &self.boxes,
            internal: &self.internal,
            parallel,
        }
    }

    /// Bind the public known-boundary edge case without changing geometry; `fixture.bind_void_first()`.
    pub fn bind_void_first(&mut self) {
        let first = self.solids.iter().next().expect("expected authored solid0");
        self.boxes.bind(
            first.clone(),
            martensite_tkmath::bnd::BoundingBox3::default(),
        );
    }

    /// Map only actual IsSame input identities to authored roles; `fixture.membership(&parts)`.
    pub fn membership(&self, parts: &InParts) -> Vec<Vec<usize>> {
        self.solids
            .iter()
            .enumerate()
            .map(|(index, solid)| {
                assert!(parts.find_key(index + 1).unwrap().is_same(solid));
                parts
                    .find_from_key(solid)
                    .unwrap()
                    .iter()
                    .map(|member| self.face_index(member))
                    .collect()
            })
            .collect()
    }
    /// Record registered and absent authored solids separately; `fixture.observed_membership(&parts)`.
    pub fn observed_membership(&self, parts: &InParts) -> Vec<Vec<usize>> {
        self.solids
            .iter()
            .map(|solid| {
                parts.seek(solid).map_or_else(Vec::new, |members| {
                    members
                        .iter()
                        .map(|member| self.face_index(member))
                        .collect()
                })
            })
            .collect()
    }

    fn face_index(&self, member: &Shape) -> usize {
        let role = self.faces.iter().position(|face| face.is_same(member));
        role.expect("classified shape must be an actual authored face")
    }

    /// Check all input topology and geometry; `fixture.require_valid()`.
    pub fn require_valid(&self) {
        for shape in self.faces.iter().chain(self.solids.iter()) {
            let analysis =
                martensite_tktopalgo::brep_check::BRepCheckAnalyzer::new(shape, true, false, true)
                    .unwrap();
            assert!(analysis.is_valid().unwrap());
        }
    }

    /// Freeze all reachable input geometry/topology bytes; `fixture.snapshot()`.
    pub fn snapshot(&self) -> Vec<Vec<u8>> {
        self.solids
            .iter()
            .chain(self.faces.iter())
            .map(serialized_shape)
            .collect()
    }
}

fn solid(x: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(Point3::new(x, 0., 0.), 10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn face(x: f64) -> Shape {
    let plane = Plane3::new(
        Point3::new(0., 0., 5.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (x, x + 2., 2., 4.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn serialized_shape(shape: &Shape) -> Vec<u8> {
    use martensite_tkernel::{message::ProgressRange, storage::IoStream};
    let mut stream = IoStream::from_bytes(Vec::new());
    martensite_tkbrep::brep_tools::write(shape, &mut stream, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; stream.tellp() as usize];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

fn recipe_solids(recipe: &str) -> List<Shape> {
    match recipe {
        "empty_solids" => List::default(),
        "empty_solid" => std::iter::once(empty_solid()).collect(),
        "two" | "no_candidate" | "empty_faces" => [0., 20.].into_iter().map(solid).collect(),
        other => panic!("recipe={other:?}; expected fixed analytical native domain"),
    }
}

fn recipe_faces(recipe: &str) -> List<Shape> {
    match recipe {
        "empty_faces" => List::default(),
        "no_candidate" => std::iter::once(face(42.)).collect(),
        "two" | "empty_solid" | "empty_solids" => [2., 22., 42.].into_iter().map(face).collect(),
        other => panic!("recipe={other:?}; expected fixed analytical native domain"),
    }
}

fn empty_solid() -> Shape {
    let mut solid = martensite_tkbrep::topods::Solid::default();
    martensite_tkbrep::brep::BRepBuilder::default().make_solid(&mut solid);
    solid.into_shape()
}
