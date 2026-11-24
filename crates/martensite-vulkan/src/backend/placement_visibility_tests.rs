use super::*;

#[test]
fn layout_keeps_partial_blocks_separate_at_segment_boundaries() {
    let layout = CompactionLayout::new(141, &[(0, 70), (70, 71)]).unwrap();
    assert_eq!(layout.segments[0].first_count_word, 705);
    assert_eq!(layout.segments[1].first_count_word, 707);
    assert_eq!(layout.size, 709 * 4);
    assert!(CompactionLayout::new(141, &[(0, 70), (69, 72)]).is_none());
    assert!(CompactionLayout::new(141, &[(0, 141), (141, 0)]).is_none());
    assert!(CompactionLayout::new(141, &[(0, 140)]).is_none());
    assert!(CompactionLayout::new(u32::MAX, &[(0, u32::MAX)]).is_none());
}

#[test]
fn candidate_indices_must_address_an_existing_draw_table_entry() {
    let mut candidates = [0; CANDIDATE_BYTES];
    assert_eq!(candidate_count(&candidates, ENTRY_BYTES as u64).unwrap(), 1);
    assert!(candidate_count(&candidates, 0).is_err());
    candidates[44..48].copy_from_slice(&1_u32.to_ne_bytes());
    assert!(candidate_count(&candidates, ENTRY_BYTES as u64).is_err());
}

#[cfg(not(feature = "command-state-diagnostics"))]
mod device {
    use super::*;
    use crate::backend::{
        host_upload_pages::FrameStoragePool,
        resource_lifecycle_test_support::create_resource_renderer,
        storage_descriptor::StorageDescriptorLayout, submission::CommandSubmission,
    };

    const SEGMENTS: [(u32, u32); 2] = [(0, 70), (70, 71)];

    #[test]
    fn compute_preserves_draw_order_and_unknown_bounds_across_partial_blocks() {
        let renderer = create_resource_renderer();
        let runtime = &renderer.runtime;
        if !runtime.placement_visibility {
            eprintln!(
                "placement visibility unavailable on {}",
                renderer.device_name()
            );
            return;
        }
        for extent in [[0; 2], [100; 2]] {
            let compaction = prepared_compaction(&renderer, extent);
            run_compaction(runtime, &compaction);
            verify_gpu_output(&compaction, extent != [0; 2]);
        }
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    fn run_compaction(
        runtime: &Arc<crate::backend::VulkanRuntime>,
        compaction: &PlacementCompaction,
    ) {
        let submission = CommandSubmission::new(runtime).unwrap();
        compaction.record_compute(submission.command);
        compaction.output_barrier(
            submission.command,
            &compaction.commands,
            vk::PipelineStageFlags2::HOST,
            vk::AccessFlags2::HOST_READ,
        );
        submission.finish().unwrap();
    }

    fn prepared_compaction(
        renderer: &crate::backend::VulkanRenderer,
        extent: [u32; 2],
    ) -> PlacementCompaction {
        let runtime = &renderer.runtime;
        let cache = FrameStoragePool::default();
        let mut pages = FrameUploadPages::storage(&cache);
        let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::VERTEX).unwrap();
        let table =
            UploadedStorage::for_frame(runtime, &layout, &table_bytes(), &mut pages).unwrap();
        let pipeline = renderer.pipeline.placement_visibility().unwrap().unwrap();
        PlacementCompaction::prepare(
            pipeline,
            &candidate_bytes(),
            &SEGMENTS,
            &table,
            (true, extent),
            &mut pages,
        )
        .unwrap()
    }

    fn table_bytes() -> Vec<u8> {
        (0..141)
            .flat_map(|_| {
                (0..48).flat_map(|word| f32::from(word < 16 && word % 5 == 0).to_ne_bytes())
            })
            .collect()
    }

    fn candidate_bytes() -> Vec<u8> {
        (0..141_u32)
            .flat_map(|index| {
                let x = if index % 3 == 1 { 2. } else { 0. };
                let known = index != 70;
                let bounds = match index {
                    // At width 100, 0.01 is a pixel center and 0.0 is a pixel boundary.
                    138 => [0.0099, 0.0099, -0.1, 1., 0.0101, 0.0101, 0.1, 0.],
                    140 => [-0.0001, -0.0001, -0.1, 1., 0.0001, 0.0001, 0.1, 0.],
                    _ => [x, -0.1, -0.1, f32::from(known), x + 0.1, 0.1, 0.1, 0.],
                };
                bounds
                    .map(f32::to_ne_bytes)
                    .into_iter()
                    .chain(candidate_draw(index).map(u32::to_ne_bytes))
                    .flatten()
            })
            .collect()
    }

    fn candidate_draw(index: u32) -> [u32; 4] {
        [index + 3, index * 3, (-(index as i32)) as u32, index]
    }

    fn verify_gpu_output(compaction: &PlacementCompaction, size_tested: bool) {
        for segment in &compaction.segments {
            for block in 0..segment.count.div_ceil(PLACEMENTS_PER_GROUP) {
                let first = segment.first + block * PLACEMENTS_PER_GROUP;
                let end = (first + PLACEMENTS_PER_GROUP).min(segment.first + segment.count);
                let expected: Vec<_> = (first..end)
                    .filter(|index| *index % 3 != 1 || *index == 70)
                    .filter(|index| !size_tested || *index != 140)
                    .collect();
                let count = read_words(compaction, segment.first_count_word + block, 1);
                assert_eq!(count, [expected.len() as u32]);
                let actual = read_words(compaction, first * COMMAND_WORDS, expected.len() * 5);
                let expected = expected
                    .into_iter()
                    .flat_map(|index| {
                        let [count, first, base, placement] = candidate_draw(index);
                        [count, 1, first, base, placement]
                    })
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
            }
        }
    }

    fn read_words(compaction: &PlacementCompaction, start: u32, words: usize) -> Vec<u32> {
        let offset = compaction.commands.offset as usize + start as usize * 4;
        let bytes = compaction
            .commands
            .buffer
            .read_at(offset, words * 4)
            .unwrap();
        bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| u32::from_ne_bytes(*word))
            .collect()
    }
}
