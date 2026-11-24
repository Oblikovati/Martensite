use super::*;

#[test]
fn partitions_keep_original_primitive_order_and_fit_device_workgroups() {
    let vertices: Vec<_> = (0..301)
        .map(|i| RenderVertex::new([i as f32, (i % 2) as f32, 0.], [1.; 4]))
        .collect();
    let indices: Vec<_> = (0..299).flat_map(|i| [i, i + 1, i + 2]).collect();
    let output = build(&vertices, &indices).unwrap();
    assert!(output.groups.len() > 1);
    let reconstructed: Vec<_> = output
        .groups
        .iter()
        .flat_map(|group| {
            assert!(group.vertex_count <= MAX_VERTICES && group.triangle_count <= MAX_TRIANGLES);
            let local = &output.vertices[group.first_vertex..][..group.vertex_count];
            output.triangles[group.first_triangle..][..group.triangle_count]
                .iter()
                .flat_map(|triangle| triangle.map(|index| local[index as usize]))
        })
        .collect();
    assert_eq!(reconstructed, indices);
}

#[test]
fn a_planar_meshlet_has_a_narrow_cone_and_reversed_faces_have_none() {
    let vertices = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]));
    let flat = build(&vertices, &[0, 1, 2]).unwrap();
    let cone = flat.groups[0].cone.unwrap();
    assert_eq!(flat.groups[0].minimum, [0.; 3]);
    assert_eq!(flat.groups[0].maximum, [1., 1., 0.]);
    assert_eq!(cone.0, [0., 0., 1.]);
    assert!(cone.1 < 0.002);
    assert!(
        build(&vertices, &[0, 1, 2, 0, 2, 1]).unwrap().groups[0]
            .cone
            .is_none()
    );
    assert!(build(&vertices, &[0, 1, 3]).is_none());
}
