// SPDX-License-Identifier: GPL-2.0-only
//! General Fuse blend assembly regression tests (#44).
use martensite_tkbrep::operation::{FailureClass, OperationKind};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkfillet::brep_fillet_api::GeneralFuseBlendAssembler;
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::f64::consts::FRAC_PI_4;

fn face_count(shape: &Shape) -> usize {
    let mut ex = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while ex.more() {
        count += 1;
        ex.next().unwrap();
    }
    count
}

fn volume_of(shape: &Shape) -> f64 {
    let mut props = GProps::default();
    BRepGProp::volume_properties(shape, &mut props, true, false, false).unwrap();
    props.mass()
}

#[test]
fn general_fuse_assembles_valid_filleted_box_with_exact_volume_and_history() {
    // 10x10x10 box with a R=2 fillet on the edge (x=10, y=10, z in [0, 10]).
    // Fillet cylinder axis at (8, 8, 0) along +Z, radius 2.
    let mut box_maker = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0).unwrap();
    let source = box_maker.shape().unwrap().clone();
    let cyl_axis = Axis3::new(Point3::new(8.0, 8.0, 0.0), Direction3::Z, Direction3::X).unwrap();
    let cyl_surf: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(cyl_axis, 2.0).unwrap()).cast();
    let fillet_face = BRepBuilderAPIMakeFace::from_surface_bounds(
        &cyl_surf,
        (0.0, std::f64::consts::FRAC_PI_2, 0.0, 10.0),
        1e-7,
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();

    let result =
        GeneralFuseBlendAssembler::assemble_solid(OperationKind::Fillet, &source, &[fillet_face])
            .unwrap();

    assert_eq!(result.operation(), OperationKind::Fillet);
    assert!(
        BRepCheckAnalyzer::new(result.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert_eq!(face_count(result.shape()), 7);
    let expected_volume = 1000.0 - (1.0 - FRAC_PI_4) * 4.0 * 10.0;
    assert!(
        (volume_of(result.shape()) - expected_volume).abs() < 1e-4,
        "volume={} expected={expected_volume}",
        volume_of(result.shape())
    );
    assert!(!result.history().is_empty());
}

#[test]
fn general_fuse_assembles_valid_chamfered_box_with_exact_volume_and_history() {
    // 10x10x10 box with a d=2 symmetric chamfer on edge (x=10, y=10, z in [0, 10]).
    // Chamfer plane passes through (10, 8, 0) and (8, 10, 0), normal (1, 1, 0)/sqrt(2).
    let mut box_maker = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0).unwrap();
    let source = box_maker.shape().unwrap().clone();
    let normal = Direction3::new(1.0, 1.0, 0.0).unwrap();
    let plane_axis = Axis3::new(Point3::new(9.0, 9.0, 5.0), normal, Direction3::Z).unwrap();
    let plane_surf: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(plane_axis)).cast();
    let chamfer_face =
        BRepBuilderAPIMakeFace::from_surface_bounds(&plane_surf, (-8.0, 8.0, -8.0, 8.0), 1e-7)
            .unwrap()
            .face()
            .unwrap()
            .into_shape();

    let result =
        GeneralFuseBlendAssembler::assemble_solid(OperationKind::Chamfer, &source, &[chamfer_face])
            .unwrap();

    assert_eq!(result.operation(), OperationKind::Chamfer);
    assert!(
        BRepCheckAnalyzer::new(result.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert_eq!(face_count(result.shape()), 7);
    let expected_volume = 1000.0 - 0.5 * 4.0 * 10.0;
    assert!(
        (volume_of(result.shape()) - expected_volume).abs() < 1e-4,
        "volume={} expected={expected_volume}",
        volume_of(result.shape())
    );
    assert!(!result.history().is_empty());
}

#[test]
fn general_fuse_blend_assembler_rejects_bad_inputs_with_typed_error() {
    let err =
        GeneralFuseBlendAssembler::assemble_solid(OperationKind::Fillet, &Shape::default(), &[])
            .unwrap_err();
    assert_eq!(err.operation(), OperationKind::Fillet);
    assert_eq!(err.failure_class(), FailureClass::BadInput);
}

#[test]
fn fillet_assembly_is_byte_identical_across_threads_and_prior_operations() {
    use martensite_tkbrep::brep::BRepTool;
    use martensite_tkbrep::topods;
    use std::fmt::Write;

    fn snapshot(shape: &Shape) -> String {
        let mut out = String::new();
        let mut vex = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
        while vex.more() {
            let v = topods::vertex(vex.current()).unwrap();
            let p = BRepTool::pnt(&v).unwrap();
            let tol = BRepTool::vertex_tolerance(&v).unwrap();
            let _ = writeln!(
                out,
                "v:{:016x}:{:016x}:{:016x}:{:016x}",
                p.x().to_bits(),
                p.y().to_bits(),
                p.z().to_bits(),
                tol.to_bits()
            );
            vex.next().unwrap();
        }
        let _ = writeln!(out, "vol:{:016x}", volume_of(shape).to_bits());
        out
    }

    fn run_fillet(radius: f64) -> String {
        let mut box_maker = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0).unwrap();
        let source = box_maker.shape().unwrap().clone();
        let cyl_axis = Axis3::new(
            Point3::new(10.0 - radius, 10.0 - radius, 0.0),
            Direction3::Z,
            Direction3::X,
        )
        .unwrap();
        let cyl_surf: Handle<dyn GeomSurface> =
            Handle::new(GeomCylindricalSurface::new(cyl_axis, radius).unwrap()).cast();
        let fillet_face = BRepBuilderAPIMakeFace::from_surface_bounds(
            &cyl_surf,
            (0.0, std::f64::consts::FRAC_PI_2, 0.0, 10.0),
            1e-7,
        )
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
        let result = GeneralFuseBlendAssembler::assemble_solid(
            OperationKind::Fillet,
            &source,
            &[fillet_face],
        )
        .unwrap();
        snapshot(result.shape())
    }

    // Thread A: fresh thread running R=2 directly.
    let bytes_a = std::thread::spawn(move || run_fillet(2.0)).join().unwrap();
    // Thread B: runs a different R=1 fillet first, then R=2 on the same thread (#62).
    let bytes_b = std::thread::spawn(move || {
        let _prior = run_fillet(1.0);
        run_fillet(2.0)
    })
    .join()
    .unwrap();
    assert_eq!(
        bytes_a, bytes_b,
        "fillet output must be byte-identical across threads and prior operations (#62)"
    );
}
