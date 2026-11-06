//! External neighbour selection uses inherited candidate, current-shape and result state.
use martensite_tkbool::top_ope_brep_build::{TopOpeBRepBuildShapeSet, TopOpeBRepBuildShapeSetBase};
use martensite_tkbrep::{brep_tools::WireExplorer, topods::Shape};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_lib::BRepLibMakePolygon;

struct OtherEdgeSet {
    base: TopOpeBRepBuildShapeSetBase,
    calls: usize,
}

impl OtherEdgeSet {
    fn new() -> Result<Self, Failure> {
        Ok(Self {
            base: TopOpeBRepBuildShapeSetBase::new(ShapeEnum::Vertex, false)?,
            calls: 0,
        })
    }
}

impl TopOpeBRepBuildShapeSet for OtherEdgeSet {
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase {
        &self.base
    }
    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }

    fn make_neighbours_list(
        &mut self,
        edge: &Shape,
        vertex: &Shape,
    ) -> Result<List<Shape>, Failure> {
        assert!(self.base.my_current_shape.is_same(edge));
        self.calls += 1;
        let candidates = self.base.my_sub_shape_map.find_from_key(vertex)?;
        self.base.my_current_shape_neighbours.clear();
        for candidate in candidates
            .iter()
            .filter(|candidate| !candidate.is_same(edge))
        {
            self.base
                .my_current_shape_neighbours
                .append(candidate.clone());
        }
        Ok(self.base.my_current_shape_neighbours.clone())
    }
}

pub(super) fn connected_edges() -> Result<[Shape; 2], Failure> {
    let wire = BRepLibMakePolygon::from_three_points(
        Point3::default(),
        Point3::new(1., 0., 0.),
        Point3::new(2., 0., 0.),
        false,
    )?
    .wire()?;
    let mut explorer = WireExplorer::with_wire(&wire)?;
    let first = explorer.current().as_ref().clone();
    explorer.next()?;
    Ok([first, explorer.current().as_ref().clone()])
}

#[test]
fn derived_neighbour_selection_skips_self_and_advances_across_empty_vertices() -> Result<(), Failure>
{
    let [first, second] = connected_edges()?;
    let mut set = OtherEdgeSet::new()?;
    set.add_element(&first)?;
    set.add_element(&second)?;
    set.init_neighbours(&first)?;
    assert!(set.more_neighbours());
    assert!(set.neighbour()?.is_same(&second));
    assert_eq!(set.calls, 2);
    assert_eq!(set.base.my_current_shape_neighbours.len(), 1);
    assert!(set.base.my_current_shape.is_same(&first));
    set.next_neighbour()?;
    assert!(!set.more_neighbours());
    Ok(())
}

#[test]
fn inherited_candidate_index_supports_the_reverse_query() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut set = OtherEdgeSet::new()?;
    set.add_element(&first)?;
    set.add_element(&second)?;
    set.init_neighbours(&second)?;
    assert!(set.more_neighbours());
    assert!(set.neighbour()?.is_same(&first));
    assert!(set.base.my_current_shape.is_same(&second));
    set.next_neighbour()?;
    assert!(!set.more_neighbours());
    assert!(set.base.my_current_shape_neighbours.is_empty());
    Ok(())
}
