//! Native regression for `tests/bugs/modalg_6/bug28054_2`.
#![allow(non_snake_case)] // Preserve the reference case identity in libtest output.
//!
//! Plain rotate is geometry-only and leaves its copied topology-command faces unchanged. Every
//! projected edge has measured 3D/2D maximum discrepancy at most 1e-7.

#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;

use martensite_tkbo::bop_tools::compute_curve_tolerance;
use martensite_tkbool::brep_proj::BRepProjProjection;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
#[path = "support/boolean_typed_explode.rs"]
mod case_typed_explode;
#[path = "support/bug28054_geometry.rs"]
mod source_geometry;

fn projection_wires(edge: &Shape, face: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut projector =
        BRepProjProjection::along(edge, face, martensite_tkmath::gp::Direction3::Z)?;
    let mut wires = Vec::new();
    projector.init();
    while projector.more() {
        wires.push(projector.current()?.into_shape());
        projector.next();
    }
    Ok(wires)
}

fn measured_projection_distance(
    edge: &Shape,
    face: &Shape,
    label: &str,
) -> Result<(f64, f64), Failure> {
    let projected_edge = topods::edge(edge)?;
    let target_face = topods::face(face)?;
    let (mut distance, mut parameter) = (1e100, 0.);
    assert!(
        compute_curve_tolerance(&target_face, &projected_edge, &mut distance, &mut parameter)?,
        "{label}: distance computation failed"
    );
    Ok((distance, parameter))
}

fn assert_projected_distance(edge: &Shape, face: &Shape, label: &str) -> Result<(), Failure> {
    let (distance, parameter) = measured_projection_distance(edge, face, label)?;
    println!(
        "Max Distance = {}; Parameter on curve = {}",
        martensite_tkernel::storage::format_general(distance, 17),
        martensite_tkernel::storage::format_general(parameter, 17)
    );
    assert!(
        distance <= 1e-7,
        "{label}: discrepancy {distance} exceeds 1e-7"
    );
    println!("OK: Max Distance is less than 1.0e-7");
    Ok(())
}

fn assert_projection_pair(edge: &Shape, face: &Shape, label: &str) -> Result<(), Failure> {
    let wires = projection_wires(edge, face)?;
    assert!(!wires.is_empty(), "{label}: no projection wires");
    for wire in wires {
        for result in case_typed_explode::all_distinct(&wire, ShapeEnum::Edge) {
            assert_projected_distance(&result, face, label)?;
        }
    }
    Ok(())
}

fn case_geometry_assertions() -> Result<(), Failure> {
    println!("========\nOCC28054\n========\n");
    let (faces, edges) = source_geometry::case_inputs_28054()?;
    for (edge_index, face_index) in [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 3)] {
        let label = format!("OCC28054: e{} on f{}", edge_index + 1, face_index + 1);
        println!("\nCheck e{} and f{}", edge_index + 1, face_index + 1);
        assert_projection_pair(&edges[edge_index], &faces[face_index], &label)?;
    }
    println!("TEST COMPLETED");
    Ok(())
}

#[path = "support/case_draw_case.rs"]
mod case_draw_case;

#[test]
#[allow(non_snake_case)]
fn Modalg6__Bug28054_2() {
    case_draw_case::run("Modalg6__Bug28054_2", || {
        case_geometry_assertions().unwrap();
    });
}
