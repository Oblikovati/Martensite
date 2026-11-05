//! Stored construction-tool references support external operation composition.
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::{TopOpeBRepDSBuildTool, TopOpeBRepDSPoint};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;

struct VertexOperation {
    builder: TopOpeBRepBuildBuilder,
}

impl VertexOperation {
    fn make_reversed_vertex(&mut self, point: &TopOpeBRepDSPoint) -> Result<Shape, Failure> {
        let tool = self.builder.change_build_tool();
        let mut vertex = Shape::default();
        tool.make_vertex(&mut vertex, point)?;
        tool.set_orientation(&mut vertex, Orientation::Reversed);
        Ok(vertex)
    }
}

#[test]
fn stored_tool_shared_and_mutable_references_address_the_same_object() {
    let source = TopOpeBRepDSBuildTool::new();
    let mut builder = TopOpeBRepBuildBuilder::new(&source);
    let shared = std::ptr::from_ref(builder.build_tool());
    let mutable = std::ptr::from_mut(builder.change_build_tool());
    assert_eq!(shared, mutable.cast_const());
    builder.change_build_tool().set_over_write(false);
    builder.change_build_tool().set_translate(false);
    assert!(!builder.build_tool().over_write());
    assert!(!builder.build_tool().translate());
    assert!(source.over_write());
    assert!(source.translate());
}

#[test]
fn stored_tool_is_copied_from_the_constructor_argument() {
    let mut source = TopOpeBRepDSBuildTool::new();
    source.set_over_write(false);
    source.set_translate(false);
    let builder = TopOpeBRepBuildBuilder::new(&source);
    source.set_over_write(true);
    source.set_translate(true);
    assert!(!builder.build_tool().over_write());
    assert!(!builder.build_tool().translate());
    assert!(source.over_write());
    assert!(source.translate());
}

#[test]
fn external_operation_uses_the_stored_tool_to_construct_geometry() -> Result<(), Failure> {
    let source = TopOpeBRepDSBuildTool::new();
    let mut operation = VertexOperation {
        builder: TopOpeBRepBuildBuilder::new(&source),
    };
    let point = TopOpeBRepDSPoint::new(Point3::new(2.0, -3.0, 5.0), 0.00002);
    let shape = operation.make_reversed_vertex(&point)?;
    let vertex = topods::vertex(&shape)?;
    assert_eq!(BRepTool::pnt(&vertex)?, point.point());
    assert_eq!(BRepTool::vertex_tolerance(&vertex)?, point.tolerance());
    assert_eq!(shape.orientation(), Orientation::Reversed);
    assert_eq!(
        operation.builder.build_tool().orientation(&shape),
        Orientation::Reversed
    );
    Ok(())
}
