//! Native replay of `tests/bugs/modalg_7/bug29573`.
//!
//! The archived input contains the two source wires selected by the command host's `explode w`.
//! Each must concatenate into one edge with two unique vertices without raising an exception.

#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;

use martensite_tkbool::brep_algo::concatenate_wire_c0;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{self, Edge, Shape, ShapeIterator, Wire},
};
use martensite_tkernel::{message::ProgressRange, standard::Failure, storage::IoStream};

const SOURCE: &str = "tests/bugs/modalg_7/bug29573";
const FIXTURE: &[u8] =
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/bug29573.brep");

fn restored_shape(bytes: &[u8]) -> Result<Shape, Failure> {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture must have a drawable header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )?;
    Ok(shape)
}

#[path = "support/bug29573_counts.rs"]
mod source_topology;

fn assert_concatenated_edge(edge: &Edge, input_index: usize) {
    let [vertex_count, edge_count] = source_topology::source_edge_counts_29573(edge);
    assert_eq!(
        vertex_count,
        2,
        "{SOURCE}: r{} must have two vertices",
        input_index + 1
    );
    assert_eq!(
        edge_count,
        1,
        "{SOURCE}: r{} must have one edge",
        input_index + 1
    );
}

fn direct_wire(children: &[Shape], index: usize) -> Result<Wire, Failure> {
    let child = children.get(index).unwrap_or_else(|| {
        panic!(
            "explode w: got{} direct children; expected w_{} Wire",
            children.len(),
            index + 1
        )
    });
    topods::wire(child)
}

#[allow(non_snake_case)]
fn case_geometry_assertions() -> Result<(), Failure> {
    println!("========\n0029573: ConcatenateWireC0 crashes on two edges wire\n========\n");
    let source = restored_shape(FIXTURE)?;
    let children: Vec<Shape> = ShapeIterator::new(&source, true, true).collect();
    let wires = [direct_wire(&children, 0)?, direct_wire(&children, 1)?];
    for (index, wire) in wires.iter().enumerate() {
        let result = concatenate_wire_c0(wire)?;
        assert_concatenated_edge(&result, index);
        println!(
            "{{\"result\":\"r{}\",\"default_VE\":{:?}}}",
            index + 1,
            source_topology::source_edge_counts_29573(&result)
        );
    }
    println!("TEST COMPLETED");
    Ok(())
}

#[path = "support/case_draw_case.rs"]
mod case_draw_case;

#[test]
#[allow(non_snake_case)]
fn Modalg7__Bug29573() {
    case_draw_case::run("Modalg7__Bug29573", || {
        case_geometry_assertions().unwrap();
    });
}
