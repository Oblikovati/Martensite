//! The shape digest of the martensite-tkprim probes, the Rust twin of
//! the recorded capture: every martensite-tkprim result is compared through the
//! martensite-tktopalgo digest (`tp_shape`, brep_lib_common.rs).
use super::brep_lib_common;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Face, Shape, Vertex};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// `pr_digest_vertex x y z tol`: the digest of a vertex built with BRepBuilder.
pub fn digest_vertex_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let point = r.pnt();
    let tolerance = r.real();
    output.attempt(|| {
        let mut vertex = Vertex::default();
        BRepBuilder::default().make_vertex_at(&mut vertex, point, tolerance)?;
        brep_lib_common::shape(output, &vertex)
    });
    output.end();
}

/// `pr_face_frames`: the first derivatives of the located surface of each face at
/// (0.3, 0.7), after their count; -1 for a face without surface.
fn face_frames(output: &BufferedTokens, s: &Shape) -> Result<(), Failure> {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(s, ShapeEnum::Face, &mut faces)?;
    output.int(faces.extent() as i32);
    for i in 1..=faces.extent() {
        let face = Face::unchecked(faces.find_key(i)?.clone());
        let surface = BRepTool::surface(&face)?;
        let Some(surface) = surface.get() else {
            output.int(-1);
            continue;
        };
        output.show(surface.d1(0.3, 0.7), |d1| {
            output.pnt(d1.point);
            output.xyz(*d1.d1u.xyz());
            output.xyz(*d1.d1v.xyz());
        });
    }
    Ok(())
}

/// `pr_shape`: the martensite-tktopalgo digest, then the face frames of a non-null shape.
pub fn shape(output: &BufferedTokens, s: &Shape) -> Result<(), Failure> {
    brep_lib_common::shape(output, s)?;
    if !s.is_null() {
        face_frames(output, s)?;
    }
    Ok(())
}

/// [`shape`] of any typed shape; `digest(face)?`.
pub fn digest<S: AsRef<Shape>>(output: &BufferedTokens, s: S) -> Result<(), Failure> {
    shape(output, s.as_ref())
}
