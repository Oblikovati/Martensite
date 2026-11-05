//! Stable node cursors observe legal list edits after initialization.
use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet,
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Builder, Compound, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_lib::BRepLibMakePolygon;

fn unique_shape() -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    compound.into_shape()
}

#[test]
fn appending_a_shape_after_initialization_is_visible_to_the_current_node() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let first = unique_shape();
    let second = unique_shape();
    set.add_shape(&first)?;
    set.init_shapes();
    set.add_shape(&second)?;
    set.next_shape();
    assert!(set.more_shapes());
    assert!(set.shape()?.is_same(&second));
    Ok(())
}

#[test]
fn editing_start_nodes_is_visible_without_reinitializing_the_cursor() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let first = unique_shape();
    let second = unique_shape();
    set.change_start_shapes().append(first);
    set.init_start_elements();
    *set.change_start_shapes().first_mut().unwrap() = second.clone();
    assert!(set.start_element()?.is_same(&second));
    set.change_start_shapes().append(unique_shape());
    set.next_start_element();
    assert!(set.more_start_elements());
    Ok(())
}

#[test]
fn incident_iteration_retains_the_actual_indexed_list() -> Result<(), Failure> {
    let wire =
        BRepLibMakePolygon::from_two_points(Point3::default(), Point3::new(1., 0., 0.))?.wire()?;
    let edge = Explorer::new(wire.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?
        .current()
        .clone();
    let vertex = Explorer::new(&edge, ShapeEnum::Vertex, ShapeEnum::Shape)?
        .current()
        .clone();
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    set.add_element(&edge)?;
    set.init_neighbours(&edge)?;
    let appended = unique_shape();
    set.shape_set_base_mut()
        .my_sub_shape_map
        .change_from_key(&vertex)?
        .append(appended.clone());
    set.next_neighbour()?;
    assert!(set.neighbour()?.is_same(&appended));
    Ok(())
}

#[test]
fn copied_sets_retain_their_existing_independent_iteration_contract() -> Result<(), Failure> {
    let mut original = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let first = unique_shape();
    original.add_shape(&first)?;
    original.init_shapes();
    let mut copied = original.clone();
    original.add_shape(&unique_shape())?;
    assert!(copied.shape()?.is_same(&first));
    copied.next_shape();
    assert!(!copied.more_shapes());
    original.next_shape();
    assert!(original.more_shapes());
    Ok(())
}

struct BorrowedProvider {
    base: martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildShapeSetBase,
    retained: martensite_tkernel::ncollection::List<Shape>,
}

impl TopOpeBRepBuildShapeSet for BorrowedProvider {
    fn shape_set_base(
        &self,
    ) -> &martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildShapeSetBase {
        &self.base
    }
    fn shape_set_base_mut(
        &mut self,
    ) -> &mut martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }
    fn borrowed_neighbours(
        &mut self,
        _: &Shape,
        _: &Shape,
    ) -> Result<&martensite_tkernel::ncollection::List<Shape>, Failure> {
        Ok(&self.retained)
    }
    fn neighbours_source(&self) -> Result<&martensite_tkernel::ncollection::List<Shape>, Failure> {
        Ok(&self.retained)
    }
}

#[test]
fn external_borrowed_provider_retains_source_nodes_after_preparation() -> Result<(), Failure> {
    let wire =
        BRepLibMakePolygon::from_two_points(Point3::default(), Point3::new(1., 0., 0.))?.wire()?;
    let edge = Explorer::new(wire.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?
        .current()
        .clone();
    let mut provider = BorrowedProvider {
        base: martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildShapeSetBase::new(
            ShapeEnum::Vertex,
            false,
        )?,
        retained: Default::default(),
    };
    let first = unique_shape();
    provider.retained.append(first);
    provider.init_neighbours(&edge)?;
    let replacement = unique_shape();
    *provider.retained.first_mut().unwrap() = replacement.clone();
    assert!(provider.neighbour()?.is_same(&replacement));
    let appended = unique_shape();
    provider.retained.append(appended.clone());
    provider.next_neighbour()?;
    assert!(provider.neighbour()?.is_same(&appended));
    Ok(())
}
