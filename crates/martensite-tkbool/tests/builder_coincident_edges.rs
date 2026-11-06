//! Existing-edge results share one copied topology across matching data curves.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSBuildTool, TopOpeBRepDSCurve, TopOpeBRepDSHDataStructure,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;

fn curves_using(source: &Shape) -> Result<Handle<TopOpeBRepDSHDataStructure>, Failure> {
    let edge = topods::edge(source)?;
    let (geometry, _, _) = BRepTool::curve(&edge)?;
    let mut curve = TopOpeBRepDSCurve::new(&geometry, 0.000001, false);
    curve.set_existing_edge(&edge, false);
    let structure = Handle::new(TopOpeBRepDSHDataStructure::new());
    for expected in 1..=4 {
        assert_eq!(
            structure
                .get()
                .expect("owned structure")
                .change_ds()
                .add_curve(&mut curve),
            expected
        );
    }
    Ok(structure)
}

fn result_edge(builder: &TopOpeBRepBuildBuilder, index: i32) -> Shape {
    let edges = builder.new_edges(index);
    assert_eq!(edges.len(), 1);
    edges.iter().next().expect("one edge").clone()
}

fn assert_copied_line(source: &Shape, result: &Shape) -> Result<(), Failure> {
    assert!(!result.is_same(source));
    assert_eq!(result.orientation(), Orientation::Forward);
    let edge = topods::edge(result)?;
    let (curve, first, last) = BRepTool::curve(&edge)?;
    assert_eq!((first, last), (0.0, 1.0));
    let geometry = curve.get().expect("line geometry");
    assert_eq!(geometry.value(first)?, Point3::new(0.0, 0.0, 0.0));
    assert_eq!(geometry.value(last)?, Point3::new(1.0, 0.0, 0.0));
    let (a, b) = topexp::vertices(&edge, false);
    let (case_a, case_b) = topexp::vertices(&topods::edge(source)?, false);
    assert!(a.is_same(&case_a) || a.is_same(&case_b));
    assert!(b.is_same(&case_a) || b.is_same(&case_b));
    assert!(!a.is_same(&b));
    Ok(())
}

#[test]
fn existing_edge_curves_reuse_one_forward_copy_with_shared_source_vertices() -> Result<(), Failure>
{
    let [edge, _] = connected_edges()?;
    for orientation in [Orientation::Forward, Orientation::Reversed] {
        let source = edge.oriented(orientation);
        let structure = curves_using(&source)?;
        let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
        builder.build_edges_of(1, &structure)?;
        builder.build_edges_of(2, &structure)?;
        let first = result_edge(&builder, 1);
        assert!(result_edge(&builder, 2).is_same(&first));
        assert_copied_line(&source, &first)?;
        assert_eq!(source.orientation(), orientation);
    }
    Ok(())
}

#[path = "builder_coincident_edges/cache.rs"]
mod cache;
