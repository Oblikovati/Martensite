//! Exact H7 tool reduction and independent per-entity preparation metadata.
#[path = "support/boolean_tolerance_values.rs"]
mod boolean_tolerance_values;
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbo::{bop_algo::Operation, bop_tools::reduce_tolerances};
use martensite_tkbrep::{
    brep::{BRepTool, access::edge_tshape},
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_lib::same_parameter_shape;

const PART: &[u8] = include_bytes!(
    "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro14240c.rle"
);
const TOOL: &[u8] =
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro14240d.rle");

fn check_tool_tolerances(tool: &Shape, vertex_tolerance: f64, edge_tolerance: f64) {
    use boolean_tolerance_values::values;
    assert_eq!(values(tool, ShapeEnum::Vertex), [vertex_tolerance; 2]);
    assert_eq!(
        values(tool, ShapeEnum::Edge),
        [edge_tolerance, 1.0e-7, 1.0e-7]
    );
    assert_eq!(values(tool, ShapeEnum::Face), [1.0e-7; 3]);
}

fn check_tool_flags(tool: &Shape) {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(tool, ShapeEnum::Edge, &mut edges).unwrap();
    assert_eq!(edges.extent(), 3);
    for (index, representations) in [2, 5, 5].into_iter().enumerate() {
        let shape = edges.find_key(index + 1).unwrap();
        let edge = topods::edge(shape).unwrap();
        assert!(BRepTool::same_parameter(&edge).unwrap());
        assert!(BRepTool::same_range(&edge).unwrap());
        assert!(!BRepTool::degenerated(&edge).unwrap());
        assert_eq!(
            edge_tshape(shape, "H7 edge").unwrap().curves().len(),
            representations
        );
    }
}

fn reduce_tool(tool: &Shape) {
    check_tool_tolerances(tool, 60.370_926_938_429, 60.370_926_938_429);
    check_tool_flags(tool);
    // btolx does not add missing pcurves; the broader preparation helper would.
    reduce_tolerances(tool, 1.0e-7).unwrap();
    same_parameter_shape(tool, 1.0e-7, true).unwrap();
    // Direct native metadata retains the vertex increment lost by BRep serialization.
    check_tool_tolerances(tool, 1.000_000_000_000_000_2e-7, 1.0e-7);
    check_tool_flags(tool);
}

/// Prepare the original tool and cut the original part; `checked_h7()`.
pub fn checked_h7() -> Shape {
    let part = boolean_unconditional_case::restored_shape(PART, ShapeEnum::Compound);
    let tool = boolean_unconditional_case::restored_shape(TOOL, ShapeEnum::Solid);
    reduce_tool(&tool);
    boolean_unconditional_case::checked_operation(&part, &tool, Operation::Cut, 2.23524e6)
}
