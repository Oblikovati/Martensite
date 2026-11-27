use super::*;

#[test]
fn a_placement_clip_is_the_camera_product_times_its_world_matrix() {
    let mut world = crate::identity_clip();
    world[12] = 3.;
    let mut projection_view = crate::identity_clip();
    projection_view[0] = 0.5;
    let instances = DrawInstances {
        templates: Vec::new(),
        placements: Arc::from([]),
        projection_view,
        visibility: None,
        hiz_history: None,
    };
    let placement = InstancePlacement {
        template: 0,
        world,
        normal: crate::identity_clip(),
    };
    let clip = instances.clip(&placement);
    assert_eq!(clip[0], 0.5);
    assert_eq!(
        clip[12], 1.5,
        "the translation goes through the camera product"
    );
}

#[test]
fn instance_references_compare_by_identity() {
    let shared = Arc::new(DrawInstances {
        templates: Vec::new(),
        placements: Arc::from([]),
        projection_view: crate::identity_clip(),
        visibility: None,
        hiz_history: None,
    });
    let other = Arc::new(DrawInstances {
        templates: Vec::new(),
        placements: Arc::from([]),
        projection_view: crate::identity_clip(),
        visibility: None,
        hiz_history: None,
    });
    assert_eq!(DrawInstancesRef(shared.clone()), DrawInstancesRef(shared));
    assert_ne!(
        DrawInstancesRef(other.clone()),
        DrawInstancesRef(Arc::new(DrawInstances {
            templates: Vec::new(),
            placements: Arc::from([]),
            projection_view: crate::identity_clip(),
            visibility: None,
            hiz_history: None,
        }))
    );
}

#[test]
fn placement_bounds_cover_each_finite_vertex_and_decline_directions_at_infinity() {
    let mut vertices = vec![
        crate::RenderVertex::new([-1., 2., 4.], [1.; 4]),
        crate::RenderVertex::new([1., -2., 3.], [1.; 4]),
        crate::RenderVertex::new([2., 1., 5.], [1.; 4]),
    ];
    let triangle =
        |vertices| PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
    let bounds = InstanceBounds::of(&triangle(vertices.clone())).unwrap();
    assert_eq!(bounds.minimum, [-1., -2., 3.]);
    assert_eq!(bounds.maximum, [2., 2., 5.]);
    vertices[0].weight = 0.;
    assert!(InstanceBounds::of(&triangle(vertices)).is_none());
}
