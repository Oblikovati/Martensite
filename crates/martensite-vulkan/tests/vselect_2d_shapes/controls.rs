//! Actual geometry and Null contexts prove shape roles and stable shared anchor ownership.
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkservice::{
    graphic3d::Graphic3dTransformPersMembers as _, null_driver::NullGraphicDriver,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

fn null_source_scene() -> ViewerSession {
    let shapes = geometry::source_shapes().unwrap();
    assert_eq!(shapes.len(), 18);
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
    scene::initial(&mut session, &shapes).unwrap();
    session
}

#[test]
fn every_clock_compound_contains_its_authored_number_of_vertices() {
    let shapes = geometry::source_shapes().unwrap();
    for index in 1..=12 {
        let name = format!("p{index}");
        let (_, shape) = shapes.iter().find(|(key, _)| *key == name).unwrap();
        let vertices =
            martensite_tkbrep::topexp::Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)
                .unwrap();
        assert_eq!(vertices.count(), index);
    }
}

#[test]
fn actual_displayed_object_handles_survive_every_shared_corner_reanchor() {
    let mut session = null_source_scene();
    let names = ["b1", "b2", "b3", "b4", "s", "t", "p1", "p12"];
    let objects: Vec<_> = names
        .iter()
        .map(|name| (*name, session.object(name).unwrap().clone()))
        .collect();
    for corner in [
        Corner::LEFT_UPPER,
        Corner::RIGHT_UPPER,
        Corner::RIGHT_LOWER,
        Corner::LEFT_LOWER,
    ] {
        scene::reanchor(&mut session, corner).unwrap();
        let shared = objects[0].1.transform_persistence();
        assert_eq!(shared.corner_2d().unwrap(), corner);
        check_shared_objects(&session, &objects, &shared);
    }
}

fn check_shared_objects(
    session: &ViewerSession,
    objects: &[(
        &str,
        martensite_tkernel::standard::Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
    )],
    shared: &martensite_tkernel::standard::Handle<
        dyn martensite_tkservice::graphic3d::Graphic3dTransformPersSource,
    >,
) {
    for (name, object) in objects {
        assert_eq!(session.object(name).unwrap(), object);
        assert_eq!(&object.transform_persistence(), shared);
        assert_eq!((object.display_mode(), object.hilight_mode()), (1, 1));
    }
}

#[test]
fn clock_marks_retain_literal_irregular_x_origins_in_the() {
    let shapes = geometry::source_shapes().unwrap();
    for (name, expected) in [
        ("p6", (-13., 12.)),
        ("p10", (-160., -115.)),
        ("p11", (-95., -45.)),
        ("p12", (-25., 30.)),
    ] {
        let (_, shape) = shapes.iter().find(|(key, _)| key == name).unwrap();
        assert_eq!(vertex_x_range(shape), expected);
    }
}

fn vertex_x_range(shape: &Shape) -> (f64, f64) {
    use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods::vertex};
    let vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    let positions = vertices.map(|item| BRepTool::pnt(&vertex(&item).unwrap()).unwrap().x());
    positions.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
        assert!(
            x.is_finite(),
            "clock vertex x{x}; expected a finite original coordinate"
        );
        (lo.min(x), hi.max(x))
    })
}
