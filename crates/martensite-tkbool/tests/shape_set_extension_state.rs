//! Extension-owned cursor sources preserve their nodes independently of the stored lists.
use martensite_tkbool::top_ope_brep_build::{
    ShapeSetExtensionState, ShapeSetExtensionStateRef, TopOpeBRepBuildPlainShapeSet,
    TopOpeBRepBuildShapeSet,
};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

fn state(set: &mut TopOpeBRepBuildPlainShapeSet) -> ShapeSetExtensionState<'_> {
    set.shape_set_base_mut().extension_state()
}

fn unique_shape() -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    compound.into_shape()
}

fn shape_list() -> (List<Shape>, Shape) {
    let shape = unique_shape();
    let mut list = List::default();
    list.append(shape.clone());
    (list, shape)
}

#[test]
fn owned_shape_rebinding_preserves_nodes_and_live_mutations() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let stored = unique_shape();
    state(&mut set).shapes_mut().append(stored.clone());
    let (source, first) = shape_list();
    let position = source.cursor();
    state(&mut set).bind_shapes(source);
    assert_eq!(*state(&mut set).shapes_cursor_mut(), position);
    assert!(set.shape()?.is_same(&first));
    let appended = unique_shape();
    state(&mut set).shape_source_mut().append(appended.clone());
    set.next_shape();
    assert!(set.shape()?.is_same(&appended));
    set.init_shapes();
    assert!(set.shape()?.is_same(&stored));
    Ok(())
}

#[test]
fn start_rebinding_does_not_replace_case_start_elements() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let (source, first) = shape_list();
    let position = source.cursor();
    state(&mut set).bind_starts(source);
    assert_eq!(*state(&mut set).starts_cursor_mut(), position);
    assert!(set.start_element()?.is_same(&first));
    let replacement = unique_shape();
    *state(&mut set).start_source_mut().first_mut().unwrap() = replacement.clone();
    assert!(set.start_element()?.is_same(&replacement));
    assert!(set.start_elements().is_empty());
    set.init_start_elements();
    assert!(!set.more_start_elements());
    Ok(())
}

#[test]
fn incident_rebinding_preserves_the_owned_source_node() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let (source, first) = shape_list();
    let position = source.cursor();
    state(&mut set).bind_incident(source);
    assert_eq!(*state(&mut set).incident_cursor_mut(), position);
    assert!(set.neighbour()?.is_same(&first));
    let replacement = unique_shape();
    *state(&mut set)
        .bound_incident_source_mut()
        .first_mut()
        .unwrap() = replacement.clone();
    assert!(set.neighbour()?.is_same(&replacement));
    set.next_neighbour()?;
    assert!(!set.more_neighbours());
    Ok(())
}

#[test]
fn actual_explorer_can_be_rebound_without_replacing_its_shape_value() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let shape = unique_shape();
    let mut view = state(&mut set);
    view.explorer_mut()
        .init(&shape, ShapeEnum::Compound, ShapeEnum::Shape)?;
    assert!(view.explorer_mut().more());
    assert!(view.explorer_mut().current().is_same(&shape));
    assert_eq!(view.explorer_mut().depth(), 0);
    view.explorer_mut().next()?;
    assert!(!view.explorer_mut().more());
    Ok(())
}

#[test]
fn explicit_extension_state_preserves_thread_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ShapeSetExtensionState<'static>>();
    assert_send_sync::<ShapeSetExtensionStateRef<'static>>();
    assert_send_sync::<TopOpeBRepBuildPlainShapeSet>();
}

#[test]
fn shared_extension_view_reads_all_retained_fields_without_mutable_access() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let shape = unique_shape();
    set.add_shape(&shape)?;
    set.init_shapes();
    let view = set.shape_set_base().extension_state_ref();
    assert!(
        view.shapes_cursor()
            .value(view.shapes())
            .unwrap()
            .is_same(&shape)
    );
    assert!(!view.starts_cursor().more());
    assert!(!view.incident_cursor().more());
    assert!(!view.explorer().more());
    Ok(())
}

#[test]
fn copied_records_rebind_all_owned_source_positions_to_their_copied_nodes() -> Result<(), Failure> {
    let mut original = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let (shapes, shape) = shape_list();
    let (starts, start) = shape_list();
    let (incident, neighbour) = shape_list();
    original
        .shape_set_base_mut()
        .extension_state()
        .bind_shapes(shapes);
    original
        .shape_set_base_mut()
        .extension_state()
        .bind_starts(starts);
    original
        .shape_set_base_mut()
        .extension_state()
        .bind_incident(incident);
    let copied = original.clone();
    assert!(copied.shape()?.is_same(&shape));
    assert!(copied.start_element()?.is_same(&start));
    assert!(copied.neighbour()?.is_same(&neighbour));
    original
        .shape_set_base_mut()
        .extension_state()
        .shape_source_mut()
        .clear();
    assert!(copied.shape()?.is_same(&shape));
    Ok(())
}

#[test]
fn explicit_node_cursor_mutation_controls_the_existing_queries() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    let (mut source, _) = shape_list();
    let last = unique_shape();
    source.append(last.clone());
    let mut view = state(&mut set);
    view.bind_shapes(source);
    let mut position = *view.shapes_cursor_mut();
    position.next(view.shape_source_mut());
    *view.shapes_cursor_mut() = position;
    assert!(set.shape()?.is_same(&last));
    *state(&mut set).shapes_cursor_mut() = Default::default();
    assert!(!set.more_shapes());
    Ok(())
}
