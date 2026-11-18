//! Owned consumer fixtures use the public wrapped-profile request and a native cylinder.
use super::*;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis2, Direction3, Point3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

pub(super) fn request(shift: f64, oblique: bool) -> WrappedNormalRequest {
    let mut request = super::super::wrapped_profile::request();
    for vertex in &mut request.profile.vertices {
        vertex.point[0] += shift;
        vertex.point[1] += 0.5;
    }
    if oblique {
        request.support.origin = [1.25, -2.5, 3.75];
        request.support.axis = [1., 2., 3.];
        request.support.radial = [3., -1., 2.];
    }
    request
}
pub(super) fn attachment(oblique: bool) -> CylinderFaceAttachment {
    let axes = axes(request(0.5, oblique).support);
    let body = BRepPrimAPIMakeCylinder::with_axes(axes, 10., 4.)
        .shape()
        .unwrap()
        .clone();
    let face = lateral_face(&body);
    CylinderFaceAttachment::capture(
        FaceAttachmentId(7),
        SupportRevision(3),
        &body,
        &face,
        super::super::refusals::options().resolution,
    )
    .unwrap()
}
fn axes(support: CylinderSupport) -> Axis2 {
    Axis2::with_x_direction(
        Point3::new(support.origin[0], support.origin[1], support.origin[2]),
        Direction3::new(support.axis[0], support.axis[1], support.axis[2]).unwrap(),
        Direction3::new(support.radial[0], support.radial[1], support.radial[2]).unwrap(),
    )
    .unwrap()
}
fn lateral_face(body: &Shape) -> Face {
    let mut explorer = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let face = topods::face(explorer.current()).unwrap();
        if BRepAdaptorSurface::with_face(&face, false)
            .unwrap()
            .get_type()
            .unwrap()
            == SurfaceType::Cylinder
        {
            return face;
        }
        explorer.next().unwrap();
    }
    panic!("fixture requires one cylindrical lateral face");
}
