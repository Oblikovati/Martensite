//! Independent interval and mesh-incidence contracts for shared geometry rendering.
use martensite_raster_core::{
    MeshEdgeSets,
    sampling::{bound, interval_bounds},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkmath::{
    gp::Point3,
    poly::{Triangle, Triangulation},
};

fn sample_interval_limits() -> Array1<'static, f64> {
    interval_bounds(2, |values| {
        values.set_value(1, -2.).unwrap();
        values.set_value(2, 0.).unwrap();
        values.set_value(3, 5.).unwrap();
        Ok(())
    })
    .unwrap()
}

#[test]
fn interval_samples_keep_one_based_bounds_and_typed_failures() {
    let bounds = sample_interval_limits();
    assert_eq!(
        [
            bound(&bounds, 1).unwrap(),
            bound(&bounds, 2).unwrap(),
            bound(&bounds, 3).unwrap()
        ],
        [-2., 0., 5.]
    );
    assert_interval_errors(&bounds);
}

fn square_mesh() -> Handle<Triangulation> {
    let mesh = Triangulation::with_sizes(4, 2, false, false).unwrap();
    for (index, (x, y)) in [(0., 0.), (1., 0.), (1., 1.), (0., 1.)]
        .into_iter()
        .enumerate()
    {
        mesh.set_node(index as i32 + 1, Point3::new(x, y, 0.))
            .unwrap();
    }
    mesh.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    mesh.set_triangle(2, Triangle::new(1, 3, 4)).unwrap();
    Handle::new(mesh)
}

#[test]
fn square_mesh_has_four_boundary_edges_and_one_shared_diagonal() {
    let sets = MeshEdgeSets::compute(&square_mesh()).unwrap();
    assert_eq!(sets.free, vec![(1, 2), (2, 3), (3, 4), (4, 1)]);
    assert_eq!(sets.internal, vec![(3, 1)]);
    assert_eq!(
        MeshEdgeSets::compute(&Handle::default()).unwrap(),
        MeshEdgeSets::default()
    );
}

fn assert_interval_errors(bounds: &Array1<'_, f64>) {
    assert_eq!(
        bound(bounds, 0).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        bound(bounds, 4).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
