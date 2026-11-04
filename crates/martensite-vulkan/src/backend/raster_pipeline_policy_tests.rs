use super::*;

#[test]
fn coordinates_and_provoking_policy_preserve_conservative_raster_nodes() {
    for coordinates in [
        AttachmentCoordinates::LegacyBottomUpV1,
        AttachmentCoordinates::TopLeftLogicalV2,
    ] {
        assert_conservative_node_survives(coordinates, false);
        assert_conservative_node_survives(coordinates, true);
    }
}

fn assert_conservative_node_survives(coordinates: AttachmentCoordinates, provoking: bool) {
    let mut conservative = vk::PipelineRasterizationConservativeStateCreateInfoEXT::default();
    let raster = vk::PipelineRasterizationStateCreateInfo::default()
        .front_face(vk::FrontFace::CLOCKWISE)
        .push_next(&mut conservative);
    let original_chain = raster.p_next;
    let mut last_vertex = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT::default();
    let provoking_address = std::ptr::from_ref(&last_vertex).cast();
    let selected = select_raster(&raster, coordinates, provoking, &mut last_vertex);
    assert_chain_preserved(
        selected.p_next,
        original_chain,
        provoking_address,
        provoking,
    );
    assert_v3_rejects_conservative_state(&selected);
}

fn assert_chain_preserved(
    selected: *const std::ffi::c_void,
    original: *const std::ffi::c_void,
    provoking_address: *const std::ffi::c_void,
    provoking: bool,
) {
    if provoking {
        assert_eq!(selected, provoking_address);
        // The selected pNext chain retains this local node until the synchronous policy call.
        // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
        let node = unsafe {
            &*provoking_address.cast::<vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT>()
        };
        assert_eq!(node.p_next, original);
    } else {
        assert_eq!(selected, original);
    }
}

fn assert_v3_rejects_conservative_state(raster: &vk::PipelineRasterizationStateCreateInfo<'_>) {
    let rejected = super::super::phong_projected_fill::require_state(
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        true,
        raster,
        2,
        false,
    );
    assert!(rejected.is_err());
}
