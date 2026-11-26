//! Contour callers own sampling and choose where diagnostic text goes.
use super::{ContapContour, walking_root::WalkingRoot};
use crate::contap::{
    ContapHContTool, ContapSamplingContext, ContapSurfFunction, ContapTheSearchInside,
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{
    adaptor3d::h_surface_tool::{Adaptor3dSurfaceHandle, get_type},
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::int_walk::TheIWFunction;
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis3, Direction3, Vector3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_top_adaptor::BRepTopAdaptorTool,
};
use std::{
    sync::{Barrier, mpsc},
    time::Duration,
};

#[test]
fn default_contour_finishes_while_stdout_is_held() {
    let wall = cylinder_wall(5., 10.);
    let stdout = std::io::stdout().lock();
    let (send, receive) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        send.send(wall_contour(&wall, None).nb_lines().unwrap())
            .unwrap();
    });
    let outcome = receive.recv_timeout(Duration::from_secs(3));
    drop(stdout);
    worker.join().unwrap();
    assert_eq!(outcome.unwrap(), 2);
}

#[test]
fn contour_search_and_walking_keep_the_historical_range() {
    let historical = plane([-4., -1., 5., 9.]);
    ContapHContTool::nb_sample_points(&historical).unwrap();
    let before = ContapHContTool::sample_point(&historical, 1).unwrap();
    let check_range = || {
        assert_eq!(
            ContapHContTool::sample_point(&historical, 1).unwrap(),
            before
        )
    };
    let mut diagnostics = Vec::new();
    let contour = wall_contour(&cylinder_wall(5., 10.), Some(&mut diagnostics));
    assert_eq!(contour.nb_lines().unwrap(), 2);
    assert!(diagnostics.is_empty());
    check_range();
    let surface = plane([-1., 1., -1., 1.]);
    assert_empty_inside_search(&surface);
    check_range();
    let mut function = ContapSurfFunction::new();
    TheIWFunction::set_surface(&mut function, &surface);
    function.take_failure().unwrap();
    check_range();
}

#[test]
fn failed_walking_projection_writes_the_native_message_to_the_caller() {
    let surface = plane([-1., 1., -1., 1.]);
    let mut function = ContapSurfFunction::new();
    function.set_direction(Direction3::Z);
    function
        .set_surface_with_context(&surface, &mut ContapSamplingContext::new())
        .unwrap();
    let mut root = WalkingRoot::new(&function, 1e-7, 1e-7).unwrap();
    let mut diagnostics = b"caller prefix\n".to_vec();
    let point = root
        .solve(
            &mut function,
            [-0.5, -0.5],
            [0.5, 0.5],
            Vector3::new(1., 0., 0.),
            &mut diagnostics,
        )
        .unwrap();
    assert_eq!(point, None);
    assert_eq!(
        diagnostics,
        b"caller prefix\nEchec recherche internal points\n"
    );
}

#[test]
fn concurrent_contours_match_the_serial_trim_endpoints() {
    let short = cylinder_wall(7., 4.);
    let tall = cylinder_wall(3., 17.);
    let serial = [&short, &tall, &short].map(wall_endpoints);
    let ready = Barrier::new(3);
    let concurrent = std::thread::scope(|scope| {
        let workers = [&short, &tall, &short].map(|wall| {
            let ready = &ready;
            scope.spawn(move || {
                ready.wait();
                wall_endpoints(wall)
            })
        });
        workers.map(|worker| worker.join().unwrap())
    });
    assert_eq!(serial, concurrent);
}

fn wall_endpoints(face: &Face) -> Vec<Vec<[u64; 3]>> {
    let contour = wall_contour(face, None);
    let count = contour.nb_lines().unwrap();
    assert_eq!(count, 2);
    (1..=count)
        .map(|index| {
            let line = contour.line(index).unwrap();
            let count = line.nb_vertex();
            assert_eq!(count, 2);
            (1..=count)
                .map(|rank| {
                    let point = line.vertex(rank).unwrap().value();
                    [point.x(), point.y(), point.z()].map(f64::to_bits)
                })
                .collect()
        })
        .collect()
}

fn cylinder_wall(radius: f64, height: f64) -> Face {
    let mut make = BRepPrimAPIMakeCylinder::new(radius, height);
    let shape = make.shape().unwrap();
    Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::face(&shape).unwrap())
        .find(|face| get_type(&face_tool(face).get_surface()).unwrap() == SurfaceType::Cylinder)
        .unwrap()
}

fn face_tool(face: &Face) -> BRepTopAdaptorTool {
    BRepTopAdaptorTool::with_face(face, precision::p_confusion()).unwrap()
}

fn assert_empty_inside_search(surface: &Adaptor3dSurfaceHandle) {
    let mut function = ContapSurfFunction::new();
    function.set_direction(Direction3::Z);
    let mut inside = ContapTheSearchInside::new();
    inside.perform_from(&mut function, surface, 0., 0.).unwrap();
    assert!(inside.is_done());
    assert_eq!(inside.nb_points().unwrap(), 0);
}

fn plane(bounds: [f64; 4]) -> Adaptor3dSurfaceHandle {
    let [u0, u1, v0, v1] = bounds;
    let geometry: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    Handle::new(GeomAdaptorSurface::with_bounds(&geometry, (u0, u1, v0, v1)).unwrap()).cast()
}

fn wall_contour(face: &Face, writer: Option<&mut dyn std::io::Write>) -> ContapContour {
    let mut contour = ContapContour::new();
    contour.init_direction(Vector3::new(1., 0., 0.)).unwrap();
    let tool = face_tool(face);
    let domain = tool.get_topol_tool();
    let domain = domain.get().unwrap();
    let surface = tool.get_surface();
    match writer {
        Some(writer) => contour.perform_with_diagnostics(&surface, domain, writer),
        None => contour.perform(&surface, domain),
    }
    .unwrap();
    assert!(contour.is_done());
    contour
}
