use super::*;

fn square() -> Vec<RenderVertex> {
    [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]))
        .to_vec()
}

#[test]
fn frame_keeps_fine_mesh_close_and_chooses_coarse_when_subpixel() {
    let level = RenderMeshLevel::new(square(), vec![0, 1, 2, 0, 2, 3], None, 0.01).unwrap();
    let mut subdivided = square();
    subdivided.push(RenderVertex::new([0.; 3], [1.; 4]));
    let draw = PrimitiveDraw::new(
        subdivided,
        vec![0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4],
        crate::identity_clip(),
    )
    .unwrap()
    .with_mesh_levels(vec![level], 0.5)
    .unwrap();
    let fine = crate::RenderFrame::new(256, 256, [0.; 4], vec![draw.clone()]).unwrap();
    let coarse = crate::RenderFrame::new(32, 32, [0.; 4], vec![draw]).unwrap();
    assert_eq!(fine.draws()[0].indices.len(), 12);
    assert_eq!(coarse.draws()[0].indices.len(), 6);
    assert!(fine.uses_mesh_lod && coarse.uses_mesh_lod);
    assert!(coarse.draws()[0].mesh_lod.is_none());
}

#[test]
fn projection_retains_fine_mesh_when_error_can_cross_the_camera_plane() {
    let mut perspective = crate::identity_clip();
    perspective[11] = 1.;
    perspective[15] = 0.1;
    let bounds = ([-1., -1., 0.], [1., 1., 0.]);
    let projected = ProjectionBounds::of(perspective, bounds, [64, 64]).unwrap();
    assert!(projected.error(0.11).is_infinite());
    assert!(projected.error(0.01).is_finite());
}

#[test]
fn renderer_selects_a_coarser_cylinder_tessellation_after_projection() {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    use martensite_tkv3d::std_prs::StdPrsShadedShape;

    let mut cylinder = BRepPrimAPIMakeCylinder::new(5., 3.);
    let part = cylinder.shape().unwrap();
    let fine = StdPrsShadedShape::tessellate_level(part, 0.01, 0.15).unwrap();
    let coarse = StdPrsShadedShape::tessellate_level(part, 0.3, 0.8).unwrap();
    let fine = RenderMeshLevel::from_triangle_array(&fine, 0.01).unwrap();
    let coarse = RenderMeshLevel::from_triangle_array(&coarse, 0.3).unwrap();
    let coarse_indices = coarse.indices.len();
    let mut distant = crate::identity_clip();
    distant[0] = 0.01;
    distant[5] = 0.01;
    let draw = PrimitiveDraw::new(fine.vertices.to_vec(), fine.indices.to_vec(), distant)
        .unwrap()
        .with_mesh_levels(vec![coarse], 0.5)
        .unwrap();
    let frame = crate::RenderFrame::new(64, 64, [0.; 4], vec![draw]).unwrap();
    assert_eq!(frame.draws()[0].indices.len(), coarse_indices);
}

#[test]
fn level_approval_stops_matching_an_edited_primary_array() {
    use martensite_tkernel::standard::Handle;
    let primary = Graphic3dArrayOfTriangles::new(3, 0, 0).unwrap();
    for [x, y, z] in [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]] {
        primary.add_vertex(x, y, z).unwrap();
    }
    let source = PrimitiveSubmission {
        kind: Graphic3dTypeOfPrimitiveArray::Triangles,
        indices: primary.indices(),
        attributes: primary.attributes(),
        bounds: Handle::default(),
        upload: Default::default(),
        uid: 0,
        retained: None,
        mesh_levels: None,
    };
    let level = RenderMeshLevel::new(square(), vec![0, 1, 2], None, 0.05).unwrap();
    let plan = Arc::new(MeshAlternates::new(vec![level], 0.5).unwrap());
    let submitted = SubmittedMeshLevels::bind(&source, plan).unwrap();
    assert!(submitted.primary.matches(&source));
    primary.set_vertice(1, 0.25, 0., 0.).unwrap();
    assert!(!submitted.primary.matches(&source));
}

#[cfg(not(feature = "command-state-diagnostics"))]
#[path = "mesh_lod_ais_tests.rs"]
mod ais;
