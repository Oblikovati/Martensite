//! The path-tracing scene of the reference scene pathtrace/pathtrace and
//! The reference scene pathtrace_adaptive/pathtrace_adaptive: a GREEN PLASTIC 1x2x3 box on a 10x10
//! STONE floor, axonometric, fitted, lit along (1, 1, -1).
use super::raytrace_scene::RaytraceScene;
use martensite_tkernel::quantity::NamedColor;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

/// A 1x2x3 box on a 10x10 preview floor, axonometric, with materials set, fitted,
/// and lit along (1, 1, -1).
pub fn box_on_floor(driver: &Handle<Graphic3dRenderDriver>) -> RaytraceScene {
    let scene = RaytraceScene::new(driver);
    scene
        .view()
        .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        .unwrap();
    let b = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        2.,
        3.,
    );
    let b = b.unwrap().shape().unwrap().clone();
    scene.display(
        &b,
        Graphic3dNameOfMaterial::Plastified,
        Some(NamedColor::GREEN),
    );
    let mut bb = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    bb.init_at_point(Point3::new(-5., -5., 0.), 10., 10., 0.)
        .unwrap();
    scene.display(
        &bb.shape().unwrap().clone(),
        Graphic3dNameOfMaterial::Stone,
        None,
    );
    scene.fit();
    let light = scene.view().active_lights()[0].clone();
    light
        .get()
        .unwrap()
        .set_direction(&Direction3::new(1., 1., -1.).unwrap())
        .unwrap();
    scene
}
