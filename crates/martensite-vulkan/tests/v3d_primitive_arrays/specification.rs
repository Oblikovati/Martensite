#[path = "polygons.rs"]
mod polygons;
#[path = "triangles.rs"]
mod triangles;
#[path = "wire.rs"]
mod wire;
use martensite_tkservice::graphic3d::Graphic3dTypeOfPrimitiveArray as Kind;
const Y: [f64; 3] = [1., 1., 0.];
const R: [f64; 3] = [1., 0., 0.];
const B: [f64; 3] = [0., 1., 1.];
const G: [f64; 3] = [0., 1., 0.];
pub(super) struct ArraySpec {
    pub name: &'static str,
    pub kind: Kind,
    pub x: f64,
    pub color: [f64; 3],
    pub vertices: &'static [usize],
    pub bounds: &'static [i32],
    pub edges: &'static [i32],
}
pub(super) fn specifications() -> impl Iterator<Item = &'static ArraySpec> {
    wire::VALUES
        .iter()
        .chain(triangles::VALUES.iter())
        .chain(polygons::VALUES.iter())
}
pub(super) fn category(kind: Kind) -> &'static str {
    match kind {
        Kind::Points => "Points",
        Kind::Segments => "Segments",
        Kind::Polylines => "Polylines",
        Kind::Triangles => "Triangles",
        Kind::Trianglefans => "TriangleFans",
        Kind::Trianglestrips => "TriangleStrips",
        Kind::Quadrangles => "Quadrangles",
        Kind::Quadranglestrips => "QuadrangleStrips",
        Kind::Polygons => "Polygons",
        _ => unreachable!("authored primitive kind={kind:?}; expected one of nine source kinds"),
    }
}
