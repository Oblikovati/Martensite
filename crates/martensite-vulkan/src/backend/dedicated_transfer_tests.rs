use super::*;
use crate::{PrimitiveDraw, RenderFrame, RenderVertex, identity_clip};
use ash::vk::Handle;

#[test]
fn select_auxiliary_transfer_queue_with_copy_only_preferred() {
    let graphics = vk::QueueFlags::GRAPHICS | vk::QueueFlags::TRANSFER;
    let compute = vk::QueueFlags::COMPUTE;
    let families = [graphics, compute, vk::QueueFlags::TRANSFER].map(|flags| {
        vk::QueueFamilyProperties::default()
            .queue_flags(flags)
            .queue_count(1)
    });
    let queues = [3, 4, 5].map(vk::Queue::from_raw);
    assert_eq!(select_transfer_queue(&families, &queues).unwrap().family, 2);
    assert_eq!(
        select_transfer_queue(&families[..2], &queues)
            .unwrap()
            .family,
        1
    );
    assert!(select_transfer_queue(&families[..1], &queues).is_none());
}

pub(in crate::backend) fn large_geometry_frame(extent: [u32; 2]) -> RenderFrame {
    let green = [0., 1., 0., 1.];
    let mut vertices = vec![RenderVertex::new([0., 0., 0.], green); 64_000];
    for (vertex, corner) in
        vertices
            .iter_mut()
            .zip([[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]])
    {
        *vertex = RenderVertex::new(corner, green);
    }
    let mut indices = vec![0; 120_000];
    indices[..6].copy_from_slice(&[0, 1, 2, 0, 2, 3]);
    let draw = PrimitiveDraw::new(vertices, indices, identity_clip()).unwrap();
    RenderFrame::new(extent[0], extent[1], [0., 0., 1., 1.], vec![draw]).unwrap()
}

#[test]
fn full_geometry_transfer_matches_inline_color_and_depth() {
    use super::super::{record, resource_lifecycle_test_support::create_resource_renderer};
    use crate::RenderBackend;

    let mut renderer = create_resource_renderer();
    let inline_frame = large_geometry_frame([32, 24]);
    let inline = renderer.prepared_owned_frame(&inline_frame).unwrap();
    let commands = CommandSubmission::new(&renderer.runtime).unwrap();
    record::record_frame(&renderer.runtime, commands.command, &inline, &inline_frame);
    let inline = renderer.finish_owned_frame(commands, inline).unwrap();
    let reference = renderer.read_frame(&inline.readback_ticket()).unwrap();
    let before = renderer.runtime.work.totals().submissions;
    let observed = renderer.render(&large_geometry_frame([32, 24])).unwrap();
    let expected = u64::from(sharing_families(&renderer.runtime).is_some());
    assert_eq!(
        renderer.runtime.work.totals().submissions - before,
        2 + expected
    );
    assert_eq!(reference.rgba, observed.rgba);
    assert_eq!(reference.depth, observed.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn shared_pages_and_sparse_uploads_stay_on_the_graphics_queue() {
    use super::super::resource_lifecycle_test_support::create_resource_renderer;
    use super::super::{
        geometry_residency::GeometryStore, geometry_transfer::geometry_staging_pool,
    };

    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let store = Arc::new(GeometryStore::new(runtime));
    let pool = geometry_staging_pool();
    let vertex = vk::BufferUsageFlags::VERTEX_BUFFER;
    let bytes = vec![0; 2 << 20];
    let mut complete = GeometryTransfers::new(&store);
    let prior = complete
        .upload(runtime, &pool, vertex, Some(32), &bytes)
        .unwrap();
    assert_eq!(
        complete.dedicated_owners().is_some(),
        sharing_families(runtime).is_some()
    );

    let mut sparse = GeometryTransfers::new(&store);
    sparse
        .patch_stream(runtime, &pool, vertex, prior, 4, &bytes[4..])
        .unwrap();
    assert!(sparse.dedicated_owners().is_none());

    let shared = complete
        .upload(runtime, &pool, vertex, Some(32), &[0; 64])
        .unwrap();
    assert!(shared.is_shared());
    assert!(complete.dedicated_owners().is_none());
}

#[test]
fn refusing_graphics_after_enqueuing_the_transfer_allows_a_fresh_retry() {
    use super::super::{record, resource_lifecycle_test_support::create_resource_renderer};
    use crate::{RenderBackend, RenderDeviceFailureClass, RenderDeviceState};

    let mut renderer = create_resource_renderer();
    if sharing_families(&renderer.runtime).is_none() {
        return;
    }
    let frame = large_geometry_frame([32, 24]);
    let resources = renderer.prepared_owned_frame(&frame).unwrap();
    let command = CommandSubmission::new(&renderer.runtime).unwrap();
    record::record_submission_frame(&command, &resources, &frame).unwrap();
    let transfer = command.geometry_transfer().expect("eligible large copy");
    command.reject_graphics_allocation.set(true);
    let error = command.finish().unwrap_err();
    assert_eq!(
        error.device_failure().unwrap().class(),
        RenderDeviceFailureClass::OutOfMemory
    );
    assert!(!transfer.commands.was_submitted());
    assert!(!command.retirement_incomplete());
    drop(command);
    drop(resources);
    assert_eq!(renderer.device_state(), RenderDeviceState::Ready);
    assert_eq!(
        renderer.render(&frame).unwrap().rgba,
        [0, 255, 0, 255].repeat(32 * 24)
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}
