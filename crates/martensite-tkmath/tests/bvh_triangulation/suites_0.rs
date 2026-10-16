// =============================================================================
// BvhTriangulation Basic Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_default_constructor() {
    let aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    assert_eq!(aTriangulation.size(), 0);
    assert_eq!(array::size(&aTriangulation.vertices), 0);
    assert_eq!(array::size(&aTriangulation.elements), 0);
}

#[test]
fn hierarchy_triangulation_add_single_triangle() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 1.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    assert_eq!(aTriangulation.size(), 1);
    assert_eq!(array::size(&aTriangulation.vertices), 3);
}

#[test]
fn hierarchy_triangulation_add_multiple_triangles() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    for i in 0..5 {
        let x = i as f64;
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(x, 0.0, 0.0));
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 1.0, 0.0, 0.0),
        );
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 0.5, 1.0, 0.0),
        );
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    assert_eq!(aTriangulation.size(), 5);
    assert_eq!(array::size(&aTriangulation.vertices), 15);
}

// =============================================================================
// BvhTriangulation Box Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_box_for_single_triangle() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 2.0, 3.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(4.0, 1.0, 2.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(2.0, 3.0, 1.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    let aBox = aTriangulation.element_box(0).unwrap();

    // Min point should be componentwise min of all vertices
    assert_near(aBox.corner_min().x, 1.0, 1e-10);
    assert_near(aBox.corner_min().y, 1.0, 1e-10);
    assert_near(aBox.corner_min().z, 1.0, 1e-10);

    // Max point should be componentwise max of all vertices
    assert_near(aBox.corner_max().x, 4.0, 1e-10);
    assert_near(aBox.corner_max().y, 3.0, 1e-10);
    assert_near(aBox.corner_max().z, 3.0, 1e-10);
}

#[test]
fn hierarchy_triangulation_box_for_degenerate_triangle() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Triangle with all vertices at same point (degenerate)
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 1.0, 1.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 1.0, 1.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 1.0, 1.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    let aBox = aTriangulation.element_box(0).unwrap();

    assert_near(aBox.corner_min().x, 1.0, 1e-10);
    assert_near(aBox.corner_min().y, 1.0, 1e-10);
    assert_near(aBox.corner_min().z, 1.0, 1e-10);
    assert_near(aBox.corner_max().x, 1.0, 1e-10);
    assert_near(aBox.corner_max().y, 1.0, 1e-10);
    assert_near(aBox.corner_max().z, 1.0, 1e-10);
}

#[test]
fn hierarchy_triangulation_box_for_flat_triangle() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Triangle flat in XY plane (Z = 0)
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(2.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 3.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    let aBox = aTriangulation.element_box(0).unwrap();

    assert_near(aBox.corner_min().z, 0.0, 1e-10);
    assert_near(aBox.corner_max().z, 0.0, 1e-10);
    assert_near(aBox.corner_max().x, 2.0, 1e-10);
    assert_near(aBox.corner_max().y, 3.0, 1e-10);
}

// =============================================================================
// BvhTriangulation Center Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_center_computation() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Triangle with vertices at (0,0,0), (3,0,0), (0,3,0)
    // Centroid should be at (1,1,0)
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(3.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 3.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    assert_near(aTriangulation.center(0, 0).unwrap(), 1.0, 1e-10); // X centroid
    assert_near(aTriangulation.center(0, 1).unwrap(), 1.0, 1e-10); // Y centroid
    assert_near(aTriangulation.center(0, 2).unwrap(), 0.0, 1e-10); // Z centroid
}

#[test]
fn hierarchy_triangulation_center_along_x_axis() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(2.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(3.0, 0.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    // Centroid X = (1 + 2 + 3) / 3 = 2.0
    assert_near(aTriangulation.center(0, 0).unwrap(), 2.0, 1e-10);
}

#[test]
fn hierarchy_triangulation_center_along_y_axis() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 2.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 4.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 6.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    // Centroid Y = (2 + 4 + 6) / 3 = 4.0
    assert_near(aTriangulation.center(0, 1).unwrap(), 4.0, 1e-10);
}

#[test]
fn hierarchy_triangulation_center_along_z_axis() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 1.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 4.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 7.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    // Centroid Z = (1 + 4 + 7) / 3 = 4.0
    assert_near(aTriangulation.center(0, 2).unwrap(), 4.0, 1e-10);
}

