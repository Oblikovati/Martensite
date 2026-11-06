//! The original profile's moving point starts at(0,0), so the tangent circle is centered at(0,1).
use crate::circular_profile::tangent_arc;
use martensite_tkbrep::topods::{Face, Shape, Wire};
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::{
    gp::{Plane3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};

pub(super) struct PrismInputs {
    // Keep the unshown source names' actual topology alive through both prism constructions.
    pub profile: Wire,
    pub face: Face,
    pub prisms: [Shape; 2],
}

fn source_profile() -> Result<Wire, Failure> {
    let (curve, angle) = tangent_arc([0.; 2], [1., 0.], 1., 360.);
    let curve3d = geom_api::to_3d(&curve, &Plane3::default())?;
    let edge = BRepBuilderAPIMakeEdge::from_curve_params(&curve3d, 0., angle)?.edge()?;
    BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()
}

fn source_prism(face: &Face, z: f64) -> Result<Shape, Failure> {
    let mut maker = BRepPrimAPIMakePrism::new(
        &face.clone().into_shape(),
        Vector3::new(0., 0., z),
        false,
        true,
    )?;
    Ok(maker.shape()?.clone())
}

/// Preserve profile→mkplane→both prisms→both location moves; `source_inputs()?.prisms[1]` extrudes downward.
pub(super) fn source_inputs() -> Result<PrismInputs, Failure> {
    let profile = source_profile()?;
    let face = BRepBuilderAPIMakeFace::from_wire(&profile, false)?.face()?;
    let first = source_prism(&face, 2.)?;
    let second = source_prism(&face, -2.)?;
    let positive = first.moved(&Location::new(Transform3::translation([1.; 3])), true)?;
    let negative = second.moved(&Location::new(Transform3::translation([-1.; 3])), true)?;
    Ok(PrismInputs {
        profile,
        face,
        prisms: [positive, negative],
    })
}
