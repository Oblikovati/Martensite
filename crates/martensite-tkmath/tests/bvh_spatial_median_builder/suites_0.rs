// =============================================================================
// BvhSpatialMedianBuilder Basic Tests
// =============================================================================

#[test]
fn hierarchy_spatial_median_builder_default_constructor() {
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    assert_eq!(aBuilder.builder_transient().leaf_node_size(), 5);
    assert_eq!(aBuilder.builder_transient().max_tree_depth(), 32);
}

#[test]
fn hierarchy_spatial_median_builder_custom_parameters() {
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::new(10, 20, true);

    assert_eq!(aBuilder.builder_transient().leaf_node_size(), 10);
    assert_eq!(aBuilder.builder_transient().max_tree_depth(), 20);
}

#[test]
fn hierarchy_spatial_median_builder_build_empty_set() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert_eq!(aTree.length(), 0);
}

#[test]
fn hierarchy_spatial_median_builder_build_single_triangle() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();

    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
    array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 1.0, 0.0));
    array::append(&mut aTriangulation.elements, BvhVec4i::new(0, 1, 2, 0));

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert_eq!(aTree.length(), 1);
    assert!(aTree.is_outer(0).unwrap());
    assert_eq!(aTree.nb_primitives(0).unwrap(), 1);
}

#[test]
fn hierarchy_spatial_median_builder_build_multiple_triangles() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 10;

    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, i as f64);
    }

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert!(aTree.length() > 0);
    assert!(aTree.depth() >= 1);
}

#[test]
fn hierarchy_spatial_median_builder_leaf_node_size_respected() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 20;

    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, i as f64);
    }

    let aLeafSize = 3;
    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::new(aLeafSize, 32, false);

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    // Check all leaf nodes respect the leaf size
    for i in 0..aTree.length() {
        if aTree.is_outer(i).unwrap() {
            assert!(aTree.nb_primitives(i).unwrap() <= aLeafSize);
        }
    }
}

#[test]
fn hierarchy_spatial_median_builder_max_depth_respected() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 100;

    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, i as f64);
    }

    let aMaxDepth = 5;
    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::new(1, aMaxDepth, false);

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert!(aTree.depth() <= aMaxDepth);
}

#[test]
fn hierarchy_spatial_median_builder_splits_at_spatial_median() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 8;

    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, i as f64);
    }

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::new(2, 32, false);

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    // For uniformly distributed data, spatial median should create balanced splits
    // Root node should have roughly equal children
    if aTree.length() > 0 && !aTree.is_outer(0).unwrap() {
        let aLeftChild = aTree.child::<0>(0).unwrap();
        let aRightChild = aTree.child::<1>(0).unwrap();

        if aLeftChild != -1 && aRightChild != -1 {
            let aLeftPrims = aTree.nb_primitives(aLeftChild).unwrap();
            let aRightPrims = aTree.nb_primitives(aRightChild).unwrap();

            // For 8 triangles, expect roughly 4-4 split (allowing some tolerance)
            assert!(aLeftPrims >= 2);
            assert!(aLeftPrims <= 6);
            assert!(aRightPrims >= 2);
            assert!(aRightPrims <= 6);
        }
    }
}

#[test]
fn hierarchy_spatial_median_builder_build2_d() {
    let mut aTriangulation = BvhTriangulation::<BvhVec2d>::default();
    let aNumTriangles = 10;

    for i in 0..aNumTriangles {
        let x = i as f64;

        array::append(&mut aTriangulation.vertices, BvhVec2d::new(x, 0.0));
        array::append(&mut aTriangulation.vertices, BvhVec2d::new(x + 0.5, 1.0));
        array::append(&mut aTriangulation.vertices, BvhVec2d::new(x + 1.0, 0.0));
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let mut aTree = BvhTree::<BvhVec2d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec2d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert!(aTree.length() > 0);
    assert!(aTree.depth() >= 1);
}

#[test]
fn hierarchy_spatial_median_builder_use_main_axis_flag() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 20;

    // Create elongated distribution along X axis
    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, (i * 10) as f64);
    }

    let mut aTreeMainAxis = BvhTree::<BvhVec3d>::default();
    let aBuilderMainAxis = BvhSpatialMedianBuilder::<BvhVec3d>::new(5, 32, true);

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilderMainAxis
        .build(&mut aTriangulation, &mut aTreeMainAxis, &aBox)
        .unwrap();

    assert!(aTreeMainAxis.length() > 0);
}

