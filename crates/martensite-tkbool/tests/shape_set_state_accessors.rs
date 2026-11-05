//! Existing accessors expose protected start-list and check-flag state without new fields.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::{TopOpeBRepBuildShapeSet, TopOpeBRepBuildShapeSetBase};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

struct AccessorEdgeSet {
    base: TopOpeBRepBuildShapeSetBase,
}

impl AccessorEdgeSet {
    fn new() -> Result<Self, Failure> {
        Ok(Self {
            base: TopOpeBRepBuildShapeSetBase::new(ShapeEnum::Vertex, true)?,
        })
    }

    fn replace_start_edges(&mut self, edges: &[Shape]) {
        *self.change_start_shapes() = List::from_iter(edges.iter().cloned());
    }
}

impl TopOpeBRepBuildShapeSet for AccessorEdgeSet {
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase {
        &self.base
    }
    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }
}

#[test]
fn start_list_accessor_payload_is_live() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut set = AccessorEdgeSet::new()?;
    let shared = std::ptr::from_ref(set.start_elements());
    let mutable = std::ptr::from_mut(set.change_start_shapes());
    assert_eq!(shared, mutable.cast_const());
    set.add_start_element(&first)?;
    set.replace_start_edges(std::slice::from_ref(&second));
    assert_eq!(set.start_elements().len(), 1);
    let start = set.start_elements().iter().next().expect("one start");
    assert!(start.is_same(&second));
    assert_eq!(shared, std::ptr::from_ref(set.start_elements()));
    Ok(())
}

#[test]
fn start_list_edits_preserve_the_dedup_index() -> Result<(), Failure> {
    let [first, second] = connected_edges()?;
    let mut set = AccessorEdgeSet::new()?;
    set.add_start_element(&first)?;
    set.replace_start_edges(std::slice::from_ref(&second));
    set.add_start_element(&first)?;
    assert_eq!(set.start_elements().len(), 1);
    set.add_start_element(&second)?;
    let starts = set.start_elements();
    assert_eq!(starts.len(), 2);
    assert!(starts.iter().all(|shape| shape.is_same(&second)));
    Ok(())
}

fn assert_enabled_shape_check(set: &mut AccessorEdgeSet, valid: &Shape) -> Result<(), Failure> {
    set.set_check_shape(true);
    assert!(set.check_shape());
    let null = Shape::default();
    let failure = set
        .check_shape_of(&null, false)
        .expect_err("checking null fails");
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert_eq!(failure.message(), "BRepCheck_Analyzer::Init() - NULL shape");
    assert!(set.check_shape());
    assert!(set.check_shape_of(valid, false)?);
    Ok(())
}

#[test]
fn check_flag_survives_a_validation_failure() -> Result<(), Failure> {
    let [valid, _] = connected_edges()?;
    let mut set = AccessorEdgeSet::new()?;
    let null = Shape::default();
    assert!(!set.check_shape());
    assert!(set.check_shape_of(&null, false)?);
    assert_enabled_shape_check(&mut set, &valid)?;
    set.set_check_shape(false);
    assert!(!set.check_shape());
    assert!(set.check_shape_of(&null, false)?);
    Ok(())
}

#[test]
fn debug_number_accessors_preserve_the_complete_signed_value() -> Result<(), Failure> {
    let mut set = AccessorEdgeSet::new()?;
    assert_eq!(set.deb_number(), 0);
    for number in [i32::MIN, -1, 0, 1, i32::MAX] {
        set.set_deb_number(number);
        assert_eq!(set.deb_number(), number);
    }
    Ok(())
}
