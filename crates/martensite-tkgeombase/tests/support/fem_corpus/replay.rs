//! Replay behavior and supporting operations.
use super::{
    fixtures::{CHUNKS, ORIGINAL_INDICES, shard_count},
    process::ShardReceipt,
};
use crate::construction_corpus::replay::checked_request;

pub fn replay_shard(shard: usize) -> ShardReceipt {
    assert!(shard < 64, "shard {shard}; expected 0..64");
    let mut actual = String::new();
    let mut rows = 0;
    for (position, (request, expected)) in original_rows().enumerate() {
        if ORIGINAL_INDICES[position] % 64 != shard {
            continue;
        }
        let family = request.split_whitespace().next().unwrap();
        actual.push_str(&checked_request(request, expected, family).unwrap());
        rows += 1;
    }
    assert_eq!(rows, shard_count(shard));
    assert!(rows > 0, "shard {shard} must execute its original requests");
    ShardReceipt {
        shard,
        rows,
        actual,
    }
}
fn original_rows() -> impl Iterator<Item = (&'static str, &'static str)> {
    assert_eq!(
        CHUNKS.iter().map(|(q, _)| q.lines().count()).sum::<usize>(),
        1230
    );
    assert_eq!(ORIGINAL_INDICES.len(), 1230);
    CHUNKS.iter().flat_map(|(q, a)| {
        assert_eq!(q.lines().count(), a.lines().count());
        q.lines().zip(a.split_inclusive('\n'))
    })
}