// =============================================================================
// BvhTriangulation Swap Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_swap_two_triangles() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Triangle 0
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 1.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    // Triangle 1
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(5.0, 5.0, 5.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(6.0, 5.0, 5.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(5.0, 6.0, 5.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(3, 4, 5, 0));

    let aCentroid0Before = aTriangulation.center(0, 0).unwrap();
    let aCentroid1Before = aTriangulation.center(1, 0).unwrap();

    aTriangulation.swap(0, 1).unwrap();

    // After swap, centroids should be swapped
    assert_near(
        aTriangulation.center(0, 0).unwrap(),
        aCentroid1Before,
        1e-10,
    );
    assert_near(
        aTriangulation.center(1, 0).unwrap(),
        aCentroid0Before,
        1e-10,
    );
}

#[test]
fn hierarchy_triangulation_swap_preserves_vertices() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Add vertices and triangles
    for i in 0..3 {
        let x = i as f64 * 10.0;
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(x, 0.0, 0.0));
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 1.0, 0.0, 0.0),
        );
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 0.5, 1.0, 0.0),
        );
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let aVertexCount = array::size(&aTriangulation.vertices);

    aTriangulation.swap(0, 2).unwrap();

    // Vertex count should not change
    assert_eq!(array::size(&aTriangulation.vertices), aVertexCount);
}

// =============================================================================
// BvhTriangulation 2D Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_triangulation2_d() {
    let mut aTriangulation2D = BvhTriangulation::<BvhVec2d>::default();

    array::append(&mut aTriangulation2D.vertices, BvhVec2d::new(0.0, 0.0));
    array::append(&mut aTriangulation2D.vertices, BvhVec2d::new(1.0, 0.0));
    array::append(&mut aTriangulation2D.vertices, BvhVec2d::new(0.5, 1.0));
    array::append(&mut aTriangulation2D.elements, BvhVec4i::new(0, 1, 2, 0));

    assert_eq!(aTriangulation2D.size(), 1);

    let aBox = aTriangulation2D.element_box(0).unwrap();
    assert_near(aBox.corner_min().x, 0.0, 1e-10);
    assert_near(aBox.corner_min().y, 0.0, 1e-10);
    assert_near(aBox.corner_max().x, 1.0, 1e-10);
    assert_near(aBox.corner_max().y, 1.0, 1e-10);
}

// =============================================================================
// BvhTriangulation Shared Vertices Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_shared_vertices() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    // Create quad with 4 vertices, 2 triangles sharing edge
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 1.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 1.0, 0.0));

    // Triangle 0: (0, 1, 2)
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));
    // Triangle 1: (0, 2, 3) - shares vertices 0 and 2 with triangle 0
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 2, 3, 0));

    assert_eq!(aTriangulation.size(), 2);
    assert_eq!(array::size(&aTriangulation.vertices), 4);

    // Both triangles should have valid boxes
    let aBox0 = aTriangulation.element_box(0).unwrap();
    let aBox1 = aTriangulation.element_box(1).unwrap();

    assert!(!aBox0.is_out(&aBox1)); // Should overlap
}

// =============================================================================
// BvhTriangulation Float Precision Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_float_precision() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3f>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3f::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3f::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3f::new(0.0, 1.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    assert_eq!(aTriangulation.size(), 1);

    let aBox = aTriangulation.element_box(0).unwrap();
    assert_near_f32(aBox.corner_max().x, 1.0, 1e-6);
    assert_near_f32(aBox.corner_max().y, 1.0, 1e-6);
}

// =============================================================================
// BvhTriangulation Negative Coordinates Tests
// =============================================================================

#[test]
fn hierarchy_triangulation_negative_coordinates() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(
        &mut aTriangulation.vertices,
        BvhVec3d::new(-1.0, -2.0, -3.0),
    );
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, -1.0, -2.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, -1.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    let aBox = aTriangulation.element_box(0).unwrap();

    assert_near(aBox.corner_min().x, -1.0, 1e-10);
    assert_near(aBox.corner_min().y, -2.0, 1e-10);
    assert_near(aBox.corner_min().z, -3.0, 1e-10);
    assert_near(aBox.corner_max().x, 1.0, 1e-10);
    assert_near(aBox.corner_max().y, 0.0, 1e-10);
    assert_near(aBox.corner_max().z, -1.0, 1e-10);
}