#[test]
fn hierarchy_spatial_median_builder_compare_with_binned_builder() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 50;

    for i in 0..aNumTriangles {
        append_row_triangle(&mut aTriangulation, i, i as f64);
    }

    let aBox = ComputeSetBox(&aTriangulation);

    // Build with SpatialMedianBuilder (uses 2 bins)
    let mut aTreeMedian = BvhTree::<BvhVec3d>::default();
    let aBuilderMedian = BvhSpatialMedianBuilder::<BvhVec3d>::default();
    aBuilderMedian
        .build(&mut aTriangulation, &mut aTreeMedian, &aBox)
        .unwrap();

    // Build with BinnedBuilder with 2 bins (should be identical)
    let mut aTreeBinned = BvhTree::<BvhVec3d>::default();
    let aBuilderBinned = BvhBinnedBuilder::<BvhVec3d, 2>::default();
    aBuilderBinned
        .build(&mut aTriangulation, &mut aTreeBinned, &aBox)
        .unwrap();

    // Both should produce trees (exact structure may vary but both should be valid)
    assert_eq!(aTreeMedian.length(), aTreeBinned.length());
    assert_eq!(aTreeMedian.depth(), aTreeBinned.depth());
}

#[test]
fn hierarchy_spatial_median_builder_clustered_data() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 30;

    // Create two clusters: 15 triangles at x=0, 15 at x=100
    for i in 0..aNumTriangles {
        let x = if i < 15 { 0.0 } else { 100.0 };
        let y = (i % 15) as f64;

        array::append(&mut aTriangulation.vertices, BvhVec3d::new(x, y, 0.0));
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 0.5, y + 1.0, 0.0),
        );
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(x + 1.0, y, 0.0));
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    // Spatial median should split the two clusters
    assert!(aTree.length() > 0);
    assert!(aTree.depth() >= 1);
}

#[test]
fn hierarchy_spatial_median_builder_float_precision() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3f>::default();
    let aNumTriangles = 10;

    for i in 0..aNumTriangles {
        let x = i as f32;

        array::append(&mut aTriangulation.vertices, BvhVec3f::new(x, 0.0, 0.0));
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3f::new(x + 0.5, 1.0, 0.0),
        );
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3f::new(x + 1.0, 0.0, 0.0),
        );
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let mut aTree = BvhTree::<BvhVec3f>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3f>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert!(aTree.length() > 0);
}

#[test]
fn hierarchy_spatial_median_builder_identical_boxes() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 10;

    // All triangles at the same position
    for i in 0..aNumTriangles {
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 0.0, 0.0));
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(1.0, 0.0, 0.0));
        array::append(&mut aTriangulation.vertices, BvhVec3d::new(0.0, 1.0, 0.0));
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    // Should create a single leaf containing all triangles
    assert!(aTree.length() > 0);
}

#[test]
fn hierarchy_spatial_median_builder_large_data_set() {
    let mut aTriangulation = BvhTriangulation::<BvhVec3d>::default();
    let aNumTriangles = 500;

    // Random-ish distribution
    for i in 0..aNumTriangles {
        let x = ((i * 7) % 100) as f64;
        let y = ((i * 11) % 100) as f64;
        let z = ((i * 13) % 100) as f64;

        array::append(&mut aTriangulation.vertices, BvhVec3d::new(x, y, z));
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 0.5, y + 1.0, z),
        );
        array::append(
            &mut aTriangulation.vertices,
            BvhVec3d::new(x + 1.0, y, z + 1.0),
        );
        array::append(
            &mut aTriangulation.elements,
            BvhVec4i::new(i * 3, i * 3 + 1, i * 3 + 2, 0),
        );
    }

    let mut aTree = BvhTree::<BvhVec3d>::default();
    let aBuilder = BvhSpatialMedianBuilder::<BvhVec3d>::default();

    let aBox = ComputeSetBox(&aTriangulation);
    aBuilder
        .build(&mut aTriangulation, &mut aTree, &aBox)
        .unwrap();

    assert!(aTree.length() > 0);
    assert!(aTree.depth() > 5); // Large dataset should produce deeper tree

    // Estimate SAH quality
    let aSAH = aTree.estimate_sah().unwrap();
    assert!(aSAH > 0.0);
}
