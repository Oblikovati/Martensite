// SPDX-License-Identifier: GPL-2.0-only
//! Independent native capture uses explicit source bindings and native incidence.
use super::material::CertifiedPlanarMaterial;
use crate::profile_extrusion::ProfileCurveId;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Face, Shape},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{
    geom::{GeomBezierSurface, GeomConicalSurface, GeomElementarySurface},
    top_abs::{Orientation, ShapeEnum},
};
mod cap;
mod side;
mod vertices;
pub(super) use vertices::complete as complete_entities;

pub(super) struct Side {
    pub id: u64,
    pub orientation: i8,
    pub rows: [Vec<[f64; 3]>; 2],
    pub weights: Vec<f64>,
    pub edges: [Shape; 4],
    pub source: side::source::SideSource,
}
pub(super) struct SideGeometry {
    pub id: u64,
    pub orientation: i8,
    pub rows: [Vec<[f64; 3]>; 2],
    pub weights: Vec<f64>,
    pub edges: [Shape; 4],
}
pub(super) struct Cap {
    pub id: u64,
    pub orientation: i8,
    pub frame: [[f64; 3]; 4],
    pub datums: [f64; 3],
    pub contours: Vec<Contour>,
}
pub(super) struct Contour {
    pub role: usize,
    pub orientation: i8,
    pub edges: Vec<UvEdge>,
}
pub(super) struct UvEdge {
    pub side: u64,
    pub row: usize,
    pub reversed: bool,
    pub poles: Vec<[f64; 2]>,
    pub weights: Vec<f64>,
    pub range: [f64; 2],
}
pub(super) fn children(shape: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape)?;
    let mut found = Vec::new();
    while explorer.more() {
        found.push(explorer.current().clone());
        explorer.next()?;
    }
    Ok(found)
}
pub(super) fn orientation(shape: &Shape) -> Result<i8, Failure> {
    match shape.orientation() {
        Orientation::Forward => Ok(1),
        Orientation::Reversed => Ok(-1),
        actual => Err(rejected(&format!(
            "orientation={actual:?}; expected Forward/Reversed"
        ))),
    }
}
pub(super) fn rejected(cause: &str) -> Failure {
    Failure::with_type(
        "ConicalActualBindingRejected",
        cause,
        "expected complete native/source-bound conical geometry",
    )
}
pub(super) fn capture_side(id: u64, face: &Face) -> Result<Side, Failure> {
    let side = side::capture(id, face)?;
    vertices::verify(&side)?;
    let source = side::source::capture(face, &side.edges)?;
    Ok(Side {
        id: side.id,
        orientation: side.orientation,
        rows: side.rows,
        weights: side.weights,
        edges: side.edges,
        source,
    })
}
pub(super) fn capture_cap(
    id: u64,
    face: &Face,
    sides: &[Side],
    material: &CertifiedPlanarMaterial,
    curves: &[(ProfileCurveId, u64)],
) -> Result<Cap, Failure> {
    cap::capture(id, face, sides, material, curves)
}
