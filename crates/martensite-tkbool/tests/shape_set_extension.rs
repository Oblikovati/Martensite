//! An external shape-set implementation can specialize its protected traversal kinds.
use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet, TopOpeBRepBuildShapeSetBase,
};
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_lib::BRepLibMakePolygon;

struct EdgeNeighbourSet {
    base: TopOpeBRepBuildShapeSetBase,
}

impl EdgeNeighbourSet {
    fn new() -> Result<Self, Failure> {
        let mut base = TopOpeBRepBuildShapeSetBase::new(ShapeEnum::Vertex, false)?;
        assert_eq!(base.my_shape_type, ShapeEnum::Edge);
        assert_eq!(base.my_sub_shape_type, ShapeEnum::Vertex);
        base.my_shape_type = ShapeEnum::Face;
        base.my_sub_shape_type = ShapeEnum::Edge;
        Ok(Self { base })
    }
}

impl TopOpeBRepBuildShapeSet for EdgeNeighbourSet {
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase {
        &self.base
    }
    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }
}

#[test]
fn external_specialization_controls_the_subshape_index() -> Result<(), Failure> {
    let wire =
        BRepLibMakePolygon::from_two_points(Point3::default(), Point3::new(1., 0., 0.))?.wire()?;
    let explorer = WireExplorer::with_wire(&wire)?;
    let edge = explorer.current().as_ref().clone();
    let mut set = EdgeNeighbourSet::new()?;
    set.add_element(wire.as_ref())?;
    let neighbours = set.make_neighbours_list(wire.as_ref(), &edge)?;
    assert_eq!(set.base.my_shape_type, ShapeEnum::Face);
    assert_eq!(neighbours.len(), 1);
    assert!(
        neighbours
            .iter()
            .next()
            .expect("one neighbour")
            .is_same(wire.as_ref())
    );
    Ok(())
}

#[test]
fn unsupported_shape_kinds_keep_the_defined_constructor_refusal() {
    for kind in [
        ShapeEnum::Compound,
        ShapeEnum::CompSolid,
        ShapeEnum::Solid,
        ShapeEnum::Shell,
        ShapeEnum::Face,
        ShapeEnum::Wire,
        ShapeEnum::Shape,
    ] {
        let Err(failure) = TopOpeBRepBuildPlainShapeSet::new(kind, false) else {
            panic!("unsupported shape kind {kind:?} unexpectedly constructed");
        };
        assert_eq!(failure.exception_type(), "Standard_ProgramError");
        assert_eq!(failure.message(), "ShapeSet : bad ShapeType");
    }
}
