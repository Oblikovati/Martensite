//! A reversed three-segment wire concatenates before and after spline conversion.
use martensite_tkbool::brep_algo::concatenate_wire_c0;
use martensite_tkbrep::topods::{self, Edge, Shape, Wire};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert,
};

fn three_segment_reversed_wire() -> Wire {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for x in [0., 1., 2., 3.] {
        polygon.add_point(Point3::new(x, 0., 0.)).unwrap();
    }
    Wire::unchecked(polygon.wire().unwrap().reversed())
}

#[path = "support/bug29573_counts.rs"]
mod source_topology;

fn assert_single_edge_with_two_vertices(edge: &Edge) {
    let counts = source_topology::source_edge_counts_29573(edge);
    println!("{{\"default_VE\":{counts:?}}}");
    assert_eq!(counts, [2, 1], "source default ShapeSet V2/E1 predicate");
}

fn case_geometry_assertions() {
    println!(
        "===================================================\n0031496: Modeling Algorithms - BRepAlgo::ConcatenateWireC0() crashes\n==================================================="
    );
    let wire = three_segment_reversed_wire();
    assert_single_edge_with_two_vertices(&concatenate_wire_c0(&wire).unwrap());
    let mut conversion = BRepBuilderAPINurbsConvert::with_shape(&wire, false).unwrap();
    let converted: Shape = conversion.shape().unwrap().clone();
    let converted_wire = topods::wire(&converted).unwrap();
    assert_single_edge_with_two_vertices(&concatenate_wire_c0(&converted_wire).unwrap());
    println!("TEST COMPLETED");
}

#[path = "support/case_draw_case.rs"]
mod case_draw_case;

#[test]
#[allow(non_snake_case)]
fn reversed_collinear_wire_concatenates_before_and_after_nurbs_conversion() {
    case_draw_case::run(
        "reversed_wire_concatenation::reversed_collinear_wire_concatenates_before_and_after_nurbs_conversion",
        || {
            case_geometry_assertions();
        },
    );
}
