//! Two-process byte-for-byte determinism gate across Boolean and section corpora (#53).
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoSection, Operation};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::fmt::Write as _;
use std::process::Command;

const WORKER_ENV: &str = "MARTENSITE_DETERMINISM_WORKER";

fn canonical_shape_digest(label: &str, shape: &Shape, out: &mut String) -> Result<(), Failure> {
    let mut vol = GProps::new();
    BRepGProp::volume_properties(shape, &mut vol, false, false, false)?;
    let mut area = GProps::new();
    BRepGProp::surface_properties(shape, &mut area, false, false)?;
    let faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?.count();
    let edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?.count();
    let vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?.count();
    let _ = writeln!(
        out,
        "{label}:f={faces}:e={edges}:v={vertices}:vol={:016x}:area={:016x}",
        vol.mass().to_bits(),
        area.mass().to_bits()
    );
    let mut vex = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while vex.more() {
        let v = topods::vertex(vex.current())?;
        let p = BRepTool::pnt(&v)?;
        let tol = BRepTool::vertex_tolerance(&v)?;
        let _ = writeln!(
            out,
            "v:{:016x}:{:016x}:{:016x}:{:016x}",
            p.x().to_bits(),
            p.y().to_bits(),
            p.z().to_bits(),
            tol.to_bits()
        );
        vex.next()?;
    }
    let mut eex = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while eex.more() {
        let e = topods::edge(eex.current())?;
        let (first, last) = BRepTool::range(&e)?;
        let tol = BRepTool::edge_tolerance(&e)?;
        let _ = writeln!(
            out,
            "e:{:016x}:{:016x}:{:016x}",
            first.to_bits(),
            last.to_bits(),
            tol.to_bits()
        );
        eex.next()?;
    }
    Ok(())
}

fn run_bop(a: &Shape, b: &Shape, op: Operation) -> Result<Shape, Failure> {
    let mut bop = BOPAlgoBOP::new();
    bop.set_non_destructive(true);
    bop.set_operation(op);
    bop.add_argument(a);
    bop.add_tool(b);
    bop.perform();
    assert!(!bop.shape().is_null(), "BOP {op:?} produced null shape");
    Ok(bop.shape().clone())
}

fn run_corpus() -> Result<String, Failure> {
    let box_a = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)?
        .shape()?
        .clone();
    let box_b = BRepPrimAPIMakeBox::with_axes(
        Axis2::new(Point3::new(5., 5., 0.), Direction3::Z)?,
        10.,
        10.,
        10.,
    )
    .shape()?
    .clone();
    let cyl = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(5., 5., -2.), Direction3::Z)?,
        3.,
        14.,
    )
    .shape()?
    .clone();
    let sphere = BRepPrimAPIMakeSphere::at_center(Point3::new(5., 5., 5.), 4.)?
        .shape()?
        .clone();

    let mut digest = String::new();
    canonical_shape_digest(
        "fuse",
        &run_bop(&box_a, &box_b, Operation::Fuse)?,
        &mut digest,
    )?;
    canonical_shape_digest("cut", &run_bop(&box_a, &cyl, Operation::Cut)?, &mut digest)?;
    canonical_shape_digest(
        "common",
        &run_bop(&box_a, &sphere, Operation::Common)?,
        &mut digest,
    )?;

    let mut section = BOPAlgoSection::new();
    section.set_non_destructive(true);
    section.add_argument(&box_a);
    section.add_argument(&cyl);
    section.perform();
    assert!(!section.shape().is_null(), "section produced null shape");
    canonical_shape_digest("section", section.shape(), &mut digest)?;
    Ok(digest)
}

#[test]
fn worker_emit_corpus_digest() {
    if std::env::var_os(WORKER_ENV).is_none() {
        return;
    }
    let digest = run_corpus().expect("corpus execution failed");
    print!("DETERMINISM_BEGIN\n{digest}DETERMINISM_END\n");
}

fn spawn_worker_digest() -> Vec<u8> {
    let exe = std::env::current_exe().expect("test executable path");
    let output = Command::new(exe)
        .env(WORKER_ENV, "1")
        .args(["--exact", "worker_emit_corpus_digest", "--nocapture"])
        .output()
        .expect("spawn determinism worker");
    assert!(
        output.status.success(),
        "determinism worker failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let start = stdout
        .find("DETERMINISM_BEGIN\n")
        .expect("missing DETERMINISM_BEGIN")
        + "DETERMINISM_BEGIN\n".len();
    let end = stdout
        .find("DETERMINISM_END\n")
        .expect("missing DETERMINISM_END");
    stdout.as_bytes()[start..end].to_vec()
}

#[test]
fn two_independent_processes_produce_byte_identical_corpus_results() {
    let first = spawn_worker_digest();
    let second = spawn_worker_digest();
    assert!(!first.is_empty(), "empty corpus digest");
    assert_eq!(
        first, second,
        "two-process determinism gate failed: process outputs differed"
    );
}
