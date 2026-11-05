//! Native replay of the OCC25028 wire-concatenation regression.

#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;

use martensite_geometry_raster::{ShapeColors, TopologyRendering, picks::LocalShapePicks};
use martensite_tkbool::brep_algo::concatenate_wire;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{self, Shape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::rc::Rc;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture: expected drawable header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture: expected readable shape");
    shape
}

fn source_whatis(profile: &Shape) {
    let drawable = TopologyRendering::new(
        profile.clone(),
        ShapeColors::standard(),
        100.,
        2,
        30,
        Rc::new(LocalShapePicks::default()),
    );
    let mut description = String::new();
    drawable
        .write_description(|part| {
            description.push_str(part);
            Ok(())
        })
        .unwrap();
    println!("{description}");
}

fn source_validity(profile: &Shape) {
    let valid = BRepCheckAnalyzer::new(profile, true, false, false)
        .expect("checkshape analyzer")
        .is_valid()
        .expect("checkshape result");
    assert!(valid, "tests/bugs/modalg_5/bug25028: checkshape s");
    println!("This shape seems to be valid");
}

#[allow(non_snake_case)]
fn case_geometry_assertions() {
    println!("============\nOCC25028\n============\n");
    let profile = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug25028_profile.brep"
    ));
    source_validity(&profile);
    source_whatis(&profile);

    concatenate_wire(
        &topods::wire(&profile).expect("{source}: expected wire"),
        Continuity::C1,
        1e-4,
    )
    .expect("{source}: concatwire r s must not raise an exception");
    println!("TEST COMPLETED");
}

#[path = "support/case_draw_case.rs"]
mod case_draw_case;

#[test]
#[allow(non_snake_case)]
fn Modalg5__Bug25028() {
    case_draw_case::run("Modalg5__Bug25028", || {
        case_geometry_assertions();
    });
}
